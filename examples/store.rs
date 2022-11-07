//! A very simple example that simulate the scheduling of two processes on one CPU
//!
//! Features shown in this example:
//! * SimpleResource
//! * Effects
//! * EndCondition::Time
//! * Simulation
#![feature(generators, generator_trait)]

use desim::resources::SimpleStore;
use desim::{Effect, EndCondition, SimState, Simulation, StoreId};
#[derive(Default, Clone, Debug)]
enum MyState {
    #[default]
    Continue,
    Push(StoreId, u32),
    Pull(StoreId),
    Wait(f64),
}

impl SimState for MyState {
    fn get_effect(&self) -> Effect {
        match self {
            MyState::Continue => Effect::TimeOut(0.),
            MyState::Push(id, _) => Effect::Push(*id),
            MyState::Pull(id) => Effect::Pull(*id),
            MyState::Wait(time) => Effect::TimeOut(*time),
        }
    }

    fn set_effect(&mut self, effect: Effect) {
        *self = match effect {
            Effect::Push(id) => MyState::Push(id, 0),
            Effect::Pull(id) => MyState::Pull(id),
            _ => unimplemented!()
        };
    }

    fn should_log(&self) -> bool {
        true
    }
}

fn main() {
    let mut s = Simulation::new();
    let queue = s.create_store(Box::new(SimpleStore::new(1)));
    let p1 = s.create_process(Box::new(move |_| {
        for i in 0..10 {
            // wait for the cpu to be available
            yield MyState::Push(queue, i);
            // do some job that requires a fixed amount of 5 time units
            // release the CPU
            yield MyState::Wait(10.0);
        }
    }));
    let p2 = s.create_process(Box::new(move |_| {
        for _ in 0..10 {
            // wait for the CPU
            let ret = yield MyState::Pull(queue);
            println!("ret: {:?}", ret);
            // do some job for a random amount of time units between 0 and 10
            // yield MyState::Wait(10.0);
            // release the CPU
        }
    }));
    // let p1 to start immediately...
    s.schedule_event(0.0, p1, MyState::default());
    // ...and p2 after 17 time units
    s.schedule_event(17.0, p2, MyState::default());

    s = s.run(EndCondition::NoEvents);

    for e in s.processed_events().iter().map(|e| format!("{:?}", e)) {
        println!("{}", e);
    }
}
