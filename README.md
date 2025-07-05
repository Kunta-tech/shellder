# 🐚 Shellder

*A lightweight, Spring-inspired Dependency Injection framework for Rust.*

*A Crab needs its Shell.*

---

## ✨ Overview

Shellder provides a minimal, ergonomic foundation for building modular Rust applications with Dependency Injection and lifecycle management.

The core design focuses on:

- **Type-based registration and resolution**
- **Lazy or eager singleton initialization**
- **Thread-safe storage with `Arc`**
- **Simple, predictable API**
- **Ergonomic procedural macros**

This crate is ideal for applications and services where you want to decouple components and manage lifecycles explicitly.

---

## 🚀 Quick Start

Add Shellder to your `Cargo.toml`:

```toml
[dependencies]
shellder = "0.2.0"
```

## Example usage:
```rust
use shellder::{component, Hooks, DEFAULT_CONTAINER};

#[component]
#[derive(Hooks)]
pub struct MyService;

fn main() {
    MyService::register(&*DEFAULT_CONTAINER);
    MyService::run();
}

```


# 🧩 Current Features
✅ Type-based registration and resolution
✅ Eager singleton registration
✅ Lazy initialization (factories run on first resolve)
✅ Thread-safe interior mutability
✅ #[component] macro to auto-register components
✅ #[derive(Hooks)] for default lifecycle behavior
✅ Minimal dependencies (once_cell, thiserror, syn, quote)

# 🛣️ Roadmap / Upcoming Features
Below are planned improvements:

- Named Registration & Resolution

    - Map components with string keys

    - Enable multiple instances of the same type

- Configuration Management

    - Load .toml / .yaml configs into typed structs automatically

- Container Builder

    - Fluent API for building and wiring dependencies

- Advanced Lifecycle Hooks

    - start() and stop() methods for components to manage initialization and cleanup

- Procedural Macros

    -#[inject] derive macros to reduce boilerplate

    - Attribute macros for custom lifecycles

- Application Runner

    - shellder::run() helper for simple startup and shutdown orchestration

- Testing Utilities

    - Helpers to swap dependencies with mocks for unit testing

# 💡 Vision
Shellder aims to be a batteries-included application framework inspired by Spring, but fully Rust-idiomatic:

- Clear compile-time safety

- Minimal runtime overhead

- Ergonomic macros and configuration

- Easy integration with web frameworks and CLI tools

# 📝 License
Licensed under Apache-2.0.

# 🤝 Contributing
Contributions, ideas, and feedback are very welcome!
Please open an issue or pull request.

# 📣 Stay Tuned
Follow the project for upcoming releases with configuration loading, lifecycle management, and expanded macro support.