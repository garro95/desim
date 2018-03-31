# desim
desim, that is Discrete-time Events SIMulator, aims to be a high quality, easy to use simulation framework for physical or logical systems that are based on processes that are triggered by events and that can cause other events to occur.

It is inspired by the Simpy environment for Python, but with the aim of being more efficient and to provide also a concurrent implementation. To achieve the same ease of use remaining light and efficient, it is based on the experimental generators (coroutine) rust feature.

You can read the [API documentation here](docs.rs/desim)

## Usage
To use the framework, add the following line to your Cargo.toml:
```
desim = 0.1.0
```
Notice that a change in the last digit (patch number) means that the interface is backward and forward compatible and contains other type of fixes, like bug fixes or documentation updates. A change in the middle digit (minor) means that the interface is backward compatible but includes something new, so that the previous version may be not forward compatible. A change in the first, left digit may means a breacking change in the interface, that will not be backward compatible anymore. Version 1.0.0 may be an exception to this and may means just that the API is stable and is considered production ready.

The simulation environment is provided by the `Simulation` struct, which exposes methods to spawn processes, allocate resources and schedule events. Moreover it offers getters to retrieve the current time and the ordered list of processed events and methods to process the next event in the simulation and to run all the events until a certain condition is verified.

A process is a generator that yields a variant of the `Effect` struct.

## Example

## Contributing
Feel free to contribute to this project with pull requests and/or issues. All contribution should be under a license compatible with the GNU GPLv3.

## Changes
0.1.0 First release