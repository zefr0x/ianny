mod config;
mod wayland;

use core::{fmt::Write, ops::AddAssign, time::Duration};
use std::{
    env,
    sync::{mpsc, LazyLock},
};

use gettextrs::{gettext, ngettext};
use single_instance::SingleInstance;

const APP_ID: &str = "io.github.zefr0x.ianny";

static CONFIG: LazyLock<config::Config> = LazyLock::new(|| {
    let config = config::Config::load();

    eprintln!("{:?}", &config);

    config
});

fn show_break_notification(break_time: Duration, notification_sound_hint: notify_rust::Hint) {
    use notify_rust::{Hint, Notification, Timeout, Urgency};

    let minutes = break_time.as_secs() / 60;
    let seconds = break_time.as_secs() % 60;

    let mut message = gettext("Take a break for");

    if minutes != 0 {
        // FIX: Languages where number should be after the word.
        write!(
            message,
            " <b>{} {}</b>",
            minutes,
            &ngettext("minute", "minutes", u32::try_from(minutes).unwrap())
        )
        .unwrap();
    }
    if minutes != 0 && seconds != 0 {
        message += &gettext(" and");
    }
    if seconds != 0 {
        // FIX: Languages where number should be after the word.
        write!(
            message,
            " <b>{} {}</b>",
            seconds,
            &ngettext("second", "seconds", u32::try_from(seconds).unwrap())
        )
        .unwrap();
    };

    let mut handle = Notification::new()
        .summary(&gettext("Break Time!"))
        .body(&message)
        .appname(&gettext("Ianny"))
        .hint(notification_sound_hint)
        .hint(Hint::Urgency(Urgency::Critical))
        .hint(Hint::Resident(true))
        .timeout(Timeout::Never)
        .show()
        .expect("Failed to send notification");

    if CONFIG.notification.show_progress_bar {
        #[allow(clippy::cast_precision_loss)]
        let step =
            CONFIG.notification.minimum_update_delay as f64 / break_time.as_secs_f64() * 100.0_f64;
        let step_duration = Duration::from_secs(CONFIG.notification.minimum_update_delay);

        let mut i: f64 = 0.0;

        #[allow(clippy::while_float)]
        while i < 100.0_f64 {
            std::thread::sleep(step_duration);

            i += step;

            // FIX: Floating point problems leads to update when not needed.
            // HACK: The f64 data type is used to minimize the impact.
            #[allow(clippy::cast_possible_truncation)]
            if (i as i32) != ((i - step) as i32) {
                // Progress bar update
                handle.hint(Hint::CustomInt("value".to_owned(), i as i32));
                handle.update();
            }
        }
    } else {
        std::thread::sleep(break_time);
    }

    handle.close();
}

fn main() {
    // Check if the app is already running
    let app_instance = SingleInstance::new(APP_ID).unwrap();
    if !app_instance.is_single() {
        eprintln!("{APP_ID} is already running.");
        std::process::exit(1);
    }

    // Find and load locale
    let app_lang = gettextrs::setlocale(
        gettextrs::LocaleCategory::LcAll,
        env::var("LC_ALL").unwrap_or_else(|_| {
            env::var("LC_CTYPE")
                .unwrap_or_else(|_| env::var("LANG").unwrap_or_else(|_| "en_US.UTF-8".to_owned()))
        }),
    )
    .expect("Failed to set locale, please use a valid system locale and make sure it's enabled");
    gettextrs::textdomain(APP_ID).unwrap();
    // FIX: Also support /usr/local/share/locale/
    gettextrs::bindtextdomain(APP_ID, "/usr/share/locale").unwrap();
    gettextrs::bind_textdomain_codeset(APP_ID, "UTF-8").unwrap();

    eprintln!("Application locale: {}", String::from_utf8_lossy(&app_lang));

    // Sync channel to share the idle/active state with the timer
    //
    // NOTE: Both idle and resume can happen during a break or timer pause,
    // so we need to buffer two messages in order to catch both.
    // Also we should guarantee that the main thread is not blocked
    // (only buffer two messages, and drop any new ones till proccessed),
    // and we must handle both messages sequentially before catching new pair
    // (one idle signal must be followed by at least one resume signal).
    // By limiting the buffer to two messages we also avoid repeating the
    // timer loop cycle for an aleady resumed idle state.
    let (signal_sender, signal_receiver) = mpsc::sync_channel(2);

    // Timer thread
    std::thread::spawn(move || {
        let pause_duration = core::cmp::min(
            gcd::binary_u64(
                CONFIG.timer.short_break_timeout,
                CONFIG.timer.long_break_timeout,
            ), // Calculate GCD
            u64::from(CONFIG.timer.idle_timeout) + 1, // NOTE: Extra one second to make sure
        ); // secands

        let mut short_time_pased = 0; // secands
        let mut long_time_pased = 0; // secands

        // TODO: Handle separate idle timeout for both long and short timers.

        // Timer loop.
        loop {
            std::thread::sleep(Duration::from_secs(pause_duration));
            short_time_pased.add_assign(pause_duration);
            long_time_pased.add_assign(pause_duration);

            if signal_receiver
                .try_recv()
                .map_or(false, |signal| signal == wayland::Signal::Idled)
            {
                // NOTE: If both idle and resume happend right here,
                // resume will be droped and a race condition will happen in the next loop.

                // Wait for change, when user resume from idle.
                assert_eq!(signal_receiver.recv().unwrap(), wayland::Signal::Resumed);

                // Avoid race condition by cleaning the channel.
                while signal_receiver.try_recv().is_ok() {}

                // Reset timers.
                long_time_pased = 0;
                short_time_pased = 0;

                eprintln!("Timer resetted");
            } else if long_time_pased >= CONFIG.timer.long_break_timeout {
                eprintln!("Long break starts");

                show_break_notification(
                    Duration::from_secs(CONFIG.timer.long_break_duration),
                    notify_rust::Hint::SoundName("suspend-error".to_owned()), // Name or file
                );

                eprintln!("Long break ends");

                // Reset timers.
                long_time_pased = 0;
                short_time_pased = 0;
            } else if short_time_pased >= CONFIG.timer.short_break_timeout {
                eprintln!("Short break starts");

                show_break_notification(
                    Duration::from_secs(CONFIG.timer.short_break_duration),
                    notify_rust::Hint::SoundName("suspend-error".to_owned()), // Name or file
                );

                eprintln!("Short break ends");

                // Reset timer.
                short_time_pased = 0;
            }
        }
    });

    // Connect to Wayland server
    let conn = wayland_client::Connection::connect_to_env()
        .expect("Not able to detect a wayland compositor");

    let mut event_queue = conn.new_event_queue::<wayland::State>();
    let queue_handle = event_queue.handle();

    let display = conn.display();

    let _registry = display.get_registry(&queue_handle, ());

    // Create main state for the app to store shared things.
    let mut state = wayland::State::new(signal_sender);

    event_queue
        .roundtrip(&mut state)
        .expect("Failed to cause a synchronous round trip with the wayland server");

    // Main loop.
    loop {
        event_queue
            .blocking_dispatch(&mut state)
            .expect("Failed to block waiting for events and dispatch them");
    }
}
