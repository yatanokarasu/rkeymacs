pub(crate) mod key;
pub(crate) mod command;
pub(crate) mod plan;
pub(crate) mod config;


use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::core::command::{Command, Intent};
use crate::core::config::keymap::{KeyMaps, Resolution};
use crate::core::key::{Chord, Event};
use crate::platform::Platform;


pub(crate) enum Disposition {
    Passthrough,
    Swallow
}


pub(crate) struct ToggleState {
    enabled_rkeymacs: Cell<bool>,
}

impl ToggleState {

    pub fn current_state(&self) -> bool {
        self.enabled_rkeymacs.get()
    }


    fn switch_toggle(&self) {
        self.enabled_rkeymacs.set(!self.enabled_rkeymacs.get())
    }

}

impl Default for ToggleState {

    fn default() -> Self {
        Self { enabled_rkeymacs: Cell::new(true) }
    }

}


pub(crate) struct Engine<P: Platform> {
    keymaps: RefCell<KeyMaps>,
    platform: P,
    toggle_state: Rc<ToggleState>,
}

impl<P: Platform> Engine<P> {

    pub fn new(keymaps: KeyMaps, platform: P, toggle_state: Rc<ToggleState>) -> Self {
        Self {
            keymaps: RefCell::new(keymaps),
            platform,
            toggle_state,
        }
    }


    pub fn on_key(&self, event: Event) -> Disposition {
        let chord = event.chord();
        println!("Engine::execute(event = {})", chord);

        match self.resolve(&chord) {
            Resolution::Execute(command) => self.execute(Intent{ command, chord }),
            Resolution::Nop => {
                println!("No binding");
                Disposition::Passthrough
            },
            Resolution::Intercept => {
                Disposition::Swallow
            },
        }
    }


    fn resolve(&self, chord: &Chord) -> Resolution {
        // let foreground_app_name = self.platform.active_application_name();
        let keymaps_as_ref_mut = self.keymaps.borrow_mut();

        keymaps_as_ref_mut.resolve("", &chord)
    }


    fn execute(&self, intent: Intent) -> Disposition {
        println!("Bind to {}", intent.command);

        match intent.command {
            Command::ToggleRkeymacs => {
                self.toggle_state.switch_toggle();
                return Disposition::Swallow
            },
            _ => self.platform.execute(intent)
        }
    }

}
