/* Copyright © 2018 Gianmarco Garrisi

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
use crate::Event;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct SimpleResource<T> {
    quantity: usize,
    available: usize,
    queue: VecDeque<Event<T>>,
}

pub trait Resource<T> {
    fn allocate_or_enqueue(&mut self, event: Event<T>) -> Option<Event<T>>;
    fn release_and_schedule_next(&mut self, event: Event<T>) -> Option<Event<T>>;
}

impl<T> Resource<T> for SimpleResource<T> {
    fn allocate_or_enqueue(&mut self, event: Event<T>) -> Option<Event<T>> {
        if self.available > 0 {
            self.available -= 1;
            Some(event)
        } else {
            self.queue.push_back(event);
            None
        }
    }

    fn release_and_schedule_next(&mut self, event: Event<T>) -> Option<Event<T>> {
        match self.queue.pop_front() {
            Some(mut request_event) => {
                request_event.time = event.time();
                Some(request_event)
            }
            None => {
                assert!(self.available < self.quantity);
                self.available += 1;
                None
            }
        }
    }
}

impl<T> SimpleResource<T> {
    pub fn new(quantity: usize) -> SimpleResource<T> {
        SimpleResource {
            quantity,
            available: quantity,
            queue: VecDeque::new(),
        }
    }
}
