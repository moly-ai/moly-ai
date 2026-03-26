use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.TooltipBase = #(Tooltip::register_widget(vm))

    mod.widgets.Tooltip = TooltipBase {
        width: Fill
        height: Fill

        flow: Overlay
        align: Align { x: 0.0 y: 0.0 }

        draw_bg +: {
            pixel: fn() -> vec4 {
                return vec4(0. 0. 0. 0.0)
            }
        }

        flow: Overlay
        width: Fit
        height: Fit

        RoundedView {
            width: Fit
            height: Fit

            padding: 16

            draw_bg +: {
                color: #fff
                border_size: 1.0
                border_color: #D0D5DD
                radius: 2.
            }

            tooltip_label := Label {
                width: 270
                draw_text +: {
                    text_style: REGULAR_FONT { font_size: 9 }
                    color: #000
                }
            }
        }
    }
}

#[derive(Script, Widget)]
pub struct Tooltip {
    #[deref]
    view: View,

    #[rust]
    draw_list: Option<DrawList2d>,

    #[live]
    draw_bg: DrawQuad,

    #[rust]
    opened: bool,

    #[rust]
    tooltip_pos: Vec2d,
}

impl ScriptHook for Tooltip {
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

impl Widget for Tooltip {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, _walk: Walk) -> DrawStep {
        let draw_list = self.draw_list.as_mut().unwrap();
        draw_list.begin_overlay_reuse(cx);

        let size = cx.current_pass_size();
        cx.begin_root_turtle(size, self.view.layout);
        self.draw_bg.begin(cx, self.view.walk, self.view.layout);

        if self.opened {
            let content_walk = self.view.walk(cx).with_abs_pos(self.tooltip_pos);
            self.view.draw_walk_all(cx, scope, content_walk);
        }

        self.draw_bg.end(cx);

        cx.end_pass_sized_turtle();
        self.draw_list.as_mut().unwrap().end(cx);

        DrawStep::done()
    }

    fn set_text(&mut self, cx: &mut Cx, text: &str) {
        self.label(cx, ids!(tooltip_label)).set_text(cx, text);
    }
}

impl Tooltip {
    /// Sets the tooltip display position.
    pub fn set_pos(&mut self, _cx: &mut Cx, pos: DVec2) {
        self.tooltip_pos = Vec2d { x: pos.x, y: pos.y };
    }

    /// Shows the tooltip.
    pub fn show(&mut self, cx: &mut Cx) {
        self.opened = true;
        self.redraw(cx);
    }

    /// Shows the tooltip with the given text at the given position.
    pub fn show_with_options(&mut self, cx: &mut Cx, pos: DVec2, text: &str) {
        self.set_text(cx, text);
        self.set_pos(cx, pos);
        self.show(cx);
    }

    /// Hides the tooltip.
    pub fn hide(&mut self, cx: &mut Cx) {
        self.opened = false;
        self.redraw(cx);
    }
}

#[allow(dead_code)]
impl TooltipRef {
    /// Sets the tooltip text.
    pub fn set_text(&mut self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_text(cx, text);
        }
    }

    /// Sets the tooltip display position.
    pub fn set_pos(&mut self, cx: &mut Cx, pos: DVec2) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_pos(cx, pos);
        }
    }

    /// Shows the tooltip.
    pub fn show(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show(cx);
        }
    }

    /// Shows the tooltip with the given text at the given position.
    pub fn show_with_options(&mut self, cx: &mut Cx, pos: DVec2, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show_with_options(cx, pos, text);
        }
    }

    /// Hides the tooltip.
    pub fn hide(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hide(cx);
        }
    }
}
