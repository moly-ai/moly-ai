//! Copy of the original modal from the main Moly app which draws its content
//! over the whole app (from its root).

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.MolyModal = MolyModal {{MolyModal}} {
        width: Fill
        height: Fill
        flow: Overlay
        align: Align { x: 0.5, y: 0.5 }

        draw_bg +: {
            pixel: fn() -> vec4 {
                return vec4(0. 0. 0. 0.0)
            }
        }

        bg_view := View {
            width: Fill
            height: Fill
            show_bg: true
            draw_bg +: {
                pixel: fn() -> vec4 {
                    return vec4(0. 0. 0. 0.7)
                }
            }
        }

        content: View {
            flow: Overlay
            width: Fit
            height: Fit
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum MolyModalAction {
    #[default]
    None,
    Dismissed,
}

#[derive(Script, Widget)]
pub struct MolyModal {
    #[source]
    source: ScriptObjectRef,
    #[live]
    #[find]
    content: View,
    #[live]
    #[area]
    bg_view: View,

    #[rust]
    draw_list: Option<DrawList2d>,

    #[live]
    draw_bg: DrawQuad,
    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,

    #[live(true)]
    dismiss_on_focus_lost: bool,

    #[rust]
    opened: bool,

    #[rust]
    desired_popup_position: Option<DVec2>,
}

impl ScriptHook for MolyModal {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        self.draw_list = Some(DrawList2d::script_new(vm));
    }

    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            if let Some(draw_list) = &self.draw_list {
                draw_list.redraw(cx);
            }
        });
    }
}

impl Widget for MolyModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.opened {
            return;
        }

        // When passing down events we need to suspend the sweep lock
        // because regular View instances won't respond to events if
        // the sweep lock is active.
        cx.sweep_unlock(self.draw_bg.area());
        self.content.handle_event(cx, event, scope);
        cx.sweep_lock(self.draw_bg.area());

        if self.dismiss_on_focus_lost {
            let content_rec = self.content.area().rect(cx);
            if let Hit::FingerUp(fe) =
                event.hits_with_sweep_area(cx, self.draw_bg.area(), self.draw_bg.area())
            {
                if !content_rec.contains(fe.abs) {
                    let widget_uid = self.content.widget_uid();
                    cx.widget_action(widget_uid, MolyModalAction::Dismissed);
                    self.close(cx);
                }
            }
        }

        self.ui_runner().handle(cx, event, scope, self);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let draw_list = self.draw_list.as_mut().unwrap();
        draw_list.begin_overlay_reuse(cx);

        cx.begin_root_turtle_for_pass(self.layout);
        self.draw_bg.begin(cx, self.walk, self.layout);

        if self.opened {
            let _ = self
                .bg_view
                .draw_walk(cx, scope, walk.with_abs_pos(DVec2 { x: 0., y: 0. }));
            self.content.draw_all(cx, scope);
        }

        self.draw_bg.end(cx);

        cx.end_pass_sized_turtle();
        self.draw_list.as_mut().unwrap().end(cx);

        if let Some(pos) = self.desired_popup_position.take() {
            self.ui_runner().defer(move |me, cx, _| {
                me.correct_popup_position(cx, pos);
            });
        }

        DrawStep::done()
    }
}

impl MolyModal {
    #[deprecated(note = "Use open_as_dialog or open_as_popup instead")]
    pub fn open(&mut self, cx: &mut Cx) {
        self.opened = true;
        self.draw_bg.redraw(cx);
        cx.sweep_lock(self.draw_bg.area());
    }

    pub fn open_as_dialog(&mut self, cx: &mut Cx) {
        script_apply_eval!(cx, self, {
            align: Align { x: 0.5, y: 0.5 }
            content: {
                margin: 0,
            }
            bg_view: {
                visible: true
            }
        });

        #[allow(deprecated)]
        self.open(cx);
    }

    pub fn open_as_popup(&mut self, cx: &mut Cx, pos: DVec2) {
        self.desired_popup_position = Some(pos);
        let screen_size = cx.display_context.screen_size;

        script_apply_eval!(cx, self, {
            align: Align { x: 0.0, y: 0.0 }
            content: {
                margin: Inset {
                    left: #(screen_size.x),
                    top: #(screen_size.y)
                }
            }
            bg_view: {
                visible: false
            }
        });

        #[allow(deprecated)]
        self.open(cx);
    }

    pub fn close(&mut self, cx: &mut Cx) {
        self.opened = false;
        self.draw_bg.redraw(cx);
        cx.sweep_unlock(self.draw_bg.area())
    }

    /// Returns whether this modal was dismissed by the given actions.
    pub fn dismissed(&self, actions: &Actions) -> bool {
        matches!(
            actions.find_widget_action(self.widget_uid()).cast(),
            MolyModalAction::Dismissed
        )
    }

    /// Returns whether this modal is currently open.
    pub fn is_open(&self) -> bool {
        self.opened
    }

    fn correct_popup_position(&mut self, cx: &mut Cx, pos: DVec2) {
        let content_size = self.content.area().rect(cx).size;
        let screen_size = cx.display_context.screen_size;

        let pos_x = if pos.x + content_size.x > screen_size.x {
            screen_size.x - content_size.x - 10.0
        } else {
            pos.x
        };

        let pos_y = if pos.y + content_size.y > screen_size.y {
            screen_size.y - content_size.y - 10.0
        } else {
            pos.y
        };

        script_apply_eval!(cx, self, {
            content: {
                margin: Inset { left: #(pos_x), top: #(pos_y) }
            }
        });

        self.redraw(cx);
    }
}

impl MolyModalRef {
    #[deprecated(note = "Use open_as_dialog or open_as_popup instead")]
    pub fn open(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            #[allow(deprecated)]
            inner.open(cx);
        }
    }

    /// Opens the modal as a centered dialog.
    pub fn open_as_dialog(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open_as_dialog(cx);
        }
    }

    /// Opens the modal as a popup at the given position.
    pub fn open_as_popup(&self, cx: &mut Cx, pos: DVec2) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open_as_popup(cx, pos);
        }
    }

    /// Closes the modal.
    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.close(cx);
        }
    }

    /// Returns whether this modal was dismissed by the given actions.
    pub fn dismissed(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            inner.dismissed(actions)
        } else {
            false
        }
    }

    /// Returns whether this modal is currently open.
    pub fn is_open(&self) -> bool {
        self.borrow().map_or(false, |inner| inner.is_open())
    }
}
