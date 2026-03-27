//! # Description
//!
//! Moly Kit is a Rust crate containing widgets and utilities to streamline the development
//! of artificial intelligence applications for the [Makepad](https://github.com/makepad/makepad)
//! framework.
//!
//! # Features
//!
//! - ⚡️ Low-config `Chat` widget that works almost out of the box.
//! - 🎨 Customize appearance thanks to Makepad DSL overrides.
//! - 🧩 Extensible with custom message contents.
//! - 🌎 Web support.
//! - 🧱 Built on top of [AITK](https://github.com/moly-ai/aitk).
//!
//! To learn how to use and integrate Moly Kit into your own Makepad app, read the
//! [documentation](https://moly-ai.github.io/moly-ai).

pub mod utils;
pub mod widgets;

pub use aitk;

pub mod prelude;
