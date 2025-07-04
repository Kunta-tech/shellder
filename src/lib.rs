// Copyright (c) 2025 Saugata Kundu
// Licensed under the Apache-2.0 License

mod container;
mod resolve_error;
pub use container::Container;

use once_cell::sync::Lazy;

pub static CONTAINER: Lazy<Container> = Lazy::new(Container::new);
