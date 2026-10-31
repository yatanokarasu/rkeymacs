
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState,
    VIRTUAL_KEY,
    VK_CONTROL, VK_MENU, VK_SHIFT
};
use windows::Win32::UI::WindowsAndMessaging::{
    KBDLLHOOKSTRUCT,
    HC_ACTION, WM_KEYUP, WM_SYSKEYUP
};

use crate::Runtime;
use crate::core::Disposition;
use crate::core::key::{Event, Modifier};
use crate::platform::windows::event_loop::hook::LowLevelHookHandler;
use crate::platform::windows::keyboard::{Keyboard, VirtualKeyMapper};


pub struct WindowsKeyCodeShim<L> {
    delegate: L,
    keyboard: Keyboard,
}

impl<L> WindowsKeyCodeShim<L>
where
    L: Fn(Event) -> Disposition
{

    pub fn new(delegate: L) -> Self {
        Self{
            delegate,
            keyboard: Keyboard::new(),
        }
    }


    fn should_passthrough(&self, code: i32, wparam: WPARAM, _lparam: LPARAM) -> bool {
        if code != HC_ACTION as i32 {
            return true;
        }

        if matches!(wparam.0 as u32, WM_KEYUP | WM_SYSKEYUP) {
            return true;
        }

        false
    }


    fn is_down(&self, vk: VIRTUAL_KEY) -> bool {
        unsafe { GetAsyncKeyState(vk.0 as i32) as u16 & 0x8000 != 0 }
    }


    fn get_modifiers(&self) -> Modifier {
        let mut mods = Modifier::empty();

        if self.is_down(VK_CONTROL) { mods |= Modifier::CTRL;  }
        if self.is_down(VK_SHIFT)   { mods |= Modifier::SHIFT; }
        if self.is_down(VK_MENU)    { mods |= Modifier::ALT;   }

        mods
    }


    fn canonicalize_event(&self, kb: &KBDLLHOOKSTRUCT) -> Option<Event> {
        let key = self.keyboard.key_from_vk(kb.vkCode)?;

        match self.get_modifiers() {
            mods if !mods.is_empty() => Some(Event {
                key,
                mods,
                injected: kb.dwExtraInfo == Runtime::SERIAL_ID,
            }),
            _ => None,
        }
    }

}

impl<L> LowLevelHookHandler for WindowsKeyCodeShim<L>
where
    L: Fn(Event) -> Disposition,
{

    fn handle(&self, code: i32, wparam: WPARAM, lparam: LPARAM) -> Disposition {
        if self.should_passthrough(code, wparam, lparam) {
            return Disposition::Passthrough;
        }

        let kb = unsafe { &*(lparam.0 as *const KBDLLHOOKSTRUCT) };

        match self.canonicalize_event(kb) {
            Some(event) => (self.delegate)(event),
            None        => Disposition::Passthrough,
        }
    }

}

