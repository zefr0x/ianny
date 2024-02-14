use std::sync::{Arc, Condvar, Mutex};

use wayland_client::protocol::{wl_registry, wl_seat};
use wayland_protocols::ext::idle_notify::v1::client::{
    ext_idle_notification_v1, ext_idle_notifier_v1,
};
use wayland_protocols_plasma::idle::client::{org_kde_kwin_idle, org_kde_kwin_idle_timeout};

use crate::CONFIG;

enum IdleInterface {
    IdleNotifier(ext_idle_notifier_v1::ExtIdleNotifierV1),
    KdeKwinIdle(org_kde_kwin_idle::OrgKdeKwinIdle),
}

pub struct State {
    idle_interface: Option<IdleInterface>,
    // PERF: Use AtomicBool with Condvar if posibble.
    is_active: Arc<(Mutex<bool>, Condvar)>,
}

impl State {
    pub fn new() -> Self {
        Self {
            idle_interface: None,
            #[allow(clippy::mutex_atomic)]
            is_active: Arc::new((Mutex::new(true), Condvar::new())),
        }
    }

    pub fn get_is_active_arc(&self) -> Arc<(Mutex<bool>, Condvar)> {
        Arc::clone(&self.is_active)
    }
}

impl wayland_client::Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        &(): &(),
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
                // First one to be offered by the compositor will be used.
                "ext_idle_notifier_v1" => {
                    if state.idle_interface.is_none() {
                        state.idle_interface = Some(IdleInterface::IdleNotifier(
                            registry.bind::<ext_idle_notifier_v1::ExtIdleNotifierV1, _, _>(
                                name,
                                1,
                                queue_handle,
                                (),
                            ),
                        ));

                        eprintln!("Binded to ext_idle_notifier_v1");
                    }
                }
                "org_kde_kwin_idle" => {
                    if state.idle_interface.is_none() {
                        state.idle_interface = Some(IdleInterface::KdeKwinIdle(
                            registry.bind::<org_kde_kwin_idle::OrgKdeKwinIdle, _, _>(
                                name,
                                1,
                                queue_handle,
                                (),
                            ),
                        ));

                        eprintln!("Binded to org_kde_kwin_idle");
                    }
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
        &(): &(),
        _conn: &wayland_client::Connection,
        queue_handle: &wayland_client::QueueHandle<Self>,
    ) {
        if let Some(idle_interface) = &state.idle_interface {
            match idle_interface {
                IdleInterface::IdleNotifier(idle_notifier) => {
                    idle_notifier.get_idle_notification(
                        CONFIG.timer.idle_timeout * 1000, // milli seconds
                        seat,
                        queue_handle,
                        (),
                    );
                }
                IdleInterface::KdeKwinIdle(kde_kwin_idle) => {
                    kde_kwin_idle.get_idle_timeout(
                        seat,
                        CONFIG.timer.idle_timeout * 1000, // milli seconds
                        queue_handle,
                        (),
                    );
                }
            }
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

impl wayland_client::Dispatch<org_kde_kwin_idle::OrgKdeKwinIdle, ()> for State {
    fn event(
        _state: &mut Self,
        _kwin_idle: &org_kde_kwin_idle::OrgKdeKwinIdle,
        _event: org_kde_kwin_idle::Event,
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
        &(): &(),
        _conn: &wayland_client::Connection,
        _queue_handle: &wayland_client::QueueHandle<Self>,
    ) {
        let (lock, cvar) = &*state.is_active;

        match event {
            ext_idle_notification_v1::Event::Idled => {
                *lock.lock().unwrap() = false;
                cvar.notify_one();

                eprintln!("Idled");
            }
            ext_idle_notification_v1::Event::Resumed => {
                *lock.lock().unwrap() = true;
                cvar.notify_one();

                eprintln!("Resumed");
            }
            _ => {}
        }
    }
}

impl wayland_client::Dispatch<org_kde_kwin_idle_timeout::OrgKdeKwinIdleTimeout, ()> for State {
    fn event(
        state: &mut Self,
        _idle_timeout: &org_kde_kwin_idle_timeout::OrgKdeKwinIdleTimeout,
        event: org_kde_kwin_idle_timeout::Event,
        &(): &(),
        _conn: &wayland_client::Connection,
        _queue_handle: &wayland_client::QueueHandle<Self>,
    ) {
        let (lock, cvar) = &*state.is_active;

        match event {
            org_kde_kwin_idle_timeout::Event::Idle => {
                *lock.lock().unwrap() = false;
                cvar.notify_one();

                eprintln!("Idled");
            }
            org_kde_kwin_idle_timeout::Event::Resumed => {
                *lock.lock().unwrap() = true;
                cvar.notify_one();

                eprintln!("Resumed");
            }
            _ => {}
        }
    }
}
