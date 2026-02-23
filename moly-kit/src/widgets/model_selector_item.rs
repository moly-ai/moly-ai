use crate::aitk::protocol::*;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.ModelSelectorItem = ModelSelectorItem {
        width: Fill
        height: Fit
        padding: Inset { left: 24, right: 16, top: 8, bottom: 8 }
        spacing: 10
        align: Align { x: 0.0, y: 0.5 }

        show_bg: true
        draw_bg +: {
            color: #xF9
            hover: instance(0.0)
            selected: instance(0.0)
            color_hover: instance(#xE9)

            pixel: fn() -> vec4 {
                return mix(self.color self.color_hover self.hover);
            }
        }

        cursor: MouseCursor.Hand

        animator: {
            hover: {
                default: @off
                off: AnimatorState {
                    from: { all: Forward { duration: 0.2 } }
                    apply: {
                        draw_bg +: { hover: 0.0 }
                    }
                }

                on: AnimatorState {
                    from: { all: Snap }
                    apply: {
                        draw_bg +: { hover: 1.0 }
                    }
                }
            }
        }

        label := Label {
            width: Fill
            draw_text +: {
                text_style: theme.font_regular { font_size: 11 }
                color: #000
            }
        }

        icon_tick_view := View {
            width: Fit, height: Fit
            visible: false
            icon_tick := Label {
                width: Fit, height: Fit
                align: Align { x: 1.0, y: 0.5 }
                text: "\u{f00c}" // fa-check
                draw_text +: {
                    text_style: theme.font_icons {
                        font_size: 12.
                    }
                    color: #000
                }
            }
        }
    }
}

/// Action dispatched when a bot is selected in the model selector item.
#[derive(Clone, Default, Debug)]
pub enum ModelSelectorItemAction {
    BotSelected(BotId),
    #[default]
    None,
}

#[derive(Script, ScriptHook, Widget, Animator)]
pub struct ModelSelectorItem {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,

    #[rust]
    bot: Option<Bot>,

    #[rust]
    selected: bool,

    #[apply_default]
    animator: Animator,
}

impl Widget for ModelSelectorItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        // Handle tap on the entire item
        match event.hits_with_capture_overload(cx, self.view.area(), true) {
            Hit::FingerDown(_) => {
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerUp(fe) => {
                self.animator_play(cx, ids!(hover.off));
                if fe.was_tap() {
                    if let Some(bot) = &self.bot {
                        cx.widget_action(
                            self.widget_uid(),
                            ModelSelectorItemAction::BotSelected(bot.id.clone()),
                        );
                    }
                }
            }
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(bot) = &self.bot {
            self.label(cx, ids!(label)).set_text(cx, &bot.name);

            // Show tick icon if this bot is selected
            self.view(cx, ids!(icon_tick_view))
                .set_visible(cx, self.selected);
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl ModelSelectorItemRef {
    /// Sets the bot to display in this selector item.
    pub fn set_bot(&mut self, bot: Bot) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.bot = Some(bot);
        }
    }

    /// Sets whether this selector item is currently selected.
    pub fn set_selected(&mut self, selected: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.selected = selected;
        }
    }
}
