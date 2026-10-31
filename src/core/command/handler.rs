use crate::core::Disposition;
use crate::core::command::Intent;
use crate::core::plan::Plan;
use crate::platform::LowLevelInstructions;


pub trait Planner {
    fn plan(&self, intent: Intent) -> Plan;
}

pub trait Compiler {
    type Input;

    fn compile(&self, plan: Plan) -> LowLevelInstructions<Self::Input>;
}

pub trait Executor {
    type Input;

    fn send(&self, instructions: &[Self::Input]);

    fn execute(&self, instructions: LowLevelInstructions<Self::Input>) -> Disposition {
        if !instructions.is_empty() {
            self.send(instructions.as_slice());
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
