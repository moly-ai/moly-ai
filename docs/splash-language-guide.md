# Splash Language Guide (Makepad v2)

Splash is Makepad's scripting language and UI DSL. It replaces the old `live_design!` macro system with `script_mod!`, a real interpreted language with a VM, garbage collector, and runtime evaluation. This guide covers everything needed to write Splash code.

---

## 1. Core Syntax Rules

- **No commas** between properties. Whitespace/newlines separate them.
- **No semicolons** needed (optional; the language is whitespace-delimited).
- Properties use `key: value` syntax. **Never** `key = value`.
- Named children use `:=` syntax: `my_label := Label{text: "Hello"}`.
- Curly braces `{}` for objects/blocks. No angle brackets for inheritance.
- Comments: `//` for line comments.

```
View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 10
    padding: 16
    my_button := Button{text: "Click me"}
    Label{text: "Anonymous child"}
}
```

## 2. `script_mod!` Macro

All Splash code in Rust files lives inside `script_mod!{}`. This replaces the old `live_design!{}`.

### Imports

```rust
script_mod!{
    // For app development (includes all standard widgets)
    use mod.prelude.widgets.*
    
    // For widget library internals (does NOT include widget definitions)
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
}
```

### Registering Widgets

```rust
script_mod!{
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    // Register a Rust struct as a base widget
    mod.widgets.MyWidgetBase = #(MyWidget::register_widget(vm))

    // Create a styled variant with defaults
    mod.widgets.MyWidget = set_type_default() do mod.widgets.MyWidgetBase{
        width: Fill
        height: Fit
        draw_bg +: {
            color: uniform(#333)
        }
    }
}
```

### Registering Non-Widget Components

```rust
// For structs that don't implement Widget but need script integration
mod.widgets.AppBase = #(App::script_component(vm))
```

### Registering Custom Draw Shaders

```rust
// Register a custom draw shader type
set_type_default() do #(DrawMyShader::script_shader(vm)){
    ..mod.draw.DrawQuad  // Inherit from DrawQuad
}
```

### App Setup Pattern

```rust
script_mod!{
    use mod.prelude.widgets.*

    // startup() or load_all_resources() initializes the app
    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(800, 600)
                body +: {
                    // UI tree here
                }
            }
        }
    }
}
```

## 3. `let` Bindings (Reusable Templates)

`let` creates a reusable template local to the current `script_mod!` block.

```rust
let TodoItem = View{
    width: Fill height: Fit
    flow: Right spacing: 10
    check := CheckBox{text: ""}
    label := Label{text: "task"}
}

// Use by instantiating:
View{
    TodoItem{label.text: "Buy groceries"}
    TodoItem{label.text: "Write tests"}
}
```

**Rules:**
- `let` bindings are LOCAL to the `script_mod!` block. They cannot be accessed from other files.
- Must be defined BEFORE use.
- To share across modules, store in `mod.*` namespace instead.

## 4. Named Children (`:=` Operator)

The `:=` operator creates a **named, addressable** child widget.

```
title := Label{text: "Default Title"}
```

Named children can be:
- Referenced from Rust: `self.ui.label(ids!(title))`
- Overridden when instantiating: `MyTemplate{title.text: "New Title"}`
- Used as templates in lists: `Item := View{...}` inside PortalList

**Plain `:` creates a static property** that is NOT addressable. Using `:` when you need `:=` makes overrides fail silently.

## 5. Property Merging (`+:`)

The `+:` operator merges with the parent definition instead of replacing:

```
mod.widgets.MyButton = mod.widgets.Button{
    draw_bg +: {
        color: uniform(#f00)  // Only overrides color, keeps all other draw_bg properties
    }
}
```

Without `+:`, you **replace** the entire property object. This is the most common mistake when overriding draw shaders, text styles, or animator blocks.

## 6. Sizing System

| Value | Meaning |
|-------|---------|
| `Fill` | Expand to fill available space |
| `Fit` | Shrink to content size |
| `200` | Fixed 200px |
| `Fill{min: 100 max: 500}` | Fill with constraints |
| `Fit{max: Abs(300)}` | Fit with maximum |

**Critical rule:** Containers inside `Fit` parents MUST use `height: Fit`. Default is `Fill`, which inside `Fit` = 0 height = invisible.

## 7. Layout System

```
View{
    flow: Down          // Down, Right, Overlay, Flow.Right{wrap: true}
    spacing: 10         // Gap between children
    padding: 16         // Uniform padding (or Inset{top: 10 left: 20 ...})
    margin: Inset{left: 5 right: 5}
    align: Center       // Center, HCenter, VCenter, TopLeft, Align{x: 0.5 y: 0.5}
    
    width: Fill
    height: Fit
}
```

### Align Shortcuts

| Shortcut | Equivalent |
|----------|------------|
| `Center` | `Align{x: 0.5 y: 0.5}` |
| `HCenter` | `Align{x: 0.5 y: 0.0}` |
| `VCenter` | `Align{x: 0.0 y: 0.5}` |
| `TopLeft` | `Align{x: 0.0 y: 0.0}` |

## 8. Available Widgets

### Containers / Views

| Widget | Description |
|--------|-------------|
| `View` | Base container (no background unless `show_bg: true`) |
| `SolidView` | Solid color background |
| `RoundedView` | Rounded corners with optional border |
| `RoundedAllView` | Per-corner radius control |
| `RectView` | Rectangle with optional border |
| `RectShadowView` | Rectangle with drop shadow |
| `RoundedShadowView` | Rounded corners with drop shadow |
| `CircleView` | Circular container |
| `HexagonView` | Hexagonal container |
| `GradientXView` / `GradientYView` | Gradient backgrounds |
| `CachedView` / `CachedRoundedView` | GPU-cached for performance |
| `ScrollXYView` / `ScrollXView` / `ScrollYView` | Scrollable containers |

**Do NOT use `View{show_bg: true}`** -- it has an ugly green test color. Use styled views instead.

### Text & Input

| Widget | Description |
|--------|-------------|
| `Label` | Static text |
| `Labelbold` | Bold label |
| `H1` / `H2` / `H3` / `H4` | Heading labels |
| `P` / `Pbold` | Paragraph text |
| `TextInput` | Editable text field |
| `TextInputFlat` | Flat-styled text input |
| `LinkLabel` | Clickable text link |
| `TextFlow` | Rich text container |
| `Markdown` | Markdown rendering |
| `Html` | HTML rendering |

### Buttons

| Widget | Description |
|--------|-------------|
| `Button` | Standard button with bevel |
| `ButtonFlat` | Flat button |
| `ButtonFlatter` | Minimally styled button |
| `ButtonIcon` / `ButtonFlatIcon` / `ButtonFlatterIcon` | Icon-only variants |
| `ButtonGradientX` / `ButtonGradientY` | Gradient buttons |

### Toggles

| Widget | Description |
|--------|-------------|
| `CheckBox` / `CheckBoxFlat` | Checkbox toggle |
| `Toggle` / `ToggleFlat` | Toggle switch |
| `RadioButton` / `RadioButtonFlat` | Radio button |

### Input Controls

| Widget | Description |
|--------|-------------|
| `Slider` / `SliderMinimal` | Value slider |
| `DropDown` / `DropDownFlat` | Dropdown selector |

### Media

| Widget | Description |
|--------|-------------|
| `Image` | Bitmap image (with `ImageFit`, `ImageAnimation`) |
| `Icon` | SVG icon |
| `Svg` | Full SVG rendering (can animate) |
| `Vector` | Declarative SVG-like drawing (Path, Rect, Circle, etc.) |
| `LoadingSpinner` | Animated spinner |
| `MathView` | LaTeX math rendering |

### Layout Utilities

| Widget | Description |
|--------|-------------|
| `Hr` / `Vr` | Horizontal/vertical rule |
| `Filler` | Flex spacer |
| `Splitter` | Draggable split pane |
| `FoldHeader` / `FoldButton` | Collapsible section |

### Lists

| Widget | Description |
|--------|-------------|
| `PortalList` | Virtualized scrolling list (only renders visible items) |
| `FlatList` | Non-virtualized list |
| `ScrollBar` | Standalone scrollbar |

### Navigation & Overlays

| Widget | Description |
|--------|-------------|
| `Modal` | Modal overlay |
| `Tooltip` | Tooltip popup |
| `PopupNotification` | Toast notification |
| `SlidePanel` | Sliding panel (left/right/top) |
| `ExpandablePanel` | Pull-to-expand panel |
| `PageFlip` | Page switching container |
| `StackNavigation` | Stack-based navigation |
| `SlidesView` | Slide presentation view |

### Dock System

| Widget | Description |
|--------|-------------|
| `Dock` | Dockable panel container |
| `DockSplitter` | Dock split |
| `DockTabs` | Tab group in dock |
| `DockTab` | Individual dock tab |

## 9. Theme Variables

Access via `theme.` prefix. Never use the old `(THEME_CONSTANT)` syntax.

```
color: theme.color_bg_app
padding: theme.space_2
font_size: theme.font_size_p
text_style: theme.font_regular
border_radius: theme.corner_radius
```

Common theme variables:
- Colors: `theme.color_bg_app`, `theme.color_outset`, `theme.color_label_inner`, `theme.color_u_hidden`, etc.
- Spacing: `theme.space_1` through `theme.space_6`, `theme.mspace_1`, `theme.mspace_v_1`
- Fonts: `theme.font_regular`, `theme.font_bold`, `theme.font_mono`
- Font sizes: `theme.font_size_p`, `theme.font_size_h1`, `theme.font_size_base`
- Dimensions: `theme.corner_radius`, `theme.beveling`, `theme.data_icon_width`

## 10. Shader System

Shaders live inside `draw_bg`, `draw_text`, `draw_icon`, etc.

### Instance vs Uniform vs Texture

```
draw_bg +: {
    hover: instance(0.0)           // Per-widget, animatable
    color: uniform(#333)           // Shared across shader instances
    tex: texture_2d(float)         // Texture sampler
    my_var: varying(0.0)           // Vertex-to-fragment interpolated
}
```

### Shader Functions

```
draw_bg +: {
    pixel: fn() {
        let sdf = Sdf2d.viewport(self.pos * self.rect_size)
        sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0)
        sdf.fill(self.color)
        return sdf.result
    }

    get_color: fn() {
        return self.color
            .mix(self.color_hover, self.hover)
            .mix(self.color_down, self.down)
    }
}
```

**Key syntax differences from old shaders:**
- `pixel: fn()` not `fn pixel(self) -> vec4`
- `Sdf2d.viewport(...)` not `Sdf2d::viewport(...)` (dot, not double-colon)
- `Math.random_2d(...)` not `Math::random_2d(...)`
- Color method chaining: `color.mix(other, t)` not `mix(color, other, t)`
- Use `modf(a, b)` not `mod(a, b)` for float modulo
- Use `atan2(y, x)` not `atan(y, x)` for two-arg arctangent

### SDF Primitives

```
sdf.circle(cx, cy, radius)
sdf.rect(x, y, w, h)
sdf.box(x, y, w, h, radius)
sdf.hexagon(cx, cy, radius)
sdf.arc(cx, cy, inner_r, outer_r, start_angle, end_angle)
sdf.hline(y, half_thickness)
sdf.move_to(x, y)
sdf.line_to(x, y)
sdf.close_path()
```

### SDF Operations

```
sdf.fill(color)              // Fill and reset
sdf.fill_keep(color)         // Fill and keep shape
sdf.stroke(color, width)     // Stroke outline
sdf.glow(color, width)       // Outer glow

sdf.union()                  // Boolean union
sdf.intersect()              // Boolean intersection
sdf.subtract()               // Boolean subtraction
sdf.gloop(radius)            // Smooth blend
sdf.blend(factor)            // Blend between shapes
```

## 11. Animator System

Animators drive smooth transitions between states on instance shader variables.

```
animator: Animator{
    hover: {
        default: @off
        off: AnimatorState{
            from: {all: Forward{duration: 0.1}}
            apply: {
                draw_bg: {hover: 0.0}
                draw_text: {hover: 0.0}
            }
        }
        on: AnimatorState{
            from: {
                all: Forward{duration: 0.1}
                down: Forward{duration: 0.01}
            }
            apply: {
                draw_bg: {hover: snap(1.0)}
                draw_text: {hover: snap(1.0)}
            }
        }
        down: AnimatorState{
            from: {all: Forward{duration: 0.2}}
            apply: {
                draw_bg: {down: snap(1.0), hover: 1.0}
            }
        }
    }
    focus: {
        default: @off
        off: AnimatorState{
            from: {all: Forward{duration: 0.2}}
            apply: {draw_bg: {focus: 0.0}}
        }
        on: AnimatorState{
            cursor: MouseCursor.Arrow
            from: {all: Forward{duration: 0.0}}
            apply: {draw_bg: {focus: 1.0}}
        }
    }
}
```

### Transition Types (Play)

| Type | Description |
|------|-------------|
| `Forward{duration: 0.2}` | Linear forward animation |
| `Snap` | Instant jump (no animation) |
| `Reverse{duration: 0.3}` | Reverse animation |
| `Loop{duration: 1.0, end: 1000.0}` | Looping animation |
| `ReverseLoop{...}` | Ping-pong loop |
| `BounceLoop{...}` | Bounce loop |
| `ExpDecay{d1: 0.8, d2: 0.97}` | Exponential decay |

### Special Values in Apply

- `snap(1.0)` -- instant jump to value (replaces old `[{time: 0.0, value: 1.0}]` keyframe syntax)
- `timeline(0.0 0.0 1.0 1.0)` -- keyframe timeline (time-value pairs)

### Ease Functions

Available in `from` blocks: `Linear`, `None`, `InQuad`, `OutQuad`, `InOutQuad`, `InCubic`, `OutCubic`, `InOutCubic`, `InQuart`, `OutQuart`, `InOutQuart`, `InQuint`, `OutQuint`, `InOutQuint`, `InSine`, `OutSine`, `InOutSine`, `InExp`, `OutExp`, `InOutExp`, `InCirc`, `OutCirc`, `InOutCirc`, `InElastic`, `OutElastic`, `InOutElastic`, `InBack`, `OutBack`, `InOutBack`, `InBounce`, `OutBounce`, `InOutBounce`, `Bezier`

## 12. Script-Side Features (on_render, on_click, etc.)

Splash is a real scripting language. Views and widgets can have inline callbacks.

### on_render (Dynamic Rendering)

`on_render` on a View (or ScrollYView, etc.) lets you dynamically build the content at render time:

```
todo_list := ScrollYView{
    width: Fill height: Fill
    flow: Down spacing: 2

    on_render: ||{
        for i in 0..todos.len() {
            let todo = todos[i]
            TodoItem{
                check.selected: todo.done
                check.on_click: |checked| toggle_todo(i, checked)
                label.text: todo.text
                delete.on_click: || delete_todo(i)
            }
        }
    }
}
```

To trigger re-render from script: `ui.todo_list.render()`

### on_click

```
add_button := Button{
    text: "Add"
    on_click: ||{
        let text = ui.todo_input.text()
        if text != "" {
            add_todo(text, "general")
            ui.todo_input.set_text("")
            ui.todo_list.render()
        }
    }
}
```

### on_return (TextInput)

```
todo_input := TextInput{
    on_return: || ui.add_button.on_click()
}
```

### on_startup (Root)

```
Root{
    on_startup: ||{
        ui.todo_list.render()
    }
    // ...
}
```

### Script-Side State and Functions

```
let todos = []

fn add_todo(text, tag) {
    todos.push({text: text, done: false, tag: tag})
}

fn toggle_todo(index, checked) {
    todos[index].done = checked
    ui.todo_list.render()
}

fn delete_todo(index) {
    todos.remove(index)
    ui.todo_list.render()
}
```

### Accessing Widgets from Script

- `ui.widget_id.text()` -- get text content
- `ui.widget_id.set_text("new text")` -- set text content
- `ui.widget_id.render()` -- trigger on_render callback
- `ui.widget_id.on_click()` -- programmatically trigger click handler

## 13. HTTP Networking (from Script)

```
net.http_request(
    net.HttpRequest{
        url: "https://api.example.com/data"
        method: "GET"
        headers: {
            "Authorization": "Bearer token123"
        }
    }
    net.HttpEvents{
        on_response: |res| {
            let data = res.body
            // process response
        }
        on_error: |e| {
            log("Error: " + e)
        }
        on_stream: |res| {
            // streaming data chunks
        }
        on_complete: |res| {
            // stream finished
        }
    }
)
```

## 14. HTML Parsing (from Script)

```
let doc = "<div class='item'><h1>Title</h1><p>Body</p></div>".parse_html()
let title = doc.query("h1").text       // "Title"
let body = doc.query("p").text         // "Body"
let attr = doc.query("div").attr("class")  // "item"
let items = doc.query(".item").array()     // array of matches
let count = items.length
```

## 15. Resource Loading

```
// From crate resources (compiled into binary)
source: crate_resource("self://resources/image.png")
svg: crate_resource("self://resources/icon.svg")

// From HTTP URL
svg: http_resource("https://example.com/icon.svg")
```

**Old syntax was:** `dep("crate://self/resources/icon.svg")` -- this no longer works.

## 16. Color Syntax

```
color: #ff0000          // 6-digit hex
color: #f00             // 3-digit hex
color: #ff000080        // 8-digit hex with alpha
color: vec4(1.0, 0.0, 0.0, 1.0)  // RGBA float
```

**Warning:** Hex colors containing `e` (e.g., `#2ecc71`) fail because the Rust tokenizer treats `2e` as scientific notation. Use the `#x` prefix: `#x2ecc71`.

Colors that need `#x`: `#x1e1e2e`, `#x2ecc71`, `#x4466ee`, `#x7799ee`, `#xbb99ee`, etc.

## 17. Vector Widget (Declarative SVG-like Drawing)

```
Vector{
    width: 24 height: 24
    Path{
        d: "M12 2L2 22h20L12 2z"
        fill: #fff
        stroke: #000
        line_width: 1.0
    }
    Circle{cx: 12 cy: 12 r: 5 fill: #f00}
    Rect{x: 0 y: 0 w: 24 h: 24 fill: #0000}
    Line{x1: 0 y1: 0 x2: 24 y2: 24 stroke: #fff line_width: 2.0}
    Group{
        Translate{x: 10 y: 10}
        Rotate{angle: 45.0}
        Rect{w: 10 h: 10 fill: #00f}
    }
}
```

Supports: `Path`, `Rect`, `Circle`, `Ellipse`, `Line`, `Polyline`, `Polygon`, `Group`, `Gradient`, `RadGradient`, `Filter` (DropShadow), transforms (`Rotate`, `Scale`, `Translate`, `SkewX`, `SkewY`), and `Tween` property animations.

## 18. Cross-Module Sharing

To share definitions between `script_mod!` blocks across files:

```rust
// file1.rs -- export to mod.widgets namespace
script_mod!{
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    
    mod.widgets.MyCustomWidget = set_type_default() do mod.widgets.ViewBase{
        width: Fill height: Fit
    }
}

// file2.rs -- import from mod.widgets namespace
script_mod!{
    use mod.prelude.widgets.*
    // MyCustomWidget is now available via mod.widgets.*
    
    View{
        MyCustomWidget{}
    }
}
```

The `mod` global object is the ONLY way to share between script_mod blocks. `let` bindings and `crate.` prefixes do NOT work.

## 19. Debug Logging

Use `~` prefix to log any expression during script evaluation:

```rust
script_mod!{
    ~mod.theme           // Logs the theme object
    ~mod.widgets         // Logs available widgets
}
```

## 20. Common Pitfalls

1. **Forgot `height: Fit`**: Default is `Fill`. Inside `Fit` containers = 0 height = invisible.
2. **Used `:` instead of `:=`**: Children become non-addressable. Overrides fail silently.
3. **Forgot `+:`**: Replaces entire property object instead of merging.
4. **Used `(THEME_X)` syntax**: Must be `theme.x` (lowercase, dot notation).
5. **Used `<Widget>` angle brackets**: Must be `Widget{}` (no angle brackets).
6. **Used commas between properties**: No commas needed.
7. **Used `dep("crate://...")`**: Must be `crate_resource("self://...")`.
8. **Used `Sdf2d::method()`**: Must be `Sdf2d.method()` (dot, not double-colon).
9. **Used `fn pixel(self) -> vec4`**: Must be `pixel: fn() { ... }`.
10. **Used `#pub` in script_mod**: No `pub` keyword. Use `mod.widgets.X = ...`.
11. **Used `border_radius: Inset{...}`**: `border_radius` takes a single float, not an Inset.
12. **Put `#[rust]` fields after `#[deref]`** in draw shaders: Non-instance data must go BEFORE `#[deref]`.
13. **Used `margin: {left: 10}`**: Must be `margin: Inset{left: 10}` (constructor syntax).
14. **Used `cursor: Hand`**: Must be `cursor: MouseCursor.Hand`.
15. **Used `mod(a, b)` in shader**: Must be `modf(a, b)`.
