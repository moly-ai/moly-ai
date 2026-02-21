# Splash Migration Guide

> Migrating from Makepad `live_design!` DSL to Splash `script_mod!`

This guide provides side-by-side comparisons for every pattern you need to change when migrating a Makepad application from the old `live_design!` macro system to the new Splash scripting system. It is organized by category so you can look up specific patterns quickly.

For a comprehensive reference on the Splash language itself, see [splash-language-reference.md](./splash-language-reference.md).

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
24. [New Capabilities (No Old Equivalent)](#24-new-capabilities-no-old-equivalent)
25. [Quick Reference Cheat Sheet](#25-quick-reference-cheat-sheet)

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

## 24. New Capabilities (No Old Equivalent)

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

## 25. Quick Reference Cheat Sheet

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
12. **Update Rust derives**: `Live` → `Script`, `LiveHook` → `ScriptHook`, add `Animator`
13. **Add required fields**: `#[uid] uid: WidgetUid`, `#[source] source: ScriptObjectRef`
14. **Replace `#[animator]`** with `#[apply_default]`
15. **Replace `DrawIcon`** with `DrawSvg`
16. **Replace `DefaultNone`** with `Default` + `#[default]`
17. **Update `lib.rs`**: `live_design(cx)` → `script_mod(vm)`, `LiveRegister` → `ScriptRegister`
18. **Test**: Compile and run to verify rendering matches
