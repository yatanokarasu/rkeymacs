use std::rc::Rc;

use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState,
    VIRTUAL_KEY,
    VK_CONTROL, VK_MENU, VK_SHIFT
};
use windows::Win32::UI::WindowsAndMessaging::HC_ACTION;

use crate::core::{Disposition, ToggleState};
use crate::core::key::{Chord, Event, Key, Modifier};
use crate::platform::windows::event_loop::event::{EventType, LowKeyEvent};
use crate::platform::windows::event_loop::hook::LowLevelHookHandler;
use crate::platform::windows::keyboard::{CanonicalizedInput, Keyboard};


pub struct WindowsKeyCodeShim<L> {
    delegate: L,
    keyboard: Keyboard,
    toggle_state: Rc<ToggleState>,
}

impl<L> WindowsKeyCodeShim<L>
where
    L: Fn(Event) -> Disposition
{

    pub fn new(delegate: L, toggle_state: Rc<ToggleState>) -> Self {
        Self{
            delegate,
            keyboard: Keyboard::new(),
            toggle_state,
        }
    }


    fn is_rkeymacs_enabled(&self) -> bool {
        self.toggle_state.current_state()
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


    fn canonicalize_event(&self, vk: u16) -> Option<Event> {
        let key = self.keyboard.resolve(vk)?;

        match key {
            CanonicalizedInput::Modiifier(_) => None,
            CanonicalizedInput::Key(key) => {
                match self.get_modifiers() {
                    mods if !mods.is_empty() => Some(Event {
                        key,
                        mods,
                    }),
                    _ => None,
                }
            },
        }
    }

}

impl<L> LowLevelHookHandler for WindowsKeyCodeShim<L>
where
    L: Fn(Event) -> Disposition,
{

    fn handle(&self, code: i32, wparam: WPARAM, lparam: LPARAM) -> Disposition {
        if code != HC_ACTION as i32 {
            return Disposition::Passthrough
        }

        let event = LowKeyEvent::new(wparam, lparam);

        if event.is_injected() {
            println!("***event injected***: {}", event);

            return Disposition::Passthrough
        }

        if event.kind == EventType::KeyUp {
            return Disposition::Passthrough
        }

        match self.canonicalize_event(event.vk) {
            Some(event) => {
                if self.is_rkeymacs_enabled() || event.chord() == Chord::new(Key::Char('q'), Modifier::CTRL) {
                    return (self.delegate)(event)
                }

                println!("Disabled rkeymacs.");

                return Disposition::Passthrough
            },
            None        => Disposition::Passthrough,
        }
    }

}

