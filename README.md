# 🐚 Shellder

*A lightweight, Spring-inspired Dependency Injection framework for Rust.*
***"A Crab needs its Shell"***

---

## ✨ Overview

**Shellder** brings a modular, ergonomic approach to Rust applications by offering:

* 📦 **Dependency Injection** via `#[component]` and `#[derive(App)]` macros
* ⚙️ **Lifecycle Hooks** (`startup`, `run`, `cleanup`)
* 🛠️ **Compile-time struct injection**
* 🧵 **Thread-safe container (`Arc`-based)**
* 🔧 **Optional lightweight logger**

Built for CLI tools, long-running services, or application backends.

---

## 🚀 Quick Start

### Add Shellder to your project

```toml
[dependencies]
shellder = "0.2.5"
```

---

### Example

```rust
use std::sync::Arc;
use shellder::{App, Container, Hooks, Lifecycle};

pub struct Logger;
impl Logger {
    pub fn log(&self, msg: &str) {
        println!("[LOG] {}", msg);
    }
}

#[derive(App)]
pub struct HelloApp {
    #[component]
    logger: Arc<Logger>,
}

impl Hooks for HelloApp {
    fn startup(&self) {
        self.logger.log("Starting up!");
    }

    fn run(&self) {
        self.logger.log("Running...");
    }

    fn cleanup(&self) {
        self.logger.log("Shutting down.");
    }
}
```

Running this app will automatically inject dependencies and call hooks in order.

---

## 🧩 Features in v0.2.5

✅ `#[component]` macro for auto-registration
✅ `#[derive(App)]` macro to auto-generate `main()`
✅ Lifecycle `Hooks` trait support
✅ Thread-safe singleton container
✅ Optional custom logger
✅ Works without `tokio` or async runtime

---

## 🛠️ Planned Roadmap

* 🔖 Named registration & resolution (`"db", "logger"`)
* 🧪 Test container for mocking dependencies
* 🧬 Config loader (from `.toml` or `.yaml`)
* 🧱 Fluent container builder
* 🧩 Macro plugins: `#[value]`, `#[config]`, etc.
* 🧵 Async hook support
* 📁 Workspace support & automatic example runner

---

## 💡 Philosophy

Shellder follows a **Rust-first** vision:

* ✅ Strong compile-time guarantees
* 🧼 Minimal runtime dependencies
* 🛠️ Clean syntax with macros
* 🤝 Integrates well with your architecture

---

## 📝 License

Licensed under **Apache-2.0**.

---

## 🤝 Contributing

Bug reports, PRs, and feedback are welcome!
Open an issue to discuss enhancements or integrations.

---

## 📣 Stay Tuned

Shellder is under active development.
Watch the repo for updates on:

* Configuration loading
* DI graphs
* CLI/web framework integrations
* Logging plugins
