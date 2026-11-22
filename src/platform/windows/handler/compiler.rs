use windows::Win32::UI::Input::KeyboardAndMouse::INPUT;

use crate::core::command::handler::Compiler;
use crate::core::plan::Plan;
use crate::platform::windows::keyboard::input::{TapInput, ToWindowsInput};


pub struct WindowsCompiler;

impl Compiler for WindowsCompiler {
    type Input = INPUT;

    fn compile(&self, plan: Plan) -> Vec<Self::Input> {
        use crate::core::plan::Action::*;

        let mut instructions = Vec::new();

        let Plan::ToExecute(actions) = plan else {
            return instructions
        };

        for an_action in actions {
            match an_action {
                Press(chord)            => instructions.extend(chord.tap()),
                ReleaseModifier(mods)   => instructions.extend(mods.key_up()),
                RestoreModifier(mods)   => instructions.extend(mods.key_down()),
            }
        }

        instructions
    }

}
