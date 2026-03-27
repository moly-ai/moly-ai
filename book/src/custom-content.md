# Custom Content

By default, Moly Kit renders bot messages using the `StandardMessageContent` widget,
which handles markdown, thinking blocks, and citations.

Some providers return content in formats that require specialized rendering beyond
what the standard widget supports. The `CustomContent` trait lets you provide your
own Makepad widget for these cases.

## Prerequisites

This guide assumes you have read the [Quickstart](quickstart.md) and are comfortable
with Makepad widget development.

## Overview

1. Create a Makepad widget for your custom content.
2. Implement the `CustomContent` trait.
3. Register it on the `Messages` widget inside `Chat`.

## Step 1: Create a widget

Create a standard Makepad widget that can display your content. It should use
`height: Fit` to avoid layout issues within the message list.

```rust
use makepad_widgets::*;
use moly_kit::prelude::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.MyCustomContentBase = #(MyCustomContent::register_widget(vm))
    mod.widgets.MyCustomContent = set_type_default() do mod.widgets.MyCustomContentBase {
        height: Fit
        label := Label {}
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct MyCustomContent {
    #[deref]
    deref: View,
}

impl Widget for MyCustomContent {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope)
    }
}

impl MyCustomContent {
    pub fn set_content(&mut self, cx: &mut Cx, content: &MessageContent) {
        self.label(cx, ids!(label)).set_text(cx, &content.text);
    }
}
```

## Step 2: Implement `CustomContent`

The `CustomContent` trait has a single method:

```rust
pub trait CustomContent {
    fn content_widget(
        &mut self,
        cx: &mut Cx,
        previous_widget: WidgetRef,
        content: &MessageContent,
    ) -> Option<WidgetRef>;
}
```

Return `Some(widget)` when your implementation should handle the given content, or
`None` to fall through to the default rendering. The `previous_widget` parameter is
the widget currently in the slot -- reuse it when possible to preserve state.

Here is an example that checks `content.data` to decide whether to handle the
message:

```rust
pub struct MyContentProvider {
    template: ScriptObjectRef,
}

impl MyContentProvider {
    pub fn new(template: ScriptObjectRef) -> Self {
        Self { template }
    }
}

impl CustomContent for MyContentProvider {
    fn content_widget(
        &mut self,
        cx: &mut Cx,
        previous_widget: WidgetRef,
        content: &MessageContent,
    ) -> Option<WidgetRef> {
        // Only handle messages with specific data.
        let data = content.data.as_deref()?;
        if !data.contains("my_custom_format") {
            return None;
        }

        // Reuse the existing widget if possible, otherwise create from template.
        let widget = if previous_widget
            .as_my_custom_content()
            .borrow()
            .is_some()
        {
            previous_widget
        } else {
            cx.with_vm(|vm| {
                let value: ScriptValue = self.template.as_object().into();
                WidgetRef::script_from_value(vm, value)
            })
        };

        widget
            .as_my_custom_content()
            .borrow_mut()
            .unwrap()
            .set_content(cx, content);

        Some(widget)
    }
}
```

## Step 3: Register it

Register the custom content provider on the `Messages` widget inside `Chat`:

```rust
self.chat(cx, ids!(chat))
    .read()
    .messages_ref(cx)
    .write()
    .register_custom_content(MyContentProvider::new(template));
```

You can register multiple `CustomContent` implementations. At draw time, the
`Messages` widget iterates through them in order. The first one to return
`Some(widget)` for a given message wins. If none match, the default
`StandardMessageContent` is used.

## Real-world example

The [Moly app](https://github.com/moly-ai/moly-ai) uses this mechanism for its
DeepInquire integration. The `DeepInquireCustomContent` checks if a message's `data`
field contains DeepInquire-formatted JSON and, if so, renders it with a specialized
multi-stage widget instead of the standard markdown view.
