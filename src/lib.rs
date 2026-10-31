mod core;
mod platform;


use core::Engine;
use core::config::RawConfig;

use crate::core::Disposition;
use crate::platform::EventLoop;



pub struct Runtime;


impl Runtime {

    pub const NAME: &str = "rusteymacs";

    pub const SERIAL_ID: usize = 0xDEAD_FACE;


    pub fn start() {
        println!("Start Runtime::start()");
        let config = RawConfig::load();
        let keymaps = config.into_keymaps();

        let (platform, event_loop) = platform::new_instance();

        let engine = Engine::new(keymaps, platform);
        event_loop.start(move |event| {
            if event.injected {
                return Disposition::Passthrough
            }

            engine.on_key(event)
        });
    }

}
