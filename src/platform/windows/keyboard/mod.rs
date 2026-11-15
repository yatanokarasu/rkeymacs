use windows::Win32::UI::Input::KeyboardAndMouse::{
    VIRTUAL_KEY,
    VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9,
    VK_A, VK_Z,
    VK_DOWN, VK_LEFT, VK_RIGHT, VK_UP,
    VK_BACK, VK_DELETE, VK_END, VK_HOME, VK_SPACE, VK_NEXT, VK_PRIOR,
    VK_CONTROL, VK_LCONTROL, VK_RCONTROL,
    VK_SHIFT, VK_LSHIFT, VK_RSHIFT,
    VK_MENU, VK_LMENU, VK_RMENU,
    VK_LWIN,
    VK_F1, VK_OEM_2, VK_OEM_COMMA, VK_OEM_PERIOD,
};

use crate::core::key::{Key, Modifier};
use crate::platform::windows::event_loop::event::RawChord;


pub(in crate::platform::windows) enum CanonicalizedKey {
    Plane(Key),
    Shifted(Key),
}


pub struct JisLayout;
pub struct UsLayout;


pub(in crate::platform::windows) trait VirtualKeyMapper {

    fn parse_specific_oem_key(&self, raw_chord: &RawChord) -> Option<CanonicalizedKey>;
    fn parse_number_key(&self, raw_chord: &RawChord) -> Option<CanonicalizedKey>;


    fn resolve(&self, raw_chord: &RawChord) -> Option<CanonicalizedKey> {
        if let Some(named_key) = self.parse_named_key(raw_chord) {
            return Some(named_key)
        }

        if let Some(alphabet) = self.parse_alphabet_key(raw_chord) {
            return Some(alphabet)
        }

        if let Some(number_or_symbol) = self.parse_number_key(raw_chord) {
            return Some(number_or_symbol)
        }

        self.parse_oem_key(raw_chord)
    }


    fn parse_alphabet_key(&self, raw_chord: &RawChord) -> Option<CanonicalizedKey> {
        let &RawChord { vk, mods: _mods } = raw_chord;
        let is_alphabet = (VK_A.0..=VK_Z.0).contains(&vk);

        if !is_alphabet {
            return None
        }

        Some(CanonicalizedKey::Plane(Key::Char((vk as u8).to_ascii_lowercase() as char)))
    }


    fn parse_named_key(&self, raw_chord: &RawChord) -> Option<CanonicalizedKey> {
        let &RawChord { vk, mods: _mods } = raw_chord;

        let key = match vk {
            _ if vk == VK_HOME.0    => Key::Home,
            _ if vk == VK_END.0     => Key::End,
            _ if vk == VK_SPACE.0   => Key::Space,
            _ if vk == VK_NEXT.0    => Key::PageDown,
            _ if vk == VK_PRIOR.0   => Key::PageUp,
            _ if vk == VK_BACK.0    => Key::Backspace,
            _ if vk == VK_DELETE.0  => Key::Delete,

            _ => return None,
        };

        Some(CanonicalizedKey::Plane(key))
    }


    fn parse_oem_key(&self, raw_chord: &RawChord) -> Option<CanonicalizedKey> {
        if let Some(common_oem_key) = self.parse_common_oem_key(raw_chord) {
            return Some(common_oem_key)
        }

        self.parse_specific_oem_key(raw_chord)
    }


    fn parse_common_oem_key(&self, raw_chord: &RawChord) -> Option<CanonicalizedKey> {
        let &RawChord { vk, mods } = raw_chord;
        let is_shifted = mods.contains(Modifier::SHIFT);

        use CanonicalizedKey::*;
        let canonicalized_key = match vk {
            _ if vk == VK_OEM_COMMA.0   => if is_shifted { Shifted(Key::Char('<')) } else { Plane(Key::Char(',')) },
            _ if vk == VK_OEM_PERIOD.0  => if is_shifted { Shifted(Key::Char('>')) } else { Plane(Key::Char('.')) },
            _ if vk == VK_OEM_2.0       => if is_shifted { Shifted(Key::Char('?')) } else { Plane(Key::Char('/')) },

            _ => return None,
        };

        Some(canonicalized_key)
    }

}

impl VirtualKeyMapper for JisLayout {

    fn parse_specific_oem_key(&self, _raw_chord: &RawChord) -> Option<CanonicalizedKey> {
        None
    }


    fn parse_number_key(&self, raw_chord: &RawChord) -> Option<CanonicalizedKey> {
        let &RawChord { vk, mods } = raw_chord;

        let is_shifted = mods.contains(Modifier::SHIFT);
        let is_number = (VK_0.0..=VK_9.0).contains(&vk);

        if !is_number {
            return None
        }

        let canonicalized_key = if is_shifted {
            let key = Key::Char(match vk {
                _ if vk == VK_0.0 => return None,
                _ if vk == VK_1.0 => '!',
                _ if vk == VK_2.0 => '"',
                _ if vk == VK_3.0 => '#',
                _ if vk == VK_4.0 => '$',
                _ if vk == VK_5.0 => '%',
                _ if vk == VK_6.0 => '&',
                _ if vk == VK_7.0 => '\'',
                _ if vk == VK_8.0 => '(',
                _ if vk == VK_9.0 => ')',

                _ => return None,
            });

            CanonicalizedKey::Shifted(key)
        } else {
            CanonicalizedKey::Plane(Key::Char(vk as u8 as char))
        };

        Some(canonicalized_key)
    }

}

impl VirtualKeyMapper for UsLayout {

    fn parse_specific_oem_key(&self, _raw_chord: &RawChord) -> Option<CanonicalizedKey> {
        None
    }

    fn parse_number_key(&self, _raw_chord: &RawChord) -> Option<CanonicalizedKey> {
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


    pub fn resolve(&self, raw_chord: &RawChord) -> Option<CanonicalizedKey> {
        match self {
            Keyboard::Jis(layout)   => layout.resolve(raw_chord),
            Keyboard::Us(layout)    => layout.resolve(raw_chord),
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


pub(in crate::platform::windows) trait FromVk: Copy {
    type Output;

    fn to_modifier(self) -> Self::Output;
}

impl FromVk for u16 {
    type Output = Modifier;

    fn to_modifier(self) -> Self::Output {
        let vk = self;

        let is_ctrl_down  = vk == VK_CONTROL.0 || vk == VK_LCONTROL.0 || vk == VK_RCONTROL.0;
        let is_shift_down = vk == VK_SHIFT.0   || vk == VK_LSHIFT.0   || vk == VK_RSHIFT.0;
        let is_alt_down   = vk == VK_MENU.0    || vk == VK_LMENU.0    || vk == VK_RMENU.0;

        if is_ctrl_down {
            return Modifier::CTRL
        }

        if is_shift_down {
            return Modifier::SHIFT
        }

        if is_alt_down {
            return Modifier::ALT
        }

        Modifier::empty()
    }

}
