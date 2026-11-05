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

    // Search and replace commands
    IsearchBackward,
    IsearchForward,

    // Edit commands
    DeleteBackwardChar,
    DeleteChar,

    BackwardKillWord,
    KillWord,

    KillLine,
    KillRegion,
    KillRingSave,

    Yank,

    // Error recovery commands
    KeyboardQuit,
    Undo,

    // Other commands
    SetMark,
    ToggleRkeymacs,
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

            IsearchBackward     => write!(f, "command:IsearchBackward"),
            IsearchForward      => write!(f, "command::IsearchForward"),

            DeleteBackwardChar  => write!(f, "command::DeleteBackwardChar"),
            DeleteChar          => write!(f, "command::DeleteChar"),

            BackwardKillWord    => write!(f, "command::BackwardKillWord"),
            KillWord            => write!(f, "command::KillWord"),

            KillLine            => write!(f, "command::KillLine"),
            KillRegion          => write!(f, "command::KillRegion"),
            KillRingSave        => write!(f, "command::KillRingSave"),

            Yank                => write!(f, "command::Yank"),

            KeyboardQuit        => write!(f, "command::KeyboardQuit"),
            Undo                => write!(f, "command::Undo"),

            SetMark             => write!(f, "command::SetMark"),
            ToggleRkeymacs      => write!(f, "command::ToggleRkeymacs"),
        }
    }

}

impl str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;

        Ok(match s {
            // Move cursor commands
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

            // Search and replace commands
            "isearch-backward"      => IsearchBackward,
            "isearch-forward"       => IsearchForward,

            // Edit commands
            "delete-backward-char"  => DeleteBackwardChar,
            "delete-char"           => DeleteChar,

            "backward-kill-word"    => BackwardKillWord,
            "kill-word"             => KillWord,

            "kill-line"             => KillLine,
            "kill-region"           => KillRegion,
            "kill-ring-save"        => KillRingSave,

            "yank"                  => Yank,

            // Error recovery
            "keyboard-quit"         => KeyboardQuit,
            "undo"                  => Undo,

            // Others
            "set-mark"              => SetMark,
            "toggle-rkeymacs"       => ToggleRkeymacs,

            _                       => return Err(format!("Unknown command: '{}'", s)),
        })
    }

}


pub struct Intent {
    pub command: Command,
    pub chord: Chord,
}
