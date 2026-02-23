use makepad_widgets::*;

use crate::shared::actions::ChatAction;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_NEW_CHAT = crate_resource("self://resources/icons/new_chat.svg")

    let HeadingLabel = Label {
        margin: Inset {left: 4 bottom: 4}
        draw_text +: {
            text_style: theme.font_bold {font_size: 10.5}
            color: #3
        }
    }

    let NoAgentsWarning = Label {
        margin: Inset {left: 4 bottom: 4}
        width: Fill
        draw_text +: {
            text_style: theme.font_regular {font_size: 8.5}
            color: #3
        }
    }

    // TODO: TogglePanel was removed from new Makepad. This is a simplified
    // replacement that just shows the chat history directly in a View.
    mod.widgets.ChatHistoryPanel = #(ChatHistoryPanel::register_widget(vm)) {
        width: Fill height: Fill
        flow: Overlay

        View {
            width: Fill height: Fill
            flow: Right

            View {
                width: Fill height: Fill
                ChatHistory {
                    margin: Inset {top: 80}
                }
            }

            right_border := View {
                width: 1.6 height: Fill
                margin: Inset {top: 15 bottom: 15}
                show_bg: true
                draw_bg +: {
                    color: #xeaeaea
                }
            }
        }

        View {
            width: Fill height: Fit
            align: Align {x: 1.0 y: 0.0}
            padding: Inset {top: 10 right: 10}

            new_chat_button := MolyButton {
                width: Fit
                height: Fit
                icon_walk +: {
                    margin: Inset {top: -1}
                    width: 21 height: 21
                }
                draw_icon +: {
                    svg: ICON_NEW_CHAT
                    get_color: fn() -> vec4 {
                        return #x475467
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ChatHistoryPanel {
    #[deref]
    deref: View,
}

impl Widget for ChatHistoryPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ChatHistoryPanel {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(cx, ids!(new_chat_button)).clicked(&actions) {
            cx.action(ChatAction::StartWithoutEntity);
            self.redraw(cx);
        }
    }
}
