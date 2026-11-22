use windows::Win32::UI::Input::KeyboardAndMouse::{
    VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9,
    VK_A, VK_Z,
    VK_BACK, VK_DELETE, VK_END, VK_HOME, VK_SPACE, VK_NEXT, VK_PRIOR,
    VK_OEM_2, VK_OEM_COMMA, VK_OEM_PERIOD,
};

use super::CanonicalizedKey;
use crate::core::key::{Key, Modifier};
use crate::platform::windows::event_loop::event::RawChord;


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

        Some(CanonicalizedKey::Plain(Key::Char((vk as u8).to_ascii_lowercase() as char)))
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

        Some(CanonicalizedKey::Plain(key))
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
            _ if vk == VK_OEM_COMMA.0   => if is_shifted { Shifted(Key::Char('<')) } else { Plain(Key::Char(',')) },
            _ if vk == VK_OEM_PERIOD.0  => if is_shifted { Shifted(Key::Char('>')) } else { Plain(Key::Char('.')) },
            _ if vk == VK_OEM_2.0       => if is_shifted { Shifted(Key::Char('?')) } else { Plain(Key::Char('/')) },

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
            CanonicalizedKey::Plain(Key::Char(vk as u8 as char))
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
