pub mod handler;


use std::fmt;
use std::str;

use crate::core::key::Chord;


#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Command {
    // Move cursor commands
    ForwardChar,
    BackwardChar,
    NextLine,
    PreviousLine,

    BeginningOfLine,
    EndOfLine,

    BeginningOfBuffer,
    EndOfBuffer,

    ForwardWord,
    BackwardWord,

    ScrollDown,
    ScrollUp,

    // Error recovery commands
    KeyboardQuit,
    Undo,

    // Other commands
    SetMark,
}

impl fmt::Display for Command {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Command::*;

        match self {
            ForwardChar         => write!(f, "command::ForwardChar"),
            BackwardChar        => write!(f, "command::BackwardChar"),
            NextLine            => write!(f, "command::NextLine"),
            PreviousLine        => write!(f, "command::PreviousLine"),

            BeginningOfLine     => write!(f, "command::BeginningOfLine"),
            EndOfLine           => write!(f, "command::EndOfLine"),

            BeginningOfBuffer   => write!(f, "command::BeginningOfBuffer"),
            EndOfBuffer         => write!(f, "command::EndOfBuffer"),

            ForwardWord         => write!(f, "command::ForwardWord"),
            BackwardWord        => write!(f, "command::BackwardWord"),

            ScrollDown          => write!(f, "command::ScrollDown"),
            ScrollUp            => write!(f, "command::ScrollUp"),

            KeyboardQuit        => write!(f, "command::KeyboardQuit"),
            Undo                => write!(f, "command::Undo"),

            SetMark             => write!(f, "command::SetMark"),
        }
    }

}

impl str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;

        Ok(match s {
            // move cursor commands
            "forward-char"          => ForwardChar,
            "backward-char"         => BackwardChar,
            "next-line"             => NextLine,
            "previous-line"         => PreviousLine,

            "beginning-of-line"     => BeginningOfLine,
            "end-of-line"           => EndOfLine,

            "beginning-of-buffer"   => BeginningOfBuffer,
            "end-of-buffer"         => EndOfBuffer,

            "forward-word"          => ForwardWord,
            "backward-word"         => BackwardWord,

            "scroll-down"           => ScrollDown,
            "scroll-up"             => ScrollUp,

            // Error recovery
            "keyboard-quit"         => KeyboardQuit,
            "undo"                  => Undo,

            "set-mark"              => SetMark,

            _                       => return Err(format!("Unknown command: '{}'", s)),
        })
    }

}


pub struct Intent {
    pub command: Command,
    pub chord: Chord,
}
