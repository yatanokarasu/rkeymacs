use std::fmt;

use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    KBDLLHOOKSTRUCT,
    WM_KEYDOWN, WM_SYSKEYDOWN,
};

use crate::Runtime;
use crate::core::key::Modifier;
use crate::platform::windows::keyboard::FromVk;


#[derive(PartialEq)]
pub(in crate::platform::windows) enum EventType {
    KeyDown,
    KeyUp,
}


pub(in crate::platform::windows) struct LowKeyEvent {
    pub vk: u16,
    pub mods: Modifier,
    pub kind: EventType,
    pub is_injected: bool,
}


impl LowKeyEvent {

    pub fn new(wparam: WPARAM, lparam: LPARAM) -> Self {
        let kb = unsafe { *(lparam.0 as *const KBDLLHOOKSTRUCT) };

        let vk = u16::try_from(kb.vkCode).expect("vkCode exceeds u16 range");
        let mods = vk.to_modifier();
        let kind = match wparam.0 as u32 {
            WM_KEYDOWN | WM_SYSKEYDOWN => EventType::KeyDown,
            _                          => EventType::KeyUp,
        };
        let is_injected = kb.dwExtraInfo == Runtime::SERIAL_ID;

        Self {
            vk,
            mods,
            kind,
            is_injected,
        }
    }


    pub fn is_injected(&self) -> bool {
        self.is_injected
    }

}


impl fmt::Display for LowKeyEvent {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "low key event(mods={}, vk={} {}){}",
            self.mods,
            self.vk,
            match self.kind {
                EventType::KeyDown => "key down",
                EventType::KeyUp   => "key up",
            },
            match self.is_injected {
                true  => "[injected]",
                false => "",
            }

        );

        Ok(())
    }

}
