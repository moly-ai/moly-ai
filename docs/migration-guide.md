# Migration Guide: Old DSL (`live_design!`) → Splash (`script_mod!`)

This guide shows how to migrate Makepad applications from the old `live_design!` macro system (v1) to the new Splash `script_mod!` system (v2). Each section shows old and new patterns side by side.

---

## 1. Macro Wrapper

**Old:**
```rust
live_design!{
    // DSL code
}
```

**New:**
```rust
script_mod!{
    // Splash code
}
```

---

## 2. Imports and Linking

**Old:**
```rust
live_design!{
    link widgets;
    use link::theme::*;
    use link::shaders::*;
}
```

**New (app code):**
```rust
script_mod!{
    use mod.prelude.widgets.*
}
```

**New (widget library internals):**
```rust
script_mod!{
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
}
```

There is no `link` directive or `use link::` in the new system. Everything lives in the `mod.*` namespace. The `use mod.prelude.widgets.*` import brings in all widgets, theme, draw shaders, math, layout types, and standard library functions. Widget library authors use `widgets_internal` (which excludes widget definitions to avoid circularity) plus `mod.widgets.*`.

---

## 3. Widget Registration

**Old:**
```rust
live_design!{
    link widgets;
    use link::theme::*;
    
    pub ButtonBase = {{Button}} {}
}
```

**New:**
```rust
script_mod!{
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.ButtonBase = #(Button::register_widget(vm))
}
```

The `{{StructName}}` syntax is replaced by `#(StructName::register_widget(vm))`. The `#(...)` syntax is a Rust escape hatch that evaluates the Rust expression inside the Splash code. The `pub` keyword is replaced by storing the result in the `mod.widgets.*` namespace.

---

## 4. Styled Widget Variants

**Old:**
```rust
live_design!{
    pub ButtonFlat = <ButtonBase> {
        text: "Button"
        width: Fit, height: Fit,
        spacing: (THEME_SPACE_2),
        align: {x: 0.5, y: 0.5},
        padding: <THEME_MSPACE_1> { left: (THEME_SPACE_2), right: (THEME_SPACE_2) }
    }
}
```

**New:**
```rust
script_mod!{
    mod.widgets.ButtonFlat = set_type_default() do mod.widgets.ButtonBase{
        text: "Button"
        width: Fit
        height: Fit
        spacing: theme.space_2
        align: Center
        padding: theme.mspace_1{left: theme.space_2, right: theme.space_2}
    }
}
```

Changes:
- `<BaseWidget>` → `mod.widgets.BaseWidget` or just `BaseWidget` (if imported via `use mod.widgets.*`)
- No angle brackets
- No commas between properties
- `(THEME_CONSTANT)` → `theme.constant_name` (lowercase, dot notation)
- `<THEME_CONSTANT>` → `theme.constant_name{}` (for struct-like theme values that can be extended)
- `{x: 0.5, y: 0.5}` → `Center` (use named alignment shortcuts)
- `set_type_default() do` wraps the definition to set it as the default values for the type

---

## 5. Angle Brackets → No Angle Brackets

**Old:**
```rust
<View> {
    <Label> { text: "Hello" }
    <Button> { text: "Click" }
}
```

**New:**
```rust
View{
    Label{text: "Hello"}
    Button{text: "Click"}
}
```

Every `<WidgetName>` becomes just `WidgetName`. No exceptions.

---

## 6. Theme Constants

**Old:**
```rust
color: (THEME_COLOR_BG_APP)
padding: <THEME_MSPACE_1> {}
font_size: (THEME_FONT_SIZE_P)
text_style: <THEME_FONT_REGULAR> {}
```

**New:**
```rust
color: theme.color_bg_app
padding: theme.mspace_1
font_size: theme.font_size_p
text_style: theme.font_regular
```

Rules:
- `(THEME_X)` → `theme.x` (lowercase, underscores, dot notation)
- `<THEME_X> {}` → `theme.x` or `theme.x{}` (if extending with overrides)
- All uppercase is converted to lowercase

Common mappings:

| Old | New |
|-----|-----|
| `(THEME_COLOR_BG_APP)` | `theme.color_bg_app` |
| `(THEME_SPACE_2)` | `theme.space_2` |
| `(THEME_FONT_SIZE_P)` | `theme.font_size_p` |
| `<THEME_FONT_REGULAR>` | `theme.font_regular` |
| `<THEME_MSPACE_1>` | `theme.mspace_1` |
| `(THEME_BEVELING)` | `theme.beveling` |
| `(THEME_CORNER_RADIUS)` | `theme.corner_radius` |

---

## 7. Commas Between Properties

**Old:**
```rust
width: Fit, height: Fit,
spacing: 10,
padding: {left: 5, right: 5},
```

**New:**
```rust
width: Fit
height: Fit
spacing: 10
padding: Inset{left: 5, right: 5}
```

No commas between top-level properties. Commas are optional inside inline objects (like `Inset{left: 5, right: 5}`) but not required.

---

## 8. Named Children (`:=`)

**Old:**
```rust
my_label = <Label> { text: "Hello" }
```

**New:**
```rust
my_label := Label{text: "Hello"}
```

The named child operator changes from `=` to `:=`. The `:=` operator makes the child addressable from Rust code and overridable from parent templates.

---

## 9. Property Merging (`+:`)

**Old (replacing a sub-object):**
```rust
draw_bg: {
    color: #ff0000
}
```

**New (merging with parent):**
```rust
draw_bg +: {
    color: #ff0000
}
```

In the old system, using `draw_bg: {}` on a derived widget would sometimes merge by default (the behavior was implicit). In the new system, you must explicitly use `+:` to merge. Without `+:`, you **replace** the entire object.

This is the most common migration mistake. Always use `+:` when overriding `draw_bg`, `draw_text`, `draw_icon`, `animator`, or any sub-object where you want to keep the parent's properties.

---

## 10. Shader Instance / Uniform Declarations

**Old:**
```rust
draw_bg: {
    instance hover: 0.0
    instance down: 0.0
    uniform color: #333
}
```

**New:**
```rust
draw_bg +: {
    hover: instance(0.0)
    down: instance(0.0)
    color: uniform(#333)
}
```

The keyword-prefix syntax (`instance`, `uniform`) becomes a function-call wrapper.

| Old | New |
|-----|-----|
| `instance x: 0.0` | `x: instance(0.0)` |
| `uniform x: #fff` | `x: uniform(#fff)` |
| `texture x: texture_2d(float)` | `x: texture_2d(float)` |
| `varying x: 0.0` | `x: varying(0.0)` |

---

## 11. Shader Function Syntax

**Old:**
```rust
draw_bg: {
    fn pixel(self) -> vec4 {
        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 2.);
        sdf.fill(self.color);
        return sdf.result;
    }
    
    fn get_color(self) -> vec4 {
        return mix(self.color, #f00, self.hover);
    }
}
```

**New:**
```rust
draw_bg +: {
    pixel: fn() {
        let sdf = Sdf2d.viewport(self.pos * self.rect_size)
        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 2.)
        sdf.fill(self.color)
        return sdf.result
    }
    
    get_color: fn() {
        return self.color.mix(#f00, self.hover)
    }
}
```

Changes:
- `fn name(self) -> vec4 { ... }` → `name: fn() { ... }`
- `Sdf2d::method()` → `Sdf2d.method()` (dot, not double-colon)
- `Math::method()` → `Math.method()` (dot, not double-colon)
- `mix(a, b, t)` → `a.mix(b, t)` (method call on color)
- Semicolons optional (but allowed)
- No explicit return type
- `mod(a, b)` → `modf(a, b)` (renamed)
- `atan(y, x)` → `atan2(y, x)` (renamed, two-arg form)

---

## 12. Animator Syntax

**Old:**
```rust
animator: {
    hover = {
        default: off
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
```rust
animator: Animator{
    hover: {
        default: @off
        off: AnimatorState{
            from: {all: Forward{duration: 0.1}}
            apply: {
                draw_bg: {hover: 0.0}
            }
        }
        on: AnimatorState{
            from: {all: Forward{duration: 0.1}}
            apply: {
                draw_bg: {hover: snap(1.0)}
            }
        }
    }
}
```

Changes:
- `animator: { ... }` → `animator: Animator{ ... }`
- `hover = { ... }` → `hover: { ... }`
- `off = { ... }` → `off: AnimatorState{ ... }`
- `default: off` → `default: @off`
- `[{time: 0.0, value: 1.0}]` → `snap(1.0)` (instant keyframe)
- State names use `:` not `=`
- Wrap states with `AnimatorState{}`
- `cursor: Arrow` → `cursor: MouseCursor.Arrow`

---

## 13. Color Hex Codes with `e`

**Old:**
```rust
color: #2ecc71
color: #1e1e2e
```

**New:**
```rust
color: #x2ecc71
color: #x1e1e2e
```

Any hex color containing the letter `e` must use the `#x` prefix because the Rust tokenizer interprets sequences like `2e` as scientific notation. Prefix with `#x` to escape.

---

## 14. Resource Paths

**Old:**
```rust
source: dep("crate://self/resources/image.png")
svg_file: dep("crate://self/resources/icon.svg")
path: dep("crate://self/resources/font.ttf")
```

**New:**
```rust
source: crate_resource("self://resources/image.png")
svg: crate_resource("self://resources/icon.svg")
path: crate_resource("self://resources/font.ttf")
```

- `dep("crate://self/...")` → `crate_resource("self://...")`
- Note: `crate://self/` becomes `self://`
- For HTTP resources: `http_resource("https://...")`

---

## 15. Margin → Inset

**Old:**
```rust
margin: {left: 10, top: 5}
padding: {left: 10, right: 10, top: 5, bottom: 5}
```

**New:**
```rust
margin: Inset{left: 10, top: 5}
padding: Inset{left: 10, right: 10, top: 5, bottom: 5}
```

Bare `{left: 10}` blocks don't work for margin/padding. Use `Inset{...}` constructor. For uniform padding, just use a number: `padding: 16`. The `Margin` type was renamed to `Inset` throughout the new codebase.

---

## 16. Align Syntax

**Old:**
```rust
align: {x: 0.5, y: 0.5}
align: {x: 0.0, y: 0.5}
```

**New:**
```rust
align: Center
align: VCenter
```

Or explicitly: `align: Align{x: 0.5, y: 0.5}`. Shortcuts: `Center`, `HCenter`, `VCenter`, `TopLeft`.

---

## 17. DrawIcon → DrawSvg

**Old:**
```rust
#[live]
draw_icon: DrawIcon,

// In DSL:
draw_icon: {
    svg_file: dep("crate://self/resources/icon.svg")
}
```

**New:**
```rust
#[live]
draw_icon: DrawSvg,

// In Splash:
draw_icon +: {
    svg: crate_resource("self://resources/icon.svg")
}
```

The `DrawIcon` type is renamed to `DrawSvg`. The `svg_file` property becomes `svg`.

---

## 18. Flow / Layout Shortcuts

**Old:**
```rust
flow: Overlay
flow: Right
flow: Down
```

**New:**
```rust
flow: Overlay
flow: Right
flow: Down
flow: Flow.Right{wrap: true}   // wrapping flow
```

Most flow values are the same. `Flow.Right{wrap: true}` for wrapping is new.

---

## 19. PortalList Template Children

**Old:**
```rust
<PortalList> {
    TodoItem = <View> {
        // template definition
    }
    EmptyItem = <View> {}
}
```

**New:**
```rust
PortalList{
    TodoItem := View{
        // template definition
    }
    EmptyItem := View{}
}
```

Templates use `:=` instead of `=`. No angle brackets.

---

## 20. `apply_over` → `script_apply_eval!`

**Old (Rust):**
```rust
self.apply_over(cx, live!{
    draw_bg: { color: (some_color) }
    visible: (is_visible)
});

// Or on a child:
item.apply_over(cx, live!{
    draw_bg: { hover: (hover_value) }
});
```

**New (Rust):**
```rust
script_apply_eval!(cx, self, {
    draw_bg +: { color: #(some_color) }
    visible: #(is_visible)
});

// Or on a child:
script_apply_eval!(cx, item, {
    draw_bg +: { hover: #(hover_value) }
});
```

Changes:
- `self.apply_over(cx, live!{...})` → `script_apply_eval!(cx, self, {...})`
- `(expr)` Rust interpolation → `#(expr)` interpolation
- Remember to use `+:` for merging sub-objects

---

## 21. live_design Registration → script_mod Registration

**Old (lib.rs):**
```rust
pub fn live_design(cx: &mut Cx) {
    makepad_widgets::live_design(cx);
    crate::my_widget::live_design(cx);
    crate::app::live_design(cx);
}
```

**Old (each module):**
```rust
// Generated by live_design! macro
pub fn live_design(cx: &mut Cx) { ... }
```

**New (lib.rs):**
```rust
pub fn script_mod(vm: &mut ScriptVm) {
    makepad_widgets::script_mod(vm);
    crate::my_widget::script_mod(vm);
    crate::app::script_mod(vm);
}
```

**New (each module):**
```rust
// Generated by script_mod! macro
pub fn script_mod(vm: &mut ScriptVm) { ... }
```

The function signature changes from `fn live_design(cx: &mut Cx)` to `fn script_mod(vm: &mut ScriptVm)`.

---

## 22. App Startup Pattern

**Old:**
```rust
live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    
    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                body = <View> {
                    // UI tree
                }
            }
        }
    }
}

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
    fn handle_startup(&mut self, cx: &mut Cx) {
        // startup logic
    }
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // handle actions
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
```

**New:**
```rust
script_mod!{
    use mod.prelude.widgets.*
    
    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(800, 600)
                body +: {
                    // UI tree
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

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        makepad_widgets::script_mod(vm);
        crate::script_mod(vm);
        vm.run()
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // startup logic
    }
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // handle actions
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
```

Key differences:
- `{{App}}` → `#(App::script_component(vm))` wrapped in `startup() do`
- `#[derive(Live, LiveHook)]` → `#[derive(Script, ScriptHook)]`
- Added `#[source] source: ScriptObjectRef` field
- `LiveRegister` impl → `fn run(vm: &mut ScriptVm)` method
- `live_design(cx)` calls → `script_mod(vm)` calls + `vm.run()`
- `app_main!` still exists and works the same way
- `MatchEvent` still exists and works the same way

---

## 23. Widget Lookup

**Old:**
```rust
// Path-based lookup (traverses through named children)
let label = self.ui.widget(id!(my_list.item_label));
let button = self.ui.button(id!(submit_btn));

// In handle_event, with Scope path tracking
fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
    // scope.path tracks the widget path for action routing
    let uid = self.widget_uid();
    cx.widget_action(uid, &scope.path, MyAction::Clicked);
}
```

**New:**
```rust
// Tree-based lookup (uses widget tree with flood-fill search)
let label = self.ui.widget(cx, ids!(my_list, item_label));
let button = self.ui.button(cx, ids!(submit_btn));

// Flood-fill search: searches children first, then expands outward
let widget = self.ui.widget_flood(cx, ids!(some_widget));

// In handle_event, no scope.path needed
fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
    let uid = self.widget_uid();
    cx.widget_action(uid, MyAction::Clicked);  // no path argument
}
```

Key changes:
- `id!(name)` → `ids!(name)` (returns `&[LiveId]`)
- `widget(id!(...))` → `widget(cx, ids!(...))` (needs `cx` for tree access)
- `button(id!(...))` → `button(cx, ids!(...))` (all typed accessors gain `cx`)
- New `widget_flood()` for proximity-based search
- `Scope::path` removed — no more path tracking through the widget tree
- `cx.widget_action(uid, &scope.path, action)` → `cx.widget_action(uid, action)`

---

## 24. `find_widgets` → `children`

**Old (implementing WidgetNode):**
```rust
impl WidgetNode for MyWidget {
    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.some_child.find_widgets(path, cached, results);
        for child in &self.children {
            child.find_widgets(path, cached, results);
        }
    }
}
```

**New (implementing WidgetNode):**
```rust
impl WidgetNode for MyWidget {
    fn children(&self, visit: &mut dyn FnMut(LiveId, WidgetRef)) {
        visit(live_id!(some_child), self.some_child.clone());
        for (id, child) in &self.children {
            visit(*id, child.clone());
        }
    }
}
```

The tree is now managed globally. Widgets just enumerate their direct children and the widget tree builds a searchable index.

---

## 25. Action Casting

**Old:**
```rust
// Finding actions for a specific widget
if let Some(action) = self.ui.button(id!(my_btn)).pressed(&actions) {
    // handle pressed
}

// Manual casting
for action in actions {
    if let Some(wa) = action.as_widget_action() {
        if let Some(btn_action) = wa.action.cast_ref::<ButtonAction>() {
            // ...
        }
    }
}

// find_widget_action with cast_ref
if let Some(action) = actions.find_widget_action(self.ui.button(id!(my_btn)).widget_uid()) {
    if let Some(clicked) = action.cast_ref::<ButtonAction>() {
        // ...
    }
}
```

**New:**
```rust
// Same high-level API still works
if let Some(action) = self.ui.button(cx, ids!(my_btn)).pressed(&actions) {
    // handle pressed
}

// find_widget_action returns Option<&WidgetAction>
if let Some(action) = actions.find_widget_action(self.ui.button(cx, ids!(my_btn)).widget_uid()) {
    if let Some(clicked) = action.cast::<ButtonAction>() {
        // cast() not cast_ref()
    }
}
```

The `WidgetAction` struct no longer has `path` or `widgets` fields. The `cast_ref()` method is renamed to `cast()` in some contexts.

---

## 26. `Scope::with_data` Pattern

**Old:**
```rust
fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
    let data = scope.data.get::<MyData>().unwrap();
    // use data
}

// Calling:
child.draw_all(cx, &mut Scope::with_data(&mut my_data));
```

**New:**
```rust
// Same pattern, Scope still has data/props
fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
    let data = scope.data.get::<MyData>().unwrap();
    // use data
}

// Calling:
child.draw_all(cx, &mut Scope::with_data(&mut my_data));
```

`Scope::with_data`, `Scope::with_props`, `Scope::empty()` still work. The only removed feature is `Scope::path` and `Scope::with_id()`.

---

## 27. Deep Path Property Overrides

**Old:**
```rust
<MyTemplate> {
    sender = {
        avatar = {
            draw_bg: { color: #008F7E }
        }
    }
}
```

**New:**
```rust
MyTemplate{
    sender.avatar.draw_bg +: { color: #008F7E }
}
```

The new system supports dot-notation paths for deep property overrides. This is more concise than nesting.

---

## 28. ComponentMap (Dynamic Widget Lists)

**Old:**
```rust
use makepad_widgets::ComponentMap;

#[live]
#[rust]
items: ComponentMap<LiveId, WidgetRef>,

// In draw:
let item = self.items.get_or_insert(cx, id, |cx| {
    WidgetRef::new_from_ptr(cx, self.template)
});
```

**New:**

The `ComponentMap` pattern is largely replaced by the `on_render` callback in Splash, or by script-side dynamic rendering. For Rust-side dynamic lists, the `PortalList` widget with `on_render` is the primary pattern.

If you still need Rust-side dynamic widget creation, the API changes are:
- `WidgetRef::new_from_ptr(cx, live_ptr)` → check the new `ScriptVm`-based creation API
- `ComponentMap` may or may not exist in the new system; the pattern is strongly discouraged in favor of script-side rendering

---

## 29. Timer / Timeout Patterns

**Old:**
```rust
#[rust]
timer: Timer,

// Start:
self.timer = cx.start_timeout(0.5);
self.timer = cx.start_interval(1.0);

// Handle:
if self.timer.is_event(event).is_some() {
    // timer fired
}
```

**New:**
The timer API is the same — `cx.start_timeout()`, `cx.start_interval()`, and `Timer::is_event()` still work.

---

## 30. Cursor Syntax

**Old:**
```rust
cursor: Arrow
cursor: Hand
cursor: Default
```

**New:**
```rust
cursor: MouseCursor.Arrow
cursor: MouseCursor.Hand
cursor: MouseCursor.Default
```

All cursor values require the `MouseCursor.` prefix.

---

## 31. Full Before/After Example: A Simple Widget

### Old (complete):

```rust
use makepad_widgets::*;

live_design!{
    link widgets;
    use link::theme::*;
    use link::shaders::*;
    
    pub CounterBase = {{Counter}} {}
    
    pub Counter = <CounterBase> {
        width: 200, height: 60,
        flow: Right,
        spacing: 10,
        align: {x: 0.5, y: 0.5},
        
        draw_bg: {
            color: (THEME_COLOR_BG_APP)
        }
        
        label = <Label> {
            text: "Count: 0"
            draw_text: {
                color: (THEME_COLOR_LABEL_INNER)
            }
        }
        
        button = <Button> {
            text: "Increment"
        }
        
        animator: {
            highlight = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.3}}
                    apply: {draw_bg: {color: (THEME_COLOR_BG_APP)}}
                }
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {draw_bg: {color: #2ecc71}}
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum CounterAction {
    None,
    Incremented(u32),
}

#[derive(Live, LiveHook, Widget)]
pub struct Counter {
    #[animator]
    animator: Animator,
    
    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    
    #[walk]
    walk: Walk,
    
    #[layout]
    layout: Layout,
    
    #[live(true)]
    #[visible]
    visible: bool,
    
    #[rust]
    count: u32,
}

impl Widget for Counter {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        self.animator_handle_event(cx, event);
        
        let actions = cx.capture_actions(|cx| {
            self.ui.handle_event(cx, event, scope);
        });
        
        if self.ui.button(id!(button)).clicked(&actions) {
            self.count += 1;
            self.ui.label(id!(label)).set_text(
                cx, &format!("Count: {}", self.count)
            );
            self.animator_play(cx, ids!(highlight.on));
            cx.widget_action(uid, &scope.path, CounterAction::Incremented(self.count));
        }
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        self.ui.draw_all(cx, scope);
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}
```

### New (complete):

```rust
use makepad_widgets::*;

script_mod!{
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.CounterBase = #(Counter::register_widget(vm))

    mod.widgets.Counter = set_type_default() do mod.widgets.CounterBase{
        width: 200
        height: 60
        flow: Right
        spacing: 10
        align: Center

        draw_bg +: {
            color: uniform(theme.color_bg_app)
        }

        label := Label{
            text: "Count: 0"
            draw_text +: {
                color: theme.color_label_inner
            }
        }

        button := Button{
            text: "Increment"
        }

        animator: Animator{
            highlight: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward{duration: 0.3}}
                    apply: {draw_bg: {color: theme.color_bg_app}}
                }
                on: AnimatorState{
                    from: {all: Forward{duration: 0.1}}
                    apply: {draw_bg: {color: #x2ecc71}}
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum CounterAction {
    #[default]
    None,
    Incremented(u32),
}

#[derive(Script, ScriptHook, Widget, Animator)]
pub struct Counter {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[apply_default]
    animator: Animator,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    #[live(true)]
    #[visible]
    visible: bool,

    #[rust]
    count: u32,
}

impl Widget for Counter {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        let uid = self.widget_uid();
        self.animator_handle_event(cx, event);

        let actions = cx.capture_actions(|cx| {
            self.ui.handle_event(cx, event, &mut Scope::empty());
        });

        if self.ui.button(cx, ids!(button)).clicked(&actions) {
            self.count += 1;
            self.ui.label(cx, ids!(label)).set_text(
                cx, &format!("Count: {}", self.count)
            );
            self.animator_play(cx, ids!(highlight.on));
            cx.widget_action(uid, CounterAction::Incremented(self.count));
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        self.ui.draw_all(cx, scope);
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}
```

---

## Quick Reference: All Syntax Changes

| Old | New |
|-----|-----|
| `live_design!{}` | `script_mod!{}` |
| `<Widget>{}` | `Widget{}` |
| `{{Struct}}` | `#(Struct::register_widget(vm))` |
| `pub Name = <Base>{}` | `mod.widgets.Name = mod.widgets.Base{}` |
| `name = <Widget>{}` | `name := Widget{}` |
| `(THEME_X)` | `theme.x` |
| `<THEME_X>{}` | `theme.x{}` |
| `instance x: 0.0` | `x: instance(0.0)` |
| `uniform x: #f` | `x: uniform(#f)` |
| `draw_bg: {}` (override) | `draw_bg +: {}` (merge) |
| `fn pixel(self) -> vec4` | `pixel: fn()` |
| `Sdf2d::viewport()` | `Sdf2d.viewport()` |
| `default: off` | `default: @off` |
| `off = {}` (animator state) | `off: AnimatorState{}` |
| `dep("crate://self/x")` | `crate_resource("self://x")` |
| `#2ecc71` | `#x2ecc71` |
| `cursor: Hand` | `cursor: MouseCursor.Hand` |
| `Margin{...}` | `Inset{...}` |
| `DrawIcon` | `DrawSvg` |
| `link widgets;` | `use mod.prelude.widgets.*` |
| `use link::theme::*;` | (included in prelude) |
| `, ` between props | whitespace/newline |
