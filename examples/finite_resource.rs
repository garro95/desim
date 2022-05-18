/* Copyright © 2021 Gianmarco Garrisi

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>. */

//! This example shows how to create a custom resource and implement the resource trait.
//!
//! Features shown in this example:
//! * Custom Resources
//! * Prelude
//! * Custom state
//! * EndCondition::NoEvents

#![feature(generators)]

use desim::prelude::*;
use desim::resources::Resource;

use rand::{
    distributions::{Distribution, Uniform},
    rngs::SmallRng as Rng,
    SeedableRng,
};

const Q_SIZE: usize = 30;
const NUM_CLIENTS: usize = 800;
const SIM_TIME: f64 = 50.0;

#[derive(Copy, Clone, Debug)]
struct State {
    effect: Effect,
    queue_full: bool,
}

impl State {
    fn new(effect: Effect) -> State {
        State {
            effect,
            queue_full: false,
        }
    }
}

struct FiniteQueue {
    quantity: usize,
    available: usize,
    queue: [Option<Event<State>>; Q_SIZE],
    queue_start: usize,
    queue_len: usize,
}

impl SimState for State {
    fn get_effect(&self) -> Effect {
        self.effect
    }
    fn set_effect(&mut self, effect: Effect) {
        self.effect = effect;
    }
    fn should_log(&self) -> bool {
        true
    }
}

impl Resource<State> for FiniteQueue {
    fn allocate_or_enqueue(&mut self, event: Event<State>) -> Vec<Event<State>> {
        if self.available > 0 {
            self.available -= 1;
            vec![event]
        } else {
            if self.queue_len == Q_SIZE {
                let mut event = event;
                event.state_mut().queue_full = true;
                vec![event]
            } else {
                let first_position = (self.queue_start + self.queue_len) % Q_SIZE;
                self.queue[first_position] = Some(event);
                self.queue_len += 1;
                vec![]
            }
        }
    }
    fn release_and_schedule_next(&mut self, event: Event<State>) -> Vec<Event<State>> {
        if self.queue_len > 0 {
            let mut next_event = self.queue[self.queue_start].take().unwrap();
            self.queue_start = (self.queue_start + 1) % Q_SIZE;
            self.queue_len -= 1;
            next_event.set_time(event.time());
            vec![next_event, event]
        } else {
            self.available += 1;
            vec![event]
        }
    }
}

fn client_process(res: ResourceId) -> Box<Process<State>> {
    Box::new(move |response: SimContext<State>| {
        yield State::new(Effect::Request(res));
        if !response.state().queue_full {
            yield State::new(Effect::TimeOut(5.0));
            yield State::new(Effect::Release(res));
        } else {
            yield State::new(Effect::Trace);
        }
    })
}

fn main() {
    let mut sim = Simulation::new();
    let unif = Uniform::new(0.0, SIM_TIME);
    let rng = Rng::from_entropy();

    let res = FiniteQueue {
        quantity: 4,
        available: 4,
        queue: [None; Q_SIZE],
        queue_start: 0,
        queue_len: 0,
    };
    let res = sim.create_resource(Box::new(res));

    // Create NUM_CLIENTS processes and schedule them at random times
    for t in unif.sample_iter(rng).take(NUM_CLIENTS) {
        let p = sim.create_process(client_process(res));
        sim.schedule_event(t, p, State::new(Effect::TimeOut(0.0)));
    }

    sim = sim.run(EndCondition::NoEvents);
    // Print the simulation log
    for (e, state) in sim.processed_events() {
        println!("{}\t{:?}\t{:?}", e.time(), e.state(), state);
    }

    println!(
        "Lost clients: {}",
        sim.processed_events()
            .iter()
            .filter(|(e, _)| e.state().queue_full)
            .count()
    );
}
