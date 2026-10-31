use std::fmt;
use std::str;

use super::{Key, Modifier};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Chord {
    pub(crate) key: Key,
    pub(crate) mods: Modifier,
}

impl Chord {

    pub(crate) fn new(key: Key, mods: Modifier) -> Chord {
        Self { key, mods }
    }


    pub(crate) fn append(&mut self, modifier: Modifier) {
        self.mods |= modifier
    }

}

impl From<Key> for Chord {

    fn from(key: Key) -> Self {
        Chord::new(key, Modifier::empty())
    }

}

impl str::FromStr for Chord {
    type Err = String;

    fn from_str(spec: &str) -> Result<Self, Self::Err> {
        let Some((mods_str, key_str)) = spec.rsplit_once('-') else {
            return Err("Invalid key spec.".to_string());
        };

        let key = key_str.parse()?;
        let mods = mods_str.split('-').into_iter().collect();

        Ok(Chord::new(key, mods))
    }

}

impl fmt::Display for Chord {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.mods, self.key)?;

        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_char() {
        assert_eq!(Chord::new(Key::Char('f'), Modifier::CTRL).to_string(), "C-f");
    }


    #[test]
    fn keyspec_to_chord() {
        assert_eq!("C-f".parse::<Chord>().unwrap(), Chord::new(Key::Char('f'), Modifier::CTRL));

        let mods = Modifier::CTRL | Modifier::ALT | Modifier::SHIFT;
        assert_eq!("C-M-S-n".parse::<Chord>().unwrap(), Chord::new(Key::Char('n'), mods));
    }
}
