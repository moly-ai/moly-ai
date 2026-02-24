# Rust API Changes: Makepad v1 → v2

This document covers all Rust-side API changes when migrating from old Makepad (v1, `Live`/`LiveHook`) to new Makepad (v2, `Script`/`ScriptHook`). It is a comprehensive reference for every struct, trait, derive, and method signature that changed.

---

## 1. Derive Macros

| Old | New | Notes |
|-----|-----|-------|
| `#[derive(Live)]` | `#[derive(Script)]` | Core deserialization derive |
| `#[derive(LiveHook)]` | `#[derive(ScriptHook)]` | Lifecycle hook derive |
| `#[derive(Widget)]` | `#[derive(Widget)]` | Unchanged |
| `#[derive(DefaultNone)]` | `#[derive(Default)]` + `#[default]` on variant | See Section 3 |
| — | `#[derive(Animator)]` | New; generates `AnimatorImpl` methods |
| `#[derive(LiveComponentRegistry)]` | Removed | Registry is manual now |

---

## 2. Required Struct Fields

Every widget struct in v2 needs two new fields:

```rust
// OLD
#[derive(Live, LiveHook, Widget)]
pub struct MyWidget {
    #[animator]
    animator: Animator,
    // ... other fields
}

// NEW
#[derive((Script, ScriptHook, Widget, Animator)]
pub struct MyWidget {
    #[uid]
    uid: WidgetUid,           // NEW: unique widget ID (auto-assigned via atomic counter)
    #[source]
    source: ScriptObjectRef,  // NEW: reference to the script object backing this widget

    #[apply_default]
    animator: Animator,       // CHANGED: #[animator] → #[apply_default]
    // ... other fields
}
```

### Field Attribute Changes

| Old | New | Notes |
|-----|-----|-------|
| `#[animator]` | `#[apply_default]` | Animator field attribute |
| `#[live]` | `#[live]` | Unchanged |
| `#[walk]` | `#[walk]` | Unchanged |
| `#[layout]` | `#[layout]` | Unchanged |
| `#[redraw]` | `#[redraw]` | Unchanged |
| `#[visible]` | `#[visible]` | Unchanged |
| `#[rust]` | `#[rust]` | Unchanged |
| `#[action_data]` | `#[action_data]` | Unchanged |
| `#[live(default)]` | `#[live(default)]` | Unchanged |
| — | `#[uid]` | NEW: required on `uid: WidgetUid` field |
| — | `#[source]` | NEW: required on `source: ScriptObjectRef` field |

---

## 3. Action Enum Changes

**Old:**
```rust
#[derive(Clone, Debug, DefaultNone)]
pub enum MyAction {
    None,
    Clicked(KeyModifiers),
    ValueChanged(f64),
}
```

**New:**
```rust
#[derive(Clone, Debug, Default)]
pub enum MyAction {
    #[default]
    None,
    Clicked(KeyModifiers),
    ValueChanged(f64),
}
```

The `DefaultNone` derive macro is replaced by standard `Default` derive with `#[default]` attribute on the `None` variant.

---

## 4. Widget Trait Changes

### Supertrait

```rust
// OLD
pub trait Widget: WidgetNode { ... }
pub trait WidgetNode: LiveApply { ... }

// NEW
pub trait Widget: WidgetNode { ... }
pub trait WidgetNode: ScriptApply { ... }
```

### New Methods on `Widget`

```rust
// NEW: Script-to-widget communication
fn script_call(
    &mut self,
    vm: &mut ScriptVm,
    method: LiveId,
    args: ScriptValue,
) -> ScriptAsyncResult {
    ScriptAsyncResult::MethodNotFound
}

// NEW: Async script result callback
fn script_result(&mut self, vm: &mut ScriptVm, id: ScriptAsyncId, result: ScriptValue) {}

// NEW: Whether widget wants mouse/touch events
fn is_interactive(&self) -> bool { true }
```

### Changed Method Signatures on `Widget`

```rust
// OLD: widget finding (no cx parameter)
fn widget(&self, path: &[LiveId]) -> WidgetRef;
fn widgets(&self, paths: &[&[LiveId]]) -> WidgetSet;

// NEW: widget finding (requires cx for tree access)
fn widget(&self, cx: &Cx, path: &[LiveId]) -> WidgetRef;
fn widgets(&self, cx: &Cx, paths: &[&[LiveId]]) -> WidgetSet;

// NEW: flood-fill search (entirely new)
fn widget_flood(&self, cx: &Cx, path: &[LiveId]) -> WidgetRef;
fn widgets_flood(&self, cx: &Cx, paths: &[&[LiveId]]) -> WidgetSet;
```

### Changed Method Signatures on `Widget` (ref_cast_type_id)

```rust
// OLD: returns LiveType
fn ref_cast_type_id(&self) -> LiveType;

// NEW: returns TypeId (std)
fn ref_cast_type_id(&self) -> TypeId;
```

### Removed Methods from `Widget`

```rust
// REMOVED: data binding (no longer exists)
fn widget_to_data(&self, cx: &mut Cx, actions: &Actions, nodes: &mut LiveNodeVec, path: &[LiveId]) -> bool;
fn data_to_widget(&mut self, cx: &mut Cx, nodes: &[LiveNode], path: &[LiveId]);
```

### Unchanged Methods on `Widget`

These methods have identical signatures in v1 and v2:
- `handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope)`
- `handle_event_with(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope, sweep_area: Area)`
- `draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep`
- `draw(&mut self, cx: &mut Cx2d, scope: &mut Scope) -> DrawStep`
- `draw_all(&mut self, cx: &mut Cx2d, scope: &mut Scope)`
- `draw_unscoped(&mut self, cx: &mut Cx2d) -> DrawStep`
- `draw_all_unscoped(&mut self, cx: &mut Cx2d)`
- `text(&self) -> String`
- `set_text(&mut self, cx: &mut Cx, v: &str)`
- `set_key_focus(&self, cx: &mut Cx)`
- `key_focus(&self, cx: &Cx) -> bool`
- `set_disabled(&mut self, cx: &mut Cx, disabled: bool)`
- `disabled(&self, cx: &Cx) -> bool`
- `ui_runner(&self) -> UiRunner<Self>`

---

## 5. WidgetNode Trait Changes

### Removed Methods

```rust
// REMOVED: path-based widget finding (replaced by tree)
fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef;
fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet);
```

### New Methods

```rust
// NEW: enumerate direct children for widget tree indexing
fn children(&self, visit: &mut dyn FnMut(LiveId, WidgetRef)) {}

// NEW: skip this node's subtree in global flood-fill search
fn skip_widget_tree_search(&self) -> bool { false }

// NEW: hit testing
fn find_widgets_from_point(&self, cx: &Cx, point: DVec2, found: &mut dyn FnMut(&WidgetRef)) {}
fn find_interactive_widget_from_point(&self, cx: &Cx, point: DVec2) -> Option<WidgetRef>;
fn point_hits_area(&self, cx: &Cx, point: DVec2) -> bool;

// NEW: selection API
fn selection_text_len(&self) -> usize;
fn selection_point_to_char_index(&self, cx: &Cx, abs: DVec2) -> Option<usize>;
fn selection_set(&mut self, anchor: usize, cursor: usize);
fn selection_clear(&mut self);
fn selection_select_all(&mut self);
fn selection_get_text_for_range(&self, start: usize, end: usize) -> String;
fn selection_get_full_text(&self) -> String;
```

### Changed: widget_uid Location

```rust
// OLD: widget_uid on Widget trait (pointer-based)
impl Widget {
    fn widget_uid(&self) -> WidgetUid {
        WidgetUid(self as *const _ as *const () as u64)
    }
}

// NEW: widget_uid on WidgetNode trait (from #[uid] field)
impl WidgetNode {
    fn widget_uid(&self) -> WidgetUid {
        WidgetUid(0)  // default; overridden by #[derive(Script)] using #[uid] field
    }
}
```

### Unchanged Methods on `WidgetNode`

- `walk(&mut self, cx: &mut Cx) -> Walk`
- `area(&self) -> Area`
- `redraw(&mut self, cx: &mut Cx)`
- `set_action_data(&mut self, data: Arc<dyn ActionTrait>)`
- `action_data(&self) -> Option<Arc<dyn ActionTrait>>`
- `set_visible(&mut self, cx: &mut Cx, visible: bool)`
- `visible(&self) -> bool`

---

## 6. WidgetAction Struct Changes

```rust
// OLD
pub struct WidgetAction {
    pub data: Option<Arc<dyn ActionTrait>>,
    pub action: Box<dyn WidgetActionTrait>,
    pub widgets: SmallVec<[WidgetRef; 4]>,  // REMOVED in v2
    pub widget_uid: WidgetUid,
    pub path: HeapLiveIdPath,               // REMOVED in v2
    pub group: Option<WidgetActionGroup>,
}

// NEW
pub struct WidgetAction {
    pub data: Option<Arc<dyn ActionTrait>>,
    pub action: Box<dyn WidgetActionTrait>,
    pub widget_uid: WidgetUid,
    pub group: Option<WidgetActionGroup>,
}
```

Removed fields: `widgets` (SmallVec of WidgetRefs) and `path` (HeapLiveIdPath). Actions are now identified solely by `widget_uid`.

---

## 7. Action Posting (Cx Extension)

```rust
// OLD
cx.widget_action(uid, &scope.path, MyAction::Clicked);
cx.widget_action_with_data(&self.action_data, uid, &scope.path, MyAction::Clicked);

// NEW
cx.widget_action(uid, MyAction::Clicked);
cx.widget_action_with_data(&self.action_data, uid, MyAction::Clicked);
```

The `&scope.path` parameter is removed from both methods. The full trait:

```rust
// OLD
pub trait WidgetActionCxExt {
    fn widget_action(&mut self, uid: WidgetUid, path: &HeapLiveIdPath, t: impl WidgetActionTrait);
    fn widget_action_with_data(
        &mut self, action_data: &WidgetActionData,
        widget_uid: WidgetUid, path: &HeapLiveIdPath,
        t: impl WidgetActionTrait,
    );
}

// NEW
pub trait WidgetActionCxExt {
    fn widget_action(&mut self, uid: WidgetUid, t: impl WidgetActionTrait);
    fn widget_action_with_data(
        &mut self, action_data: &WidgetActionData,
        widget_uid: WidgetUid,
        t: impl WidgetActionTrait,
    );
}
```

---

## 8. LiveHook → ScriptHook

### Old: `LiveHook` trait

```rust
pub trait LiveHook {
    fn apply_value_unknown(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize;
    fn skip_apply_animator(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> bool;
    fn apply_value_instance(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize;
    fn skip_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> Option<usize>;
    fn before_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]);
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]);
    fn after_apply_from(&mut self, cx: &mut Cx, apply: &mut Apply);
    fn after_new_from_doc(&mut self, cx: &mut Cx);
    fn after_update_from_doc(&mut self, cx: &mut Cx);
    fn after_apply_from_doc(&mut self, cx: &mut Cx);
    fn after_new_before_apply(&mut self, cx: &mut Cx);
}
```

### New: `ScriptHook` trait

```rust
pub trait ScriptHook {
    // Root entry points
    fn on_before_apply(&mut self, vm: &mut ScriptVm, apply: &Apply, scope: &mut Scope, value: ScriptValue);
    fn on_before_dispatch(&mut self, vm: &mut ScriptVm, apply: &Apply, scope: &mut Scope, value: ScriptValue);
    fn on_after_apply(&mut self, vm: &mut ScriptVm, apply: &Apply, scope: &mut Scope, value: ScriptValue);
    fn on_after_dispatch(&mut self, vm: &mut ScriptVm, apply: &Apply, scope: &mut Scope, value: ScriptValue);
    
    // Custom apply (return true to skip generated apply)
    fn on_custom_apply(&mut self, vm: &mut ScriptVm, apply: &Apply, scope: &mut Scope, value: ScriptValue) -> bool;
    
    // Type reflection (for proc macro)
    fn on_type_check(heap: &ScriptHeap, value: ScriptValue) -> bool;
    fn on_proto_build(vm: &mut ScriptVm, obj: ScriptObject, props: &mut ScriptTypeProps);
    fn on_proto_methods(vm: &mut ScriptVm, obj: ScriptObject);
    
    // Simple lifecycle hooks
    fn on_alive(&self);
    fn on_before_new(&mut self, vm: &mut ScriptVm);
    fn on_before_reload(&mut self, vm: &mut ScriptVm);
    fn on_after_new(&mut self, vm: &mut ScriptVm);
    fn on_after_reload(&mut self, vm: &mut ScriptVm);
    
    // With scope
    fn on_before_new_scoped(&mut self, vm: &mut ScriptVm, scope: &mut Scope);
    fn on_before_reload_scoped(&mut self, vm: &mut ScriptVm, scope: &mut Scope);
    fn on_after_new_scoped(&mut self, vm: &mut ScriptVm, scope: &mut Scope);
    fn on_after_reload_scoped(&mut self, vm: &mut ScriptVm, scope: &mut Scope);
}
```

### Migration Mapping

| Old LiveHook Method | New ScriptHook Method |
|---------------------|----------------------|
| `before_apply(cx, apply, index, nodes)` | `on_before_apply(vm, apply, scope, value)` |
| `after_apply(cx, apply, index, nodes)` | `on_after_apply(vm, apply, scope, value)` |
| `after_new_from_doc(cx)` | `on_after_new(vm)` or `on_after_new_scoped(vm, scope)` |
| `after_update_from_doc(cx)` | `on_after_reload(vm)` or `on_after_reload_scoped(vm, scope)` |
| `after_apply_from_doc(cx)` | Override `on_after_dispatch` and check `Apply::New` or `Apply::Reload` |
| `after_new_before_apply(cx)` | `on_before_new(vm)` |
| `apply_value_instance(cx, apply, index, nodes)` | `on_custom_apply(vm, apply, scope, value)` or use `vm.vec_with()` |
| `apply_value_unknown(cx, apply, index, nodes)` | Handle in `on_custom_apply` |
| `skip_apply(cx, apply, index, nodes)` | `on_custom_apply` returning `true` to skip |
| `skip_apply_animator(cx, apply, index, nodes)` | Use `#[apply_default]` attribute instead |

**Key difference:** All hooks receive `&mut ScriptVm` instead of `&mut Cx`. To access `Cx` from a `ScriptVm`, use `vm.with_cx(|cx| ...)` or `vm.with_cx_mut(|cx| ...)`.

---

## 9. Apply Enum Changes

```rust
// OLD
pub enum ApplyFrom {
    NewFromDoc { file_id: LiveFileId },
    UpdateFromDoc { file_id: LiveFileId },
    Over,
    Animate,
    // ...
}

// NEW
pub enum Apply {
    New,
    Reload,
    Animate,
    Eval,
    Default(usize),
}
```

The `Apply` enum is much simpler. `NewFromDoc` → `New`, `UpdateFromDoc` → `Reload`, `Over` → `Eval`.

---

## 10. Scope Changes

```rust
// OLD
pub struct Scope<'a, 'b> {
    pub path: HeapLiveIdPath,   // REMOVED in v2
    pub data: ScopeDataMut<'a>,
    pub props: ScopeDataRef<'b>,
    pub index: usize,
}

// OLD methods:
impl Scope {
    pub fn with_id<F, R>(&mut self, id: LiveId, f: F) -> R;  // REMOVED in v2
    pub fn with_data<T: Any>(v: &'a mut T) -> Self;
    pub fn with_props<T: Any>(w: &'b T) -> Self;
    pub fn empty() -> Self;
    // ...
}

// NEW
pub struct Scope<'a, 'b> {
    pub data: ScopeDataMut<'a>,
    pub props: ScopeDataRef<'b>,
    pub index: usize,
}

// NEW methods (same minus with_id):
impl Scope {
    pub fn with_data<T: Any>(v: &'a mut T) -> Self;
    pub fn with_props<T: Any>(w: &'b T) -> Self;
    pub fn empty() -> Self;
    // ...
}
```

Removed: `path` field and `with_id()` method. All other `Scope` methods are unchanged.

---

## 11. WidgetRef Method Changes

### Widget Finding

```rust
// OLD
let w = self.ui.widget(id!(my_widget));
let w = self.ui.button(id!(my_btn));
let w = self.ui.label(id!(my_label));
let ws = self.ui.widgets(&[id!(item1), id!(item2)]);

// NEW
let w = self.ui.widget(cx, ids!(my_widget));
let w = self.ui.button(cx, ids!(my_btn));
let w = self.ui.label(cx, ids!(my_label));
let ws = self.ui.widgets(cx, &[ids!(item1), ids!(item2)]);

// NEW: flood-fill search
let w = self.ui.widget_flood(cx, ids!(some_widget));
let ws = self.ui.widgets_flood(cx, &[ids!(w1), ids!(w2)]);
```

All typed widget accessors (`button()`, `label()`, `text_input()`, `check_box()`, `slider()`, `drop_down()`, `portal_list()`, etc.) gain a `cx: &Cx` first parameter.

The `id!()` macro is replaced by `ids!()` which returns `&[LiveId]`.

### New Methods on WidgetRef

```rust
// Script-to-widget communication
pub fn script_call(&self, vm: &mut ScriptVm, method: LiveId, args: ScriptValue) -> ScriptAsyncResult;
pub fn script_result(&self, vm: &mut ScriptVm, id: ScriptAsyncId, result: ScriptValue);

// Flood-fill search
pub fn widget_flood(&self, cx: &Cx, path: &[LiveId]) -> WidgetRef;
pub fn widgets_flood(&self, cx: &Cx, paths: &[&[LiveId]]) -> WidgetSet;
```

---

## 12. WidgetFactory Changes

```rust
// OLD
pub trait WidgetFactory {
    fn new(&self, cx: &mut Cx) -> Box<dyn Widget>;
}

// NEW
pub trait WidgetFactory: 'static {
    fn script_new(&self, vm: &mut ScriptVm) -> Box<dyn Widget>;
}
```

### Widget Registration

```rust
// OLD
pub trait LiveRegister {
    fn live_register(cx: &mut Cx);
}

// NEW
pub trait WidgetRegister {
    fn register_widget(vm: &mut ScriptVm) -> ScriptValue;
}
```

---

## 13. WidgetRegistry Changes

```rust
// OLD
#[derive(Default, LiveComponentRegistry)]
pub struct WidgetRegistry {
    pub map: BTreeMap<LiveType, (LiveComponentInfo, Box<dyn WidgetFactory>)>,
}

// NEW
#[derive(Default)]
pub struct WidgetRegistry {
    pub map: BTreeMap<TypeId, (ComponentInfo, Box<dyn WidgetFactory>)>,
}
```

The key type changes from `LiveType` to `TypeId` (std). `LiveComponentInfo` → `ComponentInfo`. No more `LiveComponentRegistry` derive.

---

## 14. app_main! Changes

```rust
// OLD: app_main! internally calls:
$app::new_main(cx)           // Create app
$app::register_main_module(&mut cx)  // Register module
live_design(&mut cx)                 // Register all DSL
app.borrow_mut().update_main(cx)     // On live edit

// NEW: app_main! internally calls:
cx.with_vm(|vm| $app::run(vm))      // Create app via ScriptVm
// No register_main_module
// No live_design (handled inside App::run)
// No update_main on live edit
```

Your `App` struct needs a `run` method:

```rust
impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        // Register all widget modules
        makepad_widgets::script_mod(vm);
        // Register your crate's script modules
        crate::script_mod(vm);
        // Run the VM (creates the app instance)
        vm.run()
    }
}
```

---

## 15. MatchEvent (Unchanged)

The `MatchEvent` trait is **identical** between v1 and v2. All methods have the same signatures:

```rust
pub trait MatchEvent {
    fn handle_startup(&mut self, cx: &mut Cx) {}
    fn handle_shutdown(&mut self, cx: &mut Cx) {}
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {}
    fn handle_action(&mut self, cx: &mut Cx, e: &Action) {}
    fn handle_signal(&mut self, cx: &mut Cx) {}
    fn handle_draw(&mut self, cx: &mut Cx, e: &DrawEvent) {}
    fn handle_timer(&mut self, cx: &mut Cx, e: &TimerEvent) {}
    fn handle_key_down(&mut self, cx: &mut Cx, e: &KeyEvent) {}
    fn handle_key_up(&mut self, cx: &mut Cx, e: &KeyEvent) {}
    fn handle_next_frame(&mut self, cx: &mut Cx, e: &NextFrameEvent) {}
    fn handle_http_response(&mut self, cx: &mut Cx, request_id: LiveId, response: &HttpResponse) {}
    fn handle_http_request_error(&mut self, cx: &mut Cx, request_id: LiveId, err: &HttpError) {}
    fn handle_http_stream(&mut self, cx: &mut Cx, request_id: LiveId, data: &HttpResponse) {}
    // ... etc.
    fn match_event(&mut self, cx: &mut Cx, event: &Event);
}
```

---

## 16. Type Renames

| Old Type | New Type | Notes |
|----------|----------|-------|
| `DrawIcon` | `DrawSvg` | SVG icon drawing |
| `Margin` | `Inset` | Edge insets (padding, margin) |
| `LiveType` | `TypeId` (std) | Type identification |
| `LiveComponentInfo` | `ComponentInfo` | Registry info |
| `LivePtr` | — | Replaced by `ScriptObjectRef` |
| `LiveNode` / `LiveNodeVec` | `ScriptValue` / `ScriptObject` | Data representation |
| `ApplyFrom` | `Apply` | Apply context enum |
| `WidgetCache` | — | Removed (tree handles caching) |
| `HeapLiveIdPath` | — | Removed from Scope and WidgetAction |

---

## 17. apply_over → script_apply_eval!

**Old pattern:**
```rust
// On self
self.apply_over(cx, live!{
    draw_bg: { color: (some_color) }
    visible: (is_visible)
});

// On a child widget
item.apply_over(cx, live!{
    draw_bg: { hover: (hover_val) }
});

// On a draw shader
self.draw_bg.apply_over(cx, live!{ active: (val) });
```

**New pattern:**
```rust
// On self
script_apply_eval!(cx, self, {
    draw_bg +: { color: #(some_color) }
    visible: #(is_visible)
});

// On a child widget
script_apply_eval!(cx, item, {
    draw_bg +: { hover: #(hover_val) }
});

// On a draw shader
script_apply_eval!(cx, self.draw_bg, { active: #(val) });
```

The `#(expr)` syntax interpolates Rust expressions into Splash code. The macro internally calls `cx.with_vm(|vm| target.script_apply_eval(vm, script))`.

---

## 18. apply_value_instance → vec_with / on_custom_apply

The old `apply_value_instance` hook was used to handle child instantiation (especially for templates in PortalList, View). The new system uses `on_custom_apply` or `vm.vec_with()`.

**Old (View children):**
```rust
impl LiveHook for View {
    fn apply_value_instance(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
        let id = nodes[index].id;
        if let Some((_, node)) = self.children.iter_mut().find(|(id2, _)| *id2 == id) {
            node.apply(cx, apply, index, nodes)
        } else {
            self.children.push((id, WidgetRef::new(cx)));
            self.children.last_mut().unwrap().1.apply(cx, apply, index, nodes)
        }
    }
}
```

**New (View children):** Children come from the script object's vector. The `#[derive(Script)]` macro generates code that calls `vm.vec_with()` to iterate over child objects from the script source. Custom widgets override `on_custom_apply` if they need special handling.

---

## 19. Script-Side Widget Communication (New)

### script_call

Widgets can receive method calls from Splash script:

```rust
impl Widget for MyWidget {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        match method {
            live_id!(set_value) => {
                let value = vm.cast::<f64>(args);
                self.value = value;
                ScriptAsyncResult::Return(TRUE)
            }
            live_id!(on_click) => {
                // Trigger the on_click script callback
                let uid = self.widget_uid();
                vm.with_cx_mut(|cx| {
                    cx.widget_to_script_call(
                        uid, NIL, self.source.clone(),
                        self.on_click.clone(), &[]
                    );
                });
                ScriptAsyncResult::Return(TRUE)
            }
            _ => ScriptAsyncResult::MethodNotFound,
        }
    }
}
```

### ScriptFnRef Fields

Widgets can have `ScriptFnRef` fields for inline event handlers defined in Splash:

```rust
#[derive(Script, ScriptHook, Widget)]
pub struct MyWidget {
    #[uid] uid: WidgetUid,
    #[source] source: ScriptObjectRef,
    
    #[live] on_click: ScriptFnRef,    // on_click: ||{ ... } in Splash
    #[live] on_render: ScriptFnRef,   // on_render: ||{ ... } in Splash
    #[live] on_return: ScriptFnRef,   // on_return: || ... in Splash
    #[live] on_startup: ScriptFnRef,  // on_startup: ||{ ... } in Splash
}
```

These are called from Rust via `cx.widget_to_script_call(uid, NIL, source, fn_ref, &args)`.

### script_result

For async operations:

```rust
fn script_result(&mut self, vm: &mut ScriptVm, id: ScriptAsyncId, result: ScriptValue) {
    // Handle the result of an async script operation
}
```

---

## 20. Widget Tree (New)

The old path-based `find_widgets` is replaced by a global widget tree. Widgets register their children, and the tree supports:

- **Within search**: `tree.find_within(uid, path)` — search descendants only
- **Flood search**: `tree.find_flood(uid, path)` — search children first, then expand outward through parents and their subtrees

To participate in the tree, implement `children()` on `WidgetNode`:

```rust
impl WidgetNode for MyWidget {
    fn children(&self, visit: &mut dyn FnMut(LiveId, WidgetRef)) {
        visit(live_id!(header), self.header.clone());
        visit(live_id!(body), self.body.clone());
        for (id, child) in &self.dynamic_children {
            visit(*id, child.clone());
        }
    }
}
```

---

## 21. Animator Changes

**Old:**
```rust
#[derive(Live, LiveHook, Widget)]
pub struct MyWidget {
    #[animator]
    animator: Animator,
}

// Animator methods come from LiveHook trait / generated code
self.animator_handle_event(cx, event);
self.animator_play(cx, ids!(hover.on));
self.animator_in_state(cx, ids!(hover.on));
self.animator_toggle(cx, condition, Animate::Yes, ids!(state.on), ids!(state.off));
```

**New:**
```rust
#[derive(Script, ScriptHook, Widget, Animator)]
pub struct MyWidget {
    #[uid] uid: WidgetUid,
    #[source] source: ScriptObjectRef,
    #[apply_default]
    animator: Animator,
}

// Animator methods come from #[derive(Animator)] — same API
self.animator_handle_event(cx, event);
self.animator_play(cx, ids!(hover.on));
self.animator_in_state(cx, ids!(hover.on));
self.animator_toggle(cx, condition, Animate::Yes, ids!(state.on), ids!(state.off));
```

The runtime API for animators is the same. The only changes are:
- `#[animator]` → `#[apply_default]`
- Add `#[derive(Animator)]` to the derives
- DSL syntax changes (see migration guide)

---

## 22. Async / UiRunner (Unchanged)

The `UiRunner` pattern for async→UI communication is unchanged:

```rust
let runner = self.ui_runner();

// Defer to UI thread
runner.defer(move |me, cx| {
    me.update_something(cx);
});

// Defer with redraw
runner.defer_with_redraw(move |me, cx| {
    me.data = new_data;
});

// Defer async
runner.defer_async(async move |me, cx| {
    let result = fetch_data().await;
    me.apply_result(cx, result);
});
```

---

## 23. Summary: Minimal Migration Checklist

For each widget struct:

1. Change `#[derive(Live, LiveHook, Widget)]` → `#[derive(Script, ScriptHook, Widget, Animator)]` (add `Animator` if using animators)
2. Add `#[uid] uid: WidgetUid` field
3. Add `#[source] source: ScriptObjectRef` field
4. Change `#[animator]` → `#[apply_default]`
5. Change `DrawIcon` → `DrawSvg`
6. Change `Margin` → `Inset`
7. Add `#[live] on_click: ScriptFnRef` (and other event fields as needed)

For each action enum:

8. Change `#[derive(DefaultNone)]` → `#[derive(Default)]` + `#[default]` on None variant

For each `handle_event`:

9. Remove `&scope.path` from `cx.widget_action()` calls
10. Change `id!(name)` → `ids!(name)` for widget lookups
11. Add `cx` parameter to widget lookups: `self.ui.button(cx, ids!(name))`

For each `live_design!`:

12. Change to `script_mod!{}` (see migration-guide.md for DSL syntax)

For registration:

13. Change `live_design(cx: &mut Cx)` → `script_mod(vm: &mut ScriptVm)` everywhere
14. Replace `LiveRegister` impl with `App::run(vm)` method
