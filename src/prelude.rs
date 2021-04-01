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

//! The desim prelude.
//!
//! The prelude is a list of things that you can import into your program with a single `use`.
//! It includes types which are used in almost every desim simulation program.
//! 
//! To use the prelude in your simulation simply add in your source
//! ```rust
//! use desim::prelude::*;
//! ```
//!
//! You can find this used in some example programs in the `examples`
//! directory of the desim repository.

pub use crate::Effect;
pub use crate::EndCondition;
pub use crate::Event;
pub use crate::Process;
pub use crate::ResourceId;
pub use crate::SimContext;
pub use crate::SimState;
pub use crate::Simulation;
