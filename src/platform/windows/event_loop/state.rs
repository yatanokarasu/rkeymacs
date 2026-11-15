use std::cell::RefCell;

use windows::Win32::UI::Input::KeyboardAndMouse::VK_MENU;

use crate::core::key::Modifier;
use crate::platform::windows::event_loop::event::{Kind, LowKeyEvent};
use crate::platform::windows::handler::compiler::ToWindowsInput;
use crate::platform::windows::event_loop::shim::HookOutcome;


pub struct AltKeyStateMachine {
    state: RefCell<AltKeyState>,
}

impl Default for AltKeyStateMachine {

    fn default() -> Self {
        Self {
            state: RefCell::new(AltKeyState::default()),
        }
    }

}

impl AltKeyStateMachine {

    pub fn process(&self, event: LowKeyEvent) -> HookOutcome {
        let mut state = self.state.borrow_mut();
        let (next_state, outcome) = state.process(event);

        *state = next_state;
        outcome
    }

}


enum AltKeyState {
    Inactive(InactiveStateAction),
    Active(ActiveStateAction),
    ActiveWithOthers(ActiveWithOthersStateAction),
}

impl Default for AltKeyState {

    fn default() -> Self {
        Self::Inactive(InactiveStateAction)
    }

}

impl AltKeyState {

    pub fn to_inactive(outcome: HookOutcome) -> (Self, HookOutcome) {
        (Self::Inactive(InactiveStateAction), outcome)
    }


    pub fn to_active(outcome: HookOutcome) -> (Self, HookOutcome) {
        (Self::Active(ActiveStateAction), outcome)
    }


    pub fn to_active_with_others(outcome: HookOutcome) -> (Self, HookOutcome) {
        (Self::ActiveWithOthers(ActiveWithOthersStateAction), outcome)
    }


    pub fn process(&self, event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        use AltKeyState::*;

        match self {
            Inactive(action) => action.process(event),
            Active(action) => action.process(event),
            ActiveWithOthers(action) => action.process(event),
        }
    }

}


trait AltStateAction {
    fn process(&self, event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        match event.kind() {
            Kind::KeyDown  => self.key_down(event),
            Kind::KeyUp    => self.key_up(event),
            Kind::ModsDown => self.mods_down(event),
            Kind::ModsUp   => self.mods_up(event),
        }
    }

    fn key_down(&self, event: LowKeyEvent) -> (AltKeyState, HookOutcome);
    fn key_up(&self, event: LowKeyEvent) -> (AltKeyState, HookOutcome);
    fn mods_down(&self, event: LowKeyEvent) -> (AltKeyState, HookOutcome);
    fn mods_up(&self, event: LowKeyEvent) -> (AltKeyState, HookOutcome);
}


struct InactiveStateAction;
impl AltStateAction for InactiveStateAction {

    fn key_down(&self, event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        let active_mods = event.active_modifiers();

        if active_mods == Modifier::empty() || active_mods == Modifier::SHIFT {
            AltKeyState::to_inactive(HookOutcome::Passthrough)
        } else {
            AltKeyState::to_inactive(HookOutcome::Propagate(event.to_raw_chord()))
        }
    }


    fn key_up(&self, _event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        AltKeyState::to_inactive(HookOutcome::Passthrough)
    }


    fn mods_down(&self, event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        if event.active_modifiers() == Modifier::ALT {
            println!("  **alt state change**: inactive -> active");

            AltKeyState::to_active(HookOutcome::Swallow)
        } else {
            AltKeyState::to_inactive(HookOutcome::Passthrough)
        }
    }


    fn mods_up(&self, _event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        AltKeyState::to_inactive(HookOutcome::Passthrough)
    }

}


struct ActiveStateAction;
impl AltStateAction for ActiveStateAction {

    fn key_down(&self, event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        let mut raw_chord = event.to_raw_chord();
        raw_chord.mods |= Modifier::ALT;

        println!("  **state change**: active -> inactive");
        AltKeyState::to_inactive(HookOutcome::Propagate(raw_chord))
    }


    fn key_up(&self, _event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        AltKeyState::to_active(HookOutcome::Passthrough)
    }


    fn mods_down(&self, event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        if event.mods == Modifier::ALT {
            AltKeyState::to_active(HookOutcome::Passthrough)
        } else {
            println!("  **state change**: active -> with others");

            AltKeyState::to_active_with_others(
                HookOutcome::InjectAndPassthrough(vec![VK_MENU.key_down()])
            )
        }
    }


    fn mods_up(&self, event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        if event.active_modifiers() == Modifier::ALT {
            println!("  **state change**: active -> inactive");

            AltKeyState::to_inactive(HookOutcome::InjectAndSwallow(
                vec![VK_MENU.key_down(), VK_MENU.key_up()]
            ))
        } else {
            AltKeyState::to_active(HookOutcome::Passthrough)
        }
    }

}


struct ActiveWithOthersStateAction;
impl AltStateAction for ActiveWithOthersStateAction {

    fn key_down(&self, event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        let mut raw_chord = event.to_raw_chord();
        raw_chord.mods |= Modifier::ALT;

        println!("  **state change**: with others -> inactive");
        AltKeyState::to_inactive(HookOutcome::Propagate(raw_chord))
    }


    fn key_up(&self, _event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        AltKeyState::to_active_with_others(HookOutcome::Passthrough)
    }


    fn mods_down(&self, _event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        AltKeyState::to_active_with_others(HookOutcome::Passthrough)
    }


    fn mods_up(&self, event: LowKeyEvent) -> (AltKeyState, HookOutcome) {
        if event.mods == Modifier::ALT {
            println!("  **state change**: with others -> inactive");

            AltKeyState::to_inactive(HookOutcome::Swallow)
        } else {
            AltKeyState::to_active_with_others(HookOutcome::Passthrough)
        }
    }

}
