# Migration Guide: Old `live_design!` to New `script_mod!` (Splash)

> This guide covers every syntactic and structural change required to migrate a Makepad project from the old `live_design!` / `Live` / `LiveHook` system to the new `script_mod!` / `Script` / `ScriptHook` (Splash) system. It is ordered by migration priority: structural changes first, then DSL syntax, then Rust code.

## Table of Contents

- [1. Macro Wrapper](#1-macro-wrapper)
- [2. Imports](#2-imports)
- [3. Struct Binding (Widget Registration)](#3-struct-binding-widget-registration)
- [4. Derive Macros](#4-derive-macros)
- [5. Struct Fields](#5-struct-fields)
- [6. Widget Inheritance](#6-widget-inheritance)
- [7. Property Syntax](#7-property-syntax)
- [8. Merge Operator](#8-merge-operator)
- [9. Named Children](#9-named-children)
- [10. Theme References](#10-theme-references)
- [11. Resource Paths](#11-resource-paths)
- [12. Colors](#12-colors)
- [13. Shader Variable Declarations](#13-shader-variable-declarations)
- [14. Shader Functions](#14-shader-functions)
- [15. Shader Method Calls](#15-shader-method-calls)
- [16. Shader Struct Constructors](#16-shader-struct-constructors)
- [17. Animator Blocks](#17-animator-blocks)
- [18. DefaultNone → Default](#18-defaultnone--default)
- [19. Runtime Property Updates (apply_over / live!)](#19-runtime-property-updates-apply_over--live)
- [20. App Bootstrapping](#20-app-bootstrapping)
- [21. Widget Registration Order (Multi-Module)](#21-widget-registration-order-multi-module)
- [22. Templates and Let Bindings](#22-templates-and-let-bindings)
- [23. Enum Values](#23-enum-values)
- [24. Padding and Inset Syntax](#24-padding-and-inset-syntax)
- [25. Alignment Syntax](#25-alignment-syntax)
- [26. Sizing and Walk](#26-sizing-and-walk)
- [27. Wrap Property](#27-wrap-property)
- [28. pub Keyword](#28-pub-keyword)
- [29. LiveRead Derive](#29-liveread-derive)
- [30. Cross-Module Sharing](#30-cross-module-sharing)
- [31. Comma Removal](#31-comma-removal)
- [32. Semicolon Removal](#32-semicolon-removal)
- [Appendix A: Full Before/After Widget Example](#appendix-a-full-beforeafter-widget-example)
- [Appendix B: Full Before/After App Example](#appendix-b-full-beforeafter-app-example)
- [Appendix C: Moly-Specific Migration Checklist](#appendix-c-moly-specific-migration-checklist)

---

## 1. Macro Wrapper

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
    use mod.prelude.widgets.*
    // ...
}
```

**Rules:**
- Replace `live_design!` → `script_mod!`
- Remove `link widgets;` line
- Remove `use link::theme::*;` and `use link::shaders::*;` lines
- Add `use mod.prelude.widgets.*` (for app code) or `use mod.prelude.widgets_internal.*` (for widget library internals)
- When defining new widgets for registration, also add `use mod.widgets.*`

---

## 2. Imports

**Old:**
```rust
use crate::{makepad_derive_widget::*, makepad_draw::*, widget::*};
```

**New:**
```rust
use crate::{
    makepad_derive_widget::*,
    makepad_draw::*,
    widget::*,
};
// If using ScriptFnRef for inline event handlers:
use crate::makepad_script::ScriptFnRef;
// If using async widget calls:
use crate::widget_async::{CxWidgetToScriptCallExt, ScriptAsyncResult};
```

The Rust-side imports are mostly the same, but you may need additional imports for script-related types. The key additions are `ScriptFnRef` and `ScriptObjectRef`.

---

## 3. Struct Binding (Widget Registration)

**Old:**
```rust
live_design! {
    pub ButtonBase = {{Button}} {}
}
```

**New:**
```rust
script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.ButtonBase = #(Button::register_widget(vm))
}
```

The double-brace `{{StructName}}` syntax is replaced by `#(StructName::register_widget(vm))` which calls into Rust at script evaluation time to register the struct with the ScriptVm.

**Full pattern with styled defaults:**

Old:
```rust
live_design! {
    pub ButtonBase = {{Button}} {}
    pub ButtonFlat = <ButtonBase> {
        text: "Button"
        width: Fit, height: Fit
        // ...
    }
}
```

New:
```rust
script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.ButtonBase = #(Button::register_widget(vm))

    mod.widgets.ButtonFlat = set_type_default() do mod.widgets.ButtonBase{
        text: "Button"
        width: Fit
        height: Fit
        // ...
    }
}
```

**Key details:**
- `set_type_default() do` wraps widget definitions that set defaults for a type
- The base registration `#(...)` creates the raw binding; `set_type_default() do BaseType{...}` creates a styled variant
- `pub` keyword is NOT used in `script_mod!` — visibility is controlled by Rust's module system

---

## 4. Derive Macros

### Widget structs

**Old:**
```rust
#[derive(Live, LiveHook, Widget)]
pub struct ChatScreen {
    #[deref] view: View,
}
```

**New:**
```rust
#[derive(Script, ScriptHook, Widget)]
pub struct ChatScreen {
    #[deref] view: View,
}
```

### Widget structs with animations

**Old:**
```rust
#[derive(Live, LiveHook, Widget)]
pub struct EntityButton {
    #[animator] animator: Animator,
    #[deref] view: View,
}
```

**New:**
```rust
#[derive(Script, ScriptHook, Widget, Animator)]
pub struct EntityButton {
    #[apply_default] animator: Animator,
    #[deref] view: View,
}
```

Note: `#[animator]` attribute → `#[apply_default]`, and `Animator` is added as a **derive** instead.

### Widget structs without LiveHook (manual impl)

**Old:**
```rust
#[derive(Live, Widget)]
pub struct ChatView {
    #[deref] view: View,
}
impl LiveHook for ChatView {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        // custom logic
    }
}
```

**New:**
```rust
#[derive(Script, Widget)]
pub struct ChatView {
    #[deref] view: View,
}
impl ScriptHook for ChatView {
    fn on_after_apply(&mut self, vm: &mut ScriptVm, apply: &Apply, scope: &mut Scope, value: ScriptValue) {
        // custom logic — note different signature
    }
}
```

### App structs

**Old:**
```rust
#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}
```

**New:**
```rust
#[derive(Script, ScriptHook)]
pub struct App {
    #[live] ui: WidgetRef,
}
```

### Data structs with Live

**Old:**
```rust
#[derive(Live, LiveHook, LiveRead, Clone, Debug)]
pub struct ProviderAddress {
    #[live] pub host: String,
    #[live] pub port: u16,
}
```

**New:**
```rust
#[derive(Script, ScriptHook, Clone, Debug)]
pub struct ProviderAddress {
    #[live] pub host: String,
    #[live] pub port: u16,
}
```

Note: `LiveRead` may no longer be needed; check if the struct is actually read from live data at runtime.

### Enum types

**Old:**
```rust
#[derive(Live, LiveHook, Clone, Copy)]
#[live_ignore]
pub enum ViewOptimize {
    #[pick] None,
    DrawList,
    Texture,
}
```

**New:**
```rust
#[derive(Script, ScriptHook, Clone, Copy)]
pub enum ViewOptimize {
    #[pick] None,
    DrawList,
    Texture,
}
```

Note: `#[live_ignore]` attribute is removed. The `#[pick]` attribute on the default variant remains.

### Summary table

| Old Derive | New Derive |
|---|---|
| `Live` | `Script` |
| `LiveHook` | `ScriptHook` |
| `Widget` | `Widget` (unchanged) |
| `LiveRegisterWidget` | `WidgetRegister` |
| `WidgetRef` | `WidgetRef` (unchanged) |
| `WidgetSet` | `WidgetSet` (unchanged) |

### Attribute changes

| Old Attribute | New Attribute |
|---|---|
| `#[animator]` | `#[apply_default]` |
| `#[live_ignore]` | removed |
| `#[live]` | `#[live]` (unchanged) |
| `#[rust]` | `#[rust]` (unchanged) |
| `#[deref]` | `#[deref]` (unchanged) |
| `#[walk]` | `#[walk]` (unchanged) |
| `#[layout]` | `#[layout]` (unchanged) |
| `#[redraw]` | `#[redraw]` (unchanged) |

### New required field

Structs that register as widgets with `#(Struct::register_widget(vm))` typically need:
```rust
#[source] source: ScriptObjectRef,
```

However, structs using `#[deref] view: View` (which inherits from View) already get this through View's own `source` field, so they don't need to add it separately.

---

## 5. Struct Fields

### Source field (new)

Widgets that register directly (not via `#[deref]`) need:

**New (added):**
```rust
#[source] source: ScriptObjectRef,
```

### UID field (new)

Direct widget structs also need:
```rust
#[uid] uid: WidgetUid,
```

Again, if you `#[deref]` to `View`, you get this for free.

---

## 6. Widget Inheritance

**Old (angle-bracket inheritance):**
```rust
live_design! {
    pub ChatScreen = {{ChatScreen}} {
        width: Fill, height: Fill
    }

    pub ModelCard = <RoundedView> {
        width: 200, height: 300
        draw_bg: { color: #333 }
    }

    pub ProviderView = {{ProviderView}} <RoundedShadowView> {
        // combines struct binding with visual inheritance
    }
}
```

**New (object-path inheritance):**
```rust
script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    // Simple widget registration
    mod.widgets.ChatScreenBase = #(ChatScreen::register_widget(vm))
    mod.widgets.ChatScreen = set_type_default() do mod.widgets.ChatScreenBase{
        width: Fill height: Fill
    }

    // Template (no struct binding, just reusable layout)
    let ModelCard = RoundedView{
        width: 200 height: 300
        draw_bg.color: #333
    }

    // Struct binding with visual inheritance
    mod.widgets.ProviderViewBase = #(ProviderView::register_widget(vm))
    mod.widgets.ProviderView = set_type_default() do mod.widgets.ProviderViewBase{
        // inherits from RoundedShadowView through Rust's #[deref]
    }
}
```

### Inheriting from standard widgets in inline usage

**Old:**
```rust
wrapper = <RoundedView> {
    width: Fill, height: Fit
    draw_bg: { color: #334 }
}
```

**New:**
```rust
wrapper := RoundedView{
    width: Fill height: Fit
    draw_bg.color: #334
}
```

The angle brackets `<Type>` become just `Type{...}`.

---

## 7. Property Syntax

### Commas → whitespace

**Old:**
```rust
width: Fill, height: Fit, flow: Down, spacing: 10,
```

**New:**
```
width: Fill height: Fit flow: Down spacing: 10
```

No commas between sibling properties. Commas are still used inside vec constructors with negative numbers: `vec4(-1.0, -1.0, -1.0, -1.0)`.

### Semicolons → nothing

**Old (in shaders):**
```rust
let color = self.color;
return sdf.result;
```

**New:**
```
let color = self.color
return sdf.result
```

No semicolons anywhere in Splash.

---

## 8. Merge Operator

**Old:** In the old system, nested blocks implicitly merged with parent:
```rust
draw_bg: {
    color: #f00    // This replaces just color, other draw_bg props preserved
}
```

**New:** You must use `+:` to merge, otherwise you replace the entire object:
```rust
draw_bg +: {
    color: #f00    // Only overrides color, keeps all other draw_bg properties
}
```

**Shorthand (dot-path):**
```
draw_bg.color: #f00
// Equivalent to: draw_bg +: { color: #f00 }
```

**Critical rule:** When migrating, almost every `draw_bg: {`, `draw_text: {`, `draw_icon: {` block should become `draw_bg +:`, `draw_text +:`, `draw_icon +:` UNLESS you intend to replace the entire object.

---

## 9. Named Children

**Old:**
```rust
header = <View> { width: Fill, height: 50 }
my_button = <Button> { text: "Click" }
```

**New:**
```
header := View{ width: Fill height: 50 }
my_button := Button{ text: "Click" }
```

The `=` assignment with `<Type>` becomes `:=` with `Type`.

**Rust-side usage is unchanged:**
```rust
self.ui.button(ids!(my_button)).clicked(actions)
self.ui.view(ids!(header))
```

---

## 10. Theme References

### Color theme constants

**Old:**
```rust
color: (THEME_COLOR_TEXT)
uniform color_hover: (THEME_COLOR_TEXT_HOVER)
```

**New:**
```
color: theme.color_text
color_hover: uniform(theme.color_text_hover)
```

### Font theme references

**Old:**
```rust
text_style: <THEME_FONT_REGULAR> {
    font_size: (THEME_FONT_SIZE_P)
}
```

**New:**
```
text_style: theme.font_regular{
    font_size: theme.font_size_p
}
```

### Spacing theme references

**Old:**
```rust
padding: <THEME_MSPACE_1> {}
spacing: (THEME_SPACE_2)
```

**New:**
```
padding: theme.mspace_1
spacing: theme.space_2
```

### Custom constants (Moly-specific)

Moly defines custom constants like `PRIMARY_COLOR`, `SIDEBAR_FONT_COLOR`, etc. in `src/shared/styles.rs`:

**Old:**
```rust
live_design! {
    PRIMARY_COLOR = #x2b55ff
    SIDEBAR_FONT_COLOR = #667085
    pub REGULAR_FONT = <THEME_FONT_REGULAR>{ font_size: (12) }
}

// Used as:
color: (PRIMARY_COLOR)
text_style: <REGULAR_FONT> {}
```

**New (option A - let bindings within same script_mod):**
```
let PRIMARY_COLOR = #x2b55ff
let SIDEBAR_FONT_COLOR = #667085
let REGULAR_FONT = theme.font_regular{font_size: 12}

// Used as:
color: PRIMARY_COLOR
text_style: REGULAR_FONT
```

**New (option B - shared via mod for cross-module access):**
```rust
// In styles.rs
script_mod! {
    use mod.prelude.widgets.*
    mod.moly_theme = {
        primary_color: #x2b55ff
        sidebar_font_color: #667085
        regular_font: theme.font_regular{font_size: 12}
    }
}

// In other files:
script_mod! {
    use mod.prelude.widgets.*
    // Access via mod.moly_theme.primary_color
}
```

### Naming convention mapping

| Old Name | New Name |
|---|---|
| `THEME_COLOR_TEXT` | `theme.color_text` |
| `THEME_COLOR_BG_APP` | `theme.color_bg_app` |
| `THEME_FONT_REGULAR` | `theme.font_regular` |
| `THEME_FONT_BOLD` | `theme.font_bold` |
| `THEME_FONT_ITALIC` | `theme.font_italic` |
| `THEME_FONT_ICONS` | `theme.font_icons` |
| `THEME_FONT_SIZE_P` | `theme.font_size_p` |
| `THEME_SPACE_1` | `theme.space_1` |
| `THEME_SPACE_2` | `theme.space_2` |
| `THEME_MSPACE_1` | `theme.mspace_1` |
| `THEME_MSPACE_V_1` | `theme.mspace_v_1` |
| `THEME_MSPACE_H_1` | `theme.mspace_h_1` |
| `THEME_CORNER_RADIUS` | `theme.corner_radius` |
| `THEME_BEVELING` | `theme.beveling` |

The pattern: `THEME_X_Y_Z` → `theme.x_y_z` (lowercase, dots instead of underscores for the `theme.` prefix).

---

## 11. Resource Paths

**Old:**
```rust
dep("crate://self/resources/icons/search.svg")
```

**New:**
```
crate_resource("self://resources/icons/search.svg")
```

The function name changes from `dep()` to `crate_resource()`, and the prefix changes from `crate://self/` to `self://`.

---

## 12. Colors

### Normal hex colors

Colors that don't contain `e` adjacent to digits work the same:
```
#f00      // Red
#ff4444   // Light red
#334      // Dark blue-gray
```

### Hex colors with 'e' (critical in script_mod!)

The Rust tokenizer parses `e` after digits as scientific notation. Use `#x` prefix:

**Old (worked in live_design!):**
```rust
color: #2ecc71
color: #1e1e2e
```

**New (must use #x prefix):**
```
color: #x2ecc71
color: #x1e1e2e
```

**Rule:** Any hex color where a digit is immediately followed by `e` or `E` needs `#x` prefix. Examples: `#1e...`, `#2e...`, `#4e...`, `#9e...`, `#0e...`, etc.

Colors like `#eee`, `#dead00`, `#beef00` are fine because the `e` is not preceded by a digit that forms a valid scientific notation prefix.

---

## 13. Shader Variable Declarations

**Old:**
```rust
draw_bg: {
    instance hover: 0.0
    instance down: 0.0
    uniform color: #fff
    uniform color_hover: #aaa
}
```

**New:**
```
draw_bg +: {
    hover: instance(0.0)
    down: instance(0.0)
    color: uniform(#fff)
    color_hover: uniform(#aaa)
}
```

**Pattern:** `instance name: value` → `name: instance(value)` and `uniform name: value` → `name: uniform(value)`.

The qualifier becomes a function-call wrapper around the value.

---

## 14. Shader Functions

**Old:**
```rust
draw_bg: {
    fn pixel(self) -> vec4 {
        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
        sdf.fill(self.color);
        return sdf.result;
    }

    fn get_color(self) -> vec4 {
        return mix(self.color, self.color_hover, self.hover);
    }
}
```

**New:**
```
draw_bg +: {
    pixel: fn() {
        let sdf = Sdf2d.viewport(self.pos * self.rect_size)
        sdf.box(0. 0. self.rect_size.x self.rect_size.y 4.0)
        sdf.fill(self.color)
        return sdf.result
    }

    get_color: fn() {
        return self.color.mix(self.color_hover, self.hover)
    }
}
```

**Changes:**
- `fn name(self) -> vec4 { ... }` → `name: fn() { ... }`
- Remove `self` parameter (it's implicit)
- Remove return type annotation (inferred)
- Remove semicolons
- `::` → `.` for associated function calls (see next section)
- Prefer method chaining: `mix(a, b, t)` → `a.mix(b, t)`

---

## 15. Shader Method Calls

**Old:**
```rust
Sdf2d::viewport(self.pos * self.rect_size)
Math::random_2d(self.pos.xy)
Pal::iq(t, a, b, c, d)
```

**New:**
```
Sdf2d.viewport(self.pos * self.rect_size)
Math.random_2d(self.pos.xy)
Pal.iq(t a b c d)
```

**Rule:** All `::` (path separator) in shader code becomes `.` (dot).

---

## 16. Shader Struct Constructors

**Old:**
```rust
return Self {
    field1: value1,
    field2: value2,
}
```

**New:**
```
return self (field1: value1, field2: value2)
```

`Self { ... }` becomes `self (...)` with a space (no braces).

---

## 17. Animator Blocks

**Old:**
```rust
animator: {
    hover = {
        default: off,
        off = {
            from: {all: Forward {duration: 0.15}}
            apply: {
                draw_bg: {hover: 0.0}
            }
        }
        on = {
            from: {all: Snap}
            apply: {
                draw_bg: {hover: 1.0}
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
            from: {all: Forward {duration: 0.15}}
            apply: {draw_bg: {hover: 0.0}}
        }
        on: AnimatorState{
            from: {all: Snap}
            apply: {draw_bg: {hover: 1.0}}
        }
    }
}
```

**Changes:**
- `animator: {` → `animator: Animator{`
- `state_name = {` → `state_name: AnimatorState{` (for the actual states)
- `track_name = {` → `track_name: {` (for the track grouping)
- `default: off` → `default: @off` (the `@` prefix is required)
- `default: on` → `default: @on`
- Remove commas
- States use `AnimatorState{...}` type wrapper

---

## 18. DefaultNone → Default

**Old:**
```rust
#[derive(Clone, Debug, DefaultNone)]
pub enum ChatAction {
    Start(ChatID),
    None,
}
```

**New:**
```rust
#[derive(Clone, Debug, Default)]
pub enum ChatAction {
    Start(ChatID),
    #[default]
    None,
}
```

- Replace `DefaultNone` derive with `Default`
- Add `#[default]` attribute on the `None` variant

---

## 19. Runtime Property Updates (apply_over / live!)

**Old:**
```rust
item.apply_over(cx, live! {
    height: (height_val)
    draw_bg: {
        color: (some_color)
        is_even: (if is_even { 1.0 } else { 0.0 })
    }
});
```

**New:**
```rust
script_apply_eval!(cx, item, {
    height: #(height_val)
    draw_bg: {
        color: #(some_color)
        is_even: #(if is_even { 1.0 } else { 0.0 })
    }
});
```

**Changes:**
- `item.apply_over(cx, live!{ ... })` → `script_apply_eval!(cx, item, { ... })`
- `(expr)` interpolation → `#(expr)` interpolation
- Remove commas and semicolons inside the block

**Common patterns in Moly:**

```rust
// Old: setting text
label.apply_over(cx, live! { text: (format!("Hello {}", name)) });
// New:
script_apply_eval!(cx, label, { text: #(format!("Hello {}", name)) });

// Old: toggling visibility
view.apply_over(cx, live! { visible: (is_visible) });
// New:
script_apply_eval!(cx, view, { visible: #(is_visible) });

// Old: setting colors
bg.apply_over(cx, live! { draw_bg: { color: (#f00) } });
// New:
script_apply_eval!(cx, bg, { draw_bg: { color: #(Vec4f::from_hex("#f00")) } });
```

---

## 20. App Bootstrapping

**Old:**
```rust
use makepad_widgets::*;

app_main!(App);

live_design! {
    link widgets;
    use link::theme::*;
    use link::shaders::*;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                body = <View> {
                    // ...
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        // register sub-modules
        crate::shared::widgets::live_design(cx);
        crate::chat::chat_screen::live_design(cx);
        // ...
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // ...
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

**New:**
```rust
use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    load_all_resources() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(800, 600)
                body +: {
                    // ...
                }
            }
        }
    }
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        crate::makepad_widgets::script_mod(vm);
        // register sub-modules
        crate::shared::widgets::script_mod(vm);
        crate::chat::chat_screen::script_mod(vm);
        // ...
        App::from_script_mod(vm, self::script_mod)
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // ...
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

**Key changes:**
- `LiveRegister` trait with `live_register(cx)` → `App::run(vm: &mut ScriptVm) -> Self`
- `crate::module::live_design(cx)` → `crate::module::script_mod(vm)`
- App definition wrapped in `load_all_resources() do #(App::script_component(vm)){...}`
- `App::from_script_mod(vm, self::script_mod)` at end of `run()`
- `<Root>` → `Root{}`
- `main_window = <Window>` → `main_window := Window{}`
- `body = <View>` → `body +: {` (merge into Window's existing body)

---

## 21. Widget Registration Order (Multi-Module)

**Old:**
```rust
// In lib.rs
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::shared::styles::live_design(cx);
        crate::shared::widgets::live_design(cx);
        crate::chat::chat_screen::live_design(cx);
        // order didn't matter as much in old system
    }
}
```

**New:**
```rust
// In app.rs
impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        // 1. Base widgets FIRST
        crate::makepad_widgets::script_mod(vm);
        // 2. Shared styles and custom widgets
        crate::shared::styles::script_mod(vm);
        crate::shared::widgets::script_mod(vm);
        // 3. Feature widgets that use shared widgets
        crate::chat::chat_screen::script_mod(vm);
        // 4. App UI LAST (uses everything above)
        App::from_script_mod(vm, self::script_mod)
    }
}
```

**Critical:** In the new system, registration order matters. Widget modules must be registered BEFORE UI modules that use them. If module B uses a widget defined in module A's `script_mod!`, then `A::script_mod(vm)` must be called before `B::script_mod(vm)`.

---

## 22. Templates and Let Bindings

**Old (local templates with angle brackets):**
```rust
live_design! {
    ModelFilesRow = <View> {
        width: Fill, height: Fit
        padding: 10
    }

    pub ModelFilesItem = {{ModelFilesItem}} <ModelFilesRow> {
        // uses ModelFilesRow as base
    }
}
```

**New (let bindings or mod.widgets):**
```rust
script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    // Local template (only usable within this script_mod)
    let ModelFilesRow = View{
        width: Fill height: Fit
        padding: 10
    }

    // Or shared template (usable across modules)
    mod.widgets.ModelFilesRow = View{
        width: Fill height: Fit
        padding: 10
    }

    mod.widgets.ModelFilesItemBase = #(ModelFilesItem::register_widget(vm))
    mod.widgets.ModelFilesItem = set_type_default() do mod.widgets.ModelFilesItemBase{
        // Note: Rust struct inherits layout via #[deref] view: View
        // Visual properties go here
    }
}
```

**Rules:**
- `let` bindings are **local** to their `script_mod!` block
- `mod.widgets.Name = ...` makes them **globally available** after registration
- `let` bindings must be defined **before** use (top-down order)

---

## 23. Enum Values

**Old:**
```rust
cursor: Hand
axis: Horizontal
fit: Stretch
```

**New:**
```
cursor: MouseCursor.Hand
axis: SplitterAxis.Horizontal
fit: ImageFit.Stretch
```

Enum values now require the full `EnumType.Variant` path with a dot separator.

---

## 24. Padding and Inset Syntax

**Old:**
```rust
padding: { left: 10, right: 10, top: 5, bottom: 5 }
padding: <THEME_MSPACE_1> { left: 20 }
```

**New:**
```
padding: Inset{left: 10 right: 10 top: 5 bottom: 5}
padding: theme.mspace_1{left: 20}
```

Bare `{ }` for Inset values must be prefixed with `Inset`. Theme references use direct object extension syntax.

---

## 25. Alignment Syntax

**Old:**
```rust
align: { x: 0.5, y: 0.5 }
```

**New:**
```
align: Align{x: 0.5 y: 0.5}
// Or use named shortcuts:
align: Center       // {x: 0.5, y: 0.5}
align: HCenter      // {x: 0.5, y: 0.0}
align: VCenter      // {x: 0.0, y: 0.5}
```

---

## 26. Sizing and Walk

**Old:**
```rust
label_walk: { width: Fit, height: Fit }
icon_walk: { width: 20, height: 20 }
```

**New:**
```
label_walk: Walk{width: Fit height: Fit}
icon_walk: Walk{width: 20 height: 20}
```

Walk values need the `Walk` type prefix.

---

## 27. Wrap Property

**Old:**
```rust
draw_text: {
    wrap: Word
}
```

**New:**
```
draw_text +: {
    // wrap property may have changed — check if still present
    // In new system, text wrapping is often default behavior
}
```

Check the new widget definitions for the current wrapping approach. The `wrap: Word` property may be handled differently.

---

## 28. pub Keyword

**Old:**
```rust
live_design! {
    pub ChatScreen = {{ChatScreen}} { ... }
    pub ICON_CLOSE = dep("crate://self/resources/icons/close.svg")
}
```

**New:**
```rust
script_mod! {
    mod.widgets.ChatScreen = ...   // No pub keyword
    // Constants go through mod namespace or let bindings
}
```

`pub` is not valid in `script_mod!`. Visibility is controlled by Rust's module system.

---

## 29. LiveRead Derive

**Old:**
```rust
#[derive(Live, LiveHook, LiveRead)]
pub struct SomeData { ... }
```

**New:**
```rust
#[derive(Script, ScriptHook)]
pub struct SomeData { ... }
```

`LiveRead` is removed. If read access is needed, handle it through normal Rust access patterns.

---

## 30. Cross-Module Sharing

**Old (link-based):**
```rust
// In widgets.rs
live_design! {
    link widgets;
    pub FadeView = {{FadeView}} { ... }
}

// In chat_screen.rs
live_design! {
    link widgets;
    use link::widgets::*;  // Brings FadeView into scope
}
```

**New (mod-based):**
```rust
// In widgets.rs
script_mod! {
    use mod.prelude.widgets_internal.*
    mod.widgets.FadeViewBase = #(FadeView::register_widget(vm))
    mod.widgets.FadeView = set_type_default() do mod.widgets.FadeViewBase{...}
}

// In chat_screen.rs
script_mod! {
    use mod.prelude.widgets.*
    // FadeView is now available because mod.prelude.widgets.* includes mod.widgets.*
    View{
        FadeView{...}
    }
}
```

The `link`/`use link::` system is replaced by the `mod` namespace. Anything registered in `mod.widgets.*` is available in any `script_mod!` that imports `mod.prelude.widgets.*` (as long as registration happened first).

---

## 31. Comma Removal

Remove all commas between sibling properties. Commas are only kept in:
- Negative number vectors: `vec4(-1.0, -1.0, -1.0, -1.0)`
- Some constructor argument lists where needed for disambiguation

**Old:**
```rust
width: Fill, height: Fit, flow: Down, spacing: 10,
```

**New:**
```
width: Fill height: Fit flow: Down spacing: 10
```

---

## 32. Semicolon Removal

Remove all semicolons from shader code and property values:

**Old:**
```rust
let sdf = Sdf2d::viewport(self.pos * self.rect_size);
sdf.fill(self.color);
return sdf.result;
```

**New:**
```
let sdf = Sdf2d.viewport(self.pos * self.rect_size)
sdf.fill(self.color)
return sdf.result
```

---

## Appendix A: Full Before/After Widget Example

### Old: EntityButton

```rust
use crate::{makepad_derive_widget::*, makepad_draw::*, widget::*};

live_design! {
    link widgets;
    use link::theme::*;
    use link::shaders::*;

    pub EntityButton = {{EntityButton}} <RoundedView> {
        width: Fill, height: Fit
        padding: 12
        spacing: 10
        flow: Right
        align: {y: 0.5}

        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance down: 0.0
            instance radius: 8.0
            color: #fff

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.radius);
                let base = self.color;
                let hover = mix(base, #eee, self.hover);
                let final_color = mix(hover, #ddd, self.down);
                sdf.fill(final_color);
                return sdf.result;
            }
        }

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.15}}
                    apply: {draw_bg: {hover: 0.0}}
                }
                on = {
                    from: {all: Forward {duration: 0.15}}
                    apply: {draw_bg: {hover: 1.0}}
                }
            }
            down = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {draw_bg: {down: 0.0}}
                }
                on = {
                    from: {all: Snap}
                    apply: {draw_bg: {down: 1.0}}
                }
            }
        }

        icon = <Icon> {
            draw_icon: {
                svg: dep("crate://self/resources/icons/entity.svg")
                color: #555
            }
            icon_walk: { width: 20, height: 20 }
        }
        label = <Label> {
            text: "Entity"
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 12 }
                color: #333
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct EntityButton {
    #[animator] animator: Animator,
    #[deref] view: View,
}
```

### New: EntityButton

```rust
use crate::{
    animator::{Animate, Animator, AnimatorAction, AnimatorImpl},
    makepad_derive_widget::*,
    makepad_draw::*,
    widget::*,
};

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.EntityButtonBase = #(EntityButton::register_widget(vm))
    mod.widgets.EntityButton = set_type_default() do mod.widgets.EntityButtonBase{
        width: Fill height: Fit
        padding: 12
        spacing: 10
        flow: Right
        align: Align{y: 0.5}

        show_bg: true
        draw_bg +: {
            hover: instance(0.0)
            down: instance(0.0)
            radius: instance(8.0)
            color: #fff

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(0. 0. self.rect_size.x self.rect_size.y self.radius)
                let base = self.color
                let hover = base.mix(#xeeeeee, self.hover)
                let final_color = hover.mix(#xdddddd, self.down)
                sdf.fill(final_color)
                return sdf.result
            }
        }

        animator: Animator{
            hover: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.15}}
                    apply: {draw_bg: {hover: 0.0}}
                }
                on: AnimatorState{
                    from: {all: Forward {duration: 0.15}}
                    apply: {draw_bg: {hover: 1.0}}
                }
            }
            down: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.1}}
                    apply: {draw_bg: {down: 0.0}}
                }
                on: AnimatorState{
                    from: {all: Snap}
                    apply: {draw_bg: {down: 1.0}}
                }
            }
        }

        icon := Icon{
            draw_icon +: {
                svg: crate_resource("self://resources/icons/entity.svg")
                color: #555
            }
            icon_walk: Walk{width: 20 height: 20}
        }
        label := Label{
            text: "Entity"
            draw_text +: {
                text_style: theme.font_regular{font_size: 12}
                color: #333
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget, Animator)]
pub struct EntityButton {
    #[apply_default] animator: Animator,
    #[deref] view: View,
}
```

---

## Appendix B: Full Before/After App Example

### Old: Minimal App

```rust
use makepad_widgets::*;

app_main!(App);

live_design! {
    link widgets;
    use link::theme::*;
    use link::shaders::*;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                body = {
                    flow: Down, spacing: 10, padding: 20
                    align: {x: 0.5, y: 0.5}

                    greeting = <Label> {
                        text: "Hello, World!"
                        draw_text: {
                            text_style: <THEME_FONT_BOLD> { font_size: 24 }
                            color: (THEME_COLOR_TEXT)
                        }
                    }
                    action_button = <Button> {
                        text: "Click Me"
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(ids!(action_button)).clicked(actions) {
            self.ui.label(ids!(greeting)).set_text(cx, "Button clicked!");
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

### New: Minimal App

```rust
use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    load_all_resources() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(800, 600)
                body +: {
                    flow: Down spacing: 10 padding: 20
                    align: Center

                    greeting := Label{
                        text: "Hello, World!"
                        draw_text +: {
                            text_style: theme.font_bold{font_size: 24}
                            color: theme.color_text
                        }
                    }
                    action_button := Button{
                        text: "Click Me"
                    }
                }
            }
        }
    }
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        crate::makepad_widgets::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(ids!(action_button)).clicked(actions) {
            self.ui.label(ids!(greeting)).set_text(cx, "Button clicked!");
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

---

## Appendix C: Moly-Specific Migration Checklist

Based on the Moly codebase audit, here is a prioritized checklist:

### Phase 1: Foundation (do first)
- [ ] Migrate `src/shared/styles.rs` — custom constants, fonts, theme definitions
- [ ] Migrate `src/shared/resource_imports.rs` — icon constants (`dep()` → `crate_resource()`)
- [ ] Migrate `src/shared/widgets.rs` — shared widget templates (32 instance/uniform declarations)
- [ ] Migrate `src/shared/desktop_buttons.rs` — custom draw shaders

### Phase 2: Shared Components
- [ ] Migrate `src/shared/tooltip.rs`
- [ ] Migrate `src/shared/popup_notification.rs`
- [ ] Migrate `src/shared/moly_server_popup.rs`
- [ ] Migrate `src/shared/download_notification_popup.rs`
- [ ] Migrate `src/shared/external_link.rs`
- [ ] Migrate `src/shared/meta.rs`
- [ ] Migrate `src/shared/list.rs`

### Phase 3: MolyKit Widgets (library — high impact)
- [ ] Migrate `moly-kit/src/widgets/chat.rs`
- [ ] Migrate `moly-kit/src/widgets/chat_line.rs` (complex: shaders, animators, THEME refs)
- [ ] Migrate `moly-kit/src/widgets/messages.rs`
- [ ] Migrate `moly-kit/src/widgets/message_markdown.rs`
- [ ] Migrate `moly-kit/src/widgets/message_loading.rs`
- [ ] Migrate `moly-kit/src/widgets/message_thinking_block.rs`
- [ ] Migrate `moly-kit/src/widgets/prompt_input.rs`
- [ ] Migrate `moly-kit/src/widgets/model_selector.rs`
- [ ] Migrate `moly-kit/src/widgets/model_selector_list.rs`
- [ ] Migrate `moly-kit/src/widgets/model_selector_item.rs`
- [ ] Migrate `moly-kit/src/widgets/stt_input.rs`
- [ ] Migrate `moly-kit/src/widgets/realtime.rs` (complex: THEME refs, shaders)
- [ ] Migrate `moly-kit/src/widgets/moly_modal.rs`
- [ ] Migrate `moly-kit/src/widgets/slot.rs`
- [ ] Migrate `moly-kit/src/widgets/standard_message_content.rs`
- [ ] Migrate `moly-kit/src/widgets/image_view.rs`
- [ ] Migrate `moly-kit/src/widgets/avatar.rs`
- [ ] Migrate `moly-kit/src/widgets/citation.rs`
- [ ] Migrate `moly-kit/src/widgets/citation_list.rs`
- [ ] Migrate `moly-kit/src/widgets/attachment_list.rs`
- [ ] Migrate `moly-kit/src/widgets/attachment_view.rs`
- [ ] Migrate `moly-kit/src/widgets/attachment_viewer_modal.rs`
- [ ] Migrate `moly-kit/src/widgets/theme_moly_kit_light.rs`

### Phase 4: All DefaultNone enums (31 across 25 files)
- [ ] Replace all `#[derive(..., DefaultNone)]` with `#[derive(..., Default)]` + `#[default]` on `None` variant

### Phase 5: Feature screens (Moly app)
- [ ] Migrate chat screens (`chat_screen.rs`, `chat_view.rs`, `chat_screen_mobile.rs`, etc.)
- [ ] Migrate landing screens (`landing_screen.rs`, `model_card.rs`, `model_list.rs`, etc.)
- [ ] Migrate settings screens (`providers.rs`, `provider_view.rs`, etc.)
- [ ] Migrate my_models screens
- [ ] Migrate MCP screens

### Phase 6: App shell
- [ ] Migrate `src/app.rs` — root app, sidebar, registration
- [ ] Update `App::live_register` → `App::run`
- [ ] Verify module registration order

### Phase 7: Examples
- [ ] Migrate `moly-kit/examples/moly-mini/`

### Phase 8: Cleanup
- [ ] Remove all `live_design!` macro calls
- [ ] Remove all `live!{}` macro calls
- [ ] Remove `DefaultNone` from all imports
- [ ] Remove `LiveRegister` impls
- [ ] Verify all `apply_over` converted to `script_apply_eval!`
- [ ] Run full build and fix remaining compilation errors
