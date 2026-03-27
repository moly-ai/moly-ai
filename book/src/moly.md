# Moly

**Moly** is a Makepad app for interacting with local and remote LLMs. You can learn
more about it on [GitHub](https://github.com/moly-ai/moly-ai).

**Moly Kit** is a Rust crate with [Makepad](https://github.com/makepad/makepad)
widgets for building AI applications. It is built on top of
[aitk](https://github.com/moly-ai/aitk), which provides framework-agnostic core
types, API clients, and state management for working with AI models.

Moly Kit takes aitk's foundation and wraps it in ready-to-use Makepad widgets --
most notably a batteries-included `Chat` widget.

The following chapters are dedicated to **Moly Kit, the crate**. They assume you have
read the [aitk documentation](https://moly-ai.github.io/aitk/) and are familiar with
its core concepts (`BotClient`, `ChatController`, `Message`, `MessageContent`, plugins,
etc.). These tutorials focus on how to use aitk with Moly Kit's Makepad widgets.
