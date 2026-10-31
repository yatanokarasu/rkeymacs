pub(crate) mod key;
pub(crate) mod command;
pub(crate) mod plan;
pub(crate) mod config;


use std::cell::RefCell;

use crate::core::command::Intent;
use crate::core::config::keymap::{KeyMaps, Resolution};
use crate::core::key::{Chord, Event};
use crate::platform::Platform;


pub(crate) enum Disposition {
    Passthrough,
    Swallow
}


pub(crate) struct Engine<P: Platform> {
    keymaps: RefCell<KeyMaps>,
    platform: P,
}

impl<P: Platform> Engine<P> {

    pub fn new(keymaps: KeyMaps, platform: P) -> Self {
        Self {
            keymaps: RefCell::new(keymaps),
            platform,
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

        self.platform.execute(intent)

        // match command {
        //     Command::SetMark => {
        //         self.state.active_mark();
        //         println!("Set mark: {:?}", self.state.mark_activated);

        //         return Disposition::Swallow;
        //     },
        //     Command::KeyboardQuit => {
        //         self.state.clear_mark();
        //         println!("Clear mark: {:?}", self.state.mark_activated);

        //         return Disposition::Swallow;
        //     }
            // _ => self.platform.execute(&command, &self.state)
        // }
    }

}
