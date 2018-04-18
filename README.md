# desim
desim stands for Discrete-time Events SIMulator.
It aims to be a high quality, easy to use simulation framework for physical or
logical systems that are based on processes that are triggered by events
and that can cause other events to occur.

It is inspired by the Simpy environment for Python,
but with the aim of being more efficient and to provide also a concurrent
implementation.
To achieve the same ease of use remaining light and efficient, it is based on
the experimental _generators_ (coroutine) rust feature.

You can read the [API documentation here](docs.rs/desim)

**This project is blocked waiting for some features related to _generators_.
In particular the possibility to resume the generator with arguments.
The implementation of the Clone trait for generators, such that one can clone
the instructions of the generator without copying the current state, would
much improve the usability of this crate.**

## Usage
To use the framework, add the following line to your Cargo.toml:
```
desim = "0.1"
```
Notice that a change in the last digit (patch number) means that the interface
is backward and forward compatible and contains other type of fixes, like bug
fixes or documentation updates.
A change in the middle digit (minor) means that the interface is backward
compatible but includes something new, so that the previous version may be not
forward compatible.
A change in the first, left digit may means a breacking change in the interface,
that will not be backward compatible anymore.
Version 1.0.0 may be an exception to this and may means just that the API is
stable and is considered production ready.

The simulation environment is provided by the `Simulation` struct, which exposes
methods to spawn processes, allocate resources and schedule events.
Moreover it offers getters to retrieve the current time and the ordered list of
processed events and methods to process the next event in the simulation and to
run all the events until a certain condition is verified.

A process is a generator that yields a variant of the `Effect` enum.
Using this type the process may interact with the simulation,
for example scheduling events or requesting resources.

For more information see the API documentation linked above.

## Example
The examples folder contains some simulations that use desim.
You can build and run them with `cargo run --example <name>`.

## Contributing
Feel free to contribute to this project with pull requests and/or issues.
All contribution should be under a license compatible with the GNU GPLv3.

> Why did you chose the GNU GPL instead of a more permissive license like Apache/MIT?
Because I wrote a piece of free software and I don't want it to be used as the
basis of a proprietary one. Improvements of this work or simulation software
written using desim as a library should be free software as well.

## Changes
0.1.0 First release
