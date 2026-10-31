use windows::Win32::UI::Input::KeyboardAndMouse::{
    VIRTUAL_KEY, VK_0, VK_9, VK_A, VK_CONTROL, VK_DOWN, VK_END, VK_HOME, VK_LEFT, VK_LWIN, VK_MENU, VK_RIGHT, VK_SHIFT, VK_SPACE, VK_UP, VK_Z
};

use crate::core::key::{Key, Modifier};


pub(in crate::platform::windows) trait VirtualKeyMapper {

    fn oem_char(&self, vk: u16) -> Option<char>;

    fn key_from_vk(&self, vk: u32) -> Option<Key> {
        let vk = u16::try_from(vk).ok()?;

        if let Some(named_key) = self.parse_named_key(vk) {
            return Some(named_key);
        }

        Some(match vk {
            v if self.is_alphabet(v)    => Key::Char((v as u8).to_ascii_lowercase() as char),
            v if self.is_number(v)      => Key::Char(v as u8 as char),

            others => Key::Char(self.oem_char(others)?),
        })
    }


    fn is_alphabet(&self, vk: u16) -> bool {
        (VK_A.0..=VK_Z.0).contains(&vk)
    }

    fn is_number(&self, vk: u16) -> bool {
        (VK_0.0..=VK_9.0).contains(&vk)
    }

    fn parse_named_key(&self, vk: u16) -> Option<Key> {
        Some(match vk {
            _ if vk == VK_HOME.0    => Key::Home,
            _ if vk == VK_END.0     => Key::End,
            _ if vk == VK_SPACE.0   => Key::Space,
            _ => return None,
        })
    }

}


pub(in crate::platform::windows) enum Keyboard {
    Jis,
    Us,
}

impl Keyboard {

    pub fn new() -> Self {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout;
        use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

        let keyboard_handle = unsafe {
            let window_handle: HWND = GetForegroundWindow();
            let thread_id = GetWindowThreadProcessId(window_handle, None);

            GetKeyboardLayout(thread_id)
        };

        let lang_id = (keyboard_handle.0 as usize & 0xFFFF) as u16;

        if lang_id == 0x_0411 {
            Keyboard::Jis
        } else {
            Keyboard::Us
        }
    }

}

impl VirtualKeyMapper for Keyboard {

    fn oem_char(&self, _vk: u16) -> Option<char> {
        None
    }
}


// struct UsKeyboard;
// struct JisKeyboard;


pub trait VirtualKeyConvertable {
    fn to_vk(self) -> Option<VIRTUAL_KEY>;
}

impl VirtualKeyConvertable for Key {
    fn to_vk(self) -> Option<VIRTUAL_KEY> {
        Some(match self {
            Key::Char(c) if c.is_ascii_alphabetic() => VIRTUAL_KEY(c.to_ascii_uppercase() as u16),

            Key::Right  => VK_RIGHT,
            Key::Left   => VK_LEFT,
            Key::Down   => VK_DOWN,
            Key::Up     => VK_UP,

            Key::Home   => VK_HOME,
            Key::End    => VK_END,

            Key::Space  => VK_SPACE,

            _ => return None,
        })
    }
}

impl VirtualKeyConvertable for Modifier {
    fn to_vk(self) -> Option<VIRTUAL_KEY> {
        Some(match self {
            Modifier::CTRL  => VK_CONTROL,
            Modifier::SHIFT => VK_SHIFT,
            Modifier::ALT   => VK_MENU,
            Modifier::WIN   => VK_LWIN,
            _               => return None,
        })
    }
}
