use std::sync::mpsc;

use wayland_client::{
    protocol::{wl_registry, wl_seat},
    Proxy,
};
use wayland_protocols::ext::idle_notify::v1::client::{
    ext_idle_notification_v1, ext_idle_notifier_v1,
};

use crate::CONFIG;

#[derive(Debug, Eq, PartialEq)]
pub enum Signal {
    Idled,
    Resumed,
}

type GlobalName = u32;

pub struct State {
    idle_notifier: Option<(GlobalName, ext_idle_notifier_v1::ExtIdleNotifierV1)>,
    idle_notification: Option<ext_idle_notification_v1::ExtIdleNotificationV1>,
    signal_sender: mpsc::SyncSender<Signal>,
}

impl State {
    pub const fn new(signal_sender: mpsc::SyncSender<Signal>) -> Self {
        Self {
            idle_notifier: None,
            idle_notification: None,
            signal_sender,
        }
    }
}

impl wayland_client::Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _data: &(),
        _conn: &wayland_client::Connection,
        queue_handle: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            wl_registry::Event::Global {
                name,
                interface,
                version,
            } => match interface.as_str() {
                "wl_seat" => {
                    // TODO: Support newest version of wl_seat.
                    let wl_seat = registry.bind::<wl_seat::WlSeat, _, _>(name, 1, queue_handle, ());

                    eprintln!("Binded to {}", wl_seat.id());
                }
                "ext_idle_notifier_v1" => {
                    let idle_notifier = registry
                        .bind::<ext_idle_notifier_v1::ExtIdleNotifierV1, _, _>(
                            name,
                            version,
                            queue_handle,
                            (),
                        );

                    eprintln!("Binded to {}", idle_notifier.id());

                    state.idle_notifier = Some((name, idle_notifier));
                }
                _ => {}
            },
            wl_registry::Event::GlobalRemove { name } => {
                if let Some((idle_notifier_name, idle_notifier)) = &state.idle_notifier {
                    if name == *idle_notifier_name {
                        idle_notifier.destroy();
                        state.idle_notifier = None;

                        eprintln!("Destroyed ext_idle_notifier_v1");

                        if let Some(idle_notification) = &state.idle_notification {
                            idle_notification.destroy();
                            state.idle_notification = None;

                            eprintln!("Destroyed ext_idle_notification_v1");
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

impl wayland_client::Dispatch<wl_seat::WlSeat, ()> for State {
    fn event(
        state: &mut Self,
        seat: &wl_seat::WlSeat,
        _event: wl_seat::Event,
        _data: &(),
        _conn: &wayland_client::Connection,
        queue_handle: &wayland_client::QueueHandle<Self>,
    ) {
        // FIX: Support multiseat configuration.
        if let Some((_, idle_notifier)) = &state.idle_notifier {
            let idle_timeout = CONFIG.timer.idle_timeout * 1000; // milli seconds

            let idle_notification = if CONFIG.timer.ignore_idle_inhibitors
                && idle_notifier.version()
                    > ext_idle_notifier_v1::REQ_GET_INPUT_IDLE_NOTIFICATION_SINCE
            {
                idle_notifier.get_input_idle_notification(idle_timeout, seat, queue_handle, ())
            } else {
                if CONFIG.timer.ignore_idle_inhibitors {
                    eprintln!(
                    "Failed to ignore idle inhibitors, your wayland compositor's idle notifier does not support this feature."
                );
                }

                idle_notifier.get_idle_notification(idle_timeout, seat, queue_handle, ())
            };

            eprintln!("Created {}", idle_notification.id());

            state.idle_notification = Some(idle_notification);
        }
    }
}

impl wayland_client::Dispatch<ext_idle_notifier_v1::ExtIdleNotifierV1, ()> for State {
    fn event(
        _state: &mut Self,
        _idle_notifier: &ext_idle_notifier_v1::ExtIdleNotifierV1,
        _event: ext_idle_notifier_v1::Event,
        &(): &(),
        _conn: &wayland_client::Connection,
        _queue_handle: &wayland_client::QueueHandle<Self>,
    ) {
        // No events
    }
}

impl wayland_client::Dispatch<ext_idle_notification_v1::ExtIdleNotificationV1, ()> for State {
    fn event(
        state: &mut Self,
        _idle_notification: &ext_idle_notification_v1::ExtIdleNotificationV1,
        event: ext_idle_notification_v1::Event,
        _data: &(),
        _conn: &wayland_client::Connection,
        _queue_handle: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            ext_idle_notification_v1::Event::Idled => {
                eprintln!("Idled");

                match state.signal_sender.try_send(Signal::Idled) {
                    Ok(()) | Err(mpsc::TrySendError::Full(_)) => (),
                    Err(mpsc::TrySendError::Disconnected(_)) => {
                        panic!("Timer disconnected, `Idled` signal could not be sent")
                    }
                }
            }
            ext_idle_notification_v1::Event::Resumed => {
                eprintln!("Resumed");

                match state.signal_sender.try_send(Signal::Resumed) {
                    Ok(()) | Err(mpsc::TrySendError::Full(_)) => (),
                    Err(mpsc::TrySendError::Disconnected(_)) => {
                        panic!("Timer disconnected, `Resumed` signal could not be sent")
                    }
                }
            }
            _ => {}
        }
    }
}
