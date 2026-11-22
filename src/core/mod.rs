pub(crate) mod key;
pub(crate) mod command;
pub(crate) mod plan;
pub(crate) mod config;
pub(crate) mod engine;
pub(in crate::core) mod keymap;


pub(crate) enum Disposition {
    Passthrough,
    Swallow
}
