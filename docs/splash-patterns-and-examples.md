# Splash Patterns and Examples

> Practical patterns for building Makepad applications with the new Splash / `script_mod!` system. Focused on patterns needed for the Moly migration: app structure, custom widgets, PortalList, modals, HTTP/streaming, custom shaders, responsive layout, and multi-module projects.

## Table of Contents

- [App Boilerplate](#app-boilerplate)
- [Custom Widget (Simple)](#custom-widget-simple)
- [Custom Widget (With Animator)](#custom-widget-with-animator)
- [Custom Widget (With Custom Shader)](#custom-widget-with-custom-shader)
- [PortalList (Virtualized List)](#portallist-virtualized-list)
- [PortalList with Multiple Templates](#portallist-with-multiple-templates)
- [PortalList with Auto-Tail (Chat Pattern)](#portallist-with-auto-tail-chat-pattern)
- [Modal Dialogs](#modal-dialogs)
- [Sidebar / Toggle Panel](#sidebar--toggle-panel)
- [Tab Navigation / PageFlip](#tab-navigation--pageflip)
- [Responsive Layout (Desktop vs Mobile)](#responsive-layout-desktop-vs-mobile)
- [Inline Event Handlers (Script-Only)](#inline-event-handlers-script-only)
- [Dynamic Rendering (on_render)](#dynamic-rendering-on_render)
- [HTTP Requests from Script](#http-requests-from-script)
- [Custom Draw Shader Registration](#custom-draw-shader-registration)
- [Hover Effect Pattern](#hover-effect-pattern)
- [Icon Button Pattern](#icon-button-pattern)
- [Form Layout Pattern](#form-layout-pattern)
- [Theme Override / Custom Styling](#theme-override--custom-styling)
- [Multi-Module Project Structure](#multi-module-project-structure)
- [Runtime Property Updates (script_apply_eval!)](#runtime-property-updates-script_apply_eval)
- [Markdown / Rich Text Rendering](#markdown--rich-text-rendering)
- [Image Loading](#image-loading)
- [Action Enum Pattern](#action-enum-pattern)
- [Scope Data Passing](#scope-data-passing)
- [Search/Filter Pattern](#searchfilter-pattern)
- [Dropdown Selection](#dropdown-selection)
- [Splitter Layout](#splitter-layout)
- [Loading Spinner](#loading-spinner)
- [DrawList2d Overlay (Modals/Popups/Tooltips)](#drawlist2d-overlay-modalspopupstooltips)
- [UiRunner Async Bridge](#uirunner-async-bridge)
- [ComponentMap Dynamic Widget Collection](#componentmap-dynamic-widget-collection)
- [Slot Widget (Replaceable Content)](#slot-widget-replaceable-content)
- [Timer-Driven Patterns](#timer-driven-patterns)
- [AdaptiveView (Desktop vs Mobile)](#adaptiveview-desktop-vs-mobile)
- [StackNavigation](#stacknavigation)
- [Scope::with\_props](#scopewith_props)

---

## App Boilerplate

Minimal app with a window, body, and event handling:

```rust
use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    load_all_resources() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(1024, 768)
                window.title: "My App"
                body +: {
                    flow: Down
                    padding: 16
                    spacing: 12

                    header := Label{
                        text: "Welcome"
                        draw_text.text_style.font_size: 18
                    }

                    content := View{
                        width: Fill height: Fill
                        // main content here
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
        // Handle widget actions here
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

**Notes:**
- `startup()` can be used instead of `load_all_resources()` — `load_all_resources()` pre-loads all `crate_resource()` refs at startup.
- `body +:` merges into Window's existing body (don't replace it).
- `window.inner_size` and `window.title` use dot-path shorthand.

---

## Custom Widget (Simple)

Widget with `#[deref] view: View` — inherits all View functionality:

```rust
use crate::{makepad_derive_widget::*, makepad_draw::*, widget::*};

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.ChatScreenBase = #(ChatScreen::register_widget(vm))
    mod.widgets.ChatScreen = set_type_default() do mod.widgets.ChatScreenBase{
        width: Fill height: Fill
        flow: Down spacing: 8

        messages := View{
            width: Fill height: Fill
        }
        input_area := View{
            width: Fill height: Fit
            flow: Right spacing: 8
            input := TextInput{width: Fill height: Fit empty_text: "Type..."}
            send := Button{text: "Send"}
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ChatScreen {
    #[deref] view: View,
    #[rust] message_count: usize,
}

impl Widget for ChatScreen {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}
```

---

## Custom Widget (With Animator)

Pattern for hoverable/clickable widgets with visual state transitions:

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

    mod.widgets.HoverCardBase = #(HoverCard::register_widget(vm))
    mod.widgets.HoverCard = set_type_default() do mod.widgets.HoverCardBase{
        width: Fill height: Fit
        padding: 12 flow: Down spacing: 8
        show_bg: true
        cursor: MouseCursor.Hand

        draw_bg +: {
            hover: instance(0.0)
            down: instance(0.0)
            color: uniform(#2a2a3a)
            color_hover: uniform(#3a3a4a)
            color_down: uniform(#1a1a2a)
            border_radius: uniform(8.0)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(0. 0. self.rect_size.x self.rect_size.y self.border_radius)
                let color = self.color
                    .mix(self.color_hover, self.hover)
                    .mix(self.color_down, self.down)
                sdf.fill(color)
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
    }
}

#[derive(Script, ScriptHook, Widget, Animator)]
pub struct HoverCard {
    #[apply_default] animator: Animator,
    #[deref] view: View,
}

impl Widget for HoverCard {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        if self.animator_handle_event(cx, event).must_redraw() {
            self.view.redraw(cx);
        }
        match event.hits(cx, self.view.area()) {
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(_) => {
                self.animator_play(cx, id!(down.on));
            }
            Hit::FingerUp(fe) => {
                self.animator_play(cx, id!(down.off));
                if fe.is_over {
                    // Dispatch action
                    cx.widget_action(self.widget_uid(), &scope.path, HoverCardAction::Clicked);
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum HoverCardAction {
    Clicked,
    #[default]
    None,
}
```

---

## Custom Widget (With Custom Shader)

Custom draw shader with its own registered type:

```rust
use crate::{makepad_derive_widget::*, makepad_draw::*, widget::*};

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    // Register custom draw shader
    set_type_default() do #(DrawProgressBar::script_shader(vm)){
        ..mod.draw.DrawQuad  // Inherit from DrawQuad base
    }

    mod.widgets.ProgressBarBase = #(ProgressBar::register_widget(vm))
    mod.widgets.ProgressBar = set_type_default() do mod.widgets.ProgressBarBase{
        width: Fill height: 6
        show_bg: true

        draw_bg +: {
            progress: instance(0.0)
            color_bg: uniform(#333)
            color_fill: uniform(#x2b55ff)
            border_radius: uniform(3.0)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                // Background
                sdf.box(0. 0. self.rect_size.x self.rect_size.y self.border_radius)
                sdf.fill(self.color_bg)
                // Fill
                let fill_width = self.rect_size.x * self.progress
                sdf.box(0. 0. fill_width self.rect_size.y self.border_radius)
                sdf.fill(self.color_fill)
                return sdf.result
            }
        }
    }
}

#[derive(Script, ScriptHook)]
#[repr(C)]
pub struct DrawProgressBar {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub progress: f32,
}

#[derive(Script, ScriptHook, Widget)]
pub struct ProgressBar {
    #[deref] view: View,
}
```

**Important:** Custom draw shader structs need `#[repr(C)]` for correct GPU memory layout. Non-instance fields must come BEFORE `#[deref]`, and instance fields AFTER.

---

## PortalList (Virtualized List)

PortalList is the primary virtualized list widget. Templates are defined as named children inside the list.

### DSL Definition

```rust
script_mod! {
    use mod.prelude.widgets.*

    // ... in your UI definition:
    my_list := PortalList{
        width: Fill height: Fill
        flow: Down

        // Templates — named with := become templates
        Item := View{
            width: Fill height: Fit
            padding: Inset{top: 8 bottom: 8 left: 12 right: 12}
            flow: Right spacing: 10
            align: Align{y: 0.5}

            title := Label{text: "" draw_text.color: #fff}
            Filler{}
            tag := Label{text: "" draw_text.color: #888 draw_text.text_style.font_size: 10}
        }

        Header := SolidView{
            width: Fill height: 40
            padding: Inset{left: 12}
            align: Align{y: 0.5}
            show_bg: true
            draw_bg.color: #222

            title := Label{text: "Section" draw_text.color: #aaa}
        }
    }
}
```

### Rust Drawing Code

```rust
#[derive(Script, ScriptHook, Widget)]
pub struct MyListView {
    #[deref] view: View,
    #[rust] items: Vec<ItemData>,
}

impl Widget for MyListView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, self.items.len());

                while let Some(item_id) = list.next_visible_item(cx) {
                    if let Some(data) = self.items.get(item_id) {
                        let item_widget = list.item(cx, item_id, id!(Item));
                        item_widget.label(ids!(title)).set_text(cx, &data.title);
                        item_widget.label(ids!(tag)).set_text(cx, &data.tag);
                        item_widget.draw_all(cx, &mut Scope::empty());
                    }
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}
```

### Key PortalList Methods

```rust
list.set_item_range(cx, first_id, count);         // Set total item count
list.next_visible_item(cx) -> Option<usize>;       // Get next item to draw
list.item(cx, item_id, template_id) -> WidgetRef;  // Get/create item from template
list.item_with_existed(cx, item_id, template_id) -> (WidgetRef, bool);  // Also returns if new
list.set_tail_range(true);                         // Enable auto-scroll to bottom
list.set_first_id_and_scroll(id, offset);          // Scroll to specific item
list.items_with_actions(actions);                   // Iterate items that have pending actions
```

---

## PortalList with Multiple Templates

Selecting different templates per item:

```rust
fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
    while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
        if let Some(mut list) = item.as_portal_list().borrow_mut() {
            list.set_item_range(cx, 0, self.messages.len());

            while let Some(item_id) = list.next_visible_item(cx) {
                let msg = &self.messages[item_id];
                let template = match msg.role {
                    Role::User => id!(User),
                    Role::Assistant => id!(Assistant),
                    Role::System => id!(System),
                };
                let item_widget = list.item(cx, item_id, template);
                item_widget.label(ids!(content)).set_text(cx, &msg.text);
                item_widget.draw_all(cx, &mut Scope::empty());
            }
        }
    }
    DrawStep::done()
}
```

---

## PortalList with Auto-Tail (Chat Pattern)

For chat UIs where new messages appear at the bottom and the list should auto-scroll:

```
list := PortalList{
    width: Fill height: Fill
    flow: Down
    drag_scrolling: false
    auto_tail: true          // Auto-scroll to bottom when new items added
    smooth_tail: true        // Smooth animation for auto-scroll
    selectable: true         // Allow text selection

    Message := RoundedView{
        width: Fill height: Fit
        padding: 12 flow: Down
        show_bg: true
        draw_bg +: { color: #2a2a3a border_radius: uniform(8.0) }

        content := Markdown{
            width: Fill height: Fit
            selectable: true
            body: ""
        }
    }
}
```

In Rust, after adding a new message:
```rust
list.set_tail_range(true);
list.set_first_id_and_scroll(items_len.saturating_sub(1), 0.0);
self.ui.redraw(cx);
```

---

## Modal Dialogs

### DSL

```rust
script_mod! {
    use mod.prelude.widgets.*

    // ... inside your window body:
    my_modal := Modal{
        content +: {
            width: 400 height: Fit
            flow: Down

            RoundedView{
                width: Fill height: Fit
                padding: 24 flow: Down spacing: 16
                show_bg: true
                draw_bg.color: #333
                draw_bg.border_radius: 12.0

                title := Label{
                    text: "Confirm Delete"
                    draw_text.text_style: theme.font_bold{font_size: 16}
                    draw_text.color: #fff
                }

                body := Label{
                    text: "Are you sure?"
                    draw_text.color: #aaa
                }

                View{
                    width: Fill height: Fit
                    flow: Right spacing: 8
                    align: Align{x: 1.0}

                    cancel_btn := Button{text: "Cancel"}
                    confirm_btn := Button{text: "Delete"}
                }
            }
        }
    }
}
```

### Rust (show/hide)

```rust
// Show modal
self.ui.modal(ids!(my_modal)).open(cx);

// Hide modal
self.ui.modal(ids!(my_modal)).close(cx);

// Handle actions
if self.ui.button(ids!(my_modal.confirm_btn)).clicked(actions) {
    // Do the thing
    self.ui.modal(ids!(my_modal)).close(cx);
}
```

---

## Sidebar / Toggle Panel

Pattern for a collapsible side panel:

```
View{
    width: Fill height: Fill
    flow: Right

    sidebar := SolidView{
        width: 260 height: Fill
        flow: Down
        show_bg: true
        draw_bg.color: #1a1a2a

        // Sidebar content
        Label{text: "Navigation" padding: 16}
    }

    // Main content
    content := View{
        width: Fill height: Fill
        flow: Down
        // ...
    }
}
```

Toggle visibility from Rust:
```rust
fn toggle_sidebar(&mut self, cx: &mut Cx) {
    let sidebar = self.ui.view(ids!(sidebar));
    let visible = sidebar.is_visible();
    sidebar.set_visible(cx, !visible);
}
```

Or use `SlidePanel` for animated sliding:
```
panel := SlidePanel{
    side: SlideSide.Left
    width: 260 height: Fill
    // content
}
```

---

## Tab Navigation / PageFlip

```
View{
    width: Fill height: Fill
    flow: Down

    // Tab bar
    tab_bar := View{
        width: Fill height: Fit
        flow: Right spacing: 0

        chat_tab := Button{text: "Chat" width: Fit}
        models_tab := Button{text: "Models" width: Fit}
        settings_tab := Button{text: "Settings" width: Fit}
    }

    // Page content
    pages := PageFlip{
        width: Fill height: Fill
        active_page := chat_page

        chat_page := View{width: Fill height: Fill}
        models_page := View{width: Fill height: Fill}
        settings_page := View{width: Fill height: Fill}
    }
}
```

Rust:
```rust
if self.ui.button(ids!(chat_tab)).clicked(actions) {
    self.ui.page_flip(ids!(pages)).set_active_page(cx, ids!(chat_page));
}
if self.ui.button(ids!(models_tab)).clicked(actions) {
    self.ui.page_flip(ids!(pages)).set_active_page(cx, ids!(models_page));
}
```

---

## Responsive Layout (Desktop vs Mobile)

Use `apply_over` / `script_apply_eval!` to switch layouts at runtime:

```rust
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let width = cx.get_global::<WindowGeom>().inner_size.x;
        let is_mobile = width < 600.0;

        if is_mobile != self.was_mobile {
            self.was_mobile = is_mobile;
            if is_mobile {
                script_apply_eval!(cx, self.ui, {
                    main_content: { flow: Down }
                    sidebar: { visible: false }
                });
            } else {
                script_apply_eval!(cx, self.ui, {
                    main_content: { flow: Right }
                    sidebar: { visible: true }
                });
            }
        }
    }
}
```

---

## Inline Event Handlers (Script-Only)

These work in standalone Splash scripts or the `Splash` widget, not in `script_mod!`:

```
View{
    flow: Down spacing: 10

    Button{
        text: "Click Me"
        on_click: || {
            log("Button clicked!")
        }
    }

    TextInput{
        empty_text: "Search..."
        on_return: || {
            log("Search submitted!")
        }
    }
}
```

---

## Dynamic Rendering (on_render)

Script-only pattern for dynamic list rendering:

```
let items = ["Apple" "Banana" "Cherry" "Date"]

list := PortalList{
    width: Fill height: Fill
    flow: Down

    on_render: || {
        for i, item in items {
            let entry = View{
                width: Fill height: 40
                padding: Inset{left: 12}
                align: Align{y: 0.5}
                Label{text: item}
            }
        }
    }
}
```

---

## HTTP Requests from Script

Script-only pattern:

```
use mod.net

fn search(query) {
    let req = net.HttpRequest{
        url: "https://api.example.com/search?q=" + query
        method: net.HttpMethod.GET
        headers: {"Content-Type": "application/json"}
    }
    net.http_request(req) do net.HttpEvents{
        on_response: |res| {
            let data = res.body.parse_json()
            // Update UI with results
            ui.results_label.text = data.count + " results found"
            ui.results_label.redraw()
        }
        on_error: |e| {
            ui.results_label.text = "Error: " + e
        }
    }
}
```

For streaming responses (like LLM APIs):
```
let req = net.HttpRequest{
    url: "https://api.openai.com/v1/chat/completions"
    method: net.HttpMethod.POST
    is_streaming: true
    headers: {
        "Content-Type": "application/json"
        "Authorization": "Bearer " + api_key
    }
    body: {
        model: "gpt-4"
        messages: [{role: "user" content: prompt}]
        stream: true
    }.to_json()
}

let total_response = ""
net.http_request(req) do net.HttpEvents{
    on_stream: |res| {
        total_response += res.body.to_string()
        ui.output.text = total_response
        ui.output.redraw()
    }
    on_complete: |res| {
        log("Streaming complete")
    }
}
```

---

## Custom Draw Shader Registration

When your widget needs a custom `DrawXxx` struct:

```rust
#[derive(Script, ScriptHook)]
#[repr(C)]
pub struct DrawRoundedShadow {
    // Non-instance fields BEFORE deref
    #[rust] cache_valid: bool,

    // Base draw type
    #[deref] pub draw_super: DrawQuad,

    // Instance fields AFTER deref (these become shader inputs)
    #[live] pub shadow_color: Vec4f,
    #[live] pub shadow_radius: f32,
    #[live] pub shadow_offset: Vec2f,
}

script_mod! {
    use mod.prelude.widgets_internal.*

    set_type_default() do #(DrawRoundedShadow::script_shader(vm)){
        ..mod.draw.DrawQuad  // Inherit from DrawQuad
    }
}
```

**Critical layout rule for `#[repr(C)]` draw shaders:**
- `#[rust]` and non-instance `#[live]` fields → BEFORE `#[deref]`
- Instance `#[live]` fields (shader inputs) → AFTER `#[deref]`

---

## Hover Effect Pattern

Minimal hover effect without full Animator (using shader only):

```
draw_bg +: {
    hover: instance(0.0)
    color: uniform(#2a2a3a)
    color_hover: uniform(#3a3a4a)

    pixel: fn() {
        let color = self.color.mix(self.color_hover, self.hover)
        return Pal.premul(color)
    }
}
```

The animator drives `hover` between 0.0 and 1.0:
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
    }
}
```

---

## Icon Button Pattern

Button with icon and optional label:

```
let IconButton = ButtonFlat{
    width: Fit height: Fit
    padding: 8
    draw_bg +: {
        color: uniform(#0000)
        color_hover: uniform(#fff1)
        border_radius: uniform(4.0)
    }
    draw_icon +: {
        color: #aaa
    }
    icon_walk: Walk{width: 20 height: 20}
    label_walk: Walk{width: 0 height: 0}  // Hide label
}

// Usage
close_btn := IconButton{
    draw_icon.svg: crate_resource("self://resources/icons/close.svg")
}
```

---

## Form Layout Pattern

Vertical form with label-input pairs:

```
let FormGroup = View{
    width: Fill height: Fit
    flow: Down spacing: 4

    label := Label{
        draw_text.color: #888
        draw_text.text_style.font_size: 11
    }
    input := TextInput{
        width: Fill height: Fit
        empty_text: ""
    }
}

// Usage
FormGroup{
    label.text: "API Key"
    input.empty_text: "Enter your API key..."
    input.is_password: true
}

FormGroup{
    label.text: "Model Name"
    input.empty_text: "e.g. gpt-4"
}
```

---

## Theme Override / Custom Styling

### Project-wide color scheme

```rust
// In styles.rs
script_mod! {
    use mod.prelude.widgets.*

    // Store custom theme values in mod namespace
    mod.moly = {
        colors: {
            primary: #x2b55ff
            primary_hover: #x4070ff
            surface: #x1e1e2e
            surface_hover: #x2a2a3e
            text: #xd4d4d4
            text_secondary: #667085
            border: #x3a3a4a
            danger: #xB42318
        }
    }
}
```

Usage in other modules:
```
RoundedView{
    show_bg: true
    draw_bg.color: mod.moly.colors.surface
    draw_bg.border_radius: 8.0

    Label{
        text: "Styled"
        draw_text.color: mod.moly.colors.text
    }
}
```

### Overriding default widget theme

```rust
script_mod! {
    use mod.prelude.widgets_internal.*

    // Override the default Button style globally
    mod.widgets.Button = set_type_default() do mod.widgets.ButtonBase{
        // Custom defaults for all Buttons in this app
        draw_bg +: {
            color: uniform(#x2b55ff)
            color_hover: uniform(#x4070ff)
            border_radius: uniform(6.0)
        }
        draw_text +: {
            color: #fff
        }
    }
}
```

---

## Multi-Module Project Structure

### File organization

```
my_app/
├── src/
│   ├── main.rs          # platform entry
│   ├── lib.rs           # module declarations
│   ├── app.rs           # App struct + script_mod with Root
│   ├── shared/
│   │   ├── mod.rs
│   │   ├── styles.rs    # Custom theme, constants
│   │   └── widgets.rs   # Shared reusable widgets
│   ├── chat/
│   │   ├── mod.rs
│   │   ├── chat_screen.rs
│   │   └── chat_view.rs
│   └── settings/
│       ├── mod.rs
│       └── providers.rs
```

### lib.rs

```rust
pub mod shared;
pub mod chat;
pub mod settings;
pub mod app;
```

### shared/widgets.rs (registers shared widgets)

```rust
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.FadeViewBase = #(FadeView::register_widget(vm))
    mod.widgets.FadeView = set_type_default() do mod.widgets.FadeViewBase{
        width: Fill height: Fill
        show_bg: true
        draw_bg +: {
            opacity: instance(1.0)
            pixel: fn() {
                return Pal.premul(vec4(0. 0. 0. (1.0 - self.opacity)))
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct FadeView {
    #[deref] view: View,
}

impl Widget for FadeView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}
```

### app.rs (registration order)

```rust
use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    load_all_resources() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    ChatScreen{}  // Uses widget registered by chat module
                }
            }
        }
    }
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        // 1. Base makepad widgets
        crate::makepad_widgets::script_mod(vm);
        // 2. Shared widgets and styles
        crate::shared::styles::script_mod(vm);
        crate::shared::widgets::script_mod(vm);
        // 3. Feature modules
        crate::chat::chat_screen::script_mod(vm);
        crate::settings::providers::script_mod(vm);
        // 4. App itself (uses everything above)
        App::from_script_mod(vm, self::script_mod)
    }
}
```

**Registration order matters!** Modules that define widgets must be registered before modules that use those widgets.

---

## Runtime Property Updates (script_apply_eval!)

Common patterns for dynamic property changes:

```rust
// Setting text content
script_apply_eval!(cx, label_ref, {
    text: #(format!("{} items", count))
});

// Setting visibility
script_apply_eval!(cx, panel_ref, {
    visible: #(is_expanded)
});

// Setting colors from Rust
let color = if is_active { Vec4f::from_hex("#2b55ff") } else { Vec4f::from_hex("#666") };
script_apply_eval!(cx, icon_ref, {
    draw_icon: { color: #(color) }
});

// Setting numeric values
script_apply_eval!(cx, progress_ref, {
    draw_bg: { progress: #(download_progress) }
});

// Setting height for collapse animation
let height = if collapsed { 0.1 } else { 200.0 };
script_apply_eval!(cx, content_ref, {
    height: #(height)
});

// Conditional styling
script_apply_eval!(cx, row_ref, {
    draw_bg: {
        is_even: #(if index % 2 == 0 { 1.0 } else { 0.0 })
        is_selected: #(if selected { 1.0 } else { 0.0 })
    }
});
```

---

## Markdown / Rich Text Rendering

```
Markdown{
    width: Fill height: Fit
    selectable: true
    use_code_block_widget: true   // Use custom code block widget
    use_math_widget: true         // Enable LaTeX math rendering
    body: "# Hello\n\nThis is **bold** and *italic*.\n\n```rust\nfn main() {}\n```"

    // Custom code block template
    code_block := View{
        width: Fill height: Fit
        flow: Overlay
        code_view := CodeView{
            editor +: {
                height: Fit
                draw_bg +: { color: #1a1a2e }
            }
        }
    }

    // Math rendering
    inline_math := MathView{font_size: 13.0}
    display_math := MathView{font_size: 15.0}
}
```

Setting markdown content from Rust:
```rust
let markdown = self.ui.markdown(cx, ids!(my_markdown));
markdown.set_text(cx, &markdown_string);
```

---

## Image Loading

```
Image{
    width: 200 height: 150
    fit: ImageFit.Stretch
    // Image source is typically set from Rust
}
```

From Rust:
```rust
// Load from URL (async)
let image = self.ui.image(ids!(my_image));
image.load_image_dep_by_path(cx, "path/to/image.png");

// Or load from bytes
// image.load_jpg_from_data(cx, &jpg_bytes);
// image.load_png_from_data(cx, &png_bytes);
```

---

## Action Enum Pattern

Standard pattern for widget-to-parent communication:

```rust
#[derive(Clone, Debug, Default)]
pub enum MyWidgetAction {
    Selected(usize),
    Deleted(usize),
    Edited(usize, String),
    #[default]
    None,
}

// Dispatching actions (in handle_event)
cx.widget_action(self.widget_uid(), &scope.path, MyWidgetAction::Selected(item_id));

// Consuming actions (in parent's handle_actions)
for action in self.ui.widget(ids!(my_widget)).actions() {
    if let MyWidgetAction::Selected(id) = action.as_widget_action().cast() {
        // Handle selection
    }
}
```

---

## Scope Data Passing

Pass data down to child widgets through Scope:

```rust
// Parent sets data in scope
let mut scope = Scope::empty();
scope.data.set(MyData { items: vec![] });
self.ui.handle_event(cx, event, &mut scope);

// Child reads from scope
impl Widget for ChildWidget {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(data) = scope.data.get::<MyData>() {
            // Use data
        }
        self.view.draw_walk(cx, scope, walk)
    }
}
```

---

## Search/Filter Pattern

Text input that filters a list:

```
View{
    flow: Down spacing: 8

    search := TextInput{
        width: Fill height: Fit
        empty_text: "Search models..."
    }

    results := PortalList{
        width: Fill height: Fill
        flow: Down

        Item := View{
            width: Fill height: 40
            padding: Inset{left: 12}
            align: Align{y: 0.5}
            name := Label{text: ""}
        }
    }
}
```

Rust:
```rust
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if let Some(text) = self.ui.text_input(cx, ids!(search)).changed(actions) {
            self.filter_text = text;
            self.update_filtered_items();
            self.ui.redraw(cx);
        }
    }
}
```

---

## Dropdown Selection

```
backend_dropdown := DropDown{
    width: 170
    labels: ["Claude" "GPT-4" "Gemini" "Ollama"]
}
```

Rust:
```rust
// Handle selection
if let Some(index) = self.ui.drop_down(cx, ids!(backend_dropdown)).selected(actions) {
    match index {
        0 => self.select_backend(Backend::Claude),
        1 => self.select_backend(Backend::Gpt4),
        2 => self.select_backend(Backend::Gemini),
        3 => self.select_backend(Backend::Ollama),
        _ => {}
    }
}

// Set selection programmatically
self.ui.drop_down(cx, ids!(backend_dropdown)).set_selected_item(cx, 0);
```

---

## Splitter Layout

Two-pane layout with draggable divider:

```
Splitter{
    width: Fill height: Fill
    axis: SplitterAxis.Horizontal
    align: SplitterAlign.FromA(280.0)

    a := SolidView{
        width: Fill height: Fill
        show_bg: true draw_bg.color: #1a1a2a
        // Left panel content
    }
    b := View{
        width: Fill height: Fill
        // Right panel content
    }
}
```

Vertical:
```
Splitter{
    axis: SplitterAxis.Vertical
    align: SplitterAlign.FromB(200.0)
    a := View{} // Top
    b := View{} // Bottom
}
```

---

## Loading Spinner

```
View{
    width: Fill height: Fill
    align: Center

    loading := LoadingSpinner{
        width: 40 height: 40
        visible: true
    }
}
```

Toggle from Rust:
```rust
self.ui.loading_spinner(ids!(loading)).set_visible(cx, is_loading);
```

---

## DrawList2d Overlay (Modals/Popups/Tooltips)

`DrawList2d` renders content in a separate draw pass that floats above
the rest of the UI. Combined with `sweep_lock` / `sweep_unlock` it can
capture all pointer events (modal behavior) or let them pass through
(non-blocking popups).

### Struct fields

```rust
#[derive(Script, ScriptHook, Widget)]
pub struct MyOverlay {
    #[deref] view: View,

    // The overlay draw list — note #[rust(...)] initializer
    #[redraw]
    #[rust(DrawList2d::new(cx))]
    draw_list: DrawList2d,

    #[live] draw_bg: DrawQuad,
    #[layout] layout: Layout,
    #[walk] walk: Walk,

    #[rust] opened: bool,
}
```

### DSL

```
mod.widgets.MyOverlay = set_type_default() do mod.widgets.MyOverlayBase{
    width: Fill height: Fill
    flow: Overlay
    align: Center

    // Transparent full-screen background
    draw_bg +: {
        pixel: fn() { return vec4(0. 0. 0. 0.0) }
    }

    // Semi-transparent scrim behind content (only for modals)
    bg_view := View{
        width: Fill height: Fill
        show_bg: true
        draw_bg.color: #0000_00B3   // black 70% opacity
    }

    content := View{
        flow: Overlay width: Fit height: Fit
        // Actual modal/popup content goes here
    }
}
```

### draw_walk — overlay rendering

```rust
fn draw_walk(
    &mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk,
) -> DrawStep {
    // 1. Begin overlay pass — content draws ABOVE everything
    self.draw_list.begin_overlay_reuse(cx);

    // 2. Full-screen turtle for positioning
    cx.begin_root_turtle_for_pass(self.layout);
    self.draw_bg.begin(cx, self.walk, self.layout);

    if self.opened {
        // Draw scrim + content
        let _ = self.bg_view.draw_walk(
            cx, scope, walk.with_abs_pos(DVec2 { x: 0., y: 0. }),
        );
        self.content.draw_all(cx, scope);
    }

    self.draw_bg.end(cx);
    cx.end_pass_sized_turtle();

    // 3. Close overlay pass
    self.draw_list.end(cx);

    DrawStep::done()
}
```

### handle_event — sweep lock for modal behavior

```rust
fn handle_event(
    &mut self, cx: &mut Cx, event: &Event, scope: &mut Scope,
) {
    if !self.opened { return; }

    // Temporarily unlock so children can receive events
    cx.sweep_unlock(self.draw_bg.area());
    self.content.handle_event(cx, event, scope);
    cx.sweep_lock(self.draw_bg.area());

    // Dismiss on click outside content
    let content_rect = self.content.area().rect(cx);
    if let Hit::FingerUp(fe) = event.hits_with_sweep_area(
        cx, self.draw_bg.area(), self.draw_bg.area(),
    ) {
        if !content_rect.contains(fe.abs) {
            self.close(cx);
            cx.widget_action(
                self.widget_uid(), &scope.path,
                MyOverlayAction::Dismissed,
            );
        }
    }
}
```

### open / close — manage sweep lock

```rust
impl MyOverlay {
    pub fn open(&mut self, cx: &mut Cx) {
        self.opened = true;
        self.draw_bg.redraw(cx);
        cx.sweep_lock(self.draw_bg.area()); // capture all events
    }
    pub fn close(&mut self, cx: &mut Cx) {
        self.opened = false;
        self.draw_bg.redraw(cx);
        cx.sweep_unlock(self.draw_bg.area()); // release events
    }
}
```

### Non-blocking variant (popup/tooltip)

For overlays that should NOT capture all events, skip `sweep_lock`
and use `begin_turtle` instead of `begin_root_turtle_for_pass`:

```rust
fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk)
    -> DrawStep
{
    self.draw_list.begin_overlay_reuse(cx);
    cx.begin_turtle(walk, self.layout);      // sized to content
    self.draw_bg.begin(cx, self.walk, self.layout);

    if self.opened {
        self.content.draw_all(cx, scope);
    }

    self.draw_bg.end(cx);
    cx.end_pass_sized_turtle();
    self.draw_list.end(cx);
    DrawStep::done()
}
```

### LiveHook — redraw on apply

```rust
impl LiveHook for MyOverlay {
    fn after_apply(
        &mut self, cx: &mut Cx, _apply: &mut Apply,
        _index: usize, _nodes: &[LiveNode],
    ) {
        self.draw_list.redraw(cx);
    }
}
```

**Key points:**
- `begin_overlay_reuse(cx)` … `end(cx)` bracket the overlay pass
- `sweep_lock` / `sweep_unlock` control whether the overlay captures
  all pointer events (modal) or lets them pass through (popup)
- `begin_root_turtle_for_pass` for full-screen positioning,
  `begin_turtle` for content-sized positioning
- `#[rust(DrawList2d::new(cx))]` initializes the draw list

---

## UiRunner Async Bridge

`UiRunner<W>` lets async tasks schedule work back on the UI thread.
This is critical for patterns where background work (HTTP, streaming,
file I/O) must update widget state.

### Core concept

`UiRunner` is obtained from a widget via `self.ui_runner()`. It can be
sent to an async task. The task calls `defer` / `defer_with_redraw` to
schedule a closure that runs on the main thread with `&mut Widget` +
`&mut Cx` access.

### Basic usage — defer with redraw

```rust
// Inside a widget's handle_event:
let runner = self.ui_runner();
spawn(async move {
    let result = fetch_data().await;
    runner.defer_with_redraw(move |widget, cx, _scope| {
        widget.data = result;
        // Widget is auto-redrawn after this closure
    });
});
```

### Awaitable defer — get a value back from the UI thread

```rust
use moly_kit::utils::makepad::ui_runner::DeferWithRedrawAsync;

let runner = self.ui_runner();
spawn(async move {
    // This awaits until the UI thread runs the closure
    let current_text: Option<String> =
        runner.defer_with_redraw_async(|widget, _cx, _scope| {
            widget.input_text.clone()
        }).await;

    if let Some(text) = current_text {
        let result = process(text).await;
        runner.defer_with_redraw(move |widget, cx, _scope| {
            widget.result = result;
        });
    }
});
```

### Simple redraw trigger

```rust
use moly_kit::utils::makepad::ui_runner::DeferRedraw;

let runner = self.ui_runner();
spawn(async move {
    // After some work completes, just trigger a redraw
    runner.defer_redraw();
});
```

### handle + event forwarding

For widgets that use `UiRunner` internally, call `handle` in
`handle_event` to process deferred closures:

```rust
fn handle_event(
    &mut self, cx: &mut Cx, event: &Event, scope: &mut Scope,
) {
    self.ui_runner().handle(cx, event, scope, self);
    self.view.handle_event(cx, event, scope);
}
```

### Extension traits

MolyKit provides these extension traits on `UiRunner<W>`:

| Trait | Method | Description |
|-------|--------|-------------|
| `DeferRedraw<W>` | `.defer_redraw()` | Fire-and-forget redraw |
| `DeferAsync<T>` | `.defer_async(closure).await` | Awaitable, no redraw |
| `DeferWithRedrawAsync<W>` | `.defer_with_redraw_async(closure).await` | Awaitable + redraw |

**Key points:**
- `UiRunner` is `Send + Sync` — safe to move into async tasks
- `defer_with_redraw` auto-triggers `redraw(cx)` after the closure
- Uses `futures::channel::oneshot` for the async variants (no Tokio
  dependency, works on WASM)

---

## ComponentMap Dynamic Widget Collection

`ComponentMap<LiveId, WidgetRef>` manages a dynamic collection of
widgets created from a template. Use this when the number of child
widgets is data-driven and not known at DSL time.

### Struct fields

```rust
#[derive(Script, ScriptHook, Widget)]
pub struct TagList {
    #[deref] view: View,
    #[walk] walk: Walk,
    #[area] area: Area,
    #[layout] layout: Layout,

    /// Template pointer (set in DSL via LivePtr reference)
    #[live] template: Option<LivePtr>,

    /// Dynamic widget collection
    #[rust] items: ComponentMap<LiveId, WidgetRef>,
}
```

### DSL — template reference

```
mod.widgets.TagList = set_type_default() do mod.widgets.TagListBase{
    width: Fill height: Fit
    flow: Right spacing: 6
    // Reference a template defined elsewhere
    template: TagItem
}

// Template definition
mod.widgets.TagItem = RoundedView{
    width: Fit height: Fit
    padding: Inset{top: 4 bottom: 4 left: 8 right: 8}
    show_bg: true
    draw_bg.color: #333
    draw_bg.border_radius: 4.0
    label := Label{draw_text.color: #aaa draw_text.text_style.font_size: 11}
}
```

### Pattern A: get_or_insert (lazy creation)

Best for draw-time creation where items persist across frames:

```rust
fn draw_walk(
    &mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk,
) -> DrawStep {
    cx.begin_turtle(walk, self.layout);

    for (i, data) in self.data_items.iter().enumerate() {
        let item_id = LiveId(i as u64).into();
        let widget = self.items.get_or_insert(cx, item_id, |cx| {
            WidgetRef::new_from_ptr(cx, self.template)
        });
        widget.label(ids!(label)).set_text(cx, &data.name);
        widget.draw_all(cx, &mut Scope::empty());
    }

    cx.end_turtle_with_area(&mut self.area);
    DrawStep::done()
}
```

### Pattern B: clear + insert (full rebuild)

Best when the item list changes wholesale:

```rust
pub fn set_tags(&mut self, cx: &mut Cx, tags: &[String]) {
    self.items.clear();
    for (i, tag) in tags.iter().enumerate() {
        let item_id = LiveId(i as u64).into();
        let widget = WidgetRef::new_from_ptr(cx, self.template);
        widget.label(ids!(label)).set_text(cx, tag);
        self.items.insert(item_id, widget);
    }
}
```

### Event forwarding

Forward events to all items in the map:

```rust
fn handle_event(
    &mut self, cx: &mut Cx, event: &Event, scope: &mut Scope,
) {
    for (_id, item) in self.items.iter_mut() {
        item.handle_event(cx, event, scope);
    }
}
```

### WidgetNode impl (required for manual Widget implementations)

```rust
impl WidgetNode for TagList {
    fn area(&self) -> Area { self.area }
    fn walk(&mut self, _cx: &mut Cx) -> Walk { self.walk }
    fn redraw(&mut self, cx: &mut Cx) { self.area.redraw(cx) }

    fn find_widgets(
        &self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet,
    ) {
        for item in self.items.values() {
            item.find_widgets(path, cached, results);
        }
    }

    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        self.items.values()
            .map(|item| item.uid_to_widget(uid))
            .find(|x| !x.is_empty())
            .unwrap_or(WidgetRef::empty())
    }
}
```

**Key points:**
- `Option<LivePtr>` holds a reference to the DSL template
- `WidgetRef::new_from_ptr(cx, self.template)` instantiates a widget
  from the template
- `get_or_insert` is idempotent — only creates on first access
- Always forward events to `items.iter_mut()` in `handle_event`

---

## Slot Widget (Replaceable Content)

`Slot` is a wrapper widget whose content can be swapped at runtime.
It holds a `default` widget (from DSL) and a `wrap` widget (active
content). Call `replace()` to swap in new content, `restore()` to
revert to the default.

### DSL

```
my_slot := Slot{
    // Default content (shown until replaced)
    default := View{
        Label{text: "Default content"}
    }
}
```

### Struct — uses `#[wrap]` not `#[deref]`

```rust
#[derive(Script, ScriptHook, Widget)]
pub struct Slot {
    /// Active content — delegates draw/event
    #[wrap] wrap: WidgetRef,

    /// DSL-defined default (can be restored)
    #[live] default: WidgetRef,
}
```

### LiveHook — clone default on creation

```rust
impl LiveHook for Slot {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.wrap = self.default.clone();
    }
}
```

### API

```rust
impl Slot {
    /// Replace active content with a different widget
    pub fn replace(&mut self, widget: WidgetRef) {
        self.wrap = widget;
    }
    /// Revert to the DSL-defined default
    pub fn restore(&mut self) {
        self.wrap = self.default.clone();
    }
    /// Get a reference to the active widget
    pub fn current(&self) -> WidgetRef { self.wrap.clone() }
    /// Get a reference to the default widget
    pub fn default(&self) -> WidgetRef { self.default.clone() }
}
```

### Usage from parent

```rust
// Replace slot content with a custom widget
let custom = WidgetRef::new_from_ptr(cx, some_template);
self.ui.slot(ids!(my_slot)).replace(custom);

// Restore to default
self.ui.slot(ids!(my_slot)).restore();
```

**Key points:**
- `#[wrap]` differs from `#[deref]` — it wraps a `WidgetRef` that can
  be entirely swapped rather than just inheriting from a base type
- The `default` field preserves the original DSL content for restoration
- This pattern is used in MolyKit for customizable content areas (e.g.,
  swapping message renderers, custom input widgets)

---

## Timer-Driven Patterns

Makepad uses `Timer` for delayed and periodic operations. There are
three main patterns: debounce, repeating animation, and intervals.

### Pattern A: Debounced timer (search input)

Cancel and restart a timer on each keystroke; only fire after the user
stops typing:

```rust
#[derive(Script, ScriptHook, Widget)]
pub struct SearchBar {
    #[deref] view: View,
    #[rust] search_timer: Timer,
    #[live(0.3)] search_debounce_time: f64,
}

impl Widget for SearchBar {
    fn handle_event(
        &mut self, cx: &mut Cx, event: &Event, scope: &mut Scope,
    ) {
        self.view.handle_event(cx, event, scope);

        // Timer fired — execute the search
        if self.search_timer.is_event(event).is_some() {
            self.search_timer = Timer::default(); // clear
            let query = self.text_input(ids!(input)).text();
            if query.len() > 2 {
                cx.action(SearchAction::Execute(query.to_string()));
            }
        }
    }
}

impl WidgetMatchEvent for SearchBar {
    fn handle_actions(
        &mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope,
    ) {
        let input = self.text_input(ids!(input));
        if input.changed(actions).is_some() {
            // Cancel any pending timer, restart
            cx.stop_timer(self.search_timer);
            self.search_timer =
                cx.start_timeout(self.search_debounce_time);
        }
    }
}
```

### Pattern B: Repeating animation (self-rescheduling)

A timer that fires, updates animation state, then reschedules itself:

```rust
#[derive(Script, ScriptHook, Widget, Animator)]
pub struct LoadingDots {
    #[deref] view: View,
    #[apply_default] animator: Animator,
    #[rust] timer: Timer,
    #[rust] current_dot: usize,
}

impl Widget for LoadingDots {
    fn handle_event(
        &mut self, cx: &mut Cx, event: &Event, scope: &mut Scope,
    ) {
        // Check if our timer fired
        if self.timer.is_event(event).is_some() {
            self.advance_animation(cx);
        }
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }
}

impl LoadingDots {
    fn advance_animation(&mut self, cx: &mut Cx) {
        self.current_dot = (self.current_dot + 1) % 3;
        // Play animator states based on current_dot...
        self.timer = cx.start_timeout(0.33); // reschedule
    }

    pub fn start(&mut self, cx: &mut Cx) {
        if self.timer.is_empty() {
            self.timer = cx.start_timeout(0.2);
        }
    }

    pub fn stop(&mut self, cx: &mut Cx) {
        cx.stop_timer(self.timer);
        self.timer = Timer::default();
    }
}
```

### Pattern C: Interval timer (periodic polling / streaming)

For regular-interval work like audio streaming or progress polling:

```rust
// Start an interval (fires repeatedly)
let timer = cx.start_interval(0.020); // every 20ms
self.audio_timer = Some(timer);

// In handle_event:
if let Some(timer) = &self.audio_timer {
    if timer.is_event(event).is_some() {
        self.process_audio_chunk(cx);
    }
}

// Stop:
if let Some(timer) = &self.audio_timer {
    cx.stop_timer(*timer);
    self.audio_timer = None;
}
```

**Key API:**
| Method | Behavior |
|--------|----------|
| `cx.start_timeout(secs)` | Fire once after `secs` seconds |
| `cx.start_interval(secs)` | Fire repeatedly every `secs` seconds |
| `cx.stop_timer(timer)` | Cancel a pending timer |
| `timer.is_event(event)` | Returns `Some(...)` if this timer fired |
| `timer.is_empty()` | True if timer is unset / default |

---

## AdaptiveView (Desktop vs Mobile)

`AdaptiveView` switches between layout variants based on screen size
or a custom selector function. It holds named templates (e.g.,
`Desktop`, `Mobile`) and instantiates only the active one.

### DSL

```
adaptive := AdaptiveView{
    width: Fill height: Fill

    Desktop := View{
        flow: Right
        sidebar := View{width: 280 height: Fill}
        main := View{width: Fill height: Fill}
    }

    Mobile := View{
        flow: Down
        main := View{width: Fill height: Fill}
        // No sidebar on mobile
    }
}
```

### Default behavior

By default, `AdaptiveView` checks `cx.display_context.is_desktop()`:
- Desktop → activates the `Desktop` template
- Mobile → activates the `Mobile` template

The selector re-evaluates whenever the parent size changes.

### Custom variant selector

Override the default selector from Rust:

```rust
let mut adaptive = self.ui.adaptive_view(ids!(adaptive));
if let Some(mut inner) = adaptive.borrow_mut() {
    inner.set_variant_selector(|cx, parent_size| {
        if parent_size.x > 800.0 {
            live_id!(Desktop)
        } else if parent_size.x > 500.0 {
            live_id!(Tablet)
        } else {
            live_id!(Mobile)
        }
    });
}
```

### With three or more variants

```
adaptive := AdaptiveView{
    Desktop := View{/* wide layout */}
    Tablet := View{/* medium layout */}
    Mobile := View{/* narrow layout */}
}
```

### Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `retain_unused_variants` | bool | false | Keep inactive variant widgets alive (preserves state) |

**Key points:**
- Only the active variant is drawn and receives events
- Template names are `LiveId`s — use `live_id!(Desktop)` in Rust
- Variant switches automatically on parent size change
- `retain_unused_variants: true` prevents losing widget state on switch

---

## StackNavigation

`StackNavigation` provides a navigation stack with animated push/pop
transitions, similar to iOS UINavigationController. It manages a root
view and a stack of overlay views that slide in from the right.

### DSL

```
nav := StackNavigation{
    width: Fill height: Fill

    // Always-visible root
    root_view := View{
        width: Fill height: Fill
        // Main list / home screen
    }

    // Named views that can be pushed onto the stack
    detail_view := StackNavigationView{
        header +: {
            content +: {
                title_label := Label{text: "Detail"}
            }
        }
        body +: {
            // Detail screen content
            Label{text: "Detail content here"}
        }
    }

    settings_view := StackNavigationView{
        header +: {
            content +: {
                title_label := Label{text: "Settings"}
            }
        }
        body +: {
            // Settings content
        }
    }
}
```

### Push / Pop from Rust

```rust
// Push a view onto the stack (animates in from right)
self.ui.stack_navigation(ids!(nav)).push(cx, ids!(detail_view));

// Pop the top view (animates out to right)
self.ui.stack_navigation(ids!(nav)).pop(cx);

// Pop all the way back to root
self.ui.stack_navigation(ids!(nav)).pop_to_root(cx);

// Check navigation state
let depth = self.ui.stack_navigation(ids!(nav)).depth();
let can_pop = self.ui.stack_navigation(ids!(nav)).can_pop();
```

### StackNavigationView properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `offset` | f64 | 4000.0 | Horizontal offset (animated) |
| `full_screen` | bool | true | Fill entire screen |

### Built-in animation

`StackNavigationView` has a built-in `slide` animator with `show` and
`hide` states using `ExpDecay` easing. The `offset` property is animated
from 4000 (off-screen right) to 0 (visible).

### Handling back navigation

```rust
fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
    // The built-in back button dispatches StackNavigationAction::Pop
    for action in actions {
        if let StackNavigationAction::Pop =
            action.as_widget_action().cast()
        {
            self.ui.stack_navigation(ids!(nav)).pop(cx);
        }
    }
}
```

### Actions

```rust
#[derive(Clone, Default, Debug)]
pub enum StackNavigationAction {
    #[default] None,
    Push(LiveId),
    Pop,
    PopToRoot,
}

#[derive(Clone, Default, Debug)]
pub enum StackNavigationTransitionAction {
    #[default] None,
    ShowBegin,
    ShowDone,
    HideBegin,
    HideEnd(WidgetUid),
}
```

**Key points:**
- `root_view` is always visible when the stack is empty
- Views slide in with an `ExpDecay` animation (~0.5s)
- The built-in `StackViewHeader` includes a back button
- Use `body +:` and `header +:` to merge into the default layout
- Works well with `AdaptiveView` — use `StackNavigation` on mobile,
  `Splitter` on desktop

---

## Scope::with_props

`Scope::with_props` passes typed, read-only data from a parent widget
to its children during drawing. Children access the data via
`scope.props.get::<T>()`. This is lighter than `Scope::with_data`
(which allows mutation) and is ideal for passing item-level data in
list rendering.

### Defining props

```rust
pub struct FileRowProps {
    pub filename: String,
    pub size_bytes: u64,
    pub is_downloaded: bool,
}
```

### Passing props (parent draw_walk)

```rust
fn draw_walk(
    &mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk,
) -> DrawStep {
    while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
        if let Some(mut list) = item.as_portal_list().borrow_mut() {
            list.set_item_range(cx, 0, self.files.len());

            while let Some(item_id) = list.next_visible_item(cx) {
                let data = &self.files[item_id];
                let widget = list.item(cx, item_id, id!(FileRow));

                // Pass data as props
                let props = FileRowProps {
                    filename: data.name.clone(),
                    size_bytes: data.size,
                    is_downloaded: data.downloaded,
                };
                let mut scope = Scope::with_props(&props);
                widget.draw_all(cx, &mut scope);
            }
        }
    }
    DrawStep::done()
}
```

### Reading props (child widget)

```rust
impl Widget for FileRow {
    fn draw_walk(
        &mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk,
    ) -> DrawStep {
        if let Some(props) = scope.props.get::<FileRowProps>() {
            self.label(ids!(filename))
                .set_text(cx, &props.filename);
            self.label(ids!(size))
                .set_text(cx, &format_bytes(props.size_bytes));
            self.view(ids!(download_icon))
                .set_visible(cx, props.is_downloaded);
        }
        self.view.draw_walk(cx, scope, walk)
    }
}
```

### with_props vs with_data

| Method | Access | Mutability | Use case |
|--------|--------|------------|----------|
| `Scope::with_props(&T)` | `scope.props.get::<T>()` | Read-only | Item data in lists |
| `Scope::with_data(&mut T)` | `scope.data.get_mut::<T>()` | Mutable | Shared app state (Store) |

### Combining props and data

```rust
// Parent passes both global data AND item props
let store = scope.data.get_mut::<Store>().unwrap();
let file_data = store.files[item_id].clone();

let props = FileRowProps { /* ... */ };
let mut child_scope = Scope::with_props(&props);
// Re-attach the store data
child_scope.data.set(store);
widget.draw_all(cx, &mut child_scope);
```

**Key points:**
- Props are passed by reference (`&T`) — no ownership transfer
- The child receives a `Scope` with the props available via `.get::<T>()`
- `with_props` is the idiomatic way to pass per-item data in
  `PortalList` / `ComponentMap` rendering loops
- Use `with_data` for app-level mutable state (like `Store`)
