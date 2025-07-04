# Mycelium

*A lightweight, Spring-inspired Dependency Injection framework for Rust.*

---

## ✨ Overview

Mycelium provides a minimal, ergonomic foundation for building modular Rust applications with Dependency Injection.

The core design focuses on:

- **Type-based registration and resolution**
- **Lazy or eager singleton initialization**
- **Thread-safe storage with `Arc`**
- **Simple, predictable API**

This crate is ideal for applications and services where you want to decouple components and manage lifecycles explicitly.

---

## 🚀 Quick Start

Add Mycelium to your `Cargo.toml`:

```toml
[dependencies]
mycelium = "0.1"
```

# 🧩 Current Features
✅ Type-based registration and resolution
✅ Eager singleton registration
✅ Lazy initialization (factories run on first resolve)
✅ Thread-safe interior mutability
✅ Minimal dependencies (once_cell, thiserror)

# 🛣️ Roadmap / Upcoming Features
Below are planned improvements:

- Named Registration & Resolution

Allow mapping components with string keys.

Enable multiple instances of the same type.

- Configuration Management

Load .toml / .yaml configs into typed structs automatically.

- Container Builder

Fluent API for building and wiring dependencies.

- Lifecycle Hooks

start() and stop() methods for components to manage initialization/cleanup.

- Procedural Macros

#[component] and #[inject] derive macros to reduce boilerplate.

- Application Runner

mycelium::run() helper for simple startup and shutdown logic.

- Testing Utilities

Helpers to swap dependencies with mocks for unit testing.

# 💡 Vision
Mycelium aims to be a batteries-included application framework inspired by Spring, but fully Rust-idiomatic:

- Clear compile-time safety
- Minimal runtime overhead
- Ergonomic macros and configuration
- Easy integration with web frameworks and CLI tools

# 📝 License
Licensed under MIT OR Apache-2.0.

# 🤝 Contributing
Contributions and feedback are very welcome!

# 📣 Stay Tuned
Follow the project for upcoming releases with configuration loading, lifecycle management, and macro support.
