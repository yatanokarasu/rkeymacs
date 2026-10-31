mod chord;
mod event;

pub(crate) use chord::Chord;
pub(crate) use event::Event;


use std::fmt;
use std::str;

use bitflags::bitflags;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Key {
    Char(char),

    Right,
    Left,
    Down,
    Up,

    Home,
    End,

    Space,

    F(u8),
}

impl Key {

    fn parse_single_key(key: &str) -> Result<Self, String> {
        Ok(key.chars().next().map(Key::Char).unwrap())
    }

    fn parse_function_key(key: &str) -> Result<Self, String> {
        let Some(num_str) = key.strip_prefix(&['f', 'F']) else {
            return Err(format!("Invalid function key: '{key}'."))
        };

        let function_num = num_str.parse::<u8>()
            .map_err(|_| format!("Invalid function key: '{key}'."))?;

        if (1..=12).contains(&function_num) {
            Ok(Key::F(function_num))
        } else {
            Err(format!("Invalid function key: '{key}'."))
        }
    }


    fn parse_named_key(key: &str) -> Result<Self, String> {
        Ok(match key.to_ascii_lowercase().as_str() {
            "right" => Key::Right,
            "left"  => Key::Left,
            "down"  => Key::Down,
            "up"    => Key::Up,

            "home"  => Key::Home,
            "end"   => Key::End,

            "space" => Key::Space,

            _       => return Err(format!("Unknown key: '{}'", key)),
        })
    }

}

impl str::FromStr for Key {
    type Err = String;

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        Ok(match key {
            _ if key.chars().count() == 1       => Self::parse_single_key(key)?,
            _ if key.starts_with(&['f', 'F'])   => Self::parse_function_key(key)?,
            _                                   => Self::parse_named_key(key)?,
        })
    }

}

impl fmt::Display for Key {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::Char(c)    => write!(f, "{c}"),

            Key::Right      => write!(f, "→"),
            Key::Left       => write!(f, "←"),
            Key::Down       => write!(f, "↓"),
            Key::Up         => write!(f, "↑"),

            Key::Home       => write!(f, "Home"),
            Key::End        => write!(f, "End"),

            Key::Space      => write!(f, "Space"),

            Key::F(n)       => write!(f, "F{}", n),
        }
    }

}


bitflags! {

    /// Represents modifier keys (Ctrl, Alt, Shift) as a bitmask.
    ///
    /// This struct allows for efficient representation and combination of modifier keys
    /// using bitwise operations. Each modifier is represented by a unique bit.
    ///
    /// # Examples
    ///
    /// ```text
    /// use crate::core::key::Modifier;
    ///
    /// let ctrl_alt = Modifier::CTRL | Modifier::ALT;
    /// let shift_only = Modifier::SHIFT;
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct Modifier: u8 {
        const CTRL  = 0b_0001;
        const ALT   = 0b_0010;
        const SHIFT = 0b_0100;
        const WIN   = 0b_1000;
    }

}

impl<S: AsRef<str>> FromIterator<S> for Modifier {

    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        let mut mods = Modifier::empty();

        for mod_str in iter {
            let bytes = mod_str.as_ref().as_bytes();

            if bytes.len() != 1 {
                return Modifier::empty();
            }

            mods |= match bytes[0] {
                b'C' => Modifier::CTRL,
                b'M' => Modifier::ALT,
                b'S' => Modifier::SHIFT,
                _    => Modifier::empty(),
            };
        }

        mods
    }

}

impl fmt::Display for Modifier {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contains(Modifier::CTRL)  { f.write_str("C-")? }
        if self.contains(Modifier::SHIFT) { f.write_str("S-")? }
        if self.contains(Modifier::ALT)   { f.write_str("M-")? }

        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_named_keys() {
        assert_eq!(Key::Char('f').to_string(), "f");
        assert_eq!(Key::Right.to_string(), "→");
    }


    #[test]
    fn display_modifier() {
        assert_eq!(Modifier::CTRL.to_string(), "C-");
    }

}
