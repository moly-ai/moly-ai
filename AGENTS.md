# Important context
- Moly is an LLM app that allows users to interact with multiple AI providers, both local and remote.
- MolyKit is an UI kit of reusable AI components, mainly a Chat widget, that allows adding an AI-assisted chat to any Makepad application.
- AITK provides the core types (protocol), built-in API clients, etc. It works on CLIs, GUIs, servers, and is not tied to Makepad.
- It's most important that the features we add to MolyKit and AITK are generic and reusable so that they can
be leveraged by multiple providers and clients.

# Implementation Requirements 
- All features must compile for all platforms, including desktop, mobile and web. 
Certain features are not yet supported in web therefore locked behind cfg flags.
- Async Runtime & Spawning:
  - AITK provides a `spawn` function handling platform differences (Tokio+Send on native, wasm-bindgen+!Send on web).
  - While AITK is runtime-agnostic, MolyKit currently relies on AITK's `spawn`. This may change in the future to allow more flexibility.
  - The Moly app uses AITK's `spawn` for simplicity. As an application (not a reusable library), it simply uses the spawner provided by AITK.
- Async Primitives: Favor `futures` crate channels over `tokio` ones for better cross-platform support.

# Code style
- Avoid unnecessary or obvious comments.
- Favor simple and elegant solutions over over-engineered ones.
- Keep library code generic and reusable.
- Limit line length to 100 characters (rustfmt default).
- Maximize code reuse (DRY).
- No extra code beyond what is absolutely necessary to solve the problem the user provides. Ask for missing details
  and suggest next steps to the user if appropriate.

# Documentation

- Must include doc comments for all public functions, structs, enums, and methods in library code.
- Must document errors and panics where applicable.
- Keep comments up-to-date with code changes.

# Type System

- Must leverage Rust's type system to prevent bugs at compile time.
- Use newtypes to distinguish semantically different values of the same underlying type.
- Prefer `Option<T>` over sentinel values.

# Error Handling

- Never use .unwrap() in library code; use .expect() only for invariant violations with a descriptive message.
- Define meaningful error types in library code.

# Function Design

- Must keep functions focused on a single responsibility.
- If ownership is not required, prefer borrowing parameters (&T, &mut T).
- If ownership is required, prefer taking parameters by value (T) over references (which would end up in unnecessary cloning).
- Limit function parameters to 5 or fewer; use a config struct for more.
- Return early to reduce nesting.
- Use iterators and combinators over explicit loops where clearer.

# Struct and Enum Design

- Must keep types focused on a single responsibility.
- Must derive common traits: Debug, Clone, PartialEq where appropriate.
- Use `#[derive(Default)]` when a sensible default exists.
- Prefer composition over inheritance-like patterns.
- Use builder pattern for complex struct construction.

# Rust Best Practices

- Never use unsafe unless absolutely necessary; document safety invariants when used.
- Must use pattern matching exhaustively; avoid catch-all _ patterns when possible.
- Must use format! macro for string formatting.
- Use iterators and iterator adapters over manual loops.
- Use enumerate() instead of manual counter variables.
- Prefer if let and while let for single-pattern matching.

# Memory and Performance

- Must avoid unnecessary allocations; prefer &str over String when possible.
- Must use Cow<'_, str> when ownership is conditionally needed.
- Use Vec::with_capacity() when the size is known.
- Prefer stack allocation over heap when appropriate.
- Use Arc and Rc judiciously; prefer borrowing.
