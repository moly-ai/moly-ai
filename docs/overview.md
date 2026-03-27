# Introduction

This document provides a high-level overview of the components that make up this project, along with useful references.

# Moly (app)

The Moly application lives at the root of this repository. It was created years ago and has gone through many versions of Makepad, so different patterns coexist in its codebase.

Originally it was an app exclusively for chatting with local models through Moly Local, but today it can be used with any remote provider as well.

The reusable parts of Moly do not live in the app itself -- they were extracted into Moly Kit so that other Makepad applications can use them.

# Moly Kit

A Makepad widget crate for building AI applications.

Its main offering is the `Chat` widget, which can be used to quickly integrate AI-powered chat into any app.

Originally Moly Kit also contained all the core logic -- AI client implementations, traits, etc. -- but this was extracted into a pure Rust crate, AITK.

Moly Kit depends on and re-exports AITK.

The latest published documentation is available at <https://moly-ai.github.io/moly-ai/>.

# AITK

AITK lives in its own repository: <https://github.com/moly-ai/aitk>.

It is a pure Rust crate (not tied to Makepad) that provides AI client implementations, MCP integration, a unified communication protocol, out-of-the-box chat state management, and more.

AITK should remain independent from Moly -- it is designed to be useful in any Rust project, not just Makepad-based GUIs. There is ongoing work tracked in the [issue tracker](https://github.com/moly-ai/aitk/issues) to further decouple the crate and improve its usability for standalone CLIs, servers, and other contexts outside of Moly.

Everything you need to know about AITK is documented at <https://moly-ai.github.io/aitk/>.

# moly-widgets

An initiative that has not been merged into `main` yet. It lives on the `moly-widgets` branch and can be previewed in the screenshots of [PR #655](https://github.com/moly-ai/moly-ai/pull/655).

The goal is to define a consistent design system for Moly and implement it as a widget crate, then use it throughout the app to fix style inconsistencies and code duplication.

# Moly Local

Lives in the repository <https://github.com/moly-ai/moly-local>.

An HTTP server for searching, downloading, and interfacing with local AI models. Similar to Ollama. It uses WasmEdge as the inference runtime for model execution.

# Moly project board

<https://github.com/orgs/moly-ai/projects/1/views/1>

Reflects the current project status, work in progress, etc.

# Moly v0.2.5-rc1

<https://github.com/moly-ai/moly-ai/releases/tag/v0.2.5-rc1>

The latest published version of Moly (not marked as latest release). It contains massive codebase changes as it is a migration to Makepad 2.0.

Release artifacts are generated from <https://github.com/moly-ai/moly-ai/actions/workflows/release.yml>.
