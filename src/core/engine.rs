use std::cell::{Cell, RefCell};

use crate::core::Disposition;
use crate::core::command::{Command, Intent};
use crate::core::keymap::{KeyMaps, Resolution};
use crate::core::key::Chord;
use crate::platform::Platform;


pub(crate) struct ToggleState {
    enabled_rkeymacs: Cell<bool>,
}

impl ToggleState {

    pub fn is_enabled(&self) -> bool {
        self.enabled_rkeymacs.get()
    }


    fn toggle(&self) {
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
    toggle_state: ToggleState,
}

impl<P: Platform> Engine<P> {

    pub fn new(keymaps: KeyMaps, platform: P) -> Self {
        Self {
            keymaps: RefCell::new(keymaps),
            platform,
            toggle_state: ToggleState::default(),
        }
    }


    pub fn on_key(&self, chord: Chord) -> Disposition {
        println!("Engine::execute(chord = {})", chord);

        match self.resolve(&chord) {
            Resolution::Execute(command) => self.execute(command, chord),
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


    fn execute(&self, command: Command, chord: Chord) -> Disposition {
        println!("Bind to {}", command);

        match command {
            Command::ToggleRkeymacs => {
                self.toggle_state.toggle();

                return Disposition::Swallow
            },
            _ if !self.toggle_state.is_enabled() => Disposition::Passthrough,
            _ => self.platform.execute(Intent { command, chord }),
        }
    }

}
