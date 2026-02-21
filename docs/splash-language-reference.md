# Splash Language Reference

> Splash is Makepad's new scripting language and DSL, replacing the old `live_design!` macro system. It defines UI layout, styling, shaders, animations, and can also perform HTTP requests, file I/O, and more. Think of it as Makepad's own "Lua" - but one that can also express GPU shaders.

## Table of Contents

- [Overview](#overview)
- [Script Structure](#script-structure)
- [Syntax Fundamentals](#syntax-fundamentals)
- [Properties and Values](#properties-and-values)
- [Widget System](#widget-system)
- [Layout System](#layout-system)
- [Styling and Drawing](#styling-and-drawing)
- [Shader System](#shader-system)
- [Animator System](#animator-system)
- [Theme System](#theme-system)
- [Templates and Composition](#templates-and-composition)
- [Scripting Beyond UI](#scripting-beyond-ui)
- [Vector Graphics](#vector-graphics)
- [Rust Integration](#rust-integration)
- [Available Widgets Reference](#available-widgets-reference)

---

## Overview

Splash replaces the old `live_design!` compile-time macro with a runtime-evaluated scripting system. The entry point in Rust is the `script_mod!{}` macro, which embeds Splash code that gets evaluated at runtime by the `ScriptVm`.

**Key characteristics:**
- No commas between sibling properties (whitespace-delimited)
- No semicolons
- `+:` operator for merge-override (keeps parent properties, only overrides what you specify)
- `:=` operator for naming children (makes them addressable/overridable)
- `let` bindings for reusable templates (local to their scope)
- `instance()` / `uniform()` for shader variable qualifiers
- `fn()` syntax for inline shader functions
- Full scripting capabilities: variables, functions, loops, HTTP, file I/O

---

## Script Structure

### In Rust files (`script_mod!`)

Every Splash script embedded in Rust starts with a `use` statement:

```rust
script_mod!{
    use mod.prelude.widgets.*

    // Definitions and UI here
    load_all_resources() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(800, 600)
                body +: {
                    // UI content
                }
            }
        }
    }
}
```

### Prelude imports

Two prelude modules exist:
- `mod.prelude.widgets.*` - For **app development** (brings all widgets into scope)
- `mod.prelude.widgets_internal.*` - For **widget library development** (lower-level)

### Standalone Splash scripts

When Splash is evaluated standalone (e.g., in the `Splash` widget), scripts start with:
```
use mod.prelude.widgets.*

View{
    flow: Down height: Fit
    Label{text: "Hello from Splash!"}
}
```

---

## Syntax Fundamentals

### Property assignment
```
key: value
```
No commas between sibling properties. Whitespace or newlines separate them:
```
width: Fill height: Fit flow: Down spacing: 10
```

### Nested objects
```
key: Type{ prop1: val1 prop2: val2 }
```

### Merge operator (`+:`)
Extends the parent value instead of replacing it entirely:
```
draw_bg +: {
    color: #f00    // Only overrides color, keeps all other draw_bg properties
}
```
Without `+:`, you **replace** the entire `draw_bg`:
```
draw_bg: {
    // This REPLACES all draw_bg properties - dangerous!
    color: #f00
}
```

### Dot-path shorthand
```
draw_bg.color: #f00
// Equivalent to: draw_bg +: { color: #f00 }

draw_text.text_style.font_size: 14
// Equivalent to: draw_text +: { text_style +: { font_size: 14 } }
```

### Named children (`:=`)
```
my_button := Button{ text: "Click" }   // Addressable child
Label{ text: "anonymous" }              // Anonymous child (not addressable)
```

### Let bindings (templates)
```
let MyCard = RoundedView{
    width: Fill height: Fit
    padding: 16 flow: Down spacing: 8
    draw_bg.color: #334
    draw_bg.border_radius: 8.0
    title := Label{text: "Default Title" draw_text.color: #fff}
}

// Instantiate and override
MyCard{title.text: "Custom Title"}
```

**Rules:**
- `let` bindings must be defined **before** they are used
- They are **local** to their scope (not accessible from other `script_mod!` blocks)
- Use `mod.widgets.MyName = ...` to share across modules

### Comments
```
// Single line comment
```

### Colors
```
#f00              // RGB short
#ff0000           // RGB full
#ff0000ff         // RGBA
#0000             // Transparent black
vec4(1.0 0.0 0.0 1.0)  // Explicit RGBA

// IMPORTANT: In script_mod!, hex colors containing 'e' need #x prefix
#x2ecc71          // Because #2ecc71 would confuse Rust tokenizer (2e = scientific notation)
#x1e1e2e          // Same issue
#ff4444           // Fine - no 'e' adjacent to digits
```

### Strings
Double quotes only:
```
text: "Hello World"
```

### Numbers
```
width: 200        // Integer (becomes Fixed(200))
opacity: 0.5      // Float
border_radius: 8.0  // Float (the .0 is required for some properties)
```

### Vectors
```
vec2(100 200)                    // No commas needed when all values positive
vec4(-1.0, -1.0, -1.0, -1.0)   // USE COMMAS when values are negative
```

---

## Properties and Values

### Sizing (`Size` enum)
```
width: Fill              // Fill available space (default for most widgets)
width: Fit               // Shrink to content
width: 200               // Fixed 200px
width: Fill{min: 100 max: 500}
width: Fit{max: Abs(300)}
height: Fill height: Fit height: 100
```

### Flow (child layout direction)
```
flow: Right              // Left-to-right (default)
flow: Down               // Top-to-bottom
flow: Overlay            // Stacked on top of each other
flow: Flow.Right{wrap: true}   // Wrapping horizontal
flow: Flow.Down{wrap: true}    // Wrapping vertical
```

### Spacing, Padding, Margin
```
spacing: 10                                       // Gap between children
padding: 15                                       // Uniform
padding: Inset{top: 5 bottom: 5 left: 10 right: 10}  // Per-side
margin: Inset{top: 2 bottom: 2 left: 5 right: 5}
margin: 0.                                        // Uniform zero
```

### Alignment
```
align: Center            // Align{x: 0.5 y: 0.5}
align: HCenter           // Align{x: 0.5 y: 0.0}
align: VCenter           // Align{x: 0.0 y: 0.5}
align: TopLeft           // Align{x: 0.0 y: 0.0}
align: Align{x: 1.0 y: 0.0}   // Custom
```

### Booleans
```
show_bg: true
visible: false
clip_x: true
grab_key_focus: true
```

### Enums
```
cursor: MouseCursor.Hand
axis: SplitterAxis.Horizontal
fit: ImageFit.Stretch
side: SlideSide.Left
```

### Resources
```
draw_icon.svg: crate_resource("self://resources/icons/my_icon.svg")
```

---

## Widget System

### Container widgets (Views)

All inherit from `ViewBase`. Use styled variants for backgrounds:

| Widget | Background | Shape |
|--------|-----------|-------|
| `View` | None (transparent) | - |
| `SolidView` | Flat color | Rectangle |
| `RoundedView` | Color | Rounded rectangle |
| `RoundedShadowView` | Color + shadow | Rounded rectangle |
| `RectView` | Color | Rectangle with border |
| `RectShadowView` | Color + shadow | Rectangle |
| `CircleView` | Color | Circle |
| `HexagonView` | Color | Hexagon |
| `GradientXView` | Horizontal gradient | Rectangle |
| `GradientYView` | Vertical gradient | Rectangle |
| `CachedView` | Texture-cached | Rectangle |

**Scrollable variants:**
```
ScrollXYView{}    // Both axes
ScrollXView{}     // Horizontal only
ScrollYView{}     // Vertical only
```

### Text widgets
```
Label{text: "Hello" draw_text.color: #fff draw_text.text_style.font_size: 12}
H1{text: "Title"}
H2{text: "Subtitle"}
H3{text: "Section"}
H4{text: "Subsection"}
P{text: "Paragraph"}
Pbold{text: "Bold paragraph"}
TextBox{text: "Full-width text"}
```

### Input widgets
```
TextInput{width: Fill height: Fit empty_text: "Placeholder"}
TextInput{is_password: true empty_text: "Password"}
TextInput{is_read_only: true}
TextInput{is_numeric_only: true}
TextInputFlat{width: Fill height: Fit empty_text: "Flat style"}
```

### Button widgets
```
Button{text: "Standard"}
ButtonFlat{text: "Flat style"}
ButtonFlatter{text: "Minimal"}

// Customized
ButtonFlat{
    text: "Custom"
    draw_bg +: { color: uniform(#336) color_hover: uniform(#449) }
    draw_text +: { color: #fff }
}
```

### Toggle widgets
```
CheckBox{text: "Enable"}
CheckBoxFlat{text: "Flat"}
Toggle{text: "Dark mode"}
ToggleFlat{text: "Flat toggle"}
RadioButton{text: "Option A"}
RadioButtonFlat{text: "Option A"}
```

### Other input widgets
```
Slider{width: Fill text: "Volume" min: 0.0 max: 100.0 default: 50.0}
SliderMinimal{text: "Value" min: 0.0 max: 1.0 step: 0.01}
DropDown{labels: ["Option A" "Option B" "Option C"]}
```

### Media widgets
```
Image{width: 200 height: 150 fit: ImageFit.Stretch}
Icon{
    draw_icon.svg: crate_resource("self://resources/icons/icon.svg")
    draw_icon.color: #0ff
    icon_walk: Walk{width: 32 height: 32}
}
LoadingSpinner{width: 40 height: 40}
Markdown{width: Fill height: Fit body: "# Title\n\nParagraph with **bold**"}
Html{width: Fill height: Fit body: "<h3>Title</h3><p>Content</p>"}
MathView{text: "x = \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}" font_size: 14.0}
```

### Layout widgets
```
Hr{}             // Horizontal rule
Vr{}             // Vertical rule
Filler{}         // Pushes siblings apart (width: Fill height: Fill)
Splitter{
    axis: SplitterAxis.Horizontal
    align: SplitterAlign.FromA(250.0)
    a := left_panel
    b := right_panel
}
```

### Navigation widgets
```
// Modal
my_modal := Modal{
    content +: {
        width: 300 height: Fit
        RoundedView{height: Fit padding: 20 flow: Down draw_bg.color: #333}
    }
}

// PageFlip
PageFlip{
    active_page := page1
    page1 := View{height: Fit ...}
    page2 := View{height: Fit ...}
}

// StackNavigation
StackNavigation{
    root_view := View{height: Fit ...}
}

// SlidePanel
panel := SlidePanel{side: SlideSide.Left width: 200 height: Fill}
```

### List widgets
```
// PortalList (virtualized, for large lists)
list := PortalList{
    width: Fill height: Fill
    flow: Down
    scroll_bar: ScrollBar{}
    Item := View{
        width: Fill height: Fit
        title := Label{text: ""}
    }
}

// FlatList (non-virtualized)
FlatList{width: Fill height: Fill flow: Down
    Item := View{height: Fit ...}
}
```

### Dock system
```
Dock{
    width: Fill height: Fill

    // Tab header templates
    tab_bar +: {
        MyTab := IconTab{
            draw_icon +: { color: #80FFBF svg: crate_resource("self://icon.svg") }
        }
    }

    // Layout tree
    root := DockSplitter{
        axis: SplitterAxis.Horizontal
        align: SplitterAlign.FromA(250.0)
        a := left_tabs  b := right_tabs
    }

    left_tabs := DockTabs{ tabs: [@my_tab] selected: 0 }

    my_tab := DockTab{ name: "Files" template := MyTab kind := MyContent }

    // Content templates
    MyContent := View{ width: Fill height: Fill Label{text: "Content"} }
}
```

---

## Layout System

### Critical rules

1. **Always set `height: Fit` on containers** when they are inside a `Fit` parent. Default is `Fill`, which creates a circular dependency and results in 0 height (invisible).

2. **Use `width: Fill` on root containers** to fill available space. Fixed pixel widths on root break responsive layout.

3. **`new_batch: true`** is required on any View with `show_bg: true` that contains text children. Without it, text renders behind backgrounds due to GPU draw call batching.

4. **`border_radius` takes a single float**, not an Inset: `draw_bg.border_radius: 16.0`

### Filler usage

Only use `Filler{}` between `width: Fit` siblings:
```
// Correct: Filler between Fit siblings
View{flow: Right
    Label{text: "left"}
    Filler{}
    Label{text: "right"}
}

// Don't use Filler next to width: Fill - they fight for space
```

---

## Styling and Drawing

### draw_bg properties (for styled Views)
```
draw_bg +: {
    color: instance(#334)
    color_2: instance(vec4(-1))       // Gradient end (-1 = disabled)
    gradient_fill_horizontal: uniform(0.0)
    border_size: uniform(1.0)
    border_radius: uniform(5.0)
    border_color: instance(#888)
    // Shadow views add:
    shadow_color: instance(#0007)
    shadow_radius: uniform(10.0)
    shadow_offset: uniform(vec2(0 0))
}
```

### draw_text properties
```
draw_text +: {
    color: #fff
    color_2: uniform(vec4(-1))
    text_style: theme.font_regular{font_size: 11}
}
```

Available fonts: `theme.font_regular`, `theme.font_bold`, `theme.font_italic`, `theme.font_bold_italic`, `theme.font_code`, `theme.font_icons`

---

## Shader System

### Instance vs Uniform vs Varying

```
draw_bg +: {
    hover: instance(0.0)          // Per-draw-call, animatable by Animator
    color: uniform(#fff)          // Shared across all instances of this shader
    tex: texture_2d(float)        // Texture sampler
    my_var: varying(vec2(0))      // Vertex->pixel interpolated
}
```

- **`instance()`**: State that varies per widget (hover, down, focus, active). Driven by Animator.
- **`uniform()`**: Theme constants shared by all instances. Cannot be animated.
- **`varying()`**: Vertex-to-pixel interpolated values.

### Pixel shader
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

### Premultiplication rule
When returning a hand-computed color (not via `sdf.result`), always premultiply:
```
pixel: fn() {
    return Pal.premul(self.color.mix(self.color_hover, self.hover))
}
```
`sdf.fill()` / `sdf.stroke()` already premultiply, so `return sdf.result` is safe.

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

### SDF primitives
```
sdf.circle(cx cy radius)
sdf.rect(x y w h)
sdf.box(x y w h border_radius)
sdf.box_all(x y w h r_lt r_rt r_rb r_lb)
sdf.hexagon(cx cy radius)
sdf.hline(y half_height)
sdf.arc_round_caps(cx cy radius start end thickness)
```

### SDF combinators
```
sdf.union()        // Merge shapes
sdf.intersect()    // Keep only overlap
sdf.subtract()     // Cut current from previous
sdf.gloop(k)       // Smooth/gooey union
sdf.blend(k)       // Linear blend
```

### SDF drawing
```
sdf.fill(color)          // Fill and reset shape
sdf.fill_keep(color)     // Fill, keep shape for stroke
sdf.stroke(color width)  // Stroke and reset
sdf.glow(color width)    // Additive glow
```

### Built-in shader variables
```
self.pos              // vec2: normalized position [0,1]
self.rect_size        // vec2: pixel size
self.rect_pos         // vec2: pixel position
self.dpi_factor       // float: DPI factor
self.draw_pass.time   // float: elapsed seconds (for animation)
self.geom_pos         // vec2: raw geometry position
```

### Color operations
```
mix(color1 color2 factor)
color1.mix(color2 factor)               // Method chaining
Pal.premul(color)                        // Premultiply alpha
Pal.hsv2rgb(vec4(h s v 1.0))           // HSV to RGB
Pal.iq(t a b c d)                       // Cosine color palette
Math.random_2d(vec2)                     // Pseudo-random
```

### Vertex shader (rare, most use default)
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

### Shader control flow
```
// Mutable variables
let mut color = self.color
if self.hover > 0.5 { color = self.color_hover }

// Match on enum instance variables
match self.block_type {
    Type.A => { ... }
    Type.B => { ... }
    _ => { ... }
}

// For loops
for i in 0..4 { ... }
```

### Math utilities
```
sin(x) cos(x) tan(x) abs(x) floor(x) ceil(x) fract(x)
min(x y) max(x y) clamp(x min max) step(edge x)
smoothstep(edge0 edge1 x)
length(v) distance(a b) dot(a b) normalize(v)
pow(x y) sqrt(x) exp(x) log(x)
PI  E  TORAD  GOLDEN
```

---

## Animator System

The Animator drives `instance()` variables over time for hover effects, transitions, and looping animations.

### Widget support
**Supports animator:** View, SolidView, RoundedView, Button, CheckBox, Toggle, RadioButton, LinkLabel, TextInput, ScrollXView, ScrollYView, ScrollXYView

**Does NOT support animator:** Label, H1-H4, P, Image, Icon, Markdown, Slider, DropDown, Splitter, Hr, Filler

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
            from: {all: Forward {duration: 0.15}}
            apply: {draw_bg: {hover: 1.0}}
        }
        down: AnimatorState{
            from: {all: Forward {duration: 0.2}}
            apply: {draw_bg: {down: snap(1.0) hover: 1.0}}
        }
    }
    focus: {
        default: @off
        off: AnimatorState{ from: {all: Snap} apply: {draw_bg: {focus: 0.0}} }
        on: AnimatorState{ from: {all: Snap} apply: {draw_bg: {focus: 1.0}} }
    }
}
```

### Transition types
```
Forward {duration: 0.2}           // Play once forward
Snap                              // Instant
Reverse {duration: 0.2 end: 1.0} // Play in reverse
Loop {duration: 1.0 end: 1000000000.0}  // Repeat
BounceLoop {duration: 1.0 end: 1.0}     // Bounce
```

### Ease functions
```
Linear  InQuad  OutQuad  InOutQuad
InCubic  OutCubic  InOutCubic
InSine  OutSine  InOutSine
InExp  OutExp  InOutExp
InElastic  OutElastic  InOutElastic
InBack  OutBack  InOutBack
InBounce  OutBounce  InOutBounce
ExpDecay{d1: 0.82 d2: 0.97 max: 100}
Bezier{cp0: 0.0 cp1: 0.0 cp2: 1.0 cp3: 1.0}
```

### Special values
```
snap(1.0)                         // Instant jump (no interpolation)
timeline(0.0 0.0  1.0 1.0)      // Keyframes (time value pairs)
```

---

## Theme System

### Colors (key ones)
```
theme.color_bg_app          theme.color_fg_app
theme.color_bg_container    theme.color_text
theme.color_text_hl         theme.color_text_disabled
theme.color_shadow          theme.color_highlight
theme.color_error           theme.color_warning
theme.color_makepad         // #FF5C39
theme.color_white           theme.color_black
theme.color_u_1 .. color_u_6    // Light scale
theme.color_d_1 .. color_d_5    // Dark scale
```

### Spacing
```
theme.space_1  theme.space_2  theme.space_3
theme.mspace_1  theme.mspace_2  theme.mspace_3      // Uniform insets
theme.mspace_h_1  theme.mspace_h_2  theme.mspace_h_3  // Horizontal insets
theme.mspace_v_1  theme.mspace_v_2  theme.mspace_v_3  // Vertical insets
```

### Typography
```
theme.font_regular  theme.font_bold  theme.font_italic
theme.font_bold_italic  theme.font_code  theme.font_icons
theme.font_size_1 .. font_size_4  theme.font_size_p  theme.font_size_base
```

### Dimensions
```
theme.corner_radius  theme.beveling  theme.tab_height
theme.splitter_size  theme.container_corner_radius
```

### Using theme values
```
Label{
    draw_text.color: theme.color_text
    draw_text.text_style: theme.font_regular{font_size: theme.font_size_p}
}
RoundedView{
    padding: theme.mspace_2
    draw_bg.color: theme.color_bg_container
    draw_bg.border_radius: theme.corner_radius
}
```

---

## Templates and Composition

### Defining reusable templates
```
let TodoItem = View{
    width: Fill height: Fit
    padding: Inset{top: 8 bottom: 8 left: 12 right: 12}
    flow: Right spacing: 10
    align: Align{y: 0.5}
    check := CheckBox{text: ""}
    label := Label{text: "task" draw_text.color: #ddd}
    Filler{}
    tag := Label{text: "" draw_text.color: #888}
}
```

### Using templates with overrides
```
TodoItem{label.text: "Buy groceries" tag.text: "errands"}
TodoItem{label.text: "Fix login bug" tag.text: "urgent" label.draw_text.color: #f00}
```

### Named children rules

1. Use `:=` to declare addressable children: `label := Label{...}`
2. Without `:=`, children are anonymous and cannot be overridden
3. Named children inside **anonymous** containers are unreachable:
```
// BAD: label is inside unnamed View, cannot be reached
let Item = View{
    View{  // anonymous!
        label := Label{text: "unreachable"}
    }
}
Item{label.text: "fails silently"}

// GOOD: name the intermediate container
let Item = View{
    texts := View{
        label := Label{text: "reachable"}
    }
}
Item{texts.label.text: "works!"}
```

### Cross-module sharing
`let` bindings are local. To share across modules, store in `mod.widgets`:
```rust
// In module_a.rs
script_mod!{
    use mod.prelude.widgets_internal.*
    mod.widgets.MyCard = RoundedView{...}
}

// In module_b.rs
script_mod!{
    use mod.prelude.widgets.*
    // MyCard is now available because mod.widgets.* was imported
    View{ MyCard{} }
}
```

---

## Scripting Beyond UI

Splash is not just a UI DSL - it's a full scripting language that can:

### Variables and state
```
let todos = []
let count = 0
var mutable_var = "hello"  // var for mutable
```

### Functions
```
fn add_todo(text) {
    todos.push({text: text, done: false})
    ui.todo_list.render()
}

fn count_remaining() {
    let mut n = 0
    for todo in todos {
        if !todo.done { n += 1 }
    }
    return n
}
```

### Event handlers (inline)
```
Button{
    text: "Search"
    on_click: || do_search(query)
}

TextInput{
    empty_text: "Type here..."
    on_return: || ui.search_button.on_click()
}
```

### Dynamic rendering
```
list := PortalList{
    on_render: || {
        for i, item in items {
            let entry = ItemTemplate{}
            entry.label.text = item.title
            entry.check.on_click = |checked| toggle(i, checked)
        }
    }
}
```

### Startup handler
```
Root{
    on_startup: || {
        // Initialize state, start services, etc.
    }
}
```

### HTTP Requests
```
use mod.net

let req = net.HttpRequest{
    url: "https://api.example.com/data"
    method: net.HttpMethod.GET
    headers: {"User-Agent": "MakepadApp/1.0"}
}
net.http_request(req) do net.HttpEvents{
    on_response: |res| {
        let json = res.body.parse_json()
        // Use data
    }
    on_error: |e| { /* handle */ }
}
```

### Streaming HTTP
```
let req = net.HttpRequest{
    url: "https://api.example.com/stream"
    method: net.HttpMethod.POST
    is_streaming: true
    body: {stream: true}.to_json()
}
net.http_request(req) do net.HttpEvents{
    on_stream: |res| { total += res.body.to_string() }
    on_complete: |res| { /* done */ }
    on_error: |e| { /* handle */ }
}
```

### Promises and async
```
fn fetch(url) {
    let p = promise()
    let req = net.HttpRequest{url: url method: net.HttpMethod.GET}
    net.http_request(req) do net.HttpEvents{
        on_response: |res| { p.resolve(res.body.parse_json()) }
        on_error: |e| { p.reject(e) }
    }
    return p
}

let data = fetch("https://api.example.com").await()
```

### HTML parsing
```
let doc = html_string.parse_html()
doc.query("p")                 // All <p> elements
doc.query("#main")             // By ID
doc.query("p.bold")            // By class
doc.query("div > p")           // Direct children
doc.query("a@href")            // Attribute values
doc.query("p[0]").text         // Text content of first <p>
```

### WebSocket (via `script!`)
```
net.web_socket("ws://localhost:8188/ws") do net.WebSocketEvents{
    on_string: |msg| { /* handle message */ }
}
```

### File I/O (via `script!`)
```
use mod.fs
let content = fs.read("path/to/file")
fs.write("path/to/output", data)
```

### Child processes (via `script!`)
```
use mod.run
run.child(run.ChildCmd{cmd: "node" args: ["-e" "console.log('hi')"]}) do run.ChildEvents{
    on_stdout: |data| { /* handle */ }
    on_stderr: |data| { /* handle */ }
    on_exit: |code| { /* handle */ }
}
```

### Timers
```
use mod.std
std.start_interval(60) do fn{ /* every 60 seconds */ }
std.start_timeout(1000, || { /* after 1 second */ })
```

---

## Vector Graphics

The `Vector{}` widget renders SVG-like graphics declaratively:

### Basic shapes
```
Vector{width: 200 height: 200 viewbox: vec4(0 0 200 200)
    Rect{x: 10 y: 10 w: 80 h: 60 rx: 5 fill: #f80}
    Circle{cx: 150 cy: 50 r: 30 fill: #08f}
    Line{x1: 10 y1: 150 x2: 190 y2: 150 stroke: #fff stroke_width: 2}
    Path{d: "M 10 10 L 100 100 Z" fill: #f00 stroke: #000 stroke_width: 2}
    Ellipse{cx: 100 cy: 50 rx: 80 ry: 40 fill: #0f8}
    Polygon{pts: [100 10 40 198 190 78] fill: #f0f}
    Polyline{pts: [10 10 50 80 100 20] fill: false stroke: #ff0 stroke_width: 2}
}
```

### Gradients
```
let my_grad = Gradient{x1: 0 y1: 0 x2: 1 y2: 1
    Stop{offset: 0 color: #ff0000}
    Stop{offset: 1 color: #0000ff}
}
let radial = RadGradient{cx: 0.5 cy: 0.5 r: 0.5
    Stop{offset: 0 color: #fff}
    Stop{offset: 1 color: #000}
}
Rect{fill: my_grad ...}
Circle{fill: radial ...}
```

### Groups and transforms
```
Group{transform: [Translate{x: 100 y: 50} Scale{x: 2 y: 2} Rotate{deg: 30}]
    Circle{cx: 0 cy: 0 r: 20 fill: #0ff}
}
```

### Filters
```
let shadow = Filter{DropShadow{dx: 2 dy: 4 blur: 6 color: #000 opacity: 0.5}}
Rect{filter: shadow ...}
```

### Tween animations
```
Circle{cx: 50 cy: 50 r: 30
    fill: Tween{dur: 1.5 loop_: true from: #ff0000 to: #0000ff}
    transform: Rotate{deg: 0 dur: 2.0 from: 0 to: 360 loop_: true}
}
```

---

## Rust Integration

### App structure
```rust
use makepad_widgets::*;

app_main!(App);

script_mod!{
    use mod.prelude.widgets.*

    load_all_resources() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(800, 600)
                body +: {
                    // UI content
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
        if self.ui.button(ids!(my_button)).clicked(actions) {
            log!("Clicked!");
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

### Widget struct pattern
```rust
#[derive(Script, ScriptHook, Widget)]
pub struct MyWidget {
    #[source] source: ScriptObjectRef,
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    #[redraw] #[live] draw_bg: DrawQuad,
    #[live] draw_text: DrawText,
    #[rust] my_state: i32,
}

// With animator:
#[derive(Script, ScriptHook, Widget, Animator)]
pub struct AnimatedWidget {
    #[source] source: ScriptObjectRef,
    #[apply_default] animator: Animator,
    // ...
}
```

### Widget registration pattern
```rust
script_mod!{
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    // 1. Register Rust struct
    mod.widgets.MyWidgetBase = #(MyWidget::register_widget(vm))

    // 2. Create styled variant with defaults
    mod.widgets.MyWidget = set_type_default() do mod.widgets.MyWidgetBase{
        width: Fill height: Fit
        draw_bg +: { color: theme.color_bg_app }
    }
}
```

### Custom draw shader registration
```rust
script_mod!{
    use mod.prelude.widgets_internal.*
    set_type_default() do #(DrawMyShader::script_shader(vm)){
        ..mod.draw.DrawQuad  // Inherit from DrawQuad
    }
    mod.widgets.MyWidgetBase = #(MyWidget::register_widget(vm))
}
```

### Runtime property updates
```rust
// Old: item.apply_over(cx, live!{...})
// New:
script_apply_eval!(cx, item, {
    height: #(height)
    draw_bg: {is_even: #(if is_even {1.0} else {0.0})}
});
```

### Multi-module registration order
```rust
impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        crate::makepad_widgets::script_mod(vm);  // Base widgets FIRST
        crate::my_widgets::script_mod(vm);        // Custom widgets
        crate::app_ui::script_mod(vm);            // UI that uses them
        App::from_script_mod(vm, self::script_mod)
    }
}
```

### Debug logging
```rust
script_mod!{
    ~mod.theme           // Logs the theme object
    ~some_variable       // Logs a variable's value
}
```

---

## Available Widgets Reference

### Core containers
`View`, `SolidView`, `RoundedView`, `RoundedAllView`, `RoundedXView`, `RoundedYView`, `RectView`, `RectShadowView`, `RoundedShadowView`, `CircleView`, `HexagonView`, `GradientXView`, `GradientYView`, `CachedView`, `CachedRoundedView`

### Scrollable containers
`ScrollXView`, `ScrollYView`, `ScrollXYView`

### Text
`Label`, `Labelbold`, `LabelGradientX`, `LabelGradientY`, `H1`, `H2`, `H3`, `H4`, `P`, `Pbold`, `TextBox`

### Buttons
`Button`, `ButtonFlat`, `ButtonFlatter`

### Toggles
`CheckBox`, `CheckBoxFlat`, `CheckBoxCustom`, `Toggle`, `ToggleFlat`, `RadioButton`, `RadioButtonFlat`

### Input
`TextInput`, `TextInputFlat`, `Slider`, `SliderMinimal`, `DropDown`, `DropDownFlat`

### Media
`Image`, `Icon`, `Svg`, `Vector`, `LoadingSpinner`, `MathView`

### Layout
`Hr`, `Vr`, `Filler`, `Splitter`, `FoldButton`, `FoldHeader`, `ScrollBar`

### Lists
`PortalList`, `FlatList`

### Navigation
`Modal`, `Tooltip`, `PopupNotification`, `SlidePanel`, `ExpandablePanel`, `PageFlip`, `StackNavigation`, `SlidesView`

### Dock
`Dock`, `DockFlat`, `DockSplitter`, `DockTabs`, `DockTab`

### Rich text
`TextFlow`, `Markdown`, `Html` (feature-gated)

### Special
`Root`, `Window`, `FileTree`, `CachedWidget`, `Splash`, `MapView`

### Link
`LinkLabel`
