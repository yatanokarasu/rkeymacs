pub(in crate::core) mod keymap;


use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::sync::LazyLock;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::Runtime;
use crate::core::key::Chord;
use crate::core::config::keymap::{BindTable, KeyMap, KeyMaps};


const BUILTIN_CONFIG_STR: &str = include_str!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/assets/builtin_config.toml")
);

static BUILTIN_CONFIG: LazyLock<RawConfig> = LazyLock::new(||
    RawConfig::parse_toml(BUILTIN_CONFIG_STR)
);


#[derive(Debug, PartialEq)]
pub(in crate::core) enum BindMode {
    Inherit,
    Override,
    Disabled,
}

impl BindMode {

    pub fn parse(mode: &str) -> Result<Self, String> {
        Ok(match mode {
            "disabled"  => BindMode::Disabled,
            "inherit"   => BindMode::Inherit,
            "override"  => BindMode::Override,
            _           => return Err("Unknown mode".to_string()),
        })
    }

}


#[derive(Debug, Default, Deserialize)]
pub(crate) struct RawConfig {
    #[serde(default)]
    default: Option<RawSection>,

    #[serde(flatten)]
    section: HashMap<String, RawSection>,
}

impl RawConfig {

    pub fn load() -> Self {
        match Self::config_path() {
            Some(path) if path.exists() => Self::load_from(&path),
            _                           => Self::default(),
        }
    }


    pub fn into_keymaps(self) -> KeyMaps {
        let RawConfig { default, section } = self;

        let default_keymap = Rc::new(Self::into_default_keymap(default));
        let apps_keymap = Self::into_apps_keymap(section, &default_keymap);

        KeyMaps::new(default_keymap, apps_keymap)
    }


    fn into_default_keymap(default: Option<RawSection>) -> KeyMap {
        let Some(mut default_section) = default else {
            return RawConfig::builtin_default_keymap();
        };

        if let Some(ref mut default_mode) = default_section.mode {
            if default_mode.eq_ignore_ascii_case("inherit") {
                eprintln!("[WARN] 'inherit' mode is not supported in default section. Treating as 'override' mode.");

                *default_mode = "override".to_string();
            }
        };

        match default_section.into_keymap(None) {
            Ok(default_keymap)  => default_keymap,
            _                           => RawConfig::builtin_default_keymap(),
        }
    }


    fn into_apps_keymap(apps_section: HashMap<String, RawSection>, default_keymap: &Rc<KeyMap>) -> HashMap<String, KeyMap> {
        let mut apps_keymap = HashMap::new();

        for (section_name, section) in apps_section {
            match section.into_keymap(Some(default_keymap)) {
                Ok(app_keymap)  => { apps_keymap.insert(section_name, app_keymap); }
                Err(err)        => { eprintln!("[WARN] Skipped section '{}': {}", section_name, err); }
            }
        }

        apps_keymap
    }


    fn config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|home|
            home
                .join(".config")
                .join(Runtime::NAME)
                .join("config.toml")
        )
    }


    fn load_from(path: &Path) -> Self {
        println!("Now loading...: {:?}", path);

        match std::fs::read_to_string(path) {
            Ok(config_src)  => Self::parse_toml(&config_src),
            _               => Self::default(),
        }
    }


    fn parse_toml(config_src: &str) -> Self {
        match toml::from_str(config_src) {
            Ok(config)  => config,
            _           => Self::default(),
        }
    }


    fn builtin_default_keymap() -> KeyMap {
        let builtin = BUILTIN_CONFIG.default.as_ref()
            .expect("Unexpected state: builtin config MUST have default section.");

        RawSection {
            mode: Some("override".to_string()),
            key_config: builtin.key_config.clone()
        }.into_keymap(None)
            .expect("Unexpected state: buildin config is invalid.")
    }

}


#[derive(Debug, Default, Deserialize)]
struct RawSection {
    #[serde(default)]
    mode: Option<String>,

    #[serde(flatten)]
    key_config: HashMap<String, String>,
}

impl RawSection {

    pub fn into_keymap(self, fallback_keymap: Option<&Rc<KeyMap>>) -> Result<KeyMap, String> {
        let bind_mode = self.canonicalize_mode()?;
        let bind_table = self.into_bind_table()?;

        Ok(match bind_mode {
            BindMode::Override  => KeyMap::new(bind_table, None),
            BindMode::Disabled  => KeyMap::new(BindTable::new(), None),
            BindMode::Inherit   => KeyMap::new(BindTable::new(), fallback_keymap.map(Rc::clone)),
        })
    }


    fn canonicalize_mode(&self) -> Result<BindMode, String> {
        Ok(match self.mode.as_deref() {
            Some(mode)  => BindMode::parse(mode)?,
            None        => BindMode::Override,
        })
    }


    fn into_bind_table(self) -> Result<BindTable, String> {
        let mut bind_table = BindTable::new();

        for (key_spec_str, command_str) in self.key_config {
            let chords: VecDeque<Chord> = key_spec_str
                .split_whitespace()
                .map(|chord| chord.parse())
                .collect::<Result<VecDeque<Chord>, String>>()?;
            let command = command_str.parse()?;

            let _ = bind_table.insert(chords, command);
        }

        Ok(bind_table)
    }

}


#[cfg(test)]
mod tests {
    use super::BUILTIN_CONFIG;
    use super::RawConfig;


    #[test]
    fn no_found_config_file() {
        let raw_config = RawConfig::load_from(&dirs::template_dir().unwrap().join("unknown-path"));

        assert!(raw_config.section.is_empty());
    }


    #[test]
    fn load_empty_config_file() {
        let raw_config = RawConfig::parse_toml("");

        assert!(raw_config.default.is_none());
        assert!(raw_config.section.is_empty());
    }


    #[test]
    fn load_invalid_section() {
        let raw_config = RawConfig::parse_toml(r#"
            # Malformed section, unclosed bracket
            [unkno
            "C-f" = "forward-char"
        "#);

        assert!(raw_config.section.is_empty());
    }


    #[test]
    fn load_default_and_others() {
        let raw_config = RawConfig::parse_toml(r#"
            [default]
            "keyspec" = "command"

            [another1]
            mode = "inherit"

            [another2]
            "mode" = "disabled"
            keyspec = "command"
        "#);
        let default = raw_config.default.unwrap();
        let another1 = raw_config.section.get("another1").unwrap();
        let another2 = raw_config.section.get("another2").unwrap();

        assert_eq!(default.mode.as_deref(), None);
        assert_eq!(default.key_config.get("keyspec").map(String::as_str), Some("command"));

        assert_eq!(another1.mode.as_deref(), Some("inherit"));
        assert!(another1.key_config.is_empty());

        assert_eq!(another2.mode.as_deref(), Some("disabled"));
        assert_eq!(another2.key_config.get("keyspec").map(String::as_str), Some("command"));
    }


    #[test]
    fn load_mode() {
        let raw_config = RawConfig::parse_toml(r#"
            [defined]
            mode = "inherit"

            [undefined]
        "#);
        let has_mode_section = raw_config.section.get("defined").unwrap();
        let has_no_mode_section = raw_config.section.get("undefined").unwrap();

        assert_eq!(has_mode_section.mode.as_deref(), Some("inherit"));
        assert_eq!(has_no_mode_section.mode.as_deref(), None);
    }


    #[test]
    fn load_empty_keyspec() {
        let raw_config = RawConfig::parse_toml(r#"
            [test]
            "" = "forward-char"
        "#);

        let empty_section = raw_config.section.get("test").unwrap();
        assert!(empty_section.mode.is_none());
        assert_eq!(empty_section.key_config.get("").unwrap().clone(), "forward-char".to_string());
    }


    #[test]
    fn load_config_has_empty_custom_section() {
        let raw_config = RawConfig::parse_toml(r#"
            [test]
        "#);
        let section = raw_config.section.get("test").unwrap();
        assert!(section.key_config.is_empty());
        assert_eq!(section.mode, None);
    }


    #[test]
    fn load_builtin_config_file() {
        let default_section = BUILTIN_CONFIG.default.as_ref().unwrap();

        assert!(!default_section.key_config.is_empty());
        assert_eq!(default_section.mode, None);
    }

}
