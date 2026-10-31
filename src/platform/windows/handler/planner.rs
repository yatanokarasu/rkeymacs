
use std::cell::Cell;

use crate::core::command::Intent;
use crate::core::command::handler::Planner;
use crate::core::key::{Key, Modifier};
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
            ForwardChar     => self.move_cursor(Key::Right),
            BackwardChar    => self.move_cursor(Key::Left),
            NextLine        => self.move_cursor(Key::Down),
            PreviousLine    => self.move_cursor(Key::Up),

            BeginningOfLine => self.move_cursor(Key::Home),
            EndOfLine       => self.move_cursor(Key::End),

            ForwardWord     => self.forward_word(),
            BackwardWord    => self.backward_word(),

            SetMark         => self.mark_state_to(true),
            KeyboardQuit    => self.mark_state_to(false),
        };

        plan.shield(chord.mods)
    }

}

impl WindowsPlanner {

    fn move_cursor(&self, key: Key) -> Plan {
        PlanBuilder::new()
            .press(key)
            .shifted_if(self.state.is_activated())
            .build()
    }


    fn forward_word(&self) -> Plan {
        PlanBuilder::new()
            .press(Key::Right)
            .with(Modifier::CTRL)
            .shifted_if(self.state.is_activated())
            .build()
    }


    fn backward_word(&self) -> Plan {
        PlanBuilder::new()
            .press(Key::Left)
            .with(Modifier::CTRL)
            .shifted_if(self.state.is_activated())
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
