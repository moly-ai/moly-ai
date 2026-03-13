use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.Slot = #(Slot::register_widget(vm))
}

/// A wrapper widget whose content can be replaced from Rust.
#[derive(Script, Widget)]
pub struct Slot {
    #[source]
    source: ScriptObjectRef,

    #[wrap]
    wrap: WidgetRef,

    /// The content defined in the DSL to be shown if it hasn't been overridden.
    ///
    /// If overridden, this can still be restored using [Self::restore].
    #[live]
    default: WidgetRef,
}

impl Widget for Slot {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.wrap.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.wrap.handle_event(cx, event, scope)
    }
}

impl ScriptHook for Slot {
    fn on_after_new(&mut self, _vm: &mut ScriptVm) {
        log!(
            "DEBUG Slot on_after_new: default_empty={}, wrap_empty={}",
            self.default.is_empty(),
            self.wrap.is_empty(),
        );
        self.wrap = self.default.clone();
    }
}

impl Slot {
    /// Replace the current widget with a new one.
    pub fn replace(&mut self, widget: WidgetRef) {
        self.wrap = widget;
    }

    /// Restore the default/original widget.
    ///
    /// Same as `self.replace(self.default())`.
    pub fn restore(&mut self) {
        self.wrap = self.default.clone();
    }

    /// Get the current widget.
    pub fn current(&self) -> WidgetRef {
        log!(
            "DEBUG Slot::current: wrap_empty={}, default_empty={}",
            self.wrap.is_empty(),
            self.default.is_empty(),
        );
        self.wrap.clone()
    }

    /// Get the default/original widget.
    pub fn default(&self) -> WidgetRef {
        self.default.clone()
    }
}

impl SlotRef {
    /// See [Slot::replace].
    pub fn replace(&mut self, widget: WidgetRef) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };

        inner.replace(widget);
    }

    /// See [Slot::restore].
    pub fn restore(&mut self) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };

        inner.restore();
    }

    /// See [Slot::current].
    pub fn current(&self) -> WidgetRef {
        let Some(inner) = self.borrow() else {
            log!(
                "DEBUG SlotRef::current: borrow FAILED, self_empty={}",
                self.is_empty()
            );
            return WidgetRef::empty();
        };

        inner.current()
    }

    /// See [Slot::default].
    pub fn default(&self) -> WidgetRef {
        let Some(inner) = self.borrow() else {
            return WidgetRef::empty();
        };

        inner.default()
    }
}
