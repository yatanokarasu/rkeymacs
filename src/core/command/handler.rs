use std::ops::Deref;

use crate::core::Disposition;
use crate::core::command::Intent;
use crate::core::plan::Plan;


pub trait Planner {
    fn plan(&self, intent: Intent) -> Plan;
}

pub trait Compiler {
    type Input;

    fn compile(&self, plan: Plan) -> Vec<Self::Input>;
}

pub trait Executor {
    type Input;

    fn send(&self, instructions: &[Self::Input]);

    fn execute(&self, instructions: Vec<Self::Input>) -> Disposition {
        let low_instructions = instructions.deref();

        if low_instructions.len() != 0 {
            self.send(instructions.deref());
        }

        Disposition::Swallow
    }

}


pub(crate) struct Handler<P, C, E> {
    planner: P,
    compiler: C,
    executor: E,
}

impl<P, C, E> Handler<P, C, E>
where
    P: Planner,
    C: Compiler,
    E: Executor<Input = C::Input>,
{

    pub fn new(planner: P, compiler: C, executor: E) -> Self {
        Self {
            planner,
            compiler,
            executor,
        }
    }


    pub fn handle(&self, intent: Intent) -> Disposition {
        let plan = self.planner.plan(intent);
        println!("{plan}");

        let instructions = self.compiler.compile(plan);

        self.executor.execute(instructions)
    }

}
