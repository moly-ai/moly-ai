# Widget::text() cannot delegate to child widgets (missing Cx)

**Status**: Unresolved upstream issue in Makepad's `Widget` trait
**Impact**: Any composite widget that stores its text in a child `TextInput` cannot
implement `Widget::text()` correctly.

## Problem

`Widget::text(&self) -> String` does not receive `Cx`, but in the Splash system, querying
child widgets requires `Cx` to refresh the widget tree cache. This means composite widgets
that delegate their text to an inner `TextInput` cannot implement `text()`.

`set_text(&mut self, cx: &mut Cx, v: &str)` works fine because it receives `cx`.

> Makepad's own `CommandTextInput` has this exact problem — its `text()` returns
`String::new()` with a comment acknowledging the limitation.

## Workarounds

- **Duplicate the string in the parent widget.** The parent keeps a copy of the text, updated
by handling the child's text-change event. Downsides: programmatic `set_text()` calls on the
child that don't fire a change event will desync the copy. Also wastes memory — each widget
in a delegation chain must keep its own copy of the string, so the deeper the chain the more
redundant copies exist.

- **Leak implementation details to callers.** Instead of calling `widget.text()`, callers query
the inner child directly via widget tree queries
(e.g. `widget.read().text_input(cx, ids!(text_input)).text()`). A helper method (e.g. `widget.read().text_input_ref(cx).text()`) can wrap this to avoid
repeating the query, but either way it couples callers to the widget's internal structure.

- **Implement `text()` as a concrete method instead of a `Widget` trait method.** Each composite
widget defines its own `text(&self, cx: &Cx) -> String` with the appropriate signature. This
avoids the trait limitation but loses the uniform `WidgetRef::text()` API.

## Possible upstream solutions

- **Add `cx` to `Widget::text()`.** Change the signature to `fn text(&self, cx: &Cx) -> String`.
This is the most direct fix but requires updating every `Widget::text()` impl and every call
site across Makepad and downstream projects.

- **Remove `text()` from the `Widget` trait entirely.** Make `text()` a method specific to each
widget (with its own signature and arguments). This acknowledges that not all widgets have a
meaningful `text()` and avoids forcing a one-size-fits-all signature.
  - The same reasoning could
apply to `set_text()` and other trait methods that are only meaningful for specific widget
types.

- **Remove the `Cx` requirement from widget tree queries.** If child widget queries could work
  without `Cx` (as they did in the old `live_design!` system), composite widgets could
  delegate `text()` to their children with the current trait signature. This would eliminate
  the problem entirely rather than working around it.

## Relevant Makepad source

- `widgets/src/widget.rs:303-305` — `Widget::text()` default impl (returns `String::new()`)
- `widgets/src/widget.rs:307` — `Widget::set_text()` (receives `cx`, works correctly)
- `widgets/src/widget.rs:1072-1078` — `WidgetRef::text()` (delegates to `Widget::text()`)
- `widgets/src/command_text_input.rs:219-224` — `CommandTextInput::text()` returns
  `String::new()` with the same limitation
