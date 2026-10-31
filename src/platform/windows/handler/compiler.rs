use windows::Win32::UI::Input::KeyboardAndMouse::{INPUT, INPUT_0, KEYBD_EVENT_FLAGS, KEYBDINPUT, VIRTUAL_KEY};
use windows::Win32::UI::Input::KeyboardAndMouse::{INPUT_KEYBOARD, KEYEVENTF_KEYUP};

use crate::Runtime;
use crate::core::command::handler::Compiler;
use crate::core::key::{Chord, Modifier};
use crate::core::plan::Plan;
use crate::platform::LowLevelInstructions;
use crate::platform::windows::keyboard::VirtualKeyConvertable;


trait ToWindowsInput: Sized {
    type Output;

    fn into_input(self, is_key_up: bool) -> Self::Output;

    fn key_down(self) -> Self::Output {
        self.into_input(false)
    }

    fn key_up(self) -> Self::Output {
        self.into_input(true)
    }
}


impl ToWindowsInput for VIRTUAL_KEY {
    type Output = INPUT;

    fn into_input(self, is_key_up: bool) -> INPUT {
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: self,
                    wScan: 0,
                    dwFlags: if is_key_up {
                        KEYEVENTF_KEYUP
                    } else {
                        KEYBD_EVENT_FLAGS(0)
                    },
                    time: 0,
                    dwExtraInfo: Runtime::SERIAL_ID,
                },
            },
        }
    }

}

impl ToWindowsInput for Modifier {
    type Output = Vec<INPUT>;

    fn into_input(self, is_key_up: bool) -> Self::Output {
        self.into_iter()
            .filter_map(Modifier::to_vk)
            .map(|mods_vk| mods_vk.into_input(is_key_up))
            .collect()
    }
}


trait TapInput {
    fn tap(self) -> Vec<INPUT>;
}

impl TapInput for Chord {

    fn tap(self) -> Vec<INPUT> {
        let mut vec = Vec::new();
        let Chord { key, mods } = self;

        if let Some(vk) = key.to_vk() {
            vec.extend(mods.key_down());
            vec.push(vk.key_down());
            vec.push(vk.key_up());
            vec.extend(mods.key_up());
        }

        vec
    }

}


pub struct WindowsCompiler;

impl Compiler for WindowsCompiler {
    type Input = INPUT;

    fn compile(&self, plan: Plan) -> LowLevelInstructions<Self::Input> {
        use crate::core::plan::Action::*;

        let Plan::ToExecute(actions) = plan else {
            return LowLevelInstructions { instructions: Vec::new() }
        };

        let mut instructions = Vec::<INPUT>::new();

        for an_action in actions {
            match an_action {
                Press(chord)            => instructions.extend(chord.tap()),
                ReleaseModifier(mods)   => instructions.extend(mods.key_up()),
                RestoreModifier(mods)   => instructions.extend(mods.key_down()),
            }
        }

        LowLevelInstructions {
            instructions,
        }
    }

}
