use super::chat_history_card::ChatHistoryCardAction;
use crate::data::chats::chat::ChatId;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_DELETE = crate_resource("self://resources/icons/delete.svg")
    let ICON_EDIT = crate_resource("self://resources/icons/edit.svg")

    mod.widgets.ChatHistoryCardOptions = #(ChatHistoryCardOptions::register_widget(vm)) {
        width: Fit
        height: Fit
        flow: Overlay

        options_content := RoundedView {
            width: Fit
            height: Fit
            flow: Down

            draw_bg +: {
                color: #fff
                border_size: 1.0
                border_color: #D0D5DD
                border_radius: 2.
            }

            edit_chat_name := MolyButton {
                width: Fit
                height: Fit
                padding: Inset {top: 12 right: 12 bottom: 12 left: 12}

                draw_bg +: {
                    border_size: 0
                    border_radius: 0
                }

                icon_walk: {width: 12 height: 12}
                draw_icon +: {
                    svg_file: (ICON_EDIT)
                    get_color: fn() -> vec4 {
                        return #000;
                    }
                }

                draw_text +: {
                    text_style: REGULAR_FONT {font_size: 9}
                    get_color: fn() -> vec4 {
                        return #000;
                    }
                }

                text: "Edit Chat Name"
            }

            delete_chat := MolyButton {
                width: Fill
                height: Fit
                padding: Inset {top: 12 right: 12 bottom: 12 left: 12}
                align: Align {x: 0.0 y: 0.5}

                draw_bg +: {
                    border_size: 0
                    border_radius: 0
                }

                icon_walk: {width: 12 height: 12}
                draw_icon +: {
                    svg_file: (ICON_DELETE)
                    get_color: fn() -> vec4 {
                        return #xB42318;
                    }
                }

                draw_text +: {
                    text_style: REGULAR_FONT {font_size: 9}
                    get_color: fn() -> vec4 {
                        return #xB42318;
                    }
                }

                text: "Delete Chat"
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ChatHistoryCardOptions {
    #[deref]
    view: View,
    #[rust]
    chat_id: ChatId,
}

impl Widget for ChatHistoryCardOptions {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ChatHistoryCardOptions {
    pub fn selected(&mut self, cx: &mut Cx, chat_id: ChatId) {
        self.chat_id = chat_id;
        self.redraw(cx);
    }
}

impl ChatHistoryCardOptionsRef {
    pub fn selected(&mut self, cx: &mut Cx, chat_id: ChatId) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.selected(cx, chat_id);
    }
}

impl WidgetMatchEvent for ChatHistoryCardOptions {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(cx, ids!(delete_chat)).clicked(actions) {
            cx.action(ChatHistoryCardAction::MenuClosed(self.chat_id));

            cx.action(ChatHistoryCardAction::DeleteChatOptionSelected(
                self.chat_id,
            ));
        }

        if self.button(cx, ids!(edit_chat_name)).clicked(actions) {
            cx.action(ChatHistoryCardAction::MenuClosed(self.chat_id));

            cx.action(ChatHistoryCardAction::ActivateTitleEdition(self.chat_id));
        }
    }
}
