use windows::Win32::UI::Input::KeyboardAndMouse::{INPUT, SendInput};

use crate::core::command::handler::Executor;


pub struct WindowsExecutor;

impl Executor for WindowsExecutor {
    type Input = INPUT;

    fn send(&self, instructions: &[Self::Input]) {
        unsafe {
            SendInput(instructions, size_of::<INPUT>() as i32)
        };
    }

}
