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
