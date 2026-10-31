mod hook;
mod shim;


use crate::core::Disposition;
use crate::platform::EventLoop as IEventLoop;
use crate::platform::windows::event_loop::{
    hook::WindowsHook,
    shim::WindowsKeyCodeShim,
};


pub struct EventLoop;

impl EventLoop {
    pub fn new() -> Self {
        Self
    }
}

impl IEventLoop for EventLoop {

    fn start<F>(self, listener: F)
    where
        F: Fn(crate::core::key::Event) -> Disposition + 'static
    {
        let mut hook = WindowsHook::new();

        if let Err(e) = hook.install(Box::new(WindowsKeyCodeShim::new(listener))) {
            println!("Failed to install WindowsHook: {e}");
        }

        let _ = hook.activate();
    }

}
