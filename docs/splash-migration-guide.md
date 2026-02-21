# Splash Migration Guide

> Migrating from Makepad `live_design!` DSL to Splash `script_mod!`

This guide provides side-by-side comparisons for every pattern you need to change when migrating a Makepad application from the old `live_design!` macro system to the new Splash scripting system. It is organized by category so you can look up specific patterns quickly.

For a comprehensive reference on the Splash language itself, see [splash-language-reference.md](./splash-language-reference.md). For Moly-specific architecture context, see [moly-handbook.md](./moly-handbook.md).

---

## Table of Contents

1. [High-Level Architecture Changes](#1-high-level-architecture-changes)
2. [Macro and Import Changes](#2-macro-and-import-changes)
3. [Widget Registration and Templates](#3-widget-registration-and-templates)
4. [Property Syntax](#4-property-syntax)
5. [Theme Constants](#5-theme-constants)
6. [Inheritance and Variants](#6-inheritance-and-variants)
7. [The Merge Operator (`+:`)](#7-the-merge-operator-)
8. [Named Children](#8-named-children)
9. [Animator System](#9-animator-system)
10. [Shader Functions](#10-shader-functions)
11. [Shader Variables (instance/uniform)](#11-shader-variables-instanceuniform)
12. [Sdf2d and Math Builtins](#12-sdf2d-and-math-builtins)
13. [Mutability in Shaders](#13-mutability-in-shaders)
14. [Struct Construction in Shaders](#14-struct-construction-in-shaders)
15. [DrawIcon to DrawSvg](#15-drawicon-to-drawsvg)
16. [Layout and Sizing](#16-layout-and-sizing)
17. [Dock, Tabs, and Splitter](#17-dock-tabs-and-splitter)
18. [PortalList](#18-portallist)
19. [App Entry Point and Structure](#19-app-entry-point-and-structure)
20. [Rust Struct Derive Changes](#20-rust-struct-derive-changes)
21. [Rust Widget Trait Changes](#21-rust-widget-trait-changes)
22. [Action Enum Changes](#22-action-enum-changes)
23. [Library `lib.rs` Registration](#23-library-librs-registration)
24. [Resource References (`dep` to `crate_resource`)](#24-resource-references-dep-to-crate_resource)
25. [Theme Linking (`link` and `cx.link`)](#25-theme-linking-link-and-cxlink)
26. [Dynamic Styling (`apply_over` + `live!`)](#26-dynamic-styling-apply_over--live)
27. [Dynamic Widget Creation (`new_from_ptr`)](#27-dynamic-widget-creation-new_from_ptr)
28. [DrawList2d Overlay Initialization](#28-drawlist2d-overlay-initialization)
29. [Template Storage (`LivePtr` to `ScriptObjectRef`)](#29-template-storage-liveptr-to-scriptobjectref)
30. [DSL Constants](#30-dsl-constants)
31. [Unchanged Patterns (Confirm Still Work)](#31-unchanged-patterns-confirm-still-work)
32. [New Capabilities (No Old Equivalent)](#32-new-capabilities-no-old-equivalent)
33. [Quick Reference Cheat Sheet](#33-quick-reference-cheat-sheet)

---

## 1. High-Level Architecture Changes

| Aspect | Old | New |
|--------|-----|-----|
| UI macro | `live_design! { ... }` | `script_mod! { ... }` |
| File format | `.rs` files with `live_design!` | `.rs` files with `script_mod!` |
| Registration fn | `pub fn live_design(cx: &mut Cx)` | `pub fn script_mod(vm: &mut ScriptVm)` |
| Entry macro | `app_main!(App)` | `app_main!(App)` (unchanged) |
| Derive traits | `Live, LiveHook, Widget` | `Script, ScriptHook, Widget` |
| Animator derive | (none, field attribute only) | `Animator` (separate derive) |
| Scripting | None — DSL only | Full scripting: variables, functions, HTTP, promises |
| Commas | Required between properties | Removed — whitespace-delimited |
| Live reloading | Via live_design system | Via Splash VM hot-reload |

---

## 2. Macro and Import Changes

### The macro itself

**Old:**
```rust
live_design! {
    link widgets;
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // ... widget definitions
}
```

**New:**
```rust
script_mod! {
    use mod.prelude.widgets.*

    // ... widget definitions
}
```

### Import variants

| Old | New |
|-----|-----|
| `link widgets;` | (not needed) |
| `use link::theme::*;` | (merged into `mod.prelude.widgets.*`) |
| `use link::shaders::*;` | (merged into `mod.prelude.widgets.*`) |
| `use link::widgets::*;` | `use mod.prelude.widgets.*` |
| `use crate::my_module::*;` | `use mod.my_module.*` (if registered) |

### Internal widget imports (inside the widgets crate itself)

**Old:**
```rust
live_design! {
    link widgets;
    use link::theme::*;
    use link::shaders::*;
    // ...
}
```

**New:**
```rust
script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    // ...
}
```

### Networking imports (new only)

```
use mod.net
```

---

## 3. Widget Registration and Templates

### Registering a Rust-backed widget

**Old:**
```
pub ButtonBase = {{Button}} {}
```

**New:**
```
mod.widgets.ButtonBase = #(Button::register_widget(vm))
```

The `{{StructName}}` syntax is replaced by `#(StructName::register_widget(vm))`.

### Defining a widget template (variant)

**Old:**
```
pub ButtonFlat = <ButtonBase> {
    text: "Button"
    width: Fit, height: Fit,
    spacing: (THEME_SPACE_2),
    // ...
}
```

**New:**
```
mod.widgets.ButtonFlat = set_type_default() do mod.widgets.ButtonBase{
    text: "Button"
    width: Fit
    height: Fit
    spacing: theme.space_2
    // ...
}
```

Key differences:
- `<ParentWidget>` becomes `mod.widgets.ParentWidget{...}`
- `pub Name = ` becomes `mod.widgets.Name = `
- `set_type_default() do` prefix is used for base definitions that establish the type default (used in the widgets crate itself; in app code you typically just use `let Name = ParentWidget{...}`)
- No commas between properties

### App-level template definitions

**Old:**
```
UIZooTab = <RectView> {
    height: Fill, width: Fill
    flow: Down,
    padding: 0
}
```

**New:**
```
let UIZooTab = RectView{
    height: Fill width: Fill
    flow: Down
    padding: 0
}
```

In application code (outside the widgets crate), use `let Name = ...` instead of `mod.widgets.Name = ...`.

---

## 4. Property Syntax

### Commas removed

**Old:**
```
width: Fit, height: Fit,
spacing: 10.,
padding: 0,
```

**New:**
```
width: Fit height: Fit
spacing: 10.
padding: 0
```

Simply remove all commas between properties. Whitespace (space or newline) separates them.

### Dot-path property access

**Old:**
```
draw_bg: {
    color: #fff
    border_radius: 5.0
}
```

**New (both forms work):**
```
// Block form
draw_bg: {color: #fff border_radius: 5.0}

// Dot-path form (shorthand for single properties)
draw_bg.color: #fff
draw_bg.border_radius: 5.0
```

---

## 5. Theme Constants

**Old:** Theme constants used `SCREAMING_SNAKE_CASE` wrapped in parentheses:
```
spacing: (THEME_SPACE_2),
color: (THEME_COLOR_LABEL_INNER)
border_size: (THEME_BEVELING)
text_style: <THEME_FONT_REGULAR> { font_size: (THEME_FONT_SIZE_P) }
padding: <THEME_MSPACE_1> { left: (THEME_SPACE_2) }
```

**New:** Theme constants use `theme.snake_case` dot notation (drop the `THEME_` prefix):
```
spacing: theme.space_2
color: theme.color_label_inner
border_size: theme.beveling
text_style: theme.font_regular{ font_size: theme.font_size_p }
padding: theme.mspace_1{ left: theme.space_2 }
```

### Pattern

| Old | New |
|-----|-----|
| `(THEME_SPACE_2)` | `theme.space_2` |
| `(THEME_COLOR_BG_APP)` | `theme.color_bg_app` |
| `(THEME_BEVELING)` | `theme.beveling` |
| `(THEME_CORNER_RADIUS)` | `theme.corner_radius` |
| `(THEME_FONT_SIZE_P)` | `theme.font_size_p` |
| `<THEME_FONT_REGULAR>` | `theme.font_regular` |
| `<THEME_MSPACE_1>` | `theme.mspace_1` |
| `<THEME_MSPACE_V_1>` | `theme.mspace_v_1` |

The rule: drop `THEME_`, lowercase everything, replace `_` connecting words as-is, use `theme.` prefix.

---

## 6. Inheritance and Variants

### Deriving a variant from a parent

**Old:**
```
pub Button = <ButtonFlat> {
    draw_bg: {
        border_color: (THEME_COLOR_BEVEL_OUTSET_1)
    }
}
```

**New:**
```
mod.widgets.Button = mod.widgets.ButtonFlat{
    draw_bg +: {
        border_color: theme.color_bevel_outset_1
    }
}
```

Note: When extending a parent's draw object, use `+:` (merge operator) instead of plain `:` to extend rather than replace. See next section.

### In app code (not the widgets crate)

**Old:**
```
MyButton = <Button> {
    text: "Click me"
    draw_bg: { color: #f00 }
}
```

**New:**
```
let MyButton = Button{
    text: "Click me"
    draw_bg +: { color: #f00 }
}
```

---

## 7. The Merge Operator (`+:`)

This is one of the most important new concepts. In the old system, when you wrote:

```
draw_bg: { color: #f00 }
```

...it was ambiguous whether you were replacing the entire `draw_bg` or just overriding `color`. The old system had implicit merging behavior.

In Splash, you must be explicit:

| Syntax | Meaning |
|--------|---------|
| `draw_bg: { ... }` | **Replace** — completely replaces the parent's `draw_bg` |
| `draw_bg +: { ... }` | **Merge** — keeps parent's `draw_bg` properties and overlays your changes |

### When to use `+:`

Use `+:` whenever you want to **extend** a parent's sub-object (draw_bg, draw_text, text_style, animator, etc.) rather than replace it entirely.

**Old (implicit merge — just worked):**
```
pub Button = <ButtonFlat> {
    draw_bg: {
        border_color: (THEME_COLOR_BEVEL_OUTSET_1)
    }
}
```

**New (explicit merge with `+:`):**
```
mod.widgets.Button = mod.widgets.ButtonFlat{
    draw_bg +: {
        border_color: theme.color_bevel_outset_1
    }
}
```

### Nested merge

```
draw_text +: {
    text_style +: {font_size: 12}
}
```

---

## 8. Named Children

### The `:=` operator

Named children that can be referenced from Rust code use `:=` instead of `=`.

**Old:**
```
body = <View> {
    width: Fill, height: Fill
    my_button = <Button> { text: "Hello" }
    my_label = <Label> { text: "World" }
}
```

**New:**
```
body +: {
    width: Fill height: Fill
    my_button := Button{ text: "Hello" }
    my_label := Label{ text: "World" }
}
```

| Old | New |
|-----|-----|
| `name = <WidgetType> { ... }` | `name := WidgetType{ ... }` |
| `caption_bar = { ... }` | `caption_bar +: { ... }` (for modifying existing named children) |

### Referencing named children from script

In Splash scripting, named children can be accessed via `ui.child_name`:
```
ui.my_label.set_text("Updated!")
ui.my_button.on_click()
```

---

## 9. Animator System

The animator underwent significant syntax changes.

### Outer structure

**Old:**
```
animator: {
    hover = {
        default: off,
        off = {
            from: {all: Forward {duration: 0.1}}
            apply: {
                draw_bg: {hover: 0.0}
            }
        }
        on = {
            from: {all: Forward {duration: 0.1}}
            apply: {
                draw_bg: {hover: [{time: 0.0, value: 1.0}]}
            }
        }
    }
}
```

**New:**
```
animator: Animator{
    hover: {
        default: @off
        off: AnimatorState{
            from: {all: Forward {duration: 0.1}}
            apply: {
                draw_bg: {hover: 0.0}
            }
        }
        on: AnimatorState{
            from: {all: Forward {duration: 0.1}}
            apply: {
                draw_bg: {hover: snap(1.0)}
            }
        }
    }
}
```

### Differences summary

| Old | New |
|-----|-----|
| `animator: { ... }` | `animator: Animator{ ... }` |
| `hover = { ... }` | `hover: { ... }` |
| `default: off,` | `default: @off` |
| `off = { ... }` | `off: AnimatorState{ ... }` |
| `on = { ... }` | `on: AnimatorState{ ... }` |
| `[{time: 0.0, value: 1.0}]` (snap) | `snap(1.0)` |
| `[{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}]` (timeline) | `timeline(0.0 0.0  1.0 1.0)` |
| `cursor: Arrow,` | `cursor: MouseCursor.Arrow` |

### Snap and Timeline shortcuts

**Old (snap — set value instantly):**
```
hover: [{time: 0.0, value: 1.0}]
```

**New:**
```
hover: snap(1.0)
```

**Old (timeline — animate from A to B):**
```
anim_time: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}]
```

**New:**
```
anim_time: timeline(0.0 0.0  1.0 1.0)
```

### Loop animation

**Old:**
```
on = {
    from: {all: Loop {duration: 1.0, end: 1000000000.0}}
    apply: {
        draw_bg: {anim_time: [{time: 0.0, value: 0.0},{time:1.0, value:1.0}]}
    }
}
```

**New:**
```
on: AnimatorState{
    from: {all: Loop {duration: 1.0, end: 1000000000.0}}
    apply: {
        draw_bg: {anim_time: timeline(0.0 0.0  1.0 1.0)}
    }
}
```

---

## 10. Shader Functions

### Function declaration

**Old:**
```
fn pixel(self) -> vec4 {
    // ...
    return sdf.result
}

fn get_color(self) -> vec4 {
    return mix(self.color, self.color_hover, self.hover)
}
```

**New:**
```
pixel: fn() {
    // ...
    return sdf.result
}

get_color: fn() {
    return self.color
        .mix(self.color_hover, self.hover)
}
```

| Old | New |
|-----|-----|
| `fn name(self) -> vec4 { ... }` | `name: fn() { ... }` |
| `fn get_color(self) -> vec4 { ... }` | `get_color: fn() { ... }` |

The function is now a property (with `: fn()` syntax). There is no explicit return type — it's inferred. `self` is always implicitly available.

### Method chaining replaces nested `mix()`

**Old:**
```
return mix(
    mix(
        mix(self.color, self.color_focus, self.focus),
        self.color_hover,
        self.hover
    ),
    self.color_down,
    self.down
)
```

**New:**
```
return self.color
    .mix(self.color_focus, self.focus)
    .mix(self.color_hover, self.hover)
    .mix(self.color_down, self.down)
```

---

## 11. Shader Variables (instance/uniform)

### Declaration syntax

**Old:**
```
instance hover: 0.0
instance down: 0.0
uniform border_size: (THEME_BEVELING)
uniform color: (THEME_COLOR_OUTSET)
```

**New:**
```
hover: instance(0.0)
down: instance(0.0)
border_size: uniform(theme.beveling)
color: uniform(theme.color_outset)
```

| Old | New |
|-----|-----|
| `instance name: value` | `name: instance(value)` |
| `uniform name: value` | `name: uniform(value)` |
| `uniform name: (THEME_CONST)` | `name: uniform(theme.const_name)` |

The keyword moves from a prefix to a wrapper function.

---

## 12. Sdf2d and Math Builtins

### Double-colon to dot

**Old:**
```
let sdf = Sdf2d::viewport(self.pos * self.rect_size)
let dither = Math::random_2d(self.pos.xy) * 0.04
```

**New:**
```
let sdf = Sdf2d.viewport(self.pos * self.rect_size)
let dither = Math.random_2d(self.pos.xy) * 0.04
```

| Old | New |
|-----|-----|
| `Sdf2d::viewport(...)` | `Sdf2d.viewport(...)` |
| `Sdf2d::viewport_skip(...)` | `Sdf2d.viewport_skip(...)` |
| `Math::random_2d(...)` | `Math.random_2d(...)` |
| `Pal::premul(...)` | `Pal.premul(...)` |
| `Pal::iq(...)` | `Pal.iq(...)` |

All static-style method calls switch from `::` to `.`.

---

## 13. Mutability in Shaders

In the old DSL, local variables were implicitly mutable. In Splash, you must explicitly use `let mut` if you plan to reassign:

**Old:**
```
let color_2 = self.color;
// later:
color_2 = self.color_2;   // works without mut
```

**New:**
```
let mut color_fill = self.color
// later:
color_fill = self.color_2    // requires mut
```

Also note: semicolons at end of lines are **optional** in Splash but still accepted.

### Inline if-else expressions

**Old (imperative):**
```
let gradient_fill_dir = gradient_fill.y;
if (self.gradient_fill_horizontal > 0.5) {
    gradient_fill_dir = gradient_fill.x;
}
```

**New (expression):**
```
let dir = if self.gradient_fill_horizontal > 0.5 gradient_fill.x else gradient_fill.y
```

Splash supports `if/else` as expressions (like Rust), and parentheses around the condition are not required.

---

## 14. Struct Construction in Shaders

**Old:**
```
Self {
    field: value
}
```

**New:**
```
self (field: value)
```

This applies when returning a constructed draw struct from a shader function.

---

## 15. DrawIcon to DrawSvg

The old `DrawIcon` type has been replaced by `DrawSvg`.

**Old (Rust struct):**
```rust
#[live]
draw_icon: DrawIcon,
```

**New (Rust struct):**
```rust
#[live]
draw_icon: DrawSvg,
```

**Old (DSL):**
```
draw_icon: {
    instance hover: 0.0
    uniform color: (THEME_COLOR_LABEL_OUTER)
    fn get_color(self) -> vec4 { ... }
}
```

**New:**
The `icon_walk` property still exists but is simplified:

```
icon_walk: Walk{width: Fit, height: Fit}
```

The `draw_icon` block with custom shader functions may be simplified or removed depending on the widget — consult the new widget source.

---

## 16. Layout and Sizing

### Walk (size/position) properties

Most layout properties are the same, but with syntax adjustments:

**Old:**
```
width: Fit, height: Fill,
margin: {left: 10, top: 5}
padding: {left: (THEME_SPACE_2), right: (THEME_SPACE_2)}
```

**New:**
```
width: Fit height: Fill
margin: Inset{left: 10 top: 5}
padding: Inset{left: theme.space_2 right: theme.space_2}
```

Note that anonymous `{left: 10}` for margins/padding may now require `Inset{...}` wrapper in some contexts.

### Walk struct reference

**Old:**
```
label_walk: { width: Fit, height: Fit },
icon_walk: { width: (THEME_DATA_ICON_WIDTH), height: Fit }
```

**New:**
```
label_walk: Walk{width: Fit, height: Fit}
icon_walk: Walk{width: Fit, height: Fit}
```

### Align shortcuts

**Old:**
```
align: {x: 0.5, y: 0.5}
```

**New (both work):**
```
align: Align{x: 0.5 y: 0.5}
align: Center                    // shortcut
```

### vec2 and vec4

**Old:**
```
inner_size: vec2(700, 500)
clear_color: vec4(0.12, 0.12, 0.14, 1.0)
```

**New:**
```
inner_size: vec2(700, 500)       // unchanged in shader context
clear_color: vec4(0.12, 0.12, 0.14, 1.0)
```

Note: In non-shader property context, commas inside `vec2()` / `vec4()` are sometimes omitted:
```
viewbox: vec4(0 0 24 24)
```

---

## 17. Dock, Tabs, and Splitter

### Dock structure

**Old:**
```
dock = <Dock> {
    height: Fill, width: Fill

    root = Splitter {
        axis: Horizontal,
        align: FromA(0.0),
        a: tab_set_1,
        b: tab_set_2
    }

    tab_set_1 = Tabs {
        tabs: [tab_a]
        selected: 0
        closable: false
    }

    tab_a = Tab {
        name: "Welcome"
        template: PermanentTab
        kind: TabOverview
    }
}
```

**New:**
```
dock := Dock{
    height: Fill width: Fill

    root := DockSplitter{
        axis: SplitterAxis.Horizontal
        align: SplitterAlign.FromA(0.0)
        a: @tab_set_1
        b: @tab_set_2
    }

    tab_set_1 := DockTabs{
        tabs: [@tab_a]
        selected: 0
        closable: false
    }

    tab_a := DockTab{
        name: "Welcome"
        template: @PermanentTab
        kind: @TabOverview
    }
}
```

### Differences

| Old | New |
|-----|-----|
| `Splitter` | `DockSplitter` |
| `Tabs` | `DockTabs` |
| `Tab` | `DockTab` |
| `axis: Horizontal` | `axis: SplitterAxis.Horizontal` |
| `align: FromA(0.0)` | `align: SplitterAlign.FromA(0.0)` |
| `a: tab_set_1` | `a: @tab_set_1` |
| `tabs: [tab_a]` | `tabs: [@tab_a]` |
| `template: PermanentTab` | `template: @PermanentTab` |
| `kind: TabOverview` | `kind: @TabOverview` |

Named references in dock/tab use `@` prefix in Splash.

---

## 18. PortalList

### Rendering via on_render

In the old system, PortalList rendering was done entirely in Rust (`handle_event` + `draw_walk`). In Splash, you can use `on_render` callbacks for dynamic list rendering:

**Old (Rust-driven):**
```rust
// In draw_walk:
while let Some(item_id) = list.next_visible_item(cx) {
    let item = list.item(cx, item_id, live_id!(MyItem)).unwrap();
    item.label(id!(label)).set_text(&data[item_id]);
    item.draw_all_unscoped(cx);
}
```

**New (Splash script-driven):**
```
todo_list := PortalList{
    width: Fill height: Fill
    flow: Down spacing: 8
    on_render: ||{
        for i, todo in todos {
            TodoItem{
                label.text: todo.text
                check.selected: todo.done
            }
        }
    }
}
```

The `on_render` callback replaces the Rust-side `draw_walk` loop for many use cases.

### PortalList still works from Rust

You can still drive PortalList from Rust if needed — the `script_call` method and `handle_event` patterns are preserved. But for pure-script apps, `on_render` is the way to go.

---

## 19. App Entry Point and Structure

### Old pattern

```rust
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    App = {{App}} {
        ui: <Window> {
            // ...
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) { ... }
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) { ... }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

### New pattern

```rust
use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    let app = startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    // ... your UI tree
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[source]
    source: ScriptObjectRef,
    #[live]
    ui: WidgetRef,
}

impl ScriptRegister for App {
    fn script_register(vm: &mut ScriptVm) {
        makepad_widgets::script_mod(vm);
    }
}
```

### Differences

| Old | New |
|-----|-----|
| `App = {{App}} { ui: <Window> { ... } }` | `let app = startup() do #(App::script_component(vm)){ ui: Root{ main_window := Window{ ... } } }` |
| `#[derive(Live, LiveHook)]` | `#[derive(Script, ScriptHook)]` |
| `impl LiveRegister for App` | `impl ScriptRegister for App` |
| `fn live_register(cx: &mut Cx)` | `fn script_register(vm: &mut ScriptVm)` |
| `makepad_widgets::live_design(cx)` | `makepad_widgets::script_mod(vm)` |
| `impl MatchEvent for App` | (use Splash `on_click`, `on_return` inline handlers or `impl MatchEvent`) |
| `<Window>` | `Window` (inside `Root`) |

Note: The `startup() do #(App::script_component(vm))` pattern registers the app component and wires it to the Splash VM. `Root` wraps the window in the new system.

---

## 20. Rust Struct Derive Changes

### Widget struct

**Old:**
```rust
#[derive(Live, LiveHook, Widget)]
pub struct Button {
    #[animator]
    animator: Animator,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[live]
    draw_icon: DrawIcon,
    // ...
}
```

**New:**
```rust
#[derive(Script, ScriptHook, Widget, Animator)]
pub struct Button {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[apply_default]
    animator: Animator,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[live]
    draw_icon: DrawSvg,
    // ...
}
```

### Field-by-field changes

| Old | New |
|-----|-----|
| `#[derive(Live, LiveHook, Widget)]` | `#[derive(Script, ScriptHook, Widget, Animator)]` |
| (none) | `#[uid] uid: WidgetUid,` (required) |
| (none) | `#[source] source: ScriptObjectRef,` (required) |
| `#[animator] animator: Animator,` | `#[apply_default] animator: Animator,` |
| `draw_icon: DrawIcon,` | `draw_icon: DrawSvg,` |
| (none) | `#[live] on_click: ScriptFnRef,` (for inline script callbacks) |
| (none) | `#[action_data] #[rust] action_data: WidgetActionData,` |

The `#[uid]` and `#[source]` fields are new required fields for all Splash-based widgets. `Animator` is now a separate derive trait.

---

## 21. Rust Widget Trait Changes

### New `script_call` method

Splash widgets can receive script-side method calls:

```rust
impl Widget for Button {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        _args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(on_click) {
            let uid = self.widget_uid();
            vm.with_cx_mut(|cx| {
                cx.widget_to_script_call(uid, NIL, self.source.clone(), self.on_click.clone(), &[]);
            });
            return ScriptAsyncResult::Return(TRUE);
        }
        ScriptAsyncResult::MethodNotFound
    }
    // ...
}
```

### handle_event changes

The `handle_event` signature is the same, but action dispatch uses `widget_action_with_data`:

**Old:**
```rust
cx.widget_action(uid, &scope.path, ButtonAction::Clicked(fe.modifiers));
```

**New:**
```rust
cx.widget_action_with_data(&self.action_data, uid, ButtonAction::Clicked(fe.modifiers));
```

### HitDesigner

New pattern for designer integration:
```rust
match event.hit_designer(cx, self.draw_bg.area()) {
    HitDesigner::DesignerPick(_e) => {
        cx.widget_action_with_data(&self.action_data, uid, WidgetDesignAction::PickedBody)
    }
    _ => (),
}
```

---

## 22. Action Enum Changes

**Old:**
```rust
#[derive(Clone, Debug, DefaultNone)]
pub enum ButtonAction {
    None,
    Pressed(KeyModifiers),
    // ...
}
```

**New:**
```rust
#[derive(Clone, Debug, Default)]
pub enum ButtonAction {
    #[default]
    None,
    Pressed(KeyModifiers),
    // ...
}
```

`DefaultNone` is replaced by standard Rust `Default` derive with `#[default]` attribute on the `None` variant.

---

## 23. Library `lib.rs` Registration

### Old pattern

```rust
pub fn live_design(cx: &mut Cx) {
    cx.link(live_id!(theme), live_id!(theme_desktop_dark));
    makepad_draw::live_design(cx);
    crate::theme_desktop_dark::live_design(cx);
    crate::label::live_design(cx);
    crate::button::live_design(cx);
    crate::view::live_design(cx);
    // ... each module called in order
}
```

### New pattern

```rust
pub fn script_mod(vm: &mut ScriptVm) {
    makepad_draw::script_mod(vm);

    vm.bx.heap.new_module(id!(prelude));
    vm.bx.heap.new_module(id!(themes));
    crate::theme_desktop_dark::script_mod(vm);
    crate::animator::script_mod(vm);

    // Build prelude for internal widgets
    {
        script_mod! {
            mod.prelude.widgets_internal = {
                ..mod.res,
                ..mod.helper,
                ..mod.animator,
                ..mod.pod,
                ..mod.math,
                ..mod.sdf,
                ..mod.shader,
                ..mod.turtle,
                ..mod.turtle.Size,
                ..mod.turtle.Flow,
                ..mod.std
                theme:mod.theme,
                draw:mod.draw,
                MouseCursor:mod.draw.MouseCursor
            }
        }
        script_mod(vm);
    }

    vm.bx.heap.new_module(id!(widgets));

    crate::scroll_bar::script_mod(vm);
    crate::view::script_mod(vm);
    crate::label::script_mod(vm);
    crate::button::script_mod(vm);
    // ... each module called in order

    // Build public prelude with all widgets
    {
        script_mod! {
            mod.prelude.widgets = {
                ..mod.res,
                ..mod.helper,
                ..mod.std,
                ..mod.pod,
                ..mod.math,
                ..mod.sdf,
                theme:mod.theme,
                draw:mod.draw,
                net:mod.net,
                ..mod.animator,
                ..mod.shader,
                ..mod.widgets,
                ..mod.turtle,
                ..mod.turtle.Size,
                ..mod.turtle.Flow,
                ..mod.draw.MouseCursor
            }
        }
        script_mod(vm);
    }
}
```

Key differences:
- `live_design(cx: &mut Cx)` → `script_mod(vm: &mut ScriptVm)`
- `cx.link(...)` for themes → `vm.bx.heap.new_module(...)` for modules
- Prelude is now explicitly constructed as a namespace merge of multiple modules
- Order still matters — widgets must be registered before they can be referenced

---

## 24. Resource References (`dep` to `crate_resource`)

The old `dep("crate://self/...")` syntax for referencing SVG, PNG, and font resources has been replaced.

**Old:**
```
svg_file: dep("crate://self/resources/icons/icon.svg")
source: dep("crate://self/resources/images/ducky.png")
font: dep("crate://self/resources/fonts/MyFont.ttf")
```

**New:**
```
svg_file: crate_resource("self:resources/icons/icon.svg")
src: crate_resource("self:resources/images/ducky.png")
res: crate_resource("self:resources/fonts/MyFont.ttf")
```

| Old | New |
|-----|-----|
| `dep("crate://self/path")` | `crate_resource("self:path")` |
| `source:` (Image property) | `src:` |
| `font:` (Font property) | `res:` (in `FontMember`) |

The `crate_resource` function is registered in the script VM as a builtin. The URL scheme simplified from `crate://self/` to `self:`.

---

## 25. Theme Linking (`link` and `cx.link`)

The old system used `link` directives and `cx.link()` for theme selection. Both are removed.

**Old (in DSL):**
```
live_design! {
    link widgets;
    link theme_moly_kit_light;
    use link::theme::*;
    use link::moly_kit_theme::*;
}
```

**Old (in Rust):**
```rust
pub fn live_design(cx: &mut Cx) {
    cx.link(live_id!(theme), live_id!(theme_desktop_dark));
    cx.link(live_id!(moly_kit_theme), live_id!(theme_moly_kit_light));
}
```

**New:** Themes are defined as named modules in `script_mod!` blocks. Theme selection happens at module registration time:

```rust
script_mod! {
    mod.themes.dark = {
        mod.theme = me          // this theme becomes active
        let theme = me          // self-reference for computed values

        color_text: #xffffff
        space_factor: 6.
        space_1: 0.5 * theme.space_factor
        // ...
    }
}
```

The prelude wires `theme:mod.theme` making `theme.color_text` etc. available everywhere:
```rust
script_mod! {
    mod.prelude.widgets = {
        ..mod.widgets,
        theme:mod.theme,
        // ...
    }
}
```

For custom library themes (like moly-kit's theme), the library's `script_mod(vm)` function registers additional theme properties into the `mod.theme` namespace.

---

## 26. Dynamic Styling (`apply_over` + `live!`)

The `apply_over` + `live!{}` pattern for runtime property changes has been **completely removed**.

**Old:**
```rust
self.apply_over(cx, live!{ visible: false });
self.view(id!(my_view)).apply_over(cx, live!{ draw_bg: { color: #f00 } });
self.draw_bg.apply_over(cx, live!{ active: 1.0 });
```

**New:** There is no direct equivalent. Use these alternatives:

### Direct field mutation
```rust
self.visible = false;
self.redraw(cx);
```

### Animator-driven changes
Use the animator system for animated property transitions — this already existed in the old system but now becomes the primary way to change draw properties at runtime.

### Script VM evaluation (for Splash-only widgets)
```rust
// For widgets that exist in Splash script context
vm.eval_with_append_source("widget_name.property = value");
```

### Widget query methods
```rust
self.widget(id!(my_label)).set_text("updated");
```

### Key implications for migration
Every `apply_over(cx, live!{...})` call in moly/moly-kit must be replaced with one of the above patterns. This is likely the **most labor-intensive** part of the migration since moly-kit uses `apply_over` extensively (in `prompt_input.rs`, `moly_modal.rs`, `messages.rs`, `attachment_view.rs`, `message_thinking_block.rs`, `model_selector.rs`, and many moly app files).

---

## 27. Dynamic Widget Creation (`new_from_ptr`)

The `WidgetRef::new_from_ptr(cx, Some(*ptr))` pattern for creating widgets from templates at runtime has been replaced.

**Old:**
```rust
// Template stored as Option<LivePtr>
#[live]
item_template: Option<LivePtr>,

// Creation
let widget = WidgetRef::new_from_ptr(cx, self.item_template);
```

**New:**
```rust
// Template stored as ScriptObjectRef
#[live]
item_template: ScriptObjectRef,

// Creation
let widget = cx.with_vm(|vm| {
    WidgetRef::script_from_value(vm, self.item_template.clone())
});
```

| Old | New |
|-----|-----|
| `Option<LivePtr>` | `ScriptObjectRef` |
| `WidgetRef::new_from_ptr(cx, ptr)` | `WidgetRef::script_from_value(vm, value)` |
| `View::new_from_ptr(cx, ptr)` | `cx.with_vm(\|vm\| WidgetRef::script_from_value(vm, value))` |

The `cx.with_vm(|vm| ...)` pattern is common because `draw_walk` and `handle_event` have access to `Cx` but not `ScriptVm`.

---

## 28. DrawList2d Overlay Initialization

The `DrawList2d` type (used for overlays, modals, popups) changed its initialization pattern.

**Old:**
```rust
#[derive(Live, LiveHook, Widget)]
pub struct MyModal {
    #[rust(DrawList2d::new(cx))]
    draw_list: DrawList2d,
}
```

**New:**
```rust
#[derive(Script, ScriptHook, Widget)]
pub struct MyModal {
    #[rust]
    draw_list: Option<DrawList2d>,
}

impl ScriptHook for MyModal {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        self.draw_list = Some(DrawList2d::script_new(vm));
    }
}
```

| Old | New |
|-----|-----|
| `#[rust(DrawList2d::new(cx))]` | `#[rust] draw_list: Option<DrawList2d>` + `on_after_new` |
| `DrawList2d::new(cx)` | `DrawList2d::script_new(vm)` |
| Direct field initialization | Two-phase: declare as Option, init in `on_after_new` |

The overlay API itself (`begin_overlay_reuse`, `begin_overlay_last`, etc.) is unchanged.

---

## 29. Template Storage (`LivePtr` to `ScriptObjectRef`)

Anywhere the old codebase stored template references as `LivePtr`, the new system uses `ScriptObjectRef`.

**Old:**
```rust
#[live]
my_template: Option<LivePtr>,

// In ComponentMap
items: ComponentMap<LiveId, (LivePtr, WidgetRef)>,
```

**New:**
```rust
#[live]
my_template: ScriptObjectRef,

// In ComponentMap
items: ComponentMap<LiveId, (ScriptObjectRef, WidgetRef)>,
```

This affects:
- `AdaptiveView`: `ComponentMap<LiveId, ScriptObjectRef>` instead of `ComponentMap<LiveId, LivePtr>`
- `PortalList` templates
- `ChatsDeck` (moly) which uses `WidgetRef::new_from_ptr`
- `SubStages` (moly) which uses `Option<LivePtr>` for stage templates
- Any custom list widget that creates items from templates

---

## 30. DSL Constants

**Old:** Constants were defined with `=` in `live_design!` and referenced with `(NAME)`:
```
live_design! {
    ANIMATION_SPEED = 0.33
    ITEM_HEIGHT = 200.0
    MY_COLOR = #ff0000

    // Usage:
    duration: (ANIMATION_SPEED)
    height: (ITEM_HEIGHT)
    color: (MY_COLOR)
}
```

**New:** Constants use `let` in `script_mod!` and are referenced directly by name:
```
script_mod! {
    let animation_speed = 0.33
    let item_height = 200.0
    let my_color = #xff0000

    // Usage:
    duration: animation_speed
    height: item_height
    color: my_color
}
```

For resource constants:
```
let ICO_SEARCH = crate_resource("self:resources/icons/search.svg")
```

Note: Theme-level constants that were `THEME_*` become `theme.*` properties (covered in section 5). App-level constants use `let`.

---

## 31. Unchanged Patterns (Confirm Still Work)

These patterns are used heavily in moly/moly-kit and remain unchanged in the new system:

| Pattern | Status |
|---------|--------|
| `#[deref]` attribute on widget structs | **Unchanged** — still delegates to parent widget |
| `#[wrap]` attribute (Slot pattern) | **Unchanged** — still wraps child widget |
| `ComponentMap` for custom list rendering | **Unchanged** — same API (but stores `ScriptObjectRef` instead of `LivePtr`) |
| `sweep_lock` / `sweep_unlock` for modal event capture | **Unchanged** |
| `DrawList2d` overlay API (`begin_overlay_reuse`, etc.) | **Unchanged** |
| `AdaptiveView` (Desktop/Mobile variants) | **Unchanged** concept — derives changed to `Script`/`ScriptHook` |
| `Scope::with_data` for passing data down widget tree | **Unchanged** |
| `WidgetMatchEvent` trait | **Unchanged** |
| `cx.widget_action` for emitting actions | Changed to `cx.widget_action_with_data(&self.action_data, uid, action)` |
| `spawn` for async tasks | **Unchanged** (from aitk) |
| `UiRunner` pattern (async → UI bridge) | **Unchanged** (moly-kit utility) |

---

## 32. New Capabilities (No Old Equivalent)

These features exist only in Splash and have no migration counterpart — they're purely new functionality.

### Inline event handlers

```
search_button := Button{
    text: "Search"
    on_click: || do_search(ui.search_input.text())
}

search_input := TextInput{
    empty_text: "Search..."
    on_return: || ui.search_button.on_click()
}
```

### Script-side state and logic

```
let todos = []
let count = 0

fn add_todo(text, tag) {
    todos.push({text: text, tag: tag, done: false})
    ui.todo_list.render()
}
```

### HTTP requests with promises

```
use mod.net

fn fetch(url) {
    let p = promise()
    net.http_request(net.HttpRequest{
        url: url
        method: net.HttpMethod.GET
        headers: {"User-Agent": "MyApp/1.0"}
    }) do net.HttpEvents{
        on_response: |res| p.resolve(res)
        on_error: |err| p.resolve(nil)
    }
    p
}

let result = fetch("https://api.example.com/data").await()
let data = result.body.to_string().parse_json()
```

### Dynamic rendering with on_render

```
results_view := ScrollYView{
    width: Fill height: Fill
    on_render: ||{
        for item in items {
            Label{text: item.name}
        }
    }
}
```

### Inline vector icons

```
let IconCheck = Vector{
    width: 18 height: 18
    viewbox: vec4(0 0 24 24)
    Path{
        d: "M20 6L9 17L4 12"
        fill: false
        stroke: #x6c6cff
        stroke_width: 2.5
        stroke_linecap: "round"
        stroke_linejoin: "round"
    }
}
```

### String/JSON operations

```
let encoded = query.url_encode()
let parts = response.body.to_string().split("vqd=\"")
let data = json_string.parse_json()
```

---

## 33. Quick Reference Cheat Sheet

### Macros and imports

| Old | New |
|-----|-----|
| `live_design! { ... }` | `script_mod! { ... }` |
| `link widgets;` | (not needed) |
| `use link::theme::*;` | `use mod.prelude.widgets.*` |
| `use link::shaders::*;` | `use mod.prelude.widgets.*` |
| `use link::widgets::*;` | `use mod.prelude.widgets.*` |
| `use crate::module::*;` | `use mod.module.*` |

### Widget definitions

| Old | New |
|-----|-----|
| `pub Name = {{RustStruct}} {}` | `mod.widgets.Name = #(RustStruct::register_widget(vm))` |
| `pub Name = <Parent> { ... }` | `mod.widgets.Name = mod.widgets.Parent{ ... }` |
| `MyWidget = <Parent> { ... }` (in app) | `let MyWidget = Parent{ ... }` |
| `<WidgetType> { ... }` (inline) | `WidgetType{ ... }` |

### Properties

| Old | New |
|-----|-----|
| `prop: value,` | `prop: value` (no comma) |
| `(THEME_XYZ)` | `theme.xyz` |
| `<THEME_XYZ>` | `theme.xyz` |
| Implicit merge of sub-objects | `+:` for merge, `:` for replace |
| `name = <Widget> {}` (named child) | `name := Widget{}` |

### Shaders

| Old | New |
|-----|-----|
| `fn pixel(self) -> vec4 { ... }` | `pixel: fn() { ... }` |
| `fn get_color(self) -> vec4 { ... }` | `get_color: fn() { ... }` |
| `instance name: 0.0` | `name: instance(0.0)` |
| `uniform name: value` | `name: uniform(value)` |
| `Sdf2d::viewport(...)` | `Sdf2d.viewport(...)` |
| `Math::random_2d(...)` | `Math.random_2d(...)` |
| `Pal::premul(...)` | `Pal.premul(...)` |
| `let x = val; x = other;` (implicit mut) | `let mut x = val; x = other` |
| `mix(mix(a, b, t), c, u)` | `a.mix(b, t).mix(c, u)` |
| `Self { field: val }` | `self (field: val)` |

### Animators

| Old | New |
|-----|-----|
| `animator: { ... }` | `animator: Animator{ ... }` |
| `state_name = { ... }` | `state_name: { ... }` |
| `default: off` | `default: @off` |
| `off = { ... }` | `off: AnimatorState{ ... }` |
| `[{time:0, value:1}]` | `snap(1.0)` |
| `[{time:0, value:0},{time:1, value:1}]` | `timeline(0 0  1 1)` |
| `cursor: Arrow` | `cursor: MouseCursor.Arrow` |

### Rust derives

| Old | New |
|-----|-----|
| `#[derive(Live, LiveHook, Widget)]` | `#[derive(Script, ScriptHook, Widget, Animator)]` |
| `#[animator]` | `#[apply_default]` |
| (none) | `#[uid] uid: WidgetUid` |
| (none) | `#[source] source: ScriptObjectRef` |
| `DrawIcon` | `DrawSvg` |
| `#[derive(DefaultNone)]` | `#[derive(Default)]` + `#[default]` |
| `impl LiveRegister` | `impl ScriptRegister` |
| `fn live_register(cx)` | `fn script_register(vm)` |
| `live_design(cx)` | `script_mod(vm)` |

### Resources and Rust-side APIs

| Old | New |
|-----|-----|
| `dep("crate://self/path")` | `crate_resource("self:path")` |
| `Image { source: dep(...) }` | `Image { src: crate_resource(...) }` |
| `MY_CONST = 0.33` then `(MY_CONST)` | `let my_const = 0.33` then `my_const` |
| `link widgets;` / `link my_theme;` | (removed — use `mod.prelude.*` imports) |
| `cx.link(live_id!(a), live_id!(b))` | (removed — theme set via `mod.theme = me` in script_mod) |
| `apply_over(cx, live!{...})` | Direct field mutation + `redraw(cx)`, or Animator |
| `live!{ prop: val }` | (removed entirely) |
| `WidgetRef::new_from_ptr(cx, ptr)` | `cx.with_vm(\|vm\| WidgetRef::script_from_value(vm, val))` |
| `Option<LivePtr>` (template ref) | `ScriptObjectRef` |
| `#[rust(DrawList2d::new(cx))]` | `#[rust] Option<DrawList2d>` + `on_after_new` init |
| `DrawList2d::new(cx)` | `DrawList2d::script_new(vm)` |
| `ComponentMap<K, LivePtr>` | `ComponentMap<K, ScriptObjectRef>` |

### Dock/Tabs

| Old | New |
|-----|-----|
| `Splitter` | `DockSplitter` |
| `Tabs` | `DockTabs` |
| `Tab` | `DockTab` |
| `axis: Horizontal` | `axis: SplitterAxis.Horizontal` |
| `align: FromA(0.0)` | `align: SplitterAlign.FromA(0.0)` |
| `a: name` | `a: @name` |
| `template: Name` | `template: @Name` |
| `kind: Name` | `kind: @Name` |

---

## Migration Checklist

When migrating a file from old to new:

### DSL Syntax (in `script_mod!` blocks)

1. **Replace macro**: `live_design!` → `script_mod!`
2. **Replace imports**: `link widgets; use link::*;` → `use mod.prelude.widgets.*`
3. **Remove all commas** between properties
4. **Replace theme refs**: `(THEME_XYZ)` → `theme.xyz`
5. **Replace inheritance**: `<Parent>` → `Parent` (or `mod.widgets.Parent` in widget crate)
6. **Replace registration**: `{{Struct}}` → `#(Struct::register_widget(vm))`
7. **Add `+:` merge operators** where you extend parent draw objects
8. **Replace named children**: `name = <Widget>{}` → `name := Widget{}`
9. **Update animators**: Add `Animator{}`, `AnimatorState{}`, `@off`, `snap()`, `timeline()`
10. **Update shaders**: `fn pixel(self) -> vec4` → `pixel: fn()`, `::` → `.`
11. **Add `let mut`** for mutable shader variables
12. **Replace resource refs**: `dep("crate://self/path")` → `crate_resource("self:path")`
13. **Replace image source**: `Image { source: ... }` → `Image { src: ... }`
14. **Replace DSL constants**: `MY_CONST = 0.33` then `(MY_CONST)` → `let my_const = 0.33` then `my_const`

### Rust-Side Changes

15. **Update Rust derives**: `Live` → `Script`, `LiveHook` → `ScriptHook`, add `Animator`
16. **Add required fields**: `#[uid] uid: WidgetUid`, `#[source] source: ScriptObjectRef`
17. **Replace `#[animator]`** with `#[apply_default]`
18. **Replace `DrawIcon`** with `DrawSvg`
19. **Replace `DefaultNone`** with `Default` + `#[default]` on the `None` variant
20. **Update `lib.rs`**: `live_design(cx)` → `script_mod(vm)`, `LiveRegister` → `ScriptRegister`
21. **Replace `apply_over` + `live!{}`** — No direct replacement. Use direct field mutation + `redraw(cx)`, `AdaptiveView`, or Animator (case-by-case analysis required)
22. **Replace `WidgetRef::new_from_ptr(cx, ptr)`** → `cx.with_vm(|vm| WidgetRef::script_from_value(vm, val))`
23. **Replace `Option<LivePtr>`** template storage → `ScriptObjectRef`
24. **Replace `DrawList2d::new(cx)`** → `DrawList2d::script_new(vm)` with `on_after_new` initialization
25. **Replace `ComponentMap<K, LivePtr>`** → `ComponentMap<K, ScriptObjectRef>`
26. **Remove `cx.link()` calls** — Replace with `mod.theme = me` in `script_mod!`
27. **Remove `link` directives** — Replace with `mod.prelude.*` imports

### Verification

28. **Compile** and fix all errors
29. **Test visually** — verify rendering matches original
30. **Test mobile layout** — `AdaptiveView`, responsive padding (especially former `apply_over` sites)
31. **Test dynamic features** — streaming chat, bot switching, attachment handling, provider connections
