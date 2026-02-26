use crate::aitk::protocol::*;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let LoadingBall = CircleView {
        width: 20.0
        height: 20.0
        margin: 0.0
        padding: 0.0
        draw_bg +: {
            border_radius: 10.0
        }
    }

    let PulsingBalls = View {
        width: Fit
        height: Fit
        align: Align { x: 0.0, y: 0.5 }
        spacing: 0.0
        flow: Right
        padding: 0
        margin: 0

        ball1 := LoadingBall {
            margin: 0.0
            padding: 0.0
            draw_bg +: {
                color: #xE55E50
            }
        }

        ball2 := LoadingBall {
            margin: 0.0
            padding: 0.0
            draw_bg +: {
                color: #x4D9CC0
            }
        }
    }

    let Collapse = RoundedView {
        width: Fill, height: Fit
        padding: Inset { top: 8, right: 12, bottom: 8, left: 12 }
        margin: 2
        cursor: Hand
        flow: Right
        align: Align { x: 0.0, y: 0.5 }

        draw_bg +: {
            border_radius: 2.5
            color: #xf7f7f7
        }

        thinking_title := Label {
            text: "Thinking..."
            draw_text +: {
                text_style: theme.font_italic {
                    font_size: 10.5
                }
                color: #000
            }
        }

        View { width: Fill, height: Fill }
        balls := PulsingBalls {}
    }

    let Content = RoundedView {
        width: Fill
        height: Fit

        flow: Right
        spacing: 12
        height: 0
        padding: Inset { left: 20, right: 8, top: 10, bottom: 15 }

        thinking_text := MessageMarkdown {
            width: Fill, height: Fit
            font_size: 10.5
        }
    }

    mod.widgets.MessageThinkingBlockBase = #(MessageThinkingBlock::register_widget(vm))
    mod.widgets.MessageThinkingBlock =
        set_type_default() do mod.widgets.MessageThinkingBlockBase {
        width: Fill
        height: Fit
        flow: Down
        show_bg: true
        padding: Inset { top: 5, bottom: 5, left: 5, right: 5 }

        inner := RoundedShadowView {
            width: 200, height: Fit
            flow: Down
            padding: 0
            draw_bg +: {
                color: #xf7f7f7
                border_radius: 4.5
                shadow_color: uniform(#0001)
                shadow_radius: 9.0
                shadow_offset: vec2(0.0 -1.0)
            }
            collapse := Collapse {}
            content := Content {}
        }

        animator: Animator {
            ball1: {
                default: @start
                start: AnimatorState {
                    redraw: true
                    from: { all: Forward { duration: 0.66 } }
                    apply: {
                        inner: { collapse: { balls: { ball1: {
                            width: 10.0
                            height: 10.0
                            draw_bg: {
                                border_radius: 5.0
                            }
                        } } } }
                    }
                }
                run: AnimatorState {
                    redraw: true
                    from: { all: Forward { duration: 0.66 } }
                    apply: {
                        inner: { collapse: { balls: { ball1: {
                            width: 20.0
                            height: 20.0
                            draw_bg: {
                                border_radius: 10.0
                            }
                        } } } }
                    }
                }
            }

            ball2: {
                default: @start
                start: AnimatorState {
                    redraw: true
                    from: { all: Forward { duration: 0.66 } }
                    apply: {
                        inner: { collapse: { balls: { ball2: {
                            width: 10.0
                            height: 10.0
                            draw_bg: {
                                border_radius: 5.0
                            }
                        } } } }
                    }
                }
                run: AnimatorState {
                    redraw: true
                    from: { all: Forward { duration: 0.66 } }
                    apply: {
                        inner: { collapse: { balls: { ball2: {
                            width: 20.0
                            height: 20.0
                            draw_bg: {
                                border_radius: 10.0
                            }
                        } } } }
                    }
                }
            }
        }
    }
}

const ANIMATION_SPEED_RUST: f64 = 0.33;

#[derive(Script, ScriptHook, Widget, Animator)]
pub struct MessageThinkingBlock {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,

    #[apply_default]
    animator: Animator,

    #[rust]
    timer: Timer,

    #[rust]
    is_expanded: bool,

    #[rust]
    is_visible: bool,

    #[rust]
    should_animate: bool,

    #[rust]
    current_animated_ball: usize,
}

impl Widget for MessageThinkingBlock {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.timer.is_event(event).is_some() {
            self.update_animation(cx);
        }

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.is_visible {
            self.view.draw_walk(cx, scope, walk)
        } else {
            DrawStep::done()
        }
    }
}

impl WidgetMatchEvent for MessageThinkingBlock {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if let Some(_evt) = self.view(cx, ids!(collapse)).finger_up(&actions) {
            self.toggle_collapse(cx);
        }
    }
}

impl MessageThinkingBlock {
    pub fn update_animation(&mut self, cx: &mut Cx) {
        self.current_animated_ball = (self.current_animated_ball + 1) % 2;

        match self.current_animated_ball {
            0 => {
                self.animator_play(cx, ids!(ball1.run));
                self.animator_play(cx, ids!(ball2.start));
            }
            1 => {
                self.animator_play(cx, ids!(ball1.start));
                self.animator_play(cx, ids!(ball2.run));
            }
            _ => unreachable!(),
        }

        self.timer = cx.start_timeout(ANIMATION_SPEED_RUST);
    }

    pub fn set_content(
        &mut self,
        cx: &mut Cx,
        content: &MessageContent,
        metadata: &MessageMetadata,
    ) {
        let content_reasoning = content.reasoning.as_str();
        let content_text = content.text.as_str();

        self.is_visible = !content_reasoning.is_empty();

        self.markdown(cx, ids!(thinking_text))
            .set_text(cx, content_reasoning);

        let is_reasoning_ongoing =
            !content_reasoning.is_empty() && content_text.is_empty() && metadata.is_writing();

        if is_reasoning_ongoing {
            if self.timer.is_empty() {
                self.should_animate = true;
                self.view(cx, ids!(balls)).set_visible(cx, true);
                self.update_animation(cx);
            }
        } else {
            self.should_animate = false;
            self.view(cx, ids!(balls)).set_visible(cx, false);
            self.animator_play(cx, ids!(ball1.start));
            self.animator_play(cx, ids!(ball2.start));
            self.view(cx, ids!(thinking_title)).set_text(
                cx,
                &format!(
                    "Thought for {:0.2} seconds",
                    metadata.reasoning_time_taken_seconds()
                ),
            );
        }
    }

    fn toggle_collapse(&mut self, cx: &mut Cx) {
        self.is_expanded = !self.is_expanded;

        if self.is_expanded {
            let mut content = self.view(cx, ids!(content));
            script_apply_eval!(cx, content, { height: Fit });
            let mut inner = self.view(cx, ids!(inner));
            script_apply_eval!(cx, inner, { width: Fill });
            let mut collapse = self.view(cx, ids!(collapse));
            script_apply_eval!(cx, collapse, {
                draw_bg +: { color: #xf0f0f0 }
            });
        } else {
            let mut content = self.view(cx, ids!(content));
            script_apply_eval!(cx, content, { height: 0.0 });
            let mut inner = self.view(cx, ids!(inner));
            script_apply_eval!(cx, inner, { width: 200 });
            let mut collapse = self.view(cx, ids!(collapse));
            script_apply_eval!(cx, collapse, {
                draw_bg +: { color: #xf7f7f7 }
            });
            self.should_animate = false;
        }
        self.redraw(cx);
    }
}
