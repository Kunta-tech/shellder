mod container;
pub use container::Container;

use once_cell::sync::Lazy;

pub static CONTAINER: Lazy<Container> = Lazy::new(Container::new);
