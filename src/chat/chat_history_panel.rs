use makepad_widgets::*;

use crate::shared::actions::ChatAction;
use crate::shared::toggle_panel::MolyTogglePanel;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_NEW_CHAT = crate_resource("self://resources/icons/new_chat.svg")

    mod.widgets.ChatHistoryPanelBase = #(ChatHistoryPanel::register_widget(vm))
    mod.widgets.ChatHistoryPanel =
        set_type_default() do mod.widgets.ChatHistoryPanelBase {
        ..MolyTogglePanel

        open_content +: {
            ChatHistory {
                margin: Inset {top: 80}
            }

            right_border := SolidView {
                width: 1.6 height: Fill
                margin: Inset {top: 15 bottom: 15}
                draw_bg +: {
                    color: #xeaeaea
                }
            }
        }

        persistent_content +: {
            default +: {
                margin: Inset { left: -10 }
                after +: {
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
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ChatHistoryPanel {
    #[deref]
    deref: MolyTogglePanel,
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
