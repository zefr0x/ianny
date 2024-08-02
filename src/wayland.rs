use std::sync::mpsc;

use wayland_client::protocol::{wl_registry, wl_seat};
use wayland_protocols::ext::idle_notify::v1::client::{
    ext_idle_notification_v1, ext_idle_notifier_v1,
};

use crate::CONFIG;

#[derive(Debug, Eq, PartialEq)]
pub enum Signal {
    Idled,
    Resumed,
}

pub struct State {
    idle_notifier: Option<ext_idle_notifier_v1::ExtIdleNotifierV1>,
    signal_sender: mpsc::SyncSender<Signal>,
}

impl State {
    pub const fn new(signal_sender: mpsc::SyncSender<Signal>) -> Self {
        Self {
            idle_notifier: None,
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
        if let wl_registry::Event::Global {
            name, interface, ..
        } = event
        {
            match interface.as_str() {
                "wl_seat" => {
                    registry.bind::<wl_seat::WlSeat, _, _>(name, 1, queue_handle, ());
                }
                "ext_idle_notifier_v1" => {
                    state.idle_notifier = Some(
                        registry.bind::<ext_idle_notifier_v1::ExtIdleNotifierV1, _, _>(
                            name,
                            1,
                            queue_handle,
                            (),
                        ),
                    );

                    eprintln!("Binded to ext_idle_notifier_v1");
                }
                _ => {}
            }
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
        if let Some(idle_notifier) = &state.idle_notifier {
            idle_notifier.get_idle_notification(
                CONFIG.timer.idle_timeout * 1000, // milli seconds
                seat,
                queue_handle,
                (),
            );

            eprintln!("Created ext_idle_notification_v1");
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
