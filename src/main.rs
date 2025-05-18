mod config;
mod wayland;

use core::{fmt::Write, ops::AddAssign, time::Duration};
use std::{
    env,
    sync::{LazyLock, mpsc},
    time::Instant,
};

use gettextrs::{gettext, ngettext};
use log::{error, info};
use single_instance::SingleInstance;

const APP_ID: &str = "io.github.zefr0x.ianny";

static CONFIG: LazyLock<config::Config> = LazyLock::new(|| {
    let config = config::Config::load();

    info!("{:?}", &config);

    config
});

/// Display a break notification for specific duration than return the real system time it toke
/// while displaying this notification.
fn show_break_notification(
    break_time: Duration,
    notification_sound_hint: notify_rust::Hint,
) -> Duration {
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
    }

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

    let mut last_time = Instant::now();
    let mut accumulative_time = Duration::from_secs(0);
    #[expect(clippy::cast_precision_loss, reason = "Working with small numbers")]
    let step =
        CONFIG.notification.minimum_update_delay as f64 / break_time.as_secs_f64() * 100.0_f64;
    let step_duration = Duration::from_secs(CONFIG.notification.minimum_update_delay);

    let mut i: f64 = 0.0;

    #[expect(clippy::while_float, reason = "Precision is not an issue")]
    while i < 100.0_f64 {
        std::thread::sleep(step_duration);
        let last_time_copy = last_time;
        last_time = Instant::now();
        let time_diff = Instant::now().duration_since(last_time_copy);

        accumulative_time += time_diff;

        i += step * time_diff.div_duration_f64(step_duration);

        if CONFIG.notification.show_progress_bar {
            // FIX: Floating point problems leads to update when not needed.
            // HACK: The f64 data type is used to minimize the impact.
            #[expect(clippy::cast_possible_truncation, reason = "Truncation is intentional")]
            if (i as i32) != ((i - step) as i32) {
                // Progress bar update
                handle.hint(Hint::CustomInt("value".to_owned(), i as i32));
            }
        }

        handle.update();
    }

    handle.close();

    accumulative_time
}

fn main() -> ! {
    simple_logger::SimpleLogger::new().init().unwrap();

    // Check if the app is already running
    let app_instance = SingleInstance::new(APP_ID).unwrap();
    if !app_instance.is_single() {
        error!("{APP_ID} is already running.");
        std::process::exit(1);
    }

    // Find and load locale
    let app_lang = gettextrs::setlocale(
        gettextrs::LocaleCategory::LcAll,
        env::var("LC_ALL").unwrap_or_else(|_| {
            env::var("LC_CTYPE").unwrap_or_else(|_| env::var("LANG").unwrap_or_default())
        }),
    )
    .expect("Failed to set locale, please use a valid system locale and make sure it's enabled");
    gettextrs::textdomain(APP_ID).unwrap();
    // FIX: Also support /usr/local/share/locale/
    gettextrs::bindtextdomain(APP_ID, "/usr/share/locale").unwrap();
    gettextrs::bind_textdomain_codeset(APP_ID, "UTF-8").unwrap();

    info!("Application locale: {}", String::from_utf8_lossy(&app_lang));

    // Sync channel to share the idle/active state with the timer
    //
    // NOTE: Both idle and resume can happen during a break or timer pause,
    // so we need to buffer two messages in order to catch both.
    // Also we should guarantee that the main thread is not blocked
    // (only buffer two messages, and drop any new ones till processed),
    // and we must handle both messages sequentially before catching new pair
    // (one idle signal must be followed by at least one resume signal).
    // By limiting the buffer to two messages we also avoid repeating the
    // timer loop cycle for an already resumed idle state.
    let (signal_sender, signal_receiver) = mpsc::sync_channel(2);

    // Timer thread
    std::thread::spawn(move || -> ! {
        let pause_duration = core::cmp::min(
            gcd::binary_u64(
                CONFIG.timer.short_break_timeout,
                CONFIG.timer.long_break_timeout,
            ), // Calculate GCD
            u64::from(CONFIG.timer.idle_timeout) + 1, // NOTE: Extra one second to make sure
        ); // seconds

        let mut short_time_pased = 0; // seconds
        let mut long_time_pased = 0; // seconds
        let mut last_time = Instant::now();

        // TODO: Handle separate idle timeout for both long and short timers.

        // Timer loop.
        loop {
            std::thread::sleep(Duration::from_secs(pause_duration));
            // NOTE: Get around freezing after calculating time_diff and
            // before resetting last_time. Since the time between will
            // be dropped without having it in the next calculations.
            let last_time_copy = last_time;
            last_time = Instant::now();

            let time_diff = Instant::now().duration_since(last_time_copy).as_secs();

            if time_diff - pause_duration >= u64::from(CONFIG.timer.idle_timeout) {
                long_time_pased = 0;
                short_time_pased = 0;
                last_time = Instant::now();

                info!("Timer resetted since idle happend while process was suspended");
            } else {
                short_time_pased.add_assign(time_diff);
                long_time_pased.add_assign(time_diff);
            }

            if signal_receiver.try_recv() == Ok(wayland::Signal::Idled) {
                // Wait for change, tell user resume from idle.
                loop {
                    if signal_receiver.recv() == Ok(wayland::Signal::Resumed) {
                        // Clean the channel from any other event.
                        while signal_receiver.try_recv().is_ok() {}

                        // Reset timers.
                        long_time_pased = 0;
                        short_time_pased = 0;
                        last_time = Instant::now();

                        info!("Timer resetted");
                        break;
                    }
                }
            } else if long_time_pased >= CONFIG.timer.long_break_timeout {
                info!("Long break starts");

                show_break_notification(
                    Duration::from_secs(CONFIG.timer.long_break_duration),
                    notify_rust::Hint::SoundName("suspend-error".to_owned()), // Name or file
                );

                info!("Long break ends");

                // Reset timers.
                long_time_pased = 0;
                short_time_pased = 0;
                last_time = Instant::now();
            } else if short_time_pased >= CONFIG.timer.short_break_timeout {
                info!("Short break starts");

                if show_break_notification(
                    Duration::from_secs(CONFIG.timer.short_break_duration),
                    notify_rust::Hint::SoundName("suspend-error".to_owned()), // Name or file
                )
                .as_secs()
                    - CONFIG.timer.short_break_duration
                    >= u64::from(CONFIG.timer.idle_timeout)
                {
                    long_time_pased = 0;

                    info!("Long break timer resetted since idle happend during short break");
                }

                info!("Short break ends");

                // Reset timer.
                short_time_pased = 0;
                last_time = Instant::now();
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

    // TODO: Make it a single threaded application.

    // Main loop.
    loop {
        event_queue
            .blocking_dispatch(&mut state)
            .expect("Failed to block waiting for events and dispatch them");
    }
}
