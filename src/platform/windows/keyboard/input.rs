use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, KEYBD_EVENT_FLAGS, KEYBDINPUT, VIRTUAL_KEY,
    INPUT_KEYBOARD, KEYEVENTF_KEYUP,
};

use crate::Runtime;
use crate::core::key::{Chord, Modifier};

use super::vk::VirtualKeyConvertable;


pub(in crate::platform::windows) trait ToWindowsInput: Sized {
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


pub(in crate::platform::windows) trait TapInput {
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
