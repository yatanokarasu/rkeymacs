mod core;
mod platform;


use core::Engine;
use core::config::RawConfig;
use std::rc::Rc;

use crate::core::ToggleState;
use crate::platform::EventLoop;



pub struct Runtime;


impl Runtime {

    pub const NAME: &str = "rusteymacs";

    pub const SERIAL_ID: usize = 0xDEAD_FACE;


    pub fn start() {
        println!("Start Runtime::start()");
        let config = RawConfig::load();
        let keymaps = config.into_keymaps();

        let (platform, mut event_loop) = platform::new_instance();
        let toggle_state = Rc::new(ToggleState::default());

        let engine = Engine::new(
            keymaps,
            platform,
            Rc::clone(&toggle_state),
        );
        event_loop.install(move |chord| engine.on_key(chord), toggle_state);
        event_loop.start_loop()
    }

}
