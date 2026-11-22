mod core;
mod platform;


use core::engine::Engine;
use core::config::RawConfig;

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

        let engine = Engine::new(
            keymaps,
            platform,
        );
        event_loop.install(move |chord| engine.on_key(chord));
        event_loop.start_loop()
    }

}
