use windows::Win32::UI::Input::KeyboardAndMouse::{
    VIRTUAL_KEY,
    VK_0, VK_9,
    VK_A, VK_Z,
    VK_DOWN, VK_LEFT, VK_RIGHT, VK_UP,
    VK_END, VK_HOME, VK_SPACE, VK_NEXT, VK_PRIOR,
    VK_CONTROL, VK_LWIN, VK_MENU, VK_SHIFT,
    VK_OEM_COMMA, VK_OEM_PERIOD,
};

use crate::core::key::{Key, Modifier};


pub struct JisLayout;
pub struct UsLayout;


pub(in crate::platform::windows) trait VirtualKeyMapper {

    fn specific_oem_char(&self, vk: u16) -> Option<char>;


    fn oem_char(&self, vk: u16) -> Option<char> {
        Some(match vk {
            _ if vk == VK_OEM_COMMA.0   => ',',
            _ if vk == VK_OEM_PERIOD.0  => '.',

            others => self.specific_oem_char(others)?,
        })
    }


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
            _ if vk == VK_NEXT.0    => Key::PageDown,
            _ if vk == VK_PRIOR.0   => Key::PageUp,

            _ => return None,
        })
    }

}

impl VirtualKeyMapper for JisLayout {

    fn specific_oem_char(&self, _vk: u16) -> Option<char> {
        None
    }

}

impl VirtualKeyMapper for UsLayout {

    fn specific_oem_char(&self, _vk: u16) -> Option<char> {
        None
    }

}


pub(in crate::platform::windows) enum Keyboard {
    Jis(JisLayout),
    Us(UsLayout),
}

impl Keyboard {

    pub fn new() -> Self {
        use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout;
        use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

        let keyboard_handle = unsafe {
            let window_handle = GetForegroundWindow();
            let thread_id = GetWindowThreadProcessId(window_handle, None);

            GetKeyboardLayout(thread_id)
        };

        let lang_id = (keyboard_handle.0 as usize & 0xFFFF) as u16;

        if lang_id == 0x_0411 {
            Keyboard::Jis(JisLayout)
        } else {
            Keyboard::Us(UsLayout)
        }
    }


    pub fn key_from_vk(&self, vk: u32) -> Option<Key> {
        match self {
            Keyboard::Jis(layout)   => layout.key_from_vk(vk),
            Keyboard::Us(layout)    => layout.key_from_vk(vk),
        }
    }

}


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

            Key::PageDown   => VK_NEXT,
            Key::PageUp     => VK_PRIOR,

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
