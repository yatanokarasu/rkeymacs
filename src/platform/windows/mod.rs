mod handler;
mod keyboard;
mod event_loop;
mod platform;

pub(in crate::platform) use platform::Platform;
pub(crate) use event_loop::EventLoop;


pub(in crate::platform) fn new_instance() -> (Platform, EventLoop) {
    (
        Platform::new(),
        EventLoop::new(),
    )
}
