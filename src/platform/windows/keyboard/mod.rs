pub(in crate::platform::windows) mod vk;
pub(in crate::platform::windows) mod input;
mod layout;


use crate::core::key::Key;
use crate::platform::windows::event_loop::event::RawChord;
use crate::platform::windows::keyboard::layout::{JisLayout, UsLayout, VirtualKeyMapper};


pub(in crate::platform::windows) enum CanonicalizedKey {
    Plain(Key),
    Shifted(Key),
}


pub(in crate::platform::windows) enum Keyboard {
    Jis(JisLayout),
    Us(UsLayout),
}

impl Keyboard {

    pub fn new() -> Self {
        use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout;
        use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

        let keyboard_handle = unsafe {
            let window_handle = GetForegroundWindow();
            let thread_id = GetWindowThreadProcessId(window_handle, None);

            GetKeyboardLayout(thread_id)
        };

        let lang_id = (keyboard_handle.0 as usize & 0xFFFF) as u16;

        if lang_id == 0x_0411 {
            Keyboard::Jis(JisLayout)
        } else {
            Keyboard::Us(UsLayout)
        }
    }


    pub fn resolve(&self, raw_chord: &RawChord) -> Option<CanonicalizedKey> {
        match self {
            Keyboard::Jis(layout)   => layout.resolve(raw_chord),
            Keyboard::Us(layout)    => layout.resolve(raw_chord),
        }
    }

}
