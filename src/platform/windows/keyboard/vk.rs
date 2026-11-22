use windows::Win32::UI::Input::KeyboardAndMouse::{
    VIRTUAL_KEY,
    VK_DOWN, VK_LEFT, VK_RIGHT, VK_UP,
    VK_BACK, VK_DELETE, VK_END, VK_HOME, VK_SPACE, VK_NEXT, VK_PRIOR,
    VK_CONTROL, VK_SHIFT, VK_MENU,
    VK_LWIN,
    VK_F1,
};

use crate::core::key::{Key, Modifier};


pub trait VirtualKeyConvertable {
    fn to_vk(self) -> Option<VIRTUAL_KEY>;
}

impl VirtualKeyConvertable for Key {
    fn to_vk(self) -> Option<VIRTUAL_KEY> {
        Some(match self {
            Key::Char(c) if c.is_ascii_alphabetic() => VIRTUAL_KEY(c.to_ascii_uppercase() as u16),

            Key::Right      => VK_RIGHT,
            Key::Left       => VK_LEFT,
            Key::Down       => VK_DOWN,
            Key::Up         => VK_UP,

            Key::Home       => VK_HOME,
            Key::End        => VK_END,

            Key::PageDown   => VK_NEXT,
            Key::PageUp     => VK_PRIOR,

            Key::Backspace  => VK_BACK,
            Key::Delete     => VK_DELETE,

            Key::Space      => VK_SPACE,

            Key::F(n) if (1..=12).contains(&n) => VIRTUAL_KEY(VK_F1.0 + u16::from(n - 1)),

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
