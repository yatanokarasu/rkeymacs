use std::fmt;

use super::{Key, Modifier};
use super::chord::Chord;


pub(crate) struct Event {
    pub(crate) key: Key,
    pub(crate) mods: Modifier,
    pub(crate) injected: bool,
}

impl Event {

    pub(crate) fn chord(&self) -> Chord {
        Chord::new(self.key, self.mods)
    }

}

impl fmt::Display for Event {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.chord())?;

        Ok(())
    }

}
