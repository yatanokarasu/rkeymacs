use std::fmt;

use crate::core::{key::{Chord, Key, Modifier}, plan::Plan::{ToExecute, ToSwallow}};


pub struct PlanBuilder {
    actions: Vec<Action>,
}

impl PlanBuilder {

    pub fn new() -> Self {
        Self {
            actions: vec![],
        }
    }


    pub fn press(mut self, key: Key) -> Self {
        self.actions.push(Action::Press(Chord::from(key)));

        self
    }


    pub fn with(mut self, pressed: Modifier) -> Self {
        if let Some(Action::Press(chord)) = self.actions.last_mut() {
            chord.append(pressed);
        }

        self
    }


    pub fn shifted_if(self, is_marked: bool) -> Self {
        if is_marked {
            self.with(Modifier::SHIFT)
        } else {
            self
        }
    }


    pub fn build(self) -> Plan {
        Plan::ToExecute(self.actions)
    }

}


#[derive(Debug, PartialEq)]
pub(crate) enum Plan {
    ToExecute(Vec<Action>),
    ToSwallow,
}

impl Plan {

    pub fn shield(self, mods: Modifier) -> Self {
        if mods.is_empty() {
            return self;
        }

        match self {
            Plan::ToSwallow => self,
            Plan::ToExecute(mut actions) => {
                actions.insert(0, Action::ReleaseModifier(mods));
                actions.push(Action::RestoreModifier(mods));

                Plan::ToExecute(actions)
            },
        }
    }

}

impl fmt::Display for Plan {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToExecute(actions) => {
                write!(f, "Plan: {:?}", actions.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))?;
                Ok(())
            },
            ToSwallow   => write!(f, "Plan to swallow.")
        }

    }

}


#[derive(Debug, PartialEq)]
pub(crate) enum Action {
    Press(Chord),
    ReleaseModifier(Modifier),
    RestoreModifier(Modifier),
}

impl fmt::Display for Action {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Action::*;

        match self {
            Press(chord)            => write!(f, "Action({})", chord)?,
            ReleaseModifier(mods)   => write!(f, "Action(ReleaseMod({}))", mods)?,
            RestoreModifier(mods)   => write!(f, "Action(RestoreMod({}))", mods)?,
        };

        Ok(())
    }

}


#[cfg(test)]
mod test {
    use crate::core::key::{Chord, Key, Modifier};
    use crate::core::plan::{Plan, Action, PlanBuilder};


    #[test]
    fn somekey_pressed_without_modifier() {
        let plan = PlanBuilder::new()
            .press(Key::Char('f'))
            .build();

        assert_eq!(
            plan,
            Plan::ToExecute(vec![
                Action::Press(Chord::from(Key::Char('f'))),
            ])
        )
    }


    #[test]
    fn somekey_pressed_with_ctrl() {
        let plan = PlanBuilder::new()
            .press(Key::Char('f'))
            .with(Modifier::CTRL)
            .build()
            .shield(Modifier::ALT);

        assert_eq!(
            plan,
            Plan::ToExecute(vec![
                Action::ReleaseModifier(Modifier::ALT),
                Action::Press(Chord::new(Key::Char('f'), Modifier::CTRL)),
                Action::RestoreModifier(Modifier::ALT),
            ])
        )
    }


    #[test]
    fn move_cursor_when_set_marked() {
        let plan = PlanBuilder::new()
            .press(Key::Right)
            .shifted_if(true)
            .build()
            .shield(Modifier::CTRL);

        assert_eq!(
            plan,
            Plan::ToExecute(vec![
                Action::ReleaseModifier(Modifier::CTRL),
                Action::Press(Chord::new(Key::Right, Modifier::SHIFT)),
                Action::RestoreModifier(Modifier::CTRL),
            ])
        )
    }

}
