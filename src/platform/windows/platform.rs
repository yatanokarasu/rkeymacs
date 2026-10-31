use crate::core::Disposition;
use crate::core::command::Intent;
use crate::core::command::handler::Handler;

use crate::platform::Platform as IPlatform;
use crate::platform::windows::handler::{WindowsPlanner, WindowsCompiler, WindowsExecutor};


pub struct Platform {
    handler: Handler<WindowsPlanner, WindowsCompiler, WindowsExecutor>,
}

impl Platform {

    pub fn new() -> Self {
        Self {
            handler: Handler::new(
                WindowsPlanner::default(),
                WindowsCompiler,
                WindowsExecutor,
            )
        }
    }

}

impl IPlatform for Platform {

    fn execute(&self, intent: Intent) -> Disposition {
        self.handler.handle(intent)
    }

}
