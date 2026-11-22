use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use crate::core::command::Command;
use crate::core::key::Chord;


#[derive(Clone, Debug, PartialEq)]
pub enum KeyBind {
    Execute(Command),
    Prefix(BindTable),
}


#[derive(Debug, PartialEq)]
pub enum Resolution {
    Execute(Command),
    Nop,
    Intercept,
}


#[derive(Clone, Debug, PartialEq)]
pub struct BindTable {
    table: HashMap<Chord, KeyBind>,
}

impl BindTable {

    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }


    pub fn resolve(&self, chord: &Chord) -> Option<&KeyBind> {
        self.table.get(&chord)
    }


    pub fn insert(&mut self, mut chords: VecDeque<Chord>, command: Command) -> Result<(), String> {
        let Some(chord) = chords.pop_front() else {
            return Ok(());
        };

        if chords.is_empty() {
            self.table.insert(chord, KeyBind::Execute(command));

            return Ok(());
        }

        let key_bind = self.table
            .entry(chord)
            .or_insert_with(|| KeyBind::Prefix(BindTable::new()));

        match key_bind {
            KeyBind::Prefix(child) => child.insert(chords, command),
            _ => return Err("Key bind conflicted".to_string()),
        }
    }

}


pub struct KeyMap {
    bind_table: BindTable,
    fallback_keymap: Option<Rc<KeyMap>>,
}

impl KeyMap {

    pub fn new(bind_table: BindTable, fallback_keymap: Option<Rc<KeyMap>>) -> Self {
        Self {
            bind_table,
            fallback_keymap,
        }
    }


    pub fn resolve(&self, chord: &Chord) -> Resolution {
        if let Some(intent) = self.resolve_myself(chord) {
            return intent;
        }

        match &self.fallback_keymap {
            Some(fallback)  => fallback.resolve(chord),
            None            => Resolution::Nop,
        }
    }


    fn resolve_myself(&self, chord: &Chord) -> Option<Resolution> {
        let key_bind = self.bind_table.resolve(chord)?;

        Some(match key_bind {
            KeyBind::Execute(command)   => Resolution::Execute(command.clone()),
            KeyBind::Prefix(_)          => Resolution::Intercept,
        })
    }

}


pub struct KeyMaps {
    default_keymap: Rc<KeyMap>,
    apps_keymap: HashMap<String, KeyMap>,
}

impl KeyMaps {

    pub fn new(default_keymap: Rc<KeyMap>, apps_keymap: HashMap<String, KeyMap>) -> Self {
        Self {
            default_keymap,
            apps_keymap,
        }
    }


    pub fn resolve(&self, app_name: &str, chord: &Chord) -> Resolution {
        match self.apps_keymap.get(app_name) {
            Some(app_keymap)    => app_keymap.resolve(chord),
            None                => self.default_keymap.resolve(chord),
        }
    }

}


#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::core::command::Command;
    use crate::core::config::keymap::Resolution;
    use crate::core::config::RawConfig;
    use crate::core::key::Chord;


    fn chord(key_spec: &str) -> Chord {
        key_spec.parse().expect("Specify key spec like 'C-f'")
    }


    fn raw_config() -> RawConfig {
        RawConfig::parse_toml(r#"
            [undefined]
            "C-f" = "forward-char"

            [override]
            mode = "override"
            "C-x C-s" = "forward-char"

            [inherit]
            mode = "inherit"
            "C-a" = "forward-char"

            [disabled]
            mode = "disabled"
            "C-n" = "forward-char"

            [unknown_mode]
            mode = "unknown"
            "C-p" = "forward-char"

            [unknown_command]
            mode = "override"
            "C-b" = "unknown"
        "#)
    }


    #[test]
    fn skip_into_keymap_when_config_is_invalid() {
        let raw_config = raw_config();

        let default_keymap = RawConfig::into_default_keymap(raw_config.default);
        let apps_keymap = RawConfig::into_apps_keymap(raw_config.section, &Rc::new(default_keymap));

        assert!(apps_keymap.get("unknown_mode").is_none());
        assert!(apps_keymap.get("unknown_command").is_none());
    }


    #[test]
    fn treat_override_mode_when_default_mode_is_inherit() {
        let raw_config = raw_config();
        let default_keymap = RawConfig::into_default_keymap(raw_config.default);

        assert_eq!(default_keymap.resolve(&chord("C-f")), Resolution::Execute(Command::ForwardChar));
    }


    #[test]
    fn resolve_single_keyspec() {
        let raw_config = raw_config();
        let keymaps = raw_config.into_keymaps();

        // treat override mode
        assert_eq!(keymaps.resolve("undefined", &chord("C-f")), Resolution::Execute(Command::ForwardChar));

        // override default
        assert_ne!(keymaps.resolve("inherit", &chord("C-a")), Resolution::Execute(Command::ForwardChar));
        assert_eq!(keymaps.resolve("inherit", &chord("C-a")), Resolution::Execute(Command::BeginningOfLine));

        // disabled
        assert_eq!(keymaps.resolve("disabled", &chord("C-n")), Resolution::Nop);

        // fallback default keymap
        assert_eq!(keymaps.resolve("unknown_mode", &chord("C-p")), Resolution::Execute(Command::PreviousLine));
        assert_eq!(keymaps.resolve("unknown_command", &chord("C-b")), Resolution::Execute(Command::BackwardChar));
    }

}
