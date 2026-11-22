pub mod event;
mod hook;
mod shim;
mod state;


use crate::core::Disposition;
use crate::platform::EventLoop as IEventLoop;
use crate::platform::windows::event_loop::{
    hook::WindowsHook,
    shim::WindowsKeyCodeShim,
};


pub struct EventLoop {
    hook: WindowsHook,
}

impl EventLoop {
    pub fn new() -> Self {
        Self {
            hook: WindowsHook::new(),
        }
    }
}

impl IEventLoop for EventLoop {

    fn install<F>(&mut self, listener: F)
    where
        F: Fn(crate::core::key::Chord) -> Disposition + 'static
    {
        let shim = WindowsKeyCodeShim::new(listener);

        if let Err(e) = self.hook.install(Box::new(shim)) {
            println!("Failed to install WindowsHook: {e}");
        }
    }


    fn start_loop(&self) {
        let _ = self.hook.activate();
    }

}
