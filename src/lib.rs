// Copyright (c) 2025 Saugata Kundu
// Licensed under the Apache-2.0 License

mod container;
mod errors;

pub use container::Container;

pub use errors::*;
pub use shellder_macros::{Hooks, inject};
use once_cell::sync::Lazy;

pub static DEFAULT_CONTAINER: Lazy<Container> = Lazy::new(Container::new);

pub trait Hooks {
    fn startup(&self);
    fn run(&self);
    fn cleanup(&self);
}

pub trait Hookable {
    fn run_hooks(&self);
}

pub trait Injection{
    fn inject(container: &Container)->Self;
}