pub mod handler;


use std::fmt;
use std::str;

use crate::core::key::Chord;


#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Command {
    ForwardChar,
    BackwardChar,
    NextLine,
    PreviousLine,

    BeginningOfLine,
    EndOfLine,

    ForwardWord,
    BackwardWord,

    SetMark,
    KeyboardQuit,
}

impl fmt::Display for Command {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Command::*;

        match self {
            ForwardChar     => write!(f, "command::ForwardChar"),
            BackwardChar    => write!(f, "command::BackwardChar"),
            NextLine        => write!(f, "command::NextLine"),
            PreviousLine    => write!(f, "command::PreviousLine"),

            BeginningOfLine => write!(f, "command::BeginningOfLine"),
            EndOfLine       => write!(f, "command::EndOfLine"),

            ForwardWord     => write!(f, "command::ForwardWord"),
            BackwardWord    => write!(f, "command::BackwardWord"),

            SetMark         => write!(f, "command::SetMark"),
            KeyboardQuit    => write!(f, "command::KeyboardQuit"),
        }
    }

}

impl str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;

        Ok(match s {
            "forward-char"      => ForwardChar,
            "backward-char"     => BackwardChar,
            "next-line"         => NextLine,
            "previous-line"     => PreviousLine,

            "beginning-of-line" => BeginningOfLine,
            "end-of-line"       => EndOfLine,

            "forward-word"      => ForwardWord,
            "backward-word"     => BackwardWord,

            "set-mark"          => SetMark,
            "keyboard-quit"     => KeyboardQuit,

            _                   => return Err("Unknown command".to_string()),
        })
    }

}


pub struct Intent {
    pub command: Command,
    pub chord: Chord,
}
