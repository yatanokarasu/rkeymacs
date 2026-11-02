use std::cell::Cell;

use crate::core::command::Intent;
use crate::core::command::handler::Planner;
use crate::core::key::{Chord, Key, Modifier};
use crate::core::plan::{Plan, PlanBuilder};


struct MarkState {
    mark_activated: Cell<bool>,
}

impl MarkState {

    fn activate(&self) {
        self.mark_activated.set(true)
    }

    fn deactivate(&self) {
        self.mark_activated.set(false)
    }

    pub fn is_activated(&self) -> bool {
        self.mark_activated.get()
    }

}

impl Default for MarkState {

    fn default() -> Self {
        Self {
            mark_activated: Cell::new(false),
        }
    }

}


pub struct WindowsPlanner {
    state: MarkState,
}

impl Default for WindowsPlanner {

    fn default() -> Self {
        Self {
            state: MarkState::default(),
        }
    }

}

impl Planner for WindowsPlanner {

    fn plan(&self, intent: Intent) -> Plan {
        use crate::core::command::Command::*;

        let Intent{ command, chord } = intent;

        let plan = match command {
            ForwardChar         => self.move_cursor(Key::Right),
            BackwardChar        => self.move_cursor(Key::Left),
            NextLine            => self.move_cursor(Key::Down),
            PreviousLine        => self.move_cursor(Key::Up),

            BeginningOfLine     => self.move_cursor(Key::Home),
            EndOfLine           => self.move_cursor(Key::End),

            BeginningOfBuffer   => self.move_cursor(Chord::new(Key::Home, Modifier::CTRL)),
            EndOfBuffer         => self.move_cursor(Chord::new(Key::End, Modifier::CTRL)),

            ForwardWord         => self.move_cursor(Chord::new(Key::Right, Modifier::CTRL)),
            BackwardWord        => self.move_cursor(Chord::new(Key::Left, Modifier::CTRL)),

            ScrollDown          => self.move_cursor(Key::PageDown),
            ScrollUp            => self.move_cursor(Key::PageUp),

            DeleteBackwardChar  => self.edit(Key::Backspace),
            DeleteChar          => self.edit(Key::Delete),

            BackwardKillWord    => self.kill_word(Key::Left),
            KillWord            => self.kill_word(Key::Right),

            KillLine            => self.kill_line(),
            KillRegion          => self.kill_region(),
            KillRingSave        => self.kill_ring_save(),

            KeyboardQuit        => self.mark_state_to(false),
            Undo                => self.undo(),

            SetMark             => self.mark_state_to(true),
        };

        plan.shield(chord.mods)
    }

}

impl WindowsPlanner {

    fn move_cursor<C: Into<Chord>>(&self, chordable: C) -> Plan
    where
        C: Into<C>
    {
        PlanBuilder::new()
            .press(chordable)
            .shifted_if(self.state.is_activated())
            .build()
    }


    fn edit<C: Into<Chord>>(&self, chordable: C) -> Plan {
        PlanBuilder::new()
            .press(chordable.into())
            .build()
    }


    fn kill_word<C: Into<Chord>>(&self, chordable: C) -> Plan {
        PlanBuilder::new()
            .press(chordable.into())
            .with(Modifier::CTRL | Modifier::SHIFT)
            .press(Key::Delete)
            .build()
    }


    fn kill_line(&self) -> Plan {
        let mut builder = PlanBuilder::new();

        if self.state.is_activated() {
            self.state.deactivate();

            builder = builder.press(Key::Left);
        }

        builder
            .press(Chord::new(Key::End, Modifier::SHIFT))
            .press(Chord::new(Key::Char('x'), Modifier::CTRL))
            .build()
    }


    fn kill_region(&self) -> Plan {
        self.state.deactivate();

        PlanBuilder::new()
            .press(Chord::new(Key::Char('x'), Modifier::CTRL))
            .build()
    }


    fn kill_ring_save(&self) -> Plan {
        self.state.deactivate();

        PlanBuilder::new()
            .press(Chord::new(Key::Char('c'), Modifier::CTRL))
            .build()
    }


    fn undo(&self) -> Plan {
        PlanBuilder::new()
            .press(Chord::new(Key::Char('z'), Modifier::CTRL))
            .build()
    }


    fn mark_state_to(&self, mark_activated: bool) -> Plan {
        if mark_activated {
            self.state.activate();
        } else {
            self.state.deactivate();
        }

        Plan::ToSwallow
    }

}
