//! Simulate cars arriving and being served at a carwash
//!
//! Features shown in this example:
//! * Custom state
//! * Simple Resource
//! * prelude

#![feature(coroutines, coroutine_trait)]
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

use rand::{
    distributions::{Distribution, Uniform},
    rngs::SmallRng as Rng,
    SeedableRng,
};
use rand_distr::Exp;

use desim::prelude::*;
use desim::resources::SimpleResource;
use CarState::*;

const NUM_MACHINES: usize = 4; // A carwash with 4 machines to wash cars
const NUM_CARS: usize = 40_000; // 40'000 cars generated
const SIM_TIME: f64 = 10000.0; // New cars are spawned randomly for 10'000 minutes
const LAMBDA_DRIVE: f32 = 5.0; // Each car drives for ~5 minutes
const LAMBDA_WASH: f32 = 2.0; // It takes ~2 minutes to wash a car

#[derive(Copy, Clone, Debug)]
enum CarState {
    Drive(f32),
    WaitMachine(ResourceId),
    Wash(f32),
    Leave(ResourceId),
}

impl SimState for CarState {
    fn get_effect(&self) -> Effect {
        match self {
            Drive(t) => Effect::TimeOut(*t as f64),
            WaitMachine(r) => Effect::Request(*r),
            Wash(t) => Effect::TimeOut(*t as f64),
            Leave(r) => Effect::Release(*r),
        }
    }
    fn set_effect(&mut self, _: Effect) {
        //
    }
    fn should_log(&self) -> bool {
        true
    }
}

impl Display for CarState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Drive(t) => write!(f, "Drive for {} minutes", t),
            WaitMachine(_) => write!(f, "Waiting a machine"),
            Wash(t) => write!(f, "Washing for {} minutes", t),
            Leave(_) => write!(f, "Clean! Leaving carwash"),
        }
    }
}

// Create a car process
fn car_process<'a>(
    carwash: ResourceId,
    rng: &'a mut Rng,
    distr_drive: &'a impl Distribution<f32>,
    distr_wash: &'a impl Distribution<f32>,
) -> Box<Process<CarState>> {
    // Generate random drive_time and wash_time at beginning
    let t_drive = distr_drive.sample(rng);
    let t_wash = distr_wash.sample(rng);
    Box::new(
        #[coroutine]
        move |_| {
            // The car drives for `t_drive` time
            yield Drive(t_drive);
            // Arrives at carwash and waits for a machine to be free
            yield WaitMachine(carwash);
            // The car wash for `t_wash` time, keeping the carwash machine (resource) occupied
            yield Wash(t_wash);
            // The car leaves the carwash, freeing the resource
            yield Leave(carwash);
        },
    )
}

fn main() {
    // Create a new simulation
    let mut sim = Simulation::new();

    // Create the carwash resource: It contains `NUM_MACHINES` machines to wash cars`
    let carwash = sim.create_resource(Box::new(SimpleResource::new(NUM_MACHINES)));

    // Create random number genrator and some distributions
    let mut rng = Rng::from_entropy();
    let unif = Uniform::new(0.0, SIM_TIME);
    let distr_drive = Exp::new(1.0 / LAMBDA_DRIVE).unwrap();
    let distr_wash = Exp::new(1.0 / LAMBDA_WASH).unwrap();

    // Create NUM_CARS car processes and schedule them at random times
    for t in unif.sample_iter(rng.clone()).take(NUM_CARS) {
        let p = sim.create_process(car_process(carwash, &mut rng, &distr_drive, &distr_wash));
        sim.schedule_event(t, p, CarState::Drive(0.0));
    }

    // Run the simulation until all cars have been washed
    sim = sim.run(EndCondition::NoEvents);

    // Print the simulation log
    for (e, state) in sim.processed_events() {
        println!("{}\t{}", e.time(), state);
    }

    // Compute average waiting time
    let mut wait_start_time = HashMap::new();

    let sum: (f64, f64) = sim
        .processed_events()
        .iter()
        .filter_map(|(e, state)| match state {
            WaitMachine(_) => {
                wait_start_time.insert(e.process(), e.time());
                None
            }
            Wash(_) => Some(e.time() - wait_start_time.remove(&e.process()).unwrap()),
            _ => None,
        })
        .fold((0.0, 0.0), |(t, c), t0| (t + t0, c + 1.0));

    println!("The average waiting time was: {}", sum.0 / sum.1);
}
