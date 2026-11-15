mod event;
mod hook;
mod shim;


use std::rc::Rc;

use crate::core::{Disposition, ToggleState};
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

    fn install<F>(&mut self, listener: F, toggle_state: Rc<ToggleState>)
    where
        F: Fn(crate::core::key::Event) -> Disposition + 'static
    {
        let shim = WindowsKeyCodeShim::new(listener, toggle_state);

        if let Err(e) = self.hook.install(Box::new(shim)) {
            println!("Failed to install WindowsHook: {e}");
        }
    }


    fn start_loop(&self) {
        let _ = self.hook.activate();
    }

}
