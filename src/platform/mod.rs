use crate::core::Disposition;
use crate::core::command::Intent;
use crate::core::key::Event;


pub struct LowLevelInstructions<T> {
    instructions: Vec<T>,
}

impl<T> LowLevelInstructions<T> {

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }


    pub fn as_slice(&self) -> &[T] {
        &self.instructions
    }

}


pub trait Platform {
    fn execute(&self, intent: Intent) -> Disposition;

    // fn active_application_name(&self) -> Option<String>;

}


pub trait EventLoop {

    fn start<F>(self, listener: F)
    where
        F: Fn(Event) -> Disposition + 'static;

}


#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as backend;

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
compile_error!("This operating system is not supported.");

pub type TargetPlatform = backend::Platform;
pub type TargetEventLoop = backend::EventLoop;


pub fn new_instance() -> (TargetPlatform, TargetEventLoop) {
    backend::new_instance()
}
