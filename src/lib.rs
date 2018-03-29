//! This crate implement a discrete event simulation framework
//! inspired by the SimPy library for Python. It uses the generator
//! feature that is nightly. Once the feature is stabilized, also this
//! crate will use stable. Generators will be the only nightly feature
//! used in this crate.
//!
//! # Simulation
//! A simulation is performed scheduling one or more processes that
//! models the environment you are going to simulate. Your model may
//! consider some kind of finite resource that must be shared among
//! the processes, e.g. a bunch of servers in a simulation on queues.
//!
//! After setting up the simulation, it can be run step-by-step, using
//! the `step()` method, or all at once, with `run()`, until and ending
//! condition is met.
//!
//! The simulation will generate a log of all the events.
//!
/*
//! `nonblocking_run` lets you run the simulation in another thread
//! so that your program can go on without waiting for the simulation
//! to finish.
//!
*/
//! # Process
//! A process is implemented using the rust generators syntax.
//! This let us avoid the overhead of spawning a new thread for each
//! process, while still keeping the use of this framework quite simple.
//!
//! When a new process is created in the simulation, an identifier, of type
//! `ProcessId` is assigned to it. That id can be used to schedule an event that
//! resume the process.
//!
//! A process can be stopped and resumed later on. To stop the process, the
//! generator yields an `Effect` that specify what the simulator should do.
//! For example, a generator can set a timeout after witch it is executed again.
//! The process may also return. In that case it can not be resumed anymore.
//!
//!
//! # Resources
//! A resource is a finite amount of entities that can be used by one process
//! a time. When all the instances of the resource of interest are being used by
//! a process, the requiring one is enqueued in a FIFO and is resumed when the
//! resource become available again. When the process does not need the resource
//! anymore, it must release it.
//!
//! A resource can be created in the simulation using the `create_resource`
//! method, which requires the amount of resource and returns an identifier
//! for that resource that can be used to require and release it.
//!
//! A resource can be required and reelased by a process yielding
//! the corresponding `Effect`. There is no check on the fact that a process
//! yielding `Release` was holding a resource with that ID, but if a resource
//! gets more release then requests, the simulation will panic.
//!

#![feature(generators, generator_trait)]
use std::ops::{Generator, GeneratorState};
use std::collections::{BinaryHeap, VecDeque};
use std::cmp::Ordering;

/// The effect is yelded by a process generator to
/// interact with the simulation environment.
pub enum Effect {
    /// The process that yields this effect will be resumed
    /// after the speified time
    TimeOut(f64),
    /// Yielding this effect it is possible to schedule the specified event
    Event(Event),
    /// This effect is yielded to request a resource
    Request(ResourceId),
    /// This effect is yielded to release a resource that is not needed anymore.
    Release(ResourceId),
    /// Keep the process' state until it is resumed by another event.
    Wait
}

/// Identifies a process. Can be used to resume it from another one.
pub type ProcessId = usize;
/// Identifies a resource. Can be used to request and release it.
pub type ResourceId = usize;


struct Resource {
    allocated: usize,
    available: usize,
    queue: VecDeque<ProcessId>
}

/// This struct provides the methods to create and run the simulation
/// in a single thread.
///
/// It provides methods to create processes and finite resources that
/// must be shared among them.
pub struct Simulation {
    time: f64,
    processes: Vec<Box<Generator<Yield = Effect, Return = ()>>>,
    future_events: BinaryHeap<Event>,
    processed_events: Vec<Event>,
    resources: Vec<Resource>
}

/*
pub struct ParallelSimulation {
    processes: Vec<Box<Generator<Yield = Effect, Return = ()>>>
}
 */

/// An event that can be scheduled by a process, yelding the `Event` `Effect`
/// or by the owner of a `Simulation` through the `schedule` method
#[derive(Copy, Clone)]
pub struct Event {
    pub time: f64,
    pub process: ProcessId,
}

/// Specify which condition must be met for the simulation to stop.
pub enum EndCondition {
    Time(f64),
}

impl Simulation {
    pub fn new(){

    }

    /// Returns the current simulation time
    pub fn time(&self) -> f64{
        self.time
    }

    /// Returns the log of processed events
    pub fn processed_events(&self) -> &[Event] {
        self.processed_events.as_slice()
    }

    /// Create a process. That is a generator that can Yield `Effect`s.
    /// An effect may be a new `Event` to schedule, a `Timeout` after which the
    /// same process should be executed, or a `Request` to hold an instance
    /// of a finite resource.
    ///
    /// Returns the identifier of the process.
    pub fn create_process(&mut self,
                          process: Box<Generator<Yield = Effect, Return = () >>)
                          -> ProcessId
    {
        let id = self.processes.len();
        self.processes.push(process);
        id
    }

    /// Create a new finite resource, of which n instancies are available.
    ///
    /// The resource can be requested by a process yielding a `Request`.
    /// If the requested resource is not available at that time, the process
    /// is enqueued until one instance of the resource is freed.
    ///
    /// When the process has done with the resource, it has to yield `Release`,
    /// so that other processes can use that.
    ///
    /// The queue has a FIFO (first in first out) policy.
    ///
    /// Returns the identifier of the resource
    pub fn create_resource(&mut self, n: usize) -> ResourceId {
        let id = self.resources.len();
        self.resources.push(Resource{
            allocated: n,
            available: n,
            queue: VecDeque::new()
        });
        id
    }

    /// Schedule a process to be executed. Another way to schedule events is
    /// yielding `Effect::Event` from a process during the simulation.
    pub fn schedule_event(&mut self, event: Event) {
        self.future_events.push(event);
    }

    /// Proceed in the simulation by 1 step
    pub fn step(&mut self) {
        match self.future_events.pop() {
            Some(event) => {
                match self.processes[event.process].resume() {
                    GeneratorState::Yielded(y) => match y {
                        Effect::TimeOut(t) =>
                            self.future_events.push(Event{
                                time: self.time + t,
                                process: event.process}),
                        Effect::Event(e) => self.future_events.push(e),
                        Effect::Request(r) => {
                            let mut res = &mut self.resources[r];
                            if res.available == 0 {
                                // enqueue the process
                                res.queue.push_back(event.process);
                            } else {
                                // the process can use the resource immediately
                                self.future_events.push(Event{
                                    time: self.time,
                                    process: event.process
                                });
                                res.available -= 1;
                            }
                        }
                        Effect::Release(r) => {
                            let res = &mut self.resources[r];
                            match res.queue.pop_front() {
                                Some(p) =>
                                // some processes in queue: schedule the next.
                                    self.future_events.push(Event{
                                        time: self.time,
                                        process: p
                                    }),
                                None => {
                                    assert!(res.available < res.allocated);
                                    res.available += 1;
                                }
                            }
                            // after releasing the resource the process
                            // can be resumed
                            self.future_events.push(Event{
                                time: self.time,
                                process: event.process
                            })
                        }
                        Effect::Wait => {}
                    },
                    GeneratorState::Complete(_) => {
                        // removing the process from the vector would invalidate
                        // all existing `ProcessId`s, but keeping it would be a
                        // waste of space since it is completed.
                        // May be worth to use another data structure
                    },
                }
                self.processed_events.push(event);
            }
            None => {},
        }
    }

    /// Run the simulation until and ending condition is met.
    pub fn run(mut self, until: EndCondition) -> Simulation {
        while self.check_ending_condition(&until) {
            self.step();
        }
        self
    }
    /*
    pub fn nonblocking_run(mut self, until: EndCondition) {

    }
     */

    /// Return `true` if the ending condition was met, `false` otherwise.
    fn check_ending_condition(&self, ending_condition: &EndCondition) -> bool {
        match ending_condition {
            &EndCondition:: Time(t) => if self.time >= t { true } else {false}
        }
    }
}

impl Default for Simulation {
    fn default() -> Self {
        Simulation{
            time: 0.0,
            processes: Vec::default(),
            future_events: BinaryHeap::default(),
            processed_events: Vec::default(),
            resources: Vec::default(),
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.time == other.time
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other:&Event) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        match self.time.partial_cmp(&other.time) {
            Some(o) => o,
            None => panic!("Event time was uncomparable. Maybe a NaN")
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
