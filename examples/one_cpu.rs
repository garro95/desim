//! A very simple example that simulate the scheduling of two processes on one CPU
//!
//! Features shown in this example:
//! * SimpleResource
//! * Effects
//! * EndCondition::Time
//! * Simulation
#![feature(generators, generator_trait)]
use rand::{rngs::SmallRng as Rng, RngCore as RngT, SeedableRng};

use desim::resources::SimpleResource;
use desim::{Effect, EndCondition, Simulation};

fn main() {
    let mut s = Simulation::new();
    let cpu = s.create_resource(Box::new(SimpleResource::new(1)));
    let p1 = s.create_process(Box::new(move |_| {
        for _ in 0..10 {
            // wait for the cpu to be available
            yield Effect::Request(cpu);
            // do some job that requires a fixed amount of 5 time units
            yield Effect::TimeOut(5.0);
            // release the CPU
            yield Effect::Release(cpu);
        }
    }));
    let p2 = s.create_process(Box::new(move |_| {
        let mut rng = Rng::from_entropy();
        loop {
            // wait for the CPU
            yield Effect::Request(cpu);
            // do some job for a random amount of time units between 0 and 10
            yield Effect::TimeOut((rng.next_u32() % 10) as f64);
            // release the CPU
            yield Effect::Release(cpu);
        }
    }));
    // let p1 to start immediately...
    s.schedule_event(0.0, p1, Effect::TimeOut(0.));
    // ...and p2 after 17 time units
    s.schedule_event(17.0, p2, Effect::TimeOut(0.));

    s = s.run(EndCondition::Time(100.0));

    for e in s.processed_events().iter().map(|e| format!("{:?}", e)) {
        println!("{}", e);
    }
}
