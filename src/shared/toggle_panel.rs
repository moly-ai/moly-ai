use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_CLOSE_LEFT_PANEL =
        crate_resource("self:resources/icons/close_left_panel.svg")
    let ICON_OPEN_LEFT_PANEL =
        crate_resource("self:resources/icons/open_left_panel.svg")

    mod.widgets.MolyTogglePanelBase =
        #(MolyTogglePanel::register_widget(vm))
    mod.widgets.MolyTogglePanel =
        set_type_default() do mod.widgets.MolyTogglePanelBase {
        flow: Overlay
        width: 300
        height: Fill

        open_content := View {
            width: Fill
            height: Fill
        }

        persistent_content := View {
            height: Fit
            width: Fill

            default := View {
                height: Fit
                width: Fill
                padding: Inset { top: 58 left: 15 right: 15 }
                spacing: 10

                before := View {
                    height: Fit
                    width: Fit
                    spacing: 10
                }

                close := TogglePanelButton {
                    draw_icon +: {
                        svg: ICON_CLOSE_LEFT_PANEL
                    }
                }

                open := TogglePanelButton {
                    visible: false
                    draw_icon +: {
                        svg: ICON_OPEN_LEFT_PANEL
                    }
                }

                after := View {
                    height: Fit
                    width: Fit
                    spacing: 10
                }
            }
        }

        animator: Animator {
            panel: {
                default: @open
                open: AnimatorState {
                    from: { all: Forward { duration: 0.3 } }
                    ease: ExpDecay { d1: 0.80 d2: 0.97 }
                    apply: { panel_progress: 1.0 }
                }
                close: AnimatorState {
                    from: { all: Forward { duration: 0.3 } }
                    ease: ExpDecay { d1: 0.80 d2: 0.97 }
                    apply: { panel_progress: 0.0 }
                }
            }
        }
    }
}

/// A toggleable side panel that can be expanded and collapsed.
///
/// Animates width between `close_size` and `open_size`. Provides named
/// slots (`before`, `after`) in `persistent_content` for additional
/// buttons that remain visible in both states, alongside the built-in
/// `open` / `close` toggle buttons.
#[derive(Script, ScriptHook, Widget, Animator)]
pub struct MolyTogglePanel {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,

    /// Animated progress (0.0 = closed, 1.0 = open). Driven by the
    /// `panel` animator track.
    #[live]
    panel_progress: f32,

    /// Width when fully open.
    #[live(300.0)]
    open_size: f32,

    /// Width when fully closed.
    #[live(110.0)]
    close_size: f32,

    #[apply_default]
    animator: Animator,
}

impl Widget for MolyTogglePanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        if let Event::Actions(actions) = event {
            let open_btn = self.button(cx, ids!(open));
            let close_btn = self.button(cx, ids!(close));

            if open_btn.clicked(actions) {
                self.set_open(cx, true);
            }

            if close_btn.clicked(actions) {
                self.set_open(cx, false);
            }
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let size_range = self.open_size - self.close_size;
        let size = self.close_size + size_range * self.panel_progress;
        let walk = Walk {
            width: Size::Fixed(size as f64),
            ..walk
        };
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MolyTogglePanel {
    /// Returns whether the panel is currently in the open state.
    pub fn is_open(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, ids!(panel.open))
    }

    /// Toggles the panel open or closed with animation.
    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        let open_btn = self.button(cx, ids!(open));
        let close_btn = self.button(cx, ids!(close));

        if open {
            open_btn.set_visible(cx, false);
            close_btn.set_visible(cx, true);
            self.animator_play(cx, ids!(panel.open));
        } else {
            close_btn.set_visible(cx, false);
            open_btn.set_visible(cx, true);
            self.animator_play(cx, ids!(panel.close));
        }
    }
}

impl MolyTogglePanelRef {
    /// Returns whether the panel is currently open.
    pub fn is_open(&self, cx: &Cx) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_open(cx)
        } else {
            false
        }
    }

    /// Toggles the panel open or closed with animation.
    pub fn set_open(&self, cx: &mut Cx, open: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_open(cx, open);
        }
    }
}
