use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    let VerticalFiller = View {
        width: Fill
        height: 1
    }

    let Bar = View {
        width: Fill
        height: 17
        show_bg: true
        draw_bg +: {
            dither: instance(0.1)

            get_color: fn() -> vec4 {
                return mix(
                    #x9CADBC
                    #xB0CBC6
                    self.pos.x + self.dither
                )
            }

            pixel: fn() -> vec4 {
                return Pal.premul(self.get_color())
            }
        }
    }

    mod.widgets.MessageLoadingBase = #(MessageLoading::register_widget(vm))
    mod.widgets.MessageLoading = set_type_default() do mod.widgets.MessageLoadingBase {
        width: Fill
        height: Fit

        flow: Down
        spacing: 4

        line1 := Bar {}
        line2 := Bar {}
        View {
            width: Fill
            height: 16
            line3 := Bar {}
            VerticalFiller {}
        }

        animator: Animator {
            line1: {
                default: @start
                start: AnimatorState {
                    redraw: true
                    from: { all: Forward { duration: 0.33 } }
                    apply: { dither1: 0.1 }
                }
                run: AnimatorState {
                    redraw: true
                    from: { all: Forward { duration: 0.33 } }
                    apply: { dither1: 0.9 }
                }
            }

            line2: {
                default: @start
                start: AnimatorState {
                    redraw: true
                    from: { all: Forward { duration: 0.33 } }
                    apply: { dither2: 0.1 }
                }
                run: AnimatorState {
                    redraw: true
                    from: { all: Forward { duration: 0.33 } }
                    apply: { dither2: 0.9 }
                }
            }

            line3: {
                default: @start
                start: AnimatorState {
                    redraw: true
                    from: { all: Forward { duration: 0.33 } }
                    apply: { dither3: 0.1 }
                }
                run: AnimatorState {
                    redraw: true
                    from: { all: Forward { duration: 0.33 } }
                    apply: { dither3: 0.9 }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget, Animator)]
pub struct MessageLoading {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,

    #[apply_default]
    animator: Animator,

    #[live]
    dither1: f64,

    #[live]
    dither2: f64,

    #[live]
    dither3: f64,

    #[rust]
    timer: Timer,

    #[rust]
    current_animated_bar: usize,
}

impl Widget for MessageLoading {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.timer.is_event(event).is_some() {
            self.update_animation(cx);
        }
        if self.animator_handle_event(cx, event).must_redraw() {
            self.apply_dithers(cx);
            self.redraw(cx);
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MessageLoading {
    fn apply_dithers(&mut self, cx: &mut Cx) {
        let d1 = self.dither1;
        let d2 = self.dither2;
        let d3 = self.dither3;
        let mut line1 = self.view(cx, ids!(line1));
        script_apply_eval!(cx, line1, { draw_bg +: { dither: #(d1) } });
        let mut line2 = self.view(cx, ids!(line2));
        script_apply_eval!(cx, line2, { draw_bg +: { dither: #(d2) } });
        let mut line3 = self.view(cx, ids!(line3));
        script_apply_eval!(cx, line3, { draw_bg +: { dither: #(d3) } });
    }

    fn update_animation(&mut self, cx: &mut Cx) {
        self.current_animated_bar = (self.current_animated_bar + 1) % 3;

        match self.current_animated_bar {
            0 => {
                self.animator_play(cx, ids!(line1.run));
                self.animator_play(cx, ids!(line3.start));
            }
            1 => {
                self.animator_play(cx, ids!(line1.start));
                self.animator_play(cx, ids!(line2.run));
            }
            2 => {
                self.animator_play(cx, ids!(line2.start));
                self.animator_play(cx, ids!(line3.run));
            }
            _ => unreachable!(),
        };

        self.timer = cx.start_timeout(0.33);
    }
}

impl MessageLoadingRef {
    /// Starts the loading animation if not already running.
    pub fn animate(&mut self, cx: &mut Cx) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        if inner.timer.is_empty() {
            inner.timer = cx.start_timeout(0.2);
        }
    }
}
