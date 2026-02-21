# Moly Migration Handbook

> AI-agent-focused reference for migrating Moly and MolyKit from the
> old Makepad `live_design!` / `Live` / `LiveHook` system to the new
> Splash / `script_mod!` / `Script` / `ScriptHook` system.
>
> **Companion docs:**
> - `splash-language-reference.md` — full Splash syntax
> - `migration-guide-old-to-splash.md` — pattern-by-pattern translation
> - `splash-patterns-and-examples.md` — idiomatic patterns with code

---

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Migration Strategy](#migration-strategy)
- [Migration Phases](#migration-phases)
- [Phase 0: Foundation (data layer, actions, styles)](#phase-0-foundation)
- [Phase 1: MolyKit Widgets](#phase-1-molykit-widgets)
- [Phase 2: Moly Shared Layer](#phase-2-moly-shared-layer)
- [Phase 3: Moly Feature Screens](#phase-3-moly-feature-screens)
- [Phase 4: App Root and Integration](#phase-4-app-root-and-integration)
- [File-by-File Reference](#file-by-file-reference)
- [Tricky Migration Cases](#tricky-migration-cases)
- [Pattern Quick-Reference](#pattern-quick-reference)
- [Verification Checklist](#verification-checklist)

---

## Architecture Overview

### Moly App (`moly/src/`)

```
app.rs ─── Root widget, Window, main navigation
│
├── shared/          ─── Styles, reusable widgets, actions, utilities
│   ├── styles.rs         Theme constants, colors, typography
│   ├── widgets.rs        Custom widgets (MolyButton, MolyCodeView, etc.)
│   ├── resource_imports  Font + icon imports
│   ├── tooltip.rs        DrawList2d overlay tooltip
│   ├── popup_notification DrawList2d non-blocking popup
│   └── actions.rs        Global action enums
│
├── chat/            ─── Chat UI (largest module, 16 files)
│   ├── chat_screen.rs       Desktop chat layout
│   ├── chat_screen_mobile   Mobile chat layout
│   ├── chat_view.rs         Core chat rendering (PortalList + custom shader)
│   ├── chat_history*.rs     Chat list sidebar (ComponentMap not used here)
│   ├── chats_deck.rs        Multi-chat tab management
│   ├── chat_params.rs       Temperature/top-p sliders
│   ├── deep_inquire_*.rs    Multi-stage AI reasoning UI (complex animator)
│   └── entity_button.rs     Inline clickable entities
│
├── landing/         ─── Model discovery (13 files)
│   ├── landing_screen.rs    Main landing page
│   ├── model_list.rs        Search results list (PortalList)
│   ├── model_card.rs        Individual model card
│   ├── model_files*.rs      File list (ComponentMap + LivePtr)
│   ├── search_bar.rs        Debounced search (Timer + Animator)
│   └── downloads.rs         Download progress (Animator)
│
├── settings/        ─── Provider configuration (8 files)
│   ├── providers.rs         Provider list (LiveDependency for icons)
│   ├── provider_view.rs     Edit provider details
│   └── *_modal.rs           Various modal dialogs
│
├── my_models/       ─── Downloaded model management (6 files)
│   ├── my_models_screen.rs  Main screen
│   ├── downloaded_files_*   Table with Scope::with_props
│   └── delete_model_modal   Confirmation dialog
│
├── mcp/             ─── MCP server management (3 files)
│
└── data/            ─── State management (NO UI, 15 files)
    ├── store.rs             Central Store struct
    ├── providers.rs         Provider state (Live/LiveHook derives)
    └── ...                  Chats, downloads, search, preferences
```

### MolyKit (`moly/moly-kit/src/`)

```
lib.rs ─── Crate root, live_design! registration chain
│
├── widgets.rs ─── Widget registration + cx.link() theme aliasing
│
├── widgets/
│   ├── chat.rs              Top-level Chat widget
│   ├── messages.rs          Message list (PortalList)
│   ├── chat_line.rs         Single message row
│   ├── prompt_input.rs      Text input area
│   ├── model_selector*.rs   Model dropdown (ComponentMap)
│   ├── moly_modal.rs        DrawList2d overlay modal
│   ├── slot.rs              Replaceable content (#[wrap])
│   ├── message_loading.rs   Loading animation (Timer + Animator)
│   ├── message_thinking*.rs Thinking block (Timer + Animator)
│   ├── message_markdown.rs  Markdown config DSL
│   ├── standard_message*.rs Standard message renderer
│   ├── image_view.rs        Image display
│   ├── avatar.rs            User/bot avatar
│   ├── citation*.rs         Citation links
│   ├── attachment*.rs       File attachments
│   ├── stt_input.rs         Speech-to-text (Timer)
│   ├── realtime.rs          Realtime voice chat (Timer interval)
│   └── theme_*.rs           Theme definitions
│
└── utils/
    └── makepad/
        ├── ui_runner.rs     Async→UI bridge extensions
        ├── portal_list.rs   PortalList utilities
        ├── hits.rs          Hit test helpers
        └── events.rs        Event utilities
```

### Data Flow

```
Store (data/) ──Scope::with_data──▶ App (app.rs)
                                      │
                           ┌──────────┼──────────┐
                           ▼          ▼          ▼
                       chat/      landing/    settings/
                           │
              Scope::with_props for list items
                           │
                           ▼
                    MolyKit widgets (via Slot + ChatController)
```

---

## Migration Strategy

### Guiding Principles

1. **Bottom-up**: Migrate leaf widgets first, then composites, then
   screens, then the app root. This way each migrated widget can be
   tested in isolation.

2. **MolyKit before Moly**: MolyKit is a dependency. Migrate it first
   so the Moly app can consume the new API.

3. **Data layer is trivial**: Files in `data/` that use `Live` derives
   only need derive renames — no DSL changes.

4. **One file at a time**: Each file with `live_design!` becomes a
   `script_mod!`. The Rust struct derives change. Test compilation
   after each file.

5. **Registration order**: The `App::run()` (currently `app_main!` +
   `LiveRegister`) must call `script_mod` functions in dependency
   order. This is determined by widget usage — if module A uses a
   widget defined in module B, then B's `script_mod` must run first.

### What Changes Per File

For every file with `live_design!`:

| Old | New |
|-----|-----|
| `live_design! { ... }` | `script_mod! { ... }` |
| `use link::theme::*;` | `use mod.prelude.widgets.*` (or `widgets_internal.*` for widget defs) |
| `use link::widgets::*;` | (merged into the above) |
| `use link::my_crate::*;` | `use mod.widgets.*` (or specific path) |
| `<WidgetType> { ... }` | `WidgetType{ ... }` |
| `name = <Type> { }` | `name := Type{ }` |
| `prop: value,` | `prop: value` (no trailing comma) |
| `(THEME_REF)` | `theme.ref_name` |
| `dep("crate://self/path")` | `crate_resource("self://path")` |
| `instance hover: 0.0` | `hover: instance(0.0)` |
| `fn pixel(self) -> vec4 { }` | `pixel: fn() { }` |
| `;` in shaders | removed |
| `draw_bg: { ... }` (merge) | `draw_bg +: { ... }` |
| `{{StructName}}` | `#(Struct::register_widget(vm))` in script_mod |
| `#[derive(Live, LiveHook)]` | `#[derive(Script, ScriptHook)]` |
| `#[derive(Widget)]` | `#[derive(Widget)]` (unchanged) |
| `#[derive(DefaultNone)]` | `#[derive(Default)]` + `#[default]` on None |
| `#[animator]` | `#[apply_default]` + add `Animator` to derives |
| `#[live_ignore]` | removed |
| `apply_over(cx, live!{...})` | `script_apply_eval!(cx, item, {...})` |
| `Pal::iq(...)` | `Pal.iq(...)` |
| `cursor: Hand` | `cursor: MouseCursor.Hand` |

---

## Migration Phases

### Phase 0: Foundation

**Goal:** Migrate non-UI code and shared style definitions.

**Files (13 total):**

| File | Complexity | Notes |
|------|-----------|-------|
| `data/providers.rs` | Low | `derive(Live, LiveHook)` → `derive(Script, ScriptHook)` on Provider enum |
| `data/store.rs` | Low | `derive(DefaultNone)` on StoreAction → `derive(Default)` + `#[default]`; LiveDependency pattern |
| `data/moly_client.rs` | Low | `derive(DefaultNone)` only |
| `data/deep_inquire_client.rs` | Low | `LivePtr` usage — just type, no DSL |
| `shared/actions.rs` | Low | `derive(DefaultNone)` on action enums |
| `shared/styles.rs` | Medium | `live_design!` with theme constants → `script_mod!` with `mod.moly` namespace |
| `shared/resource_imports.rs` | Medium | `live_design!` with `dep()` → `crate_resource()` |
| `shared/desktop_buttons.rs` | Low | DSL-only, no Rust structs |
| `landing/shared.rs` | Low | DSL-only constants |
| `chat/shared.rs` | Low | DSL-only shared components + simple widget |
| `chat/model_info.rs` | Low | DSL-only |

**Order:** Actions → Data layer → Styles → Resources → DSL-only files

### Phase 1: MolyKit Widgets

**Goal:** Migrate all MolyKit widgets. This must be done before Moly
because Moly depends on MolyKit.

**Tier 1 — Leaf widgets (no MolyKit deps):**

| File | Complexity | Patterns |
|------|-----------|----------|
| `widgets/slot.rs` | Low | `#[wrap]`, LiveHook |
| `widgets/avatar.rs` | Low | Standard widget |
| `widgets/citation.rs` | Low | DefaultNone action |
| `widgets/citation_list.rs` | Low | Standard widget |
| `widgets/image_view.rs` | Low | apply_over |
| `widgets/attachment_view.rs` | Low | apply_over |
| `widgets/attachment_list.rs` | Low | Standard widget |
| `widgets/attachment_viewer_modal.rs` | Low | Standard widget |

**Tier 2 — Widgets with animation/timers:**

| File | Complexity | Patterns |
|------|-----------|----------|
| `widgets/message_loading.rs` | Medium | `#[animator]`, Timer, self-rescheduling |
| `widgets/message_thinking_block.rs` | Medium | `#[animator]`, Timer, apply_over |
| `widgets/model_selector_item.rs` | Medium | `#[animator]`, DefaultNone |
| `widgets/stt_input.rs` | Medium | Timer, DefaultNone |

**Tier 3 — Widgets with complex patterns:**

| File | Complexity | Patterns |
|------|-----------|----------|
| `widgets/moly_modal.rs` | High | DrawList2d, sweep_lock, apply_over |
| `widgets/model_selector_list.rs` | Medium | ComponentMap, LivePtr |
| `widgets/prompt_input.rs` | Medium | apply_over, custom draw |
| `widgets/messages.rs` | High | PortalList, DefaultNone, apply_over, complex draw_walk |
| `widgets/chat_line.rs` | Medium | DefaultNone, delegates to Slot |

**Tier 4 — Composite / theme:**

| File | Complexity | Patterns |
|------|-----------|----------|
| `widgets/standard_message_content.rs` | Low | Standard widget |
| `widgets/message_markdown.rs` | Low | DSL-only |
| `widgets/model_selector.rs` | Medium | apply_over, WidgetMatchEvent |
| `widgets/chat.rs` | Medium | Top-level composite |
| `widgets/realtime.rs` | High | Timer interval, audio, DefaultNone |
| `widgets/theme_moly_kit_light.rs` | Medium | cx.link theme aliasing |
| `widgets.rs` | Medium | Registration chain + cx.link |

**Order:** Tier 1 → Tier 2 → Tier 3 → Tier 4 → `widgets.rs` → `lib.rs`

### Phase 2: Moly Shared Layer

**Goal:** Migrate shared widgets and utilities.

| File | Complexity | Patterns |
|------|-----------|----------|
| `shared/widgets.rs` | Medium | DSL-only, defines MolyButton (custom vertex shader), MolyCodeView (#[wrap]) |
| `shared/tooltip.rs` | High | DrawList2d overlay, apply_over |
| `shared/popup_notification.rs` | High | DrawList2d non-blocking overlay |
| `shared/external_link.rs` | Low | WidgetMatchEvent |
| `shared/list.rs` | Low | Standard widget |
| `shared/meta.rs` | Low | Standard widget |
| `shared/moly_server_popup.rs` | Medium | DefaultNone, WidgetMatchEvent |
| `shared/download_notification_popup.rs` | Medium | DefaultNone, WidgetMatchEvent |

**Order:** DSL-only (widgets.rs) → Leaf (external_link, list, meta)
→ Popups → DrawList2d overlays (tooltip, popup_notification)

### Phase 3: Moly Feature Screens

**Goal:** Migrate all feature screens bottom-up within each module.

#### Settings (8 files)

| File | Complexity | Patterns |
|------|-----------|----------|
| `settings/moly_server_screen.rs` | Low | WidgetMatchEvent |
| `settings/add_provider_modal.rs` | Medium | DefaultNone, WidgetMatchEvent |
| `settings/utilities_modal.rs` | Medium | DefaultNone, WidgetMatchEvent |
| `settings/sync_modal.rs` | Medium | DefaultNone, WidgetMatchEvent |
| `settings/provider_view.rs` | Medium | DefaultNone, apply_over |
| `settings/providers.rs` | Medium | DefaultNone, apply_over, LiveDependency |
| `settings/providers_screen.rs` | Low | WidgetMatchEvent |

**Order:** moly_server_screen → modals → provider_view → providers
→ providers_screen

#### My Models (6 files)

| File | Complexity | Patterns |
|------|-----------|----------|
| `my_models/delete_model_modal.rs` | Low | DefaultNone |
| `my_models/model_info_modal.rs` | Medium | DefaultNone |
| `my_models/downloaded_files_row.rs` | Low | WidgetMatchEvent |
| `my_models/downloaded_files_table.rs` | Medium | WidgetMatchEvent, Scope::with_props |
| `my_models/my_models_screen.rs` | Medium | DefaultNone |

**Order:** delete_model_modal → model_info_modal → row → table
→ screen

#### MCP (3 files)

| File | Complexity | Patterns |
|------|-----------|----------|
| `mcp/mcp_servers.rs` | Low | WidgetMatchEvent |
| `mcp/mcp_screen.rs` | Low | WidgetMatchEvent |

**Order:** mcp_servers → mcp_screen

#### Landing (13 files)

| File | Complexity | Patterns |
|------|-----------|----------|
| `landing/shared.rs` | Low | DSL-only |
| `landing/download_item.rs` | Low | apply_over |
| `landing/sorting.rs` | Low | apply_over |
| `landing/model_card.rs` | Medium | DefaultNone |
| `landing/model_files_tags.rs` | Medium | ComponentMap, LivePtr, apply_over |
| `landing/model_files_item.rs` | Medium | DefaultNone, apply_over |
| `landing/model_files_list.rs` | High | ComponentMap, LivePtr, manual Widget/WidgetNode impl |
| `landing/model_files.rs` | Medium | `#[animator]`, apply_over |
| `landing/search_loading.rs` | Medium | `#[animator]`, Timer |
| `landing/search_bar.rs` | Medium | `#[animator]`, Timer, apply_over |
| `landing/model_list.rs` | Medium | DefaultNone, apply_over, Timer |
| `landing/downloads.rs` | Medium | `#[animator]`, apply_over |
| `landing/landing_screen.rs` | Medium | WidgetMatchEvent |

**Order:** shared → download_item → sorting → model_card
→ model_files_tags → model_files_item → model_files_list
→ model_files → search_loading → search_bar → model_list
→ downloads → landing_screen

#### Chat (16 files)

| File | Complexity | Patterns |
|------|-----------|----------|
| `chat/shared.rs` | Low | DSL + simple widget |
| `chat/model_info.rs` | Low | DSL-only |
| `chat/entity_button.rs` | Low | Standard widget |
| `chat/deep_inquire_content.rs` | Low | Standard widget |
| `chat/chat_params.rs` | Low | WidgetMatchEvent |
| `chat/chat_history_card_options.rs` | Low | WidgetMatchEvent |
| `chat/chat_history_card.rs` | Medium | DefaultNone, apply_over |
| `chat/chat_history.rs` | Medium | WidgetMatchEvent |
| `chat/chat_history_panel.rs` | Medium | WidgetMatchEvent |
| `chat/delete_chat_modal.rs` | Low | DefaultNone |
| `chat/deep_inquire_stages.rs` | **Very High** | `#[animator]` (5+ states), Timer, apply_over, LivePtr |
| `chat/chat_view.rs` | **Very High** | Custom vertex shader, apply_over, LivePtr, WidgetMatchEvent |
| `chat/chats_deck.rs` | High | apply_over, LivePtr |
| `chat/chat_screen.rs` | Medium | LiveDependency, WidgetMatchEvent |
| `chat/chat_screen_mobile.rs` | Medium | apply_over, WidgetMatchEvent |

**Order:** shared → model_info → entity_button → deep_inquire_content
→ chat_params → history_card_options → history_card → history
→ history_panel → delete_chat_modal → deep_inquire_stages
→ chat_view → chats_deck → chat_screen → chat_screen_mobile

### Phase 4: App Root and Integration

| File | Complexity | Patterns |
|------|-----------|----------|
| `app.rs` | High | Root widget, Window, Timer, all module registration |

**app.rs migration:**
1. Replace `live_design!` with `script_mod!` containing `Root` widget
2. Replace `LiveRegister` impl with `App::run()` calling all
   `script_mod` functions in correct order
3. Replace `Live/LiveHook` derives with `Script/ScriptHook`
4. Replace `DefaultNone` on AppAction
5. Update Timer usage

---

## File-by-File Reference

### Files with NO `live_design!` (data layer — derive changes only)

These files only need derive macro renames and have no DSL:

| File | Changes Needed |
|------|---------------|
| `data/providers.rs` | `Live,LiveHook` → `Script,ScriptHook` on Provider enum; `#[pick]` stays |
| `data/store.rs` | `DefaultNone` → `Default` + `#[default]` on StoreAction; LiveDependency stays |
| `data/moly_client.rs` | `DefaultNone` → `Default` + `#[default]` |
| `data/deep_inquire_client.rs` | LivePtr type stays as-is |
| `shared/actions.rs` | `DefaultNone` → `Default` + `#[default]` on all action enums |

### Files that are DSL-only (no Rust widget structs)

These files only need DSL syntax conversion:

| File | Key DSL Elements |
|------|-----------------|
| `shared/styles.rs` | Color constants, font refs, theme overrides |
| `shared/resource_imports.rs` | `dep()` → `crate_resource()` |
| `shared/desktop_buttons.rs` | Platform button styles |
| `landing/shared.rs` | Shared constants/styles |
| `chat/model_info.rs` | Model info panel layout |
| `widgets/message_markdown.rs` | Markdown widget config |

### Files with DrawList2d (overlay pattern)

| File | Variant | Notes |
|------|---------|-------|
| `shared/tooltip.rs` | Non-blocking | No sweep_lock; positioned |
| `shared/popup_notification.rs` | Non-blocking | Auto-dismiss timer |
| `widgets/moly_modal.rs` | Modal (blocking) | sweep_lock/unlock; dismiss on outside click |

### Files with ComponentMap

| File | Template Source | Creation Pattern |
|------|---------------|-----------------|
| `landing/model_files_list.rs` | `Option<LivePtr>` | `get_or_insert` in draw |
| `landing/model_files_tags.rs` | `Option<LivePtr>` | `clear` + `insert` |
| `widgets/model_selector_list.rs` | `Option<LivePtr>` | `get_or_insert` in draw |

### Files with `#[animator]` (complex animation)

| File | States | Notes |
|------|--------|-------|
| `landing/search_bar.rs` | hover | Plus Timer debounce |
| `landing/search_loading.rs` | rotation | Plus Timer |
| `landing/model_files.rs` | expand/collapse | Height animation |
| `landing/downloads.rs` | expand/collapse | Height animation |
| `chat/deep_inquire_stages.rs` | **5+ states** | Most complex; Timer-driven sequencing |
| `widgets/message_loading.rs` | 3 line states | Timer self-rescheduling |
| `widgets/message_thinking_block.rs` | expand/spin | Timer + apply_over |
| `widgets/model_selector_item.rs` | hover | Standard hover pattern |

### Files with Timer

| File | Timer Type | Purpose |
|------|-----------|---------|
| `app.rs` | Timeout | Startup delay |
| `landing/model_list.rs` | Timeout | Debounce |
| `landing/search_bar.rs` | Timeout | Search debounce |
| `landing/search_loading.rs` | Timeout | Animation advance |
| `chat/deep_inquire_stages.rs` | Timeout | Stage sequencing |
| `widgets/message_loading.rs` | Timeout | Self-rescheduling animation |
| `widgets/message_thinking_block.rs` | Timeout | Self-rescheduling |
| `widgets/stt_input.rs` | Timeout | Recording timeout |
| `widgets/realtime.rs` | **Interval** | Audio streaming (20ms) |

---

## Tricky Migration Cases

### 1. Custom Vertex Shaders (`chat/chat_view.rs`, `shared/widgets.rs`)

`chat_view.rs` has a custom vertex shader with `varying` fields.
In old DSL:

```
fn vertex(self) -> vec4 {
    let pos = vec2(self.a_xs.x, self.a_xs.y);
    // ...
    return self.camera_projection * vec4(pos, 0.0, 1.0);
}
```

In new Splash:
```
vertex: fn() {
    let pos = vec2(self.a_xs.x self.a_xs.y)
    // ... (no semicolons, method calls use dot syntax)
    return self.camera_projection * vec4(pos 0.0 1.0)
}
```

`shared/widgets.rs` MolyButton has `clip_and_transform_vertex` override.

**Watch for:**
- Semicolons removed inside shaders
- `Pal::method()` → `Pal.method()`
- `vec2(a, b)` → `vec2(a b)` (comma removal in fn calls)
- `varying` field syntax may differ

### 2. Deep Inquire Stages (5+ Animator States)

`chat/deep_inquire_stages.rs` has the most complex animator in the
codebase with 5+ named states and Timer-driven sequencing. Each state
becomes an `AnimatorState{}` block. `default: off` → `default: @off`.

### 3. Manual Widget/WidgetNode Impls (`landing/model_files_list.rs`)

This file uses `LiveRegisterWidget` and manually implements `Widget`
and `WidgetNode` traits (not via `#[derive(Widget)]`). Migration:
- `LiveRegisterWidget` → manual `register_widget` fn
- `Widget` and `WidgetNode` impls stay mostly the same
- The `live_design!` macro still → `script_mod!`

### 4. DrawList2d Initialization

```rust
// Old
#[rust] draw_list: DrawList2d,
// + in after_new_from_doc: self.draw_list = DrawList2d::new(cx);

// New
#[rust(DrawList2d::new(cx))] draw_list: DrawList2d,
```

### 5. cx.link() Theme Aliasing (MolyKit `widgets.rs`)

```rust
// Old
cx.link(live_id!(button), live_id!(moly_kit_button));

// New — same API, but live_id! might need to change to id!()
// depending on final Splash API
cx.link(live_id!(button), live_id!(moly_kit_button));
```

### 6. LiveDependency for Icon Loading

Used in `chat/chat_screen.rs`, `settings/providers.rs`, `data/store.rs`.
Old:
```rust
#[live] icon_favorites: LiveDependency,
```
New — stays the same type, but resource path changes from
`dep("crate://...")` to `crate_resource("self://...")` in DSL.

### 7. apply_over with live! macro

Every `apply_over(cx, live!{...})` becomes
`script_apply_eval!(cx, widget_ref, {...})`.

The inner DSL follows new Splash syntax:
- No commas
- No semicolons
- `vec3(r, g, b)` → `vec3(r g b)`
- String interpolation: `text: (value)` → `text: #(value)`

### 8. #[derive(DefaultNone)] on Action Enums

Old:
```rust
#[derive(Clone, DefaultNone, Debug)]
pub enum FooAction {
    Selected(usize),
    None,
}
```

New:
```rust
#[derive(Clone, Default, Debug)]
pub enum FooAction {
    Selected(usize),
    #[default]
    None,
}
```

Found in **19 files** across Moly + MolyKit. This is the single most
common mechanical change.

### 9. Scope::with_props in List Rendering

Used in `my_models/downloaded_files_table.rs` and
`landing/model_files_list.rs`. The pattern itself doesn't change — only
the DSL that defines the list templates needs Splash conversion.

### 10. Realtime Widget (Audio + Timer Interval)

`widgets/realtime.rs` uses `cx.start_interval(0.020)` for audio
streaming. The Timer API is unchanged. The main migration work is the
DSL and derives.

---

## Pattern Quick-Reference

### Derive Changes

| Old | New |
|-----|-----|
| `#[derive(Live)]` | `#[derive(Script)]` |
| `#[derive(LiveHook)]` | `#[derive(ScriptHook)]` |
| `#[derive(Live, LiveHook)]` | `#[derive(Script, ScriptHook)]` |
| `#[derive(Live, LiveHook, Widget)]` | `#[derive(Script, ScriptHook, Widget)]` |
| `#[derive(DefaultNone)]` | `#[derive(Default)]` + `#[default]` on None variant |
| `#[animator]` | `#[apply_default]` (and add `Animator` to derives) |
| `#[live_ignore]` | Remove entirely |

### DSL Syntax

| Old | New |
|-----|-----|
| `live_design! { }` | `script_mod! { }` |
| `use link::theme::*;` | `use mod.prelude.widgets.*` |
| `use link::widgets::*;` | (included in above) |
| `<View> { }` | `View{ }` |
| `name = <View> { }` | `name := View{ }` |
| `prop: value,` | `prop: value` |
| `(THEME_COLOR)` | `theme.color` |
| `dep("crate://self/x")` | `crate_resource("self://x")` |
| `{{Struct}}` | `#(Struct::register_widget(vm))` |
| `draw_bg: { ... }` (merge) | `draw_bg +: { ... }` |
| `instance x: 0.0` | `x: instance(0.0)` |
| `uniform x: 0.0` | `x: uniform(0.0)` |
| `fn pixel(self) -> vec4` | `pixel: fn()` |
| `statement;` (in shaders) | `statement` (no semicolons) |
| `fn(a, b)` (in shaders) | `fn(a b)` (no commas) |
| `default: off` (animator) | `default: @off` |
| `cursor: Hand` | `cursor: MouseCursor.Hand` |
| `Pal::iq(a, b)` | `Pal.iq(a b)` |
| `{` state block (animator) | `AnimatorState{` |
| `apply_over(cx, live!{})` | `script_apply_eval!(cx, w, {})` |

### Registration

| Old | New |
|-----|-----|
| `impl LiveRegister for App { fn live_register(cx) { ... } }` | `impl App { fn run(vm) -> Self { ... } }` |
| `crate::module::live_design(cx)` | `crate::module::script_mod(vm)` |
| `app_main!(App)` | `app_main!(App)` (unchanged) |

---

## Verification Checklist

After migrating each file:

- [ ] `cargo check` passes
- [ ] No `live_design!` macro remains
- [ ] No `#[derive(Live)]` or `#[derive(LiveHook)]` remains (use `Script`/`ScriptHook`)
- [ ] No `#[derive(DefaultNone)]` remains
- [ ] No `#[animator]` remains (use `#[apply_default]`)
- [ ] No `#[live_ignore]` remains
- [ ] No `dep("crate://...")` remains (use `crate_resource()`)
- [ ] No `<Widget>` angle-bracket syntax in DSL
- [ ] No commas between DSL properties
- [ ] No semicolons in shader code
- [ ] All `apply_over(cx, live!{...})` replaced with `script_apply_eval!`
- [ ] Merge operator `+:` used where inheriting/extending base widget properties

After migrating all files:

- [ ] `cargo build` succeeds for all targets (desktop, mobile, WASM)
- [ ] App launches and renders correctly
- [ ] All navigation works (tabs, modals, stack nav)
- [ ] Chat send/receive works
- [ ] Model search and download works
- [ ] Settings save/load works
- [ ] Animations are smooth (hover, expand/collapse, loading)
- [ ] Overlays render correctly (tooltip, modals, popups)

---

## Migration File Count Summary

| Category | Files | Complexity |
|----------|-------|-----------|
| Data layer (derives only) | 5 | Low |
| DSL-only (no Rust structs) | 6 | Low |
| Standard widgets (Live+Widget) | ~30 | Low–Medium |
| Animator widgets | 8 | Medium |
| DrawList2d overlays | 3 | High |
| ComponentMap widgets | 3 | Medium |
| Custom shader widgets | 2 | High |
| Complex composites (chat_view, deep_inquire) | 2 | Very High |
| App root | 1 | High |
| MolyKit registration/theme | 2 | Medium |
| **Total** | **~62** | |
