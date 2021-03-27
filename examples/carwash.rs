// Simulate cars arriving and being served at a carwash
#![feature(generators, generator_trait)]
use std::fmt::{Display, Formatter, Result};

use rand::{
    distributions::{Distribution, Uniform},
    rngs::SmallRng as Rng,
    Rng as RngT, SeedableRng,
};
use rand_distr::Exp;

use desim::{Effect, EndCondition, ResourceId, SimGen, SimState, Simulation};
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

fn car_process<'a>(
    carwash: ResourceId,
    rng: &'a mut Rng,
    distr_drive: &'a impl Distribution<f32>,
    distr_wash: &'a impl Distribution<f32>,
) -> Box<SimGen<CarState>> {
    let t_drive = distr_drive.sample(rng);
    let t_wash = distr_wash.sample(rng);
    Box::new(move |_| {
        yield Drive(t_drive);
        yield WaitMachine(carwash);
        yield Wash(t_wash);
        yield Leave(carwash);
    })
}

fn main() {
    let mut sim = Simulation::new();

    let carwash = sim.create_resource(NUM_MACHINES);
    let mut rng = Rng::from_entropy();
    let unif = Uniform::new(0.0, SIM_TIME);
    let distr_drive = Exp::new(1.0 / LAMBDA_DRIVE).unwrap();
    let distr_wash = Exp::new(1.0 / LAMBDA_WASH).unwrap();

    for t in unif.sample_iter(rng.clone()).take(NUM_CARS) {
        let p = sim.create_process(car_process(carwash, &mut rng, &distr_drive, &distr_wash));
        sim.schedule_event(t, p, CarState::Drive(0.0));
    }

    sim = sim.run(EndCondition::NoEvents);

    for (e, state) in sim.processed_events() {
        println!("{}\t{}", e.time(), state);
    }
}
