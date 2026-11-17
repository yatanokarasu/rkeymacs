use std::rc::Rc;

use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{SendInput, INPUT, VK_SHIFT};
use windows::Win32::UI::WindowsAndMessaging::HC_ACTION;

use crate::core::{Disposition, ToggleState};
use crate::core::key::{Chord, Modifier};
use crate::platform::windows::event_loop::event::{LowKeyEvent, RawChord};
use crate::platform::windows::event_loop::hook::LowLevelHookHandler;
use crate::platform::windows::event_loop::state::AltKeyStateMachine;
use crate::platform::windows::keyboard::{CanonicalizedKey, Keyboard};
use crate::platform::windows::keyboard::vk::ToWindowsInput;


pub(in crate::platform::windows::event_loop) enum HookOutcome {
    Passthrough,
    Swallow,
    Propagate(RawChord),
    InjectAndPassthrough(Vec<INPUT>),
    InjectAndSwallow(Vec<INPUT>),
}


pub struct WindowsKeyCodeShim<L> {
    delegate: L,
    keyboard: Keyboard,
    toggle_state: Rc<ToggleState>,
    alt_state: AltKeyStateMachine,
}

impl<L> WindowsKeyCodeShim<L>
where
    L: Fn(Chord) -> Disposition
{

    pub fn new(delegate: L, toggle_state: Rc<ToggleState>) -> Self {
        Self{
            delegate,
            keyboard: Keyboard::new(),
            toggle_state,
            alt_state: AltKeyStateMachine::default(),
        }
    }


    fn is_rkeymacs_enabled(&self) -> bool {
        self.toggle_state.current_state()
    }


    fn propagate(&self, raw_chord: RawChord) -> Disposition {
        let Some(canonicalized_key) = self.keyboard.resolve(&raw_chord) else {
            return Disposition::Passthrough
        };

        let mut mods = raw_chord.mods;

        let key = match canonicalized_key {
            CanonicalizedKey::Plane(key) => key,
            CanonicalizedKey::Shifted(key) => {
                mods.remove(Modifier::SHIFT);
                self.inject(vec![VK_SHIFT.key_down()]);

                key
            },
        };

        (self.delegate)(Chord::new(key, mods))
    }


    fn inject(&self, inputs: Vec<INPUT>) -> &Self {
        unsafe {
            SendInput(&inputs, size_of::<INPUT>() as i32)
        };

        self
    }


    fn and_passthrough(&self) -> Disposition {
        Disposition::Passthrough
    }


    fn and_swallow(&self) -> Disposition {
        Disposition::Swallow
    }

}

impl<L> LowLevelHookHandler for WindowsKeyCodeShim<L>
where
    L: Fn(Chord) -> Disposition,
{

    fn handle(&self, code: i32, wparam: WPARAM, lparam: LPARAM) -> Disposition {
        if code != HC_ACTION as i32 {
            return Disposition::Passthrough
        }

        let event = LowKeyEvent::new(wparam, lparam);
        println!("low event = {}", event);

        if event.is_injected() {
            return Disposition::Passthrough
        }

        match self.alt_state.process(event) {
            HookOutcome::Passthrough => Disposition::Passthrough,
            HookOutcome::Swallow => Disposition::Swallow,
            HookOutcome::Propagate(raw_chord) => self.propagate(raw_chord),
            HookOutcome::InjectAndPassthrough(inputs) => self.inject(inputs).and_passthrough(),
            HookOutcome::InjectAndSwallow(inputs) => self.inject(inputs).and_swallow(),
        }
    }

}

