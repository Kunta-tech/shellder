// Copyright (c) 2025 Saugata Kundu
// Licensed under the Apache-2.0 License

mod container;
mod resolve_error;
pub use container::Container;

pub use shellder_macros::component;
use once_cell::sync::Lazy;

pub static CONTAINER: Lazy<Container> = Lazy::new(Container::new);

pub trait Registerable {
    fn register(container: &Container);
}

pub trait Hooks {
    fn startup(&self);
    fn run(&self);
    fn cleanup(&self);
}
