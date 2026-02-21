# Splash Language Reference

> Makepad's new UI scripting language — replaces the old `live_design!` DSL.

Splash is a full scripting language (not just a layout DSL). It can define UI layouts, shaders, animations, and also execute logic: HTTP requests, JSON parsing, promises, state management, and more. Think of it as Makepad's own "Lua" — embedded in Rust via the `script_mod!{}` macro.

---

## Table of Contents

1. [Embedding Splash in Rust](#1-embedding-splash-in-rust)
2. [Script Structure](#2-script-structure)
3. [Syntax Fundamentals](#3-syntax-fundamentals)
4. [Widget Definitions and Templates](#4-widget-definitions-and-templates)
5. [Property System](#5-property-system)
6. [Named Children (`:=` Operator)](#6-named-children--operator)
7. [Inheritance and Merging (`+:`)](#7-inheritance-and-merging-)
8. [Layout System](#8-layout-system)
9. [View Widgets (Containers)](#9-view-widgets-containers)
10. [Text Widgets](#10-text-widgets)
11. [Button Widgets](#11-button-widgets)
12. [Input Widgets](#12-input-widgets)
13. [List Widgets](#13-list-widgets)
14. [Navigation Widgets](#14-navigation-widgets)
15. [Media Widgets](#15-media-widgets)
16. [Shader System](#16-shader-system)
17. [Animator System](#17-animator-system)
18. [Scripting Capabilities](#18-scripting-capabilities)
19. [Inline Event Handlers](#19-inline-event-handlers)
20. [Vector Graphics](#20-vector-graphics)
21. [Widget Registration (Rust Side)](#21-widget-registration-rust-side)
22. [App Architecture](#22-app-architecture)
23. [Theme System](#23-theme-system)
24. [Draw Batching](#24-draw-batching)

---

## 1. Embedding Splash in Rust

Splash code lives inside `.rs` files within `script_mod!{}` macro blocks. There are **no** `.splx` files — everything is inline in Rust.

```rust
use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    // Splash code goes here — UI, templates, logic, shaders
    View{
        flow: Down height: Fit
        Label{text: "Hello from Splash"}
    }
}
```

### Key differences from old system

| Old (`live_design!`) | New (`script_mod!`) |
|---|---|
| `live_design! { ... }` | `script_mod! { ... }` |
| `link widgets; use link::theme::*;` | `use mod.prelude.widgets.*` |
| Commas between properties | No commas — whitespace-delimited |
| `<ParentWidget> { ... }` for inheritance | `mod.widgets.Parent{ ... }` or `let` bindings |
| `{{RustStruct}}` for base registration | `#(RustStruct::register_widget(vm))` |
| `(THEME_CONSTANT)` for theme values | `theme.constant_name` |
| `fn pixel(self) -> vec4 { ... }` | `pixel: fn() { ... }` |

---

## 2. Script Structure

Every Splash script starts with `use` statements to bring widgets into scope:

```
use mod.prelude.widgets.*

// Now all widgets (View, Label, Button, etc.) are available
```

For internal widget definitions (inside makepad widgets crate itself):
```
use mod.prelude.widgets_internal.*
use mod.widgets.*
```

Additional imports for specific capabilities:
```
use mod.net              // HTTP networking
use mod.widgets.CodeView // Specific widget imports
```

---

## 3. Syntax Fundamentals

### Properties — No commas, whitespace-delimited

```
// Property assignment (space-separated, no commas)
width: Fill height: Fit flow: Down spacing: 10

// Nested object
padding: Inset{top: 5 bottom: 5 left: 10 right: 10}

// Dot-path shorthand
draw_bg.color: #f00
draw_text.text_style.font_size: 12

// The dot-path is equivalent to merge:
// draw_bg +: { color: #f00 }
```

### Colors

```
#f00              // RGB short
#ff0000           // RGB full
#ff0000ff         // RGBA
#0000             // transparent black
vec4(1.0 0.0 0.0 1.0)  // explicit RGBA
```

### Comments

```
// Single-line comment
```

### Let bindings

```
let MyThing = View{ height: Fit width: Fill }
MyThing{}  // instantiate
```

### Variables

```
let results = []          // array
let count = 0             // number
let name = "hello"        // string
let data = {key: "value"} // object/map
```

---

## 4. Widget Definitions and Templates

### `let` bindings for reusable templates

Use `let` to define reusable widget templates. Must be defined **before** use (no hoisting).

```
use mod.prelude.widgets.*

let MyCard = RoundedView{
    width: Fill height: Fit
    padding: 15 flow: Down spacing: 8
    draw_bg.color: #334
    draw_bg.border_radius: 8.0
    title := Label{text: "default" draw_text.color: #fff}
    body := Label{text: "" draw_text.color: #aaa}
}

// Use the template, overriding named children
View{
    flow: Down height: Fit spacing: 12
    MyCard{title.text: "First Card" body.text: "Content here"}
    MyCard{title.text: "Second Card" body.text: "More content"}
}
```

### Registering widget types into the module namespace

Inside widget library code (not app code), widgets are registered into `mod.widgets.*`:

```
// Register the Rust struct as a base widget type
mod.widgets.ButtonBase = #(Button::register_widget(vm))

// Create a styled default using set_type_default()
mod.widgets.ButtonFlat = set_type_default() do mod.widgets.ButtonBase{
    text: "Button"
    width: Fit height: Fit
    // ... full definition ...
}

// Derive from an existing widget
mod.widgets.ButtonFlatter = mod.widgets.ButtonFlat{
    draw_bg +: {
        color: theme.color_u_hidden
        // override just the colors
    }
}

// Chain of inheritance
mod.widgets.Button = mod.widgets.ButtonFlat{ ... }
mod.widgets.ButtonGradientX = mod.widgets.Button{ ... }
mod.widgets.ButtonGradientY = mod.widgets.ButtonGradientX{
    draw_bg.gradient_fill_horizontal: 1.0
}
```

---

## 5. Property System

### Sizing

```
width: Fill          // Fill available space (default)
width: Fit           // Shrink to content
width: 200           // Fixed 200px (bare number)
width: Fill{min: 100 max: 500}
width: Fit{max: Abs(300)}
```

### Layout direction

```
flow: Right          // default — horizontal
flow: Down           // vertical
flow: Overlay        // stacked
flow: Flow.Right{wrap: true}   // wrapping horizontal
flow: Flow.Down{wrap: true}    // wrapping vertical
```

### Spacing, Padding, Margin

```
spacing: 10
padding: 15                    // uniform
padding: Inset{top: 5 bottom: 5 left: 10 right: 10}
margin: 0.
```

### Alignment

```
align: Center         // Align{x: 0.5 y: 0.5}
align: HCenter        // Align{x: 0.5 y: 0.0}
align: VCenter        // Align{x: 0.0 y: 0.5}
align: TopLeft
align: Align{x: 1.0 y: 0.0}  // top-right
```

### Visibility and Display

```
visible: true
show_bg: true
cursor: MouseCursor.Hand
grab_key_focus: true
clip_x: true
clip_y: true
```

---

## 6. Named Children (`:=` Operator)

The `:=` operator declares a named/dynamic child that can be overridden per-instance. This is **critical** for templates.

```
let TodoItem = View{
    width: Fill height: Fit flow: Right spacing: 8
    check := CheckBox{text: ""}
    label := Label{text: "task" draw_text.color: #ddd}
    Filler{}
    tag := Label{text: "" draw_text.color: #888}
}

// Override named children using dot-path
TodoItem{label.text: "Buy groceries" tag.text: "errands"}
```

### Rules

- Use `:=` for any child you want to reference or override later
- Using `:` (colon) instead of `:=` makes it a static property — not addressable
- Named children inside anonymous containers are **unreachable** — every container in the path must have a `:=` name:

```
let Item = View{
    texts := View{                    // named with :=
        flow: Down
        label := Label{text: "default"}
    }
}
Item{texts.label.text: "new text"}    // full path through named containers
```

---

## 7. Inheritance and Merging (`+:`)

### Merge operator (`+:`)

The `+:` operator extends/merges a parent definition without replacing it entirely:

```
draw_bg +: {
    color: instance(#334)
    border_radius: uniform(5.0)
    pixel: fn() { ... }
}
```

Without `+:`, using `:` would **replace** the entire `draw_bg` object. With `+:`, only the specified properties are overridden/added.

### Dot-path shorthand

Dot-path is equivalent to a merge on a single property:

```
draw_bg.color: #f00
// equivalent to:
draw_bg +: { color: #f00 }
```

### Multi-level inheritance

```
mod.widgets.ButtonFlat = set_type_default() do mod.widgets.ButtonBase{
    // Full definition
}

mod.widgets.Button = mod.widgets.ButtonFlat{
    draw_bg +: { border_color: theme.color_bevel_outset_1 }
}

mod.widgets.ButtonGradientX = mod.widgets.Button{
    draw_bg +: { color: theme.color_outset_1 color_2: theme.color_outset_2 }
}
```

### Text style inheritance

```
draw_text +: {
    text_style: theme.font_regular{ font_size: 12 }
    // Inherits from theme.font_regular, overrides font_size
}

// Or with +: merge syntax
draw_text +: {
    text_style +: { font_size: 16 }
}
```

---

## 8. Layout System

### Flow (direction children are laid out)

```
flow: Right          // default — left-to-right (no wrap)
flow: Down           // top-to-bottom
flow: Overlay        // stacked on top of each other
flow: Flow.Right{wrap: true}  // wrapping horizontal
```

### Critical: Always set `height: Fit` on containers

The default is `height: Fill`. Inside a `Fit` parent, `Fill` creates a circular dependency and results in **0 height** (invisible UI).

```
// CORRECT:
View{ height: Fit flow: Down padding: 10
    Label{text: "Visible!"}
}

// WRONG — invisible:
View{ flow: Down padding: 10
    Label{text: "Invisible — default height: Fill inside Fit parent"}
}
```

### Filler (spacer)

```
View{ flow: Right
    Label{text: "left"}
    Filler{}
    Label{text: "right"}
}
```

**Caveat:** Don't use `Filler{}` next to a `width: Fill` sibling — they compete for space and split 50/50.

---

## 9. View Widgets (Containers)

All inherit from `ViewBase`. Default: no background.

| Widget | Background | Shape |
|--------|-----------|-------|
| `View` | none (invisible container) | — |
| `SolidView` | flat color | rectangle |
| `RoundedView` | color | rounded rect |
| `RectView` | color + border | rectangle |
| `RoundedShadowView` | color + shadow | rounded rect |
| `RectShadowView` | color + shadow | rectangle |
| `CircleView` | color | circle |
| `HexagonView` | color | hexagon |
| `GradientXView` | horizontal gradient | rectangle |
| `GradientYView` | vertical gradient | rectangle |
| `CachedView` | texture-cached | rectangle |

### Scrollable Views

```
ScrollXYView{}     // scroll both axes
ScrollXView{}      // horizontal scroll
ScrollYView{}      // vertical scroll
```

### Background properties

```
draw_bg +: {
    color: instance(#334)
    border_size: uniform(1.0)
    border_radius: uniform(5.0)    // single f32, NOT Inset
    border_color: instance(#888)
    shadow_color: instance(#0007)  // shadow views only
    shadow_radius: uniform(10.0)
    shadow_offset: uniform(vec2(0 0))
}
```

---

## 10. Text Widgets

### Label

```
Label{text: "Hello"}
Label{
    width: Fit height: Fit
    draw_text.color: #fff
    draw_text.text_style.font_size: 12
    text: "Styled"
}
```

**Label does NOT support `animator` or `cursor`.** To make hoverable text, wrap in a View.

### Headings

```
H1{text: "Title"}        // font_size_1
H2{text: "Subtitle"}     // font_size_2
H3{text: "Section"}
H4{text: "Subsection"}
```

### Label variants

| Widget | Description |
|--------|------------|
| `Label` | Default |
| `Labelbold` | Bold font |
| `LabelGradientX` | Horizontal text gradient |
| `LabelGradientY` | Vertical text gradient |
| `TextBox` / `P` | Paragraph (full-width) |
| `Pbold` | Bold paragraph |

### TextInput

```
TextInput{width: Fill height: Fit empty_text: "Placeholder"}
TextInputFlat{width: Fill height: Fit empty_text: "Type here"}
TextInput{is_password: true empty_text: "Password"}
TextInput{is_read_only: true}
TextInput{is_numeric_only: true}
```

### Markdown / Html

```
Markdown{
    width: Fill height: Fit selectable: true
    body: "# Title\n\nParagraph with **bold**"
}
Html{
    width: Fill height: Fit
    body: "<h3>Title</h3><p>Content</p>"
}
```

---

## 11. Button Widgets

```
Button{text: "Standard"}
ButtonFlat{text: "Flat"}          // no bevel border
ButtonFlatter{text: "Minimal"}    // invisible bg

// Customize colors
ButtonFlat{
    text: "Custom"
    draw_bg +: {
        color: uniform(#336)
        color_hover: uniform(#449)
        color_down: uniform(#225)
    }
    draw_text +: { color: #fff }
}
```

### Button variants

`Button`, `ButtonFlat`, `ButtonFlatter`, `ButtonIcon`, `ButtonGradientX`, `ButtonGradientY`, `ButtonFlatIcon`, `ButtonFlatterIcon`

---

## 12. Input Widgets

### CheckBox / Toggle

```
CheckBox{text: "Enable"}
CheckBoxFlat{text: "Flat style"}
Toggle{text: "Dark mode"}
ToggleFlat{text: "Flat toggle"}
```

### RadioButton

```
RadioButton{text: "Option A"}
RadioButtonFlat{text: "Option A"}
```

### Slider

```
Slider{width: Fill text: "Volume" min: 0.0 max: 100.0 default: 50.0}
SliderMinimal{text: "Value" min: 0.0 max: 1.0 step: 0.01 precision: 2}
```

### DropDown

```
DropDown{labels: ["Option A" "Option B" "Option C"]}
DropDownFlat{labels: ["Small" "Medium" "Large"]}
```

---

## 13. List Widgets

### PortalList (virtualized)

```
list := PortalList{
    width: Fill height: Fill
    flow: Down
    scroll_bar: ScrollBar{}
    drag_scrolling: false
    auto_tail: true

    Item := View{
        width: Fill height: Fit
        title := Label{text: ""}
    }
    Header := View{height: Fit ...}
}
```

Templates defined with `:=` are instantiated by host Rust code at draw time via `list.item(cx, item_id, id!(Item))`.

### FlatList (non-virtualized)

```
FlatList{
    width: Fill height: Fill flow: Down
    Item := View{height: Fit ...}
}
```

---

## 14. Navigation Widgets

### Modal

```
my_modal := Modal{
    content +: {
        width: 300 height: Fit
        RoundedView{height: Fit padding: 20 flow: Down spacing: 10
            draw_bg.color: #333
            Label{text: "Dialog Title"}
            close := ButtonFlat{text: "Close"}
        }
    }
}
```

### PageFlip

```
PageFlip{
    active_page := page1
    page1 := View{height: Fit ...}
    page2 := View{height: Fit ...}
}
```

### SlidePanel

```
panel := SlidePanel{
    side: SlideSide.Left
    width: 200 height: Fill
    // child content
}
```

### Dock (tabbed panel layout)

```
Dock{
    width: Fill height: Fill

    // Tab header templates
    tab_bar +: {
        FilesTab := IconTab{
            draw_icon +: {
                color: #80FFBF
                svg: crate_resource("self://resources/icons/icon_file.svg")
            }
        }
    }

    // Layout tree
    root := DockSplitter{
        axis: SplitterAxis.Horizontal
        align: SplitterAlign.FromA(250.0)
        a := left_tabs
        b := right_tabs
    }

    left_tabs := DockTabs{
        tabs: [@files_tab]
        selected: 0
    }

    // Tab definitions
    files_tab := DockTab{
        name: "Files"
        template := FilesTab
        kind := FileTreeContent
    }

    // Content templates
    FileTreeContent := View{
        flow: Down width: Fill height: Fill
        Label{text: "File tree here"}
    }
}
```

---

## 15. Media Widgets

### Image

```
Image{width: 200 height: 150 fit: ImageFit.Stretch}
// ImageFit: Stretch | Horizontal | Vertical | Smallest | Biggest | Size
```

### Icon

```
Icon{
    draw_icon.svg: crate_resource("self://resources/icons/my_icon.svg")
    draw_icon.color: #0ff
    icon_walk: Walk{width: 32 height: 32}
}
```

### LoadingSpinner

```
LoadingSpinner{width: 40 height: 40}
```

---

## 16. Shader System

Shaders are written inline inside `draw_bg +: { ... }` blocks.

### Instance vs Uniform vs Varying

```
draw_bg +: {
    hover: instance(0.0)       // per-widget, animatable by Animator
    color: uniform(#fff)       // shared across all instances
    tex: texture_2d(float)     // texture sampler
    my_var: varying(vec2(0))   // vertex->pixel interpolated
}
```

- **`instance()`** — state that varies per widget (hover, down, focus, colors). Driven by the Animator.
- **`uniform()`** — constants shared by all instances (border sizes, theme colors). Cannot be animated.
- **`varying()`** — data computed in vertex shader and interpolated for pixel shader.

### Pixel Shader

```
draw_bg +: {
    pixel: fn() {
        let sdf = Sdf2d.viewport(self.pos * self.rect_size)
        sdf.box(0. 0. self.rect_size.x self.rect_size.y 4.0)
        sdf.fill(#f00)
        return sdf.result
    }
}
```

**Critical: Premultiply alpha!** When returning a color directly (not via `sdf.result`):
```
pixel: fn() {
    return Pal.premul(self.color.mix(self.color_hover, self.hover))
}
```

`sdf.fill()` / `sdf.stroke()` already premultiply internally.

### Custom shader functions

```
draw_bg +: {
    get_color: fn() {
        return self.color
            .mix(self.color_hover, self.hover)
            .mix(self.color_down, self.down)
    }
    pixel: fn() {
        return Pal.premul(self.get_color())
    }
}
```

### SDF Primitives

```
sdf.circle(cx cy radius)
sdf.rect(x y w h)
sdf.box(x y w h border_radius)
sdf.box_all(x y w h r_lt r_rt r_rb r_lb)   // per-corner radius
sdf.hexagon(cx cy radius)
sdf.hline(y half_height)
sdf.arc_round_caps(cx cy radius start end thickness)
```

### SDF Operations

```
sdf.fill(color)           // fill and reset
sdf.fill_keep(color)      // fill, keep shape for stroke
sdf.stroke(color width)   // stroke outline
sdf.glow(color width)     // additive glow

sdf.union()               // merge shapes
sdf.intersect()           // keep overlap
sdf.subtract()            // cut shapes
sdf.blend(k)              // linear blend (0=prev, 1=current)
```

### SDF Transforms

```
sdf.translate(x y)
sdf.rotate(angle cx cy)
sdf.scale(factor cx cy)
```

### Built-in Variables

```
self.pos              // vec2: normalized position [0,1]
self.rect_size        // vec2: pixel size
self.rect_pos         // vec2: pixel position
self.dpi_factor       // float: display DPI
self.draw_pass.time   // float: elapsed seconds (for animation)
```

### Color Operations

```
mix(color1 color2 factor)
color1.mix(color2 factor)
Pal.premul(color)                      // premultiply alpha
Pal.hsv2rgb(vec4(h s v 1.0))
Pal.rgb2hsv(color)
Pal.iq(t a b c d)                      // Inigo Quilez cosine palette
```

### Math Utilities

```
Math.random_2d(vec2)      // pseudo-random 0-1
Math.rotate_2d(v angle)   // 2D rotation
sin(x) cos(x) tan(x) abs(x) floor(x) ceil(x) fract(x)
pow(x y) sqrt(x) clamp(x min max) smoothstep(e0 e1 x)
min(x y) max(x y) mod(x y) length(v) normalize(v) dot(a b)
```

### Vertex Shader (rare, usually default is fine)

```
draw_bg +: {
    my_scale: varying(vec2(0))
    vertex: fn() {
        let dpi = self.dpi_factor
        let ceil_size = ceil(self.rect_size * dpi) / dpi
        self.my_scale = self.rect_size / ceil_size
        return self.clip_and_transform_vertex(self.rect_pos self.rect_size)
    }
}
```

---

## 17. Animator System

The animator drives `instance()` variables over time. Only certain widgets support it (View, Button, CheckBox, Toggle, RadioButton, LinkLabel, TextInput — **NOT** Label, Image, Icon, Slider, DropDown).

### Structure

```
animator: Animator{
    hover: {
        default: @off
        off: AnimatorState{
            from: {all: Forward {duration: 0.15}}
            apply: {draw_bg: {hover: 0.0}}
        }
        on: AnimatorState{
            from: {
                all: Forward {duration: 0.1}
                down: Forward {duration: 0.01}
            }
            apply: {draw_bg: {hover: snap(1.0)}}
        }
        down: AnimatorState{
            from: {all: Forward {duration: 0.2}}
            apply: {draw_bg: {down: snap(1.0), hover: 1.0}}
        }
    }
    focus: { ... }
    disabled: { ... }
}
```

### `snap()` — instant jump (no interpolation)

```
apply: {draw_bg: {down: snap(1.0), hover: 1.0}}
```

### `timeline()` — keyframes

```
apply: {draw_bg: {anim_time: timeline(0.0 0.0  1.0 1.0)}}
```

### Play types

```
Forward {duration: 0.2}              // play once
Snap                                  // instant
Loop {duration: 1.0, end: 1e9}      // repeat
BounceLoop {duration: 1.0, end: 1.0} // bounce
```

### Ease functions

`Linear`, `InQuad`, `OutQuad`, `InOutQuad`, `InCubic`, `OutCubic`, `InOutCubic`, `InElastic`, `OutElastic`, `InBounce`, `OutBounce`, etc.

---

## 18. Scripting Capabilities

Splash is a full scripting language, not just a layout DSL.

### Variables and Data

```
let results = []
let count = 0
let data = {title: "hello" tags: ["a" "b"]}
```

### Functions

```
fn add_todo(text, tag) {
    todos.push({text: text, tag: tag, done: false})
    ui.todo_list.render()
}

fn tag_color(tag) {
    if tag == "dev" "#4466ee"
    else if tag == "urgent" "#ee4455"
    else "#7a7acc"
}
```

### Control Flow

```
if condition { ... }
else if other { ... }
else { ... }

for item in collection { ... }
for i, item in collection { ... }   // with index

// Retain (filter in place)
todos.retain(|todo| !todo.done)
```

### HTTP Requests and Promises

```
use mod.net

fn fetch(url, extra_headers) {
    let p = promise()
    let h = {"User-Agent": "Mozilla/5.0 ..."}
    if extra_headers != nil {
        for k, v in extra_headers { h[k] = v }
    }
    net.http_request(net.HttpRequest{
        url: url
        method: net.HttpMethod.GET
        headers: h
    }) do net.HttpEvents{
        on_response: |res| p.resolve(res)
        on_error: |err| p.resolve(nil)
    }
    p
}

fn do_search(query) {
    let q = query.url_encode()
    let page = fetch("https://example.com/?q=" + q, nil).await()
    if page == nil { return }
    let data = page.body.to_string().parse_json()
    // ... process data ...
}
```

### String Operations

```
query.url_encode()
response.body.to_string()
string.split("delimiter")
string.parse_json()
```

### Array Operations

```
results.clear()
results.push(item)
results.remove(index)
results.retain(|item| condition)
results.len()
```

### UI Interaction from Script

```
ui.search_input.text()          // get text
ui.search_input.set_text("")    // set text
ui.todo_list.render()           // re-render a view
ui.search_button.on_click()     // programmatically trigger click
```

---

## 19. Inline Event Handlers

Splash supports inline event handlers directly on widgets — no need for Rust action handling for simple cases.

```
search_button := Button{
    text: "Search"
    on_click: || do_search(ui.search_input.text())
}

todo_input := TextInput{
    width: Fill height: Fit
    empty_text: "What needs to be done?"
    on_return: || ui.add_button.on_click()
}

check := CheckBox{
    text: ""
    on_click: |checked| toggle_todo(i, checked)
}
```

### on_render — Dynamic UI rendering

The `on_render` callback allows procedural/dynamic UI:

```
results_view := ScrollYView{
    width: Fill height: Fill
    on_render: || {
        if results.len() == 0 {
            View{
                width: Fill height: 200 align: Center
                Label{text: "No results"}
            }
        }
        else for result in results {
            ImageCard{
                thumb.src: http_resource(result.thumbnail)
                title.text: result.title
            }
        }
    }
}
```

### on_startup

```
Root{
    on_startup: || {
        ui.todo_list.render()
    }
    // ...
}
```

---

## 20. Vector Graphics

Splash supports inline SVG-like vector graphics:

```
let IconCheck = Vector{
    width: 18 height: 18 viewbox: vec4(0 0 24 24)
    Path{
        d: "M20 6L9 17L4 12"
        fill: false
        stroke: #x6c6cff
        stroke_width: 2.5
        stroke_linecap: "round"
        stroke_linejoin: "round"
    }
}

let IconTrash = Vector{
    width: 14 height: 14 viewbox: vec4(0 0 24 24)
    Path{
        d: "M3 6h18M8 6V4a2 2 0 0 1 2-2h4..."
        fill: false stroke: #x555 stroke_width: 1.8
        stroke_linecap: "round" stroke_linejoin: "round"
    }
}
```

Supports `Path`, `Rect`, `Circle`, `Gradient`, `Filter` for full SVG-like vector graphics.

---

## 21. Widget Registration (Rust Side)

### Rust struct for a widget

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
    draw_text: DrawText,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[live(true)]
    #[visible]
    visible: bool,

    #[live]
    pub text: ArcStringMut,

    #[live]
    on_click: ScriptFnRef,     // Inline event handler

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}
```

Key derive macros:
- **`Script`** — enables Splash scripting integration (replaces `Live`)
- **`ScriptHook`** — enables lifecycle hooks (replaces `LiveHook`)
- **`Widget`** — the Widget trait
- **`Animator`** — animator support

Key attributes:
- `#[uid]` — unique widget ID
- `#[source]` — script object reference
- `#[apply_default]` — apply default from script
- `#[live]` — live-reloadable field
- `#[walk]` — walk/sizing field
- `#[layout]` — layout field
- `#[redraw]` — marks field that triggers redraw
- `#[visible]` — visibility field
- `#[action_data]` — action data storage
- `#[rust]` — rust-only field (not exposed to DSL)

### Registration in Splash

```
mod.widgets.ButtonBase = #(Button::register_widget(vm))
```

The `#(...)` syntax calls Rust code from within Splash, registering the widget struct.

### Custom widget with full Splash definition

```rust
#[derive(Script, ScriptHook, Widget)]
pub struct ChatList {
    #[deref]
    view: View,
    #[rust]
    animating_msg: Option<usize>,
}
```

In Splash:
```
let ChatList = #(ChatList::register_widget(vm)) {
    width: Fill height: Fill
    list := PortalList{
        width: Fill height: Fill flow: Down
        Item := View{ ... }
    }
}
```

---

## 22. App Architecture

### Minimal app boilerplate

```rust
use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    // All UI, templates, logic, state here

    let app = startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(700, 750)
                body +: {
                    flow: Down
                    Label{text: "Hello World"}
                }
            }
        }
    }
    app
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        crate::makepad_widgets::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl MatchEvent for App {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

### Rust action handling (unchanged pattern)

```rust
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(send_button)).clicked(actions) {
            self.send_message(cx);
        }
        if self.ui.text_input(cx, ids!(input)).returned(actions).is_some() {
            self.send_message(cx);
        }
        if let Some(index) = self.ui.drop_down(cx, ids!(dropdown)).selected(actions) {
            // handle selection
        }
    }
}
```

### Using Scope-based data (unchanged)

Widget queries use the same `self.ui.button(cx, ids!(name))`, `self.ui.label(cx, ids!(name))` pattern.

---

## 23. Theme System

Theme values are accessed with dot notation on `theme.*`:

### Colors
```
theme.color_bg_app
theme.color_text
theme.color_text_hl
theme.color_label_inner
theme.color_label_inner_hover
theme.color_outset
theme.color_outset_hover
theme.color_outset_down
theme.color_bevel
theme.color_shadow
theme.color_u_hidden        // transparent
theme.color_d_hidden
theme.color_error
theme.color_warning
```

### Spacing
```
theme.space_1  theme.space_2  theme.space_3
theme.mspace_1  theme.mspace_2  theme.mspace_3   // uniform insets
theme.mspace_h_1  theme.mspace_h_2               // horizontal insets
theme.mspace_v_1  theme.mspace_v_2               // vertical insets
```

### Typography
```
theme.font_regular
theme.font_bold
theme.font_italic
theme.font_bold_italic
theme.font_code
theme.font_icons
theme.font_size_1  theme.font_size_2  theme.font_size_3  theme.font_size_4
theme.font_size_p
theme.font_size_base
```

### Dimensions
```
theme.corner_radius
theme.beveling
theme.tab_height
theme.splitter_size
```

---

## 24. Draw Batching

Widgets using the same shader are batched into a single GPU draw call. This can cause text to render **behind** backgrounds.

**Set `new_batch: true` on any View with `show_bg: true` that contains text:**

```
RoundedView{
    flow: Down height: Fit
    new_batch: true           // ensures bg draws before text
    show_bg: true
    draw_bg.color: #334
    Label{text: "Text on top of background"}
}
```

**Critical for hover effects:** Without `new_batch: true`, hover background becoming opaque covers text.

```
let HoverItem = View{
    width: Fill height: Fit
    new_batch: true           // MUST have this
    show_bg: true
    draw_bg +: {
        color: uniform(#0000)
        color_hover: uniform(#fff2)
        hover: instance(0.0)
        pixel: fn(){
            return Pal.premul(self.color.mix(self.color_hover, self.hover))
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
    }
    label := Label{text: "hoverable item" draw_text.color: #fff}
}
```
