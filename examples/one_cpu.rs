//simulate the scheduling of two processes on one CPU
#![feature(generators, generator_trait)]
extern crate desim;
extern crate rand;

use rand::{Rng as RngT, XorShiftRng as Rng};

use desim::{Effect, EndCondition, Simulation};

fn main() {
    let mut s = Simulation::new();
    let cpu = s.create_resource(1);
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
        let mut rng = Rng::new_unseeded();
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
    s.schedule_event(0.0, p1);
    // ...and p2 after 17 time units
    s.schedule_event(17.0, p2);

    s.run(EndCondition::Time(100.0));
}
