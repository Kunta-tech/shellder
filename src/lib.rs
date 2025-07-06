// Copyright (c) 2025 Saugata Kundu
// Licensed under the Apache-2.0 License

mod container;
mod errors;
pub use container::Container;

pub use errors::*;
pub use shellder_macros::{component, Hooks};
use once_cell::sync::Lazy;

pub static DEFAULT_CONTAINER: Lazy<Container> = Lazy::new(Container::new);

pub trait Hooks {
    fn startup(&self);
    fn run(&self);
    fn cleanup(&self);
}
