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

## Usage
To use the framework, add the following line to your Cargo.toml:
```
desim = "0.1"
```
Version numbers follow the [semver](https://semver.org/) convention.

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
0.2.0 With generators resume arguments support, add a Simulation Context that is passed to processes on resume and can be used to retrieve the simulation time or the 
0.1.0 First release
