//! This crate implement a discrete event simulation framework
//! inspired by the SimPy library for Python. It uses the generator
//! feature that is nightly

#![feature(generators, generator_trait)]
use std::ops::{Generator, GeneratorState};
use std::collections::BinaryHeap;

pub enum Effect {
    TimeOut(f64),
    Event(Event),
    
}

pub struct Simulation {
    time: f64,
    processes: Vec<Box<Generator<Yield = Effect, Return = ()>>>,
    future_events: BinaryHeap<Event>,
    processed_events: Vec<Event>,
}

pub struct ParallelSimulation {
    processes: Vec<Box<Generator<Yield = Effect, Return = ()>>>
}

pub struct Resource {

}

pub struct Event {
    time: f64,
    
}

pub enum EndCondition {
    Time(f64),
}

impl Simulation {
    pub fn new(){
        
    }
    
    /// Proceed in the simulation by 1 step
    pub fn step(&mut self) {
        
    }

    /// Run the simulation until and ending condition is met.
    pub fn run(self, until: EndCondition) {
        
    }

    pub fn nonblocking_run(self, until: EndCondition) {
        
    }

    /// Return `true` if the ending condition was met, `false` otherwise.
    fn check_ending_condition(&self, ending_condition: EndCondition) -> bool {
        
    }
}

impl Default for Simulation {
    fn default() -> Self {
        Simulation{
            time: 0,
            processes: Vec::default(),
            future_events: BinaryHeap::default(),
            processed_events: Vec::default()
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
