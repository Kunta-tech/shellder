// Copyright (c) 2025 Saugata Kundu - kundusaugata576@gmail.com
// Licensed under the Apache-2.0 License

mod container;
mod errors;
mod logger;
pub use logger::{Logger,CliLogger};
pub use container::Container;

pub use errors::*;
pub use shellder_macros::{Hooks, Inject, App};
use once_cell::sync::Lazy;

pub static DEFAULT_CONTAINER: Lazy<Container> = Lazy::new(Container::new);

pub trait Hooks {
    fn startup(&self);
    fn run(&self);
    fn cleanup(&self);
}

pub trait Lifecycle {
    fn run_hooks(&self);
}
