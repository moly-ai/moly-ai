# Widget Tree Query Returns Deepest Match Instead of Shallowest

**Status**: Unresolved upstream issue in Makepad's `WidgetTree::find_within`
**Impact**: Any widget using `self.view(cx, ids!(name))` or `self.widget(cx, &[id!(name)])` may
silently get the wrong widget when descendants share the same name.
**Workaround**: Use `Widget::child(id)` for direct-child-only lookup (see below).

## Problem Summary

When a widget has a direct child named `content`, and also contains nested descendants
(e.g. inside a modal) that themselves have children named `content`, the standard widget tree
query `self.view(cx, ids!(content))` returns the **last** (deepest) match instead of the
shallowest (direct child). This is counterintuitive — developers expect the query to return
the nearest/shallowest match, especially when a direct child with that name exists.

## How Makepad Resolves Widget Tree Queries

The call chain for `self.view(cx, ids!(content))` is:

1. **`WidgetRef::widget(cx, path)`** (`makepad/widgets/src/widget.rs:907`)
   Refreshes the widget tree cache for the caller's subtree, then calls `find_within`.

2. **`WidgetTree::find_within(root_uid, path)`** (`makepad/widgets/src/widget_tree.rs:1547`)
   Calls `find_all_within_cached_graph` to collect **all** matches, then returns
   `results.pop()` — the **last** element of the results vector.

3. **`find_all_within_cached_graph`** (`widget_tree.rs:1044`)
   First checks a path cache (HashMap). On cache miss, calls `collect_within_graph`.

4. **`collect_within_graph`** (`widget_tree.rs:953`)
   Performs a full depth-first traversal of the entire subtree rooted at `root_uid`. Every
   node whose name matches the target is pushed onto the `results` vector. Children are
   visited in declaration order (first child first), so results are ordered from
   shallowest/earliest to deepest/latest in the tree.

5. Back in `find_within`, **`results.pop()`** returns the last element — the deepest/latest
   match.

### Key Code (`widget_tree.rs:1547-1551`)

```rust
pub fn find_within(&self, root_uid: WidgetUid, path: &[LiveId]) -> WidgetRef {
    let mut inner = self.inner.borrow_mut();
    let mut results = Self::find_all_within_cached_graph(&mut inner, root_uid, path);
    results.pop().unwrap_or_else(WidgetRef::empty)
    //     ^^^^^ returns LAST match, not first
}
```

If this used `results.into_iter().next()` (or `results.swap_remove(0)`) instead of `.pop()`,
it would return the shallowest match — which is what most callers expect.

Note: `find_within_graph` (the uncached single-result version at line 860) has the same
behavior — it overwrites `result` on every match (`result = widget;` at line 917) without
returning early, so it also returns the last/deepest match.

## Our Concrete Example

### Widget Structure

`ChatHistoryCard` (a View) has this DSL structure:

```
ChatHistoryCard (View)
├── content := RoundedView { cursor: MouseCursor.Hand, ... }  ← WANT THIS
├── chat_history_card_options_modal := MolyModal {
│   └── content := View { ... }                                ← MolyModal's own content
│       └── (inner MolyModal content)
└── delete_chat_modal := MolyModal {
    └── content := View { ... }                                ← MolyModal's own content
        └── (inner MolyModal content)
```

`MolyModal` (defined in `moly-kit/src/widgets/moly_modal.rs`) has a child named `content`
in its own DSL definition. When `ChatHistoryCard` contains two MolyModals, there are **three**
widgets named `content` in its subtree.

### What Happens

```rust
// In ChatHistoryCard::handle_actions:
self.view(cx, ids!(content)).finger_down(actions)
```

`find_within` collects all three `content` matches in depth-first order:
1. `content` (direct child RoundedView) — UID 525
2. `content` (inside chat_history_card_options_modal) — UID 543
3. `content` (inside delete_chat_modal) — UID 550

Then `.pop()` returns **UID 550** — the `delete_chat_modal`'s `content`, not the card's
direct child.

### Empirical Proof

We added diagnostic logging that compared three UIDs:

```
CONTENT: query=WidgetUid(550) direct=Some(WidgetUid(525))
         modal1_content=WidgetUid(543) modal2_content=WidgetUid(550)
         query_matches_direct=false
         query_matches_modal1=false
         query_matches_modal2=true
```

This pattern was consistent across all 16 visible cards in the PortalList. The query
**always** returned the `delete_chat_modal`'s `content` (the last match).

### User-Visible Bug

Clicking a chat history card did nothing — `finger_down(actions)` was called on the wrong
widget (the modal's `content` View), which never receives FingerDown events because it's
hidden. The card's actual clickable `content` RoundedView was ignored.

## How to Reproduce in an Isolated Example

```rust
script_mod! {
    use mod.prelude.widgets.*

    // A modal-like widget that has its own `content` child
    mod.widgets.InnerWidget = View {
        content := View {
            Label { text: "I am the inner content" }
        }
    }

    load_all_resources() do #(App::script_component(vm)) {
        ui: Root {
            main_window := Window {
                body +: {
                    outer := View {
                        // Direct child named "content"
                        content := View {
                            cursor: MouseCursor.Hand
                            Label { text: "I am the OUTER content (click me)" }
                        }
                        // Nested widget that also has a "content" child
                        inner := InnerWidget {}
                    }
                }
            }
        }
    }
}

// In handle_actions:
// self.view(cx, ids!(content))  ← returns inner's content, not outer's!
```

## Workaround: Use `Widget::child()` for Direct-Child Lookup

The `Widget` trait provides `child(id: LiveId) -> WidgetRef` (`widget.rs:125`) which only
searches **direct children** — no deep subtree traversal. This returns the correct widget
regardless of name collisions in descendants.

```rust
// Instead of:
//   self.view(cx, ids!(content)).finger_down(actions)
//
// Use:
let content_ref = self.view.child(id!(content));
if let Some(item) = actions.find_widget_action(content_ref.widget_uid()) {
    if let ViewAction::FingerDown(fe) = item.cast() {
        // handle click
    }
}
```

The downside is that `child()` returns an untyped `WidgetRef`, so typed convenience methods
like `finger_down()` (which is on `ViewRef`) are not directly available. You need to manually
call `actions.find_widget_action()` and cast the action yourself, as shown above.

### Performance Comparison

| Approach | Cost |
|----------|------|
| `self.view(cx, ids!(content))` | Cache hit: O(matches) with HashMap overhead. Cache miss: full depth-first traversal of the entire subtree + cache store. Always traverses entire subtree on miss. |
| `self.view.child(id!(content))` | O(direct_children) — iterates only the immediate children list. No HashMap, no caching overhead. |

For a typical widget with 3-10 direct children, `child()` is faster in all cases. The widget
tree cache benefits repeated queries across frames, but for widgets with name collisions the
cache returns the *wrong* result, making the performance point moot.

### Alternative Workaround: Rename the Child

Renaming the child to a unique name (e.g. `card_content`) avoids the collision entirely:

```rust
// In DSL:
card_content := RoundedView { ... }

// In Rust:
self.view(cx, ids!(card_content)).finger_down(actions)
```

This works but requires changing the DSL, which may not be desirable when the name `content`
is semantically correct.

### Why Multi-Segment Queries Don't Help Here

The `ids!()` macro supports multi-segment paths like `ids!(parent_name, child_name)`, which
constrain the search to match the path hierarchy. In theory this could disambiguate — e.g.
`ids!(some_wrapper, content)` would only match a `content` that is a child of
`some_wrapper`.

However, in our case `content` is a direct child of the root widget with no named wrapper
between them. Adding a wrapper just to use multi-segment queries would be more complex than
simply renaming. And even with multi-segment queries, if a descendant happens to have a
parent with the same wrapper name, the collision reappears at a different level.

## Relevant Makepad Source Files

All paths relative to the Makepad repository root:

- **`widgets/src/widget_tree.rs:1547-1551`** — `find_within`: the `.pop()` that returns the
  last match
- **`widgets/src/widget_tree.rs:860-951`** — `find_within_graph`: uncached depth-first
  traversal that overwrites `result` on every match
- **`widgets/src/widget_tree.rs:953-1042`** — `collect_within_graph`: depth-first traversal
  that pushes all matches into a results vector
- **`widgets/src/widget_tree.rs:1044-1106`** — `find_all_within_cached_graph`: cached wrapper
- **`widgets/src/widget.rs:125-133`** — `Widget::child()`: direct-child-only lookup (the
  workaround)
- **`widgets/src/widget.rs:907-920`** — `WidgetRef::widget()`: entry point that calls
  `find_within`
- **`widgets/src/view.rs:320-327`** — `ViewRef::finger_down()`: the method that was silently
  failing due to wrong UID

## Suggested Fix for Makepad

`find_within` should return the **shallowest** match (closest to the root), not the deepest.
This matches developer intuition: if a direct child has the queried name, it should be
returned regardless of whether deeper descendants also have that name.

Options:
1. Change `results.pop()` to `results.into_iter().next()` — returns first/shallowest match.
   This is a one-line change.
2. Make `collect_within_graph` stop after the first match at the shallowest depth — same
   semantics as option 1 but avoids collecting all matches when only the first is needed.
