use crate::{
    data::{chats::chat::ChatId, store::Store},
    shared::{actions::ChatAction, utils::human_readable_name},
};

use makepad_widgets::*;
use moly_kit::prelude::*;

use super::delete_chat_modal::DeleteChatModalWidgetExt;
use super::{
    chat_history_card_options::ChatHistoryCardOptionsWidgetExt,
    delete_chat_modal::DeleteChatModalAction,
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_DELETE = crate_resource("self://resources/icons/delete.svg")

    let EditTextInput = MolyTextInput {
        width: Fill
        height: Fit
        padding: 6
        empty_text: ""

        draw_text +: {
            text_style: REGULAR_FONT {font_size: 10}

            prompt_enabled: instance(0.0)
            get_color: fn() -> vec4 {
                return #000;
            }
        }
    }

    let EditActionButton = MolyButton {
        width: 56
        height: 31
        spacing: 6

        draw_bg +: { color: #x099250 }

        draw_text +: {
            text_style: REGULAR_FONT {font_size: 9}
            get_color: fn() -> vec4 {
                return #fff;
            }
        }
    }

    let SaveButton = EditActionButton {
        text: "Save"
    }

    let CancelButton = EditActionButton {
        draw_bg +: {
            border_color_1: #xD0D5DD border_size: 1.0 color: #fff
        }

        draw_text +: {
            text_style: REGULAR_FONT {font_size: 9}
            get_color: fn() -> vec4 {
                return #000;
            }
        }
        text: "Cancel"
    }

    mod.widgets.ChatHistoryCardBase = #(ChatHistoryCard::register_widget(vm))
    mod.widgets.ChatHistoryCard = set_type_default() do mod.widgets.ChatHistoryCardBase {
        flow: Overlay
        width: Fill
        height: 56

        selected_bg := RoundedInnerShadowView {
            width: Fill
            height: Fill
            padding: Inset {left: 8 right: 8}

            show_bg: true

            draw_bg +: {
                border_radius: 5.0
                shadow_color: #x47546722
                shadow_radius: 25.0
                shadow_offset: vec2(-2.0 1.0)
                border_color: #xD0D5DD
            }
        }

        content := RoundedView {
            width: Fill
            height: Fill
            flow: Right
            padding: Inset {left: 8 right: 8}
            spacing: 6

            cursor: MouseCursor.Hand
            show_bg: true
            draw_bg +: {
                down: instance(0.0)
                color: #x0000
                border_size: 0
                border_radius: 5
            }

            View {
                width: Fill
                height: Fill
                flow: Down
                align: Align {y: 0.5 x: 0.0}
                spacing: 3
                padding: Inset {left: 6 top: 10 bottom: 6}

                View {
                    width: Fill height: Fit
                    spacing: 8
                    model_or_agent_name_label := Label {
                        width: Fit
                        height: Fit
                        padding: 0
                        draw_text +: {
                            text_style: BOLD_FONT {font_size: 8.2}
                            color: #x475467
                        }
                    }

                    unread_message_badge := RoundedView {
                        visible: false
                        width: 12 height: 12
                        show_bg: true
                        draw_bg +: {
                            border_radius: 3.0
                            color: #xe81313
                        }
                    }
                }

                View {
                    width: Fill
                    height: Fill
                    flow: Right
                    spacing: 5
                    padding: Inset {top: 2 bottom: 2}
                    align: Align {y: 0.5}

                    View {
                        width: Fill
                        height: Fill
                        flow: Down
                        align: Align {y: 0.5}

                        title_input_container := View {
                            visible: false
                            width: Fill
                            height: Fit
                            title_input := EditTextInput {}
                        }

                        title_label_container := View {
                            visible: false
                            width: Fill
                            height: Fit

                            title_label := Label {
                                padding: Inset {left: 0}
                                width: Fill
                                height: Fit
                                draw_text +: {
                                    text_style: REGULAR_FONT {font_size: 11}
                                    color: #x101828
                                }
                                text: ""
                            }
                        }

                        edit_buttons := View {
                            visible: false
                            width: Fit
                            height: Fit
                            margin: Inset {top: 10}
                            spacing: 6
                            save := SaveButton {}
                            cancel := CancelButton {}
                        }
                    }
                }
            }

            chat_options_wrapper := View {
                width: Fit
                height: Fill
                padding: 4

                chat_options := MolyButton {
                    width: Fit
                    height: Fit
                    padding: Inset {top: 0 right: 4 bottom: 6 left: 4}

                    draw_bg +: {
                        border_radius: 5
                    }

                    draw_text +: {
                        text_style: BOLD_FONT {font_size: 12}
                        color: #x667085
                    }
                    text: "..."

                    reset_hover_on_click: false
                }
            }
            animator: Animator {
                hover: {
                    default: @off
                    off: AnimatorState {
                        from: {all: Forward {duration: 0.15}}
                        apply: {
                            draw_bg: {color: #xF2F4F700}
                        }
                    }
                    on: AnimatorState {
                        from: {all: Forward {duration: 0.15}}
                        apply: {
                            draw_bg: {color: #xEAECEF88}
                        }
                    }
                }
                down: {
                    default: @off
                    off: AnimatorState {
                        from: {all: Forward {duration: 0.5}}
                        ease: OutExp
                        apply: {
                            draw_bg: {down: instance(0.0)}
                        }
                    }
                    on: AnimatorState {
                        ease: OutExp
                        from: {
                            all: Forward {duration: 0.2}
                        }
                        apply: {
                            draw_bg: {down: instance(1.0)}
                        }
                    }
                }
            }
        }

        chat_history_card_options_modal := MolyModal {
            align: Align {x: 0.0 y: 0.0}
            bg_view +: {
                visible: false
            }
            content +: {
                chat_history_card_options := mod.widgets.ChatHistoryCardOptions {}
            }
        }

        delete_chat_modal := MolyModal {
            content +: {
                delete_chat_modal_inner := mod.widgets.DeleteChatModal {}
            }
        }
    }
}

#[derive(Default, Debug, PartialEq)]
enum TitleState {
    OnEdit,
    #[default]
    Editable,
}

#[derive(Script, ScriptHook, Widget)]
pub struct ChatHistoryCard {
    #[deref]
    view: View,
    #[rust]
    chat_id: ChatId,

    #[rust]
    title_edition_state: TitleState,
}

impl Widget for ChatHistoryCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get_mut::<Store>().unwrap();
        let chat = store
            .chats
            .saved_chats
            .iter()
            .find(|c| c.borrow().id == self.chat_id)
            .unwrap();

        if let Some(current_chat_id) = store.chats.get_current_chat_id() {
            let mut content_view_highlight = self.view(cx, ids!(selected_bg));

            if current_chat_id == self.chat_id {
                let color = vec4(0.922, 0.929, 0.933, 1.0);
                script_apply_eval!(cx, content_view_highlight, {
                    draw_bg +: {color: #(color)}
                });
            } else {
                if chat.borrow().has_unread_messages {
                    self.view(cx, ids!(unread_message_badge))
                        .set_visible(cx, true);
                }
                let color = vec4(0.0, 0.0, 0.0, 0.0);
                script_apply_eval!(cx, content_view_highlight, {
                    draw_bg +: {color: #(color)}
                });
            }
        }

        let caption = store.get_chat_associated_bot(self.chat_id).map(|bot_id| {
            store
                .chats
                .available_bots
                .get(&bot_id)
                .map(|m| m.name.clone())
                .unwrap_or("Unknown".to_string())
        });
        self.set_title_text(
            cx,
            chat.borrow_mut().get_title(),
            &caption.clone().unwrap_or_default(),
        );
        self.update_title_visibility(cx);

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ChatHistoryCard {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        match self.title_edition_state {
            TitleState::Editable => self.handle_title_editable_actions(cx, actions, scope),
            TitleState::OnEdit => self.handle_title_on_edit_actions(cx, actions, scope),
        }

        let chat_options_wrapper_rect = self.view(cx, ids!(chat_options_wrapper)).area().rect(cx);
        if self.button(cx, ids!(chat_options)).clicked(actions) {
            let wrapper_coords = chat_options_wrapper_rect.pos;
            let coords = dvec2(
                wrapper_coords.x - 100.,
                wrapper_coords.y + chat_options_wrapper_rect.size.y - 12.0,
            );

            self.chat_history_card_options(cx, ids!(chat_history_card_options))
                .selected(cx, self.chat_id);

            let modal = self.moly_modal(cx, ids!(chat_history_card_options_modal));
            modal.open_as_popup(cx, coords);
            return;
        }

        // Use `child()` for direct-child-only lookup instead of
        // `self.view(cx, ids!(content))`. The widget tree query (`find_within`)
        // returns the last/deepest match when multiple descendants share the
        // same name. Here, MolyModal children also have a `content` child, so
        // the query returns the modal's `content` instead of our direct child.
        let content = self.view.child(id!(content));
        if let Some(item) = actions.find_widget_action(content.widget_uid()) {
            if let ViewAction::FingerDown(fe) = item.cast() {
                if fe.tap_count == 1 {
                    let store = scope.data.get_mut::<Store>().unwrap();
                    store.chats.set_current_chat(Some(self.chat_id));

                    if let Some(chat) = store.chats.get_chat_by_id(self.chat_id) {
                        chat.borrow_mut().has_unread_messages = false;
                        self.view(cx, ids!(unread_message_badge))
                            .set_visible(cx, false);
                    }

                    cx.action(ChatAction::ChatSelected(self.chat_id));
                    self.redraw(cx);
                }
            }
        }

        for action in actions {
            if matches!(
                action.cast(),
                DeleteChatModalAction::Cancelled
                    | DeleteChatModalAction::CloseButtonClicked
                    | DeleteChatModalAction::ChatDeleted
            ) {
                self.moly_modal(cx, ids!(delete_chat_modal)).close(cx);
            }
        }
    }
}

impl ChatHistoryCard {
    pub fn set_chat_id(&mut self, id: ChatId) {
        if id != self.chat_id {
            self.chat_id = id;
            self.title_edition_state = TitleState::Editable;
        }
    }

    fn set_title_text(&mut self, cx: &mut Cx, text: &str, caption: &str) {
        self.view
            .label(cx, ids!(title_label))
            .set_text(cx, text.trim());
        if let TitleState::Editable = self.title_edition_state {
            self.view
                .text_input(cx, ids!(title_input))
                .set_text(cx, text.trim());
        }
        self.label(cx, ids!(model_or_agent_name_label))
            .set_text(cx, &human_readable_name(caption));
    }

    fn update_title_visibility(&mut self, cx: &mut Cx) {
        let on_edit = matches!(self.title_edition_state, TitleState::OnEdit);
        self.view(cx, ids!(edit_buttons)).set_visible(cx, on_edit);
        self.view(cx, ids!(title_input_container))
            .set_visible(cx, on_edit);
        self.button(cx, ids!(chat_options))
            .set_visible(cx, !on_edit);

        let editable = matches!(self.title_edition_state, TitleState::Editable);
        self.view(cx, ids!(title_label_container))
            .set_visible(cx, editable);
    }

    fn transition_title_state(&mut self, cx: &mut Cx) {
        self.title_edition_state = match self.title_edition_state {
            TitleState::OnEdit => TitleState::Editable,
            TitleState::Editable => TitleState::OnEdit,
        };

        self.update_title_visibility(cx);

        match self.title_edition_state {
            TitleState::OnEdit => {
                script_apply_eval!(cx, self, { height: 108 });
            }
            TitleState::Editable => {
                script_apply_eval!(cx, self, { height: 56 });
            }
        }

        self.redraw(cx);
    }

    pub fn handle_title_editable_actions(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        _scope: &mut Scope,
    ) {
        for action in actions {
            match action.cast() {
                ChatHistoryCardAction::MenuClosed(chat_id) => {
                    if chat_id == self.chat_id {
                        self.button(cx, ids!(chat_options)).reset_hover(cx);
                        self.moly_modal(cx, ids!(chat_history_card_options_modal))
                            .close(cx);
                    }
                }
                ChatHistoryCardAction::ActivateTitleEdition(chat_id) => {
                    if chat_id == self.chat_id {
                        self.transition_title_state(cx);
                    }
                }
                ChatHistoryCardAction::DeleteChatOptionSelected(chat_id) => {
                    if chat_id == self.chat_id {
                        let mut delete_modal_inner =
                            self.delete_chat_modal(cx, ids!(delete_chat_modal_inner));
                        delete_modal_inner.set_chat_id(self.chat_id);

                        self.moly_modal(cx, ids!(delete_chat_modal))
                            .open_as_dialog(cx);
                    }
                }
                _ => {}
            }

            if self
                .moly_modal(cx, ids!(chat_history_card_options_modal))
                .dismissed(actions)
            {
                self.button(cx, ids!(chat_options)).reset_hover(cx);
            }
        }
    }

    fn handle_title_on_edit_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let store = scope.data.get_mut::<Store>().unwrap();

        if self.button(cx, ids!(save)).clicked(actions) {
            let updated_title = self.text_input(cx, ids!(title_input)).text();
            let chat = store
                .chats
                .saved_chats
                .iter()
                .find(|c| c.borrow().id == self.chat_id)
                .unwrap();

            if !updated_title.trim().is_empty() && chat.borrow().get_title() != updated_title {
                chat.borrow_mut().set_title(updated_title.clone());
                chat.borrow().save_and_forget();
            }

            self.transition_title_state(cx)
        }

        if let Some((val, _)) = self.text_input(cx, ids!(title_input)).returned(actions) {
            let chat = store
                .chats
                .saved_chats
                .iter()
                .find(|c| c.borrow().id == self.chat_id)
                .unwrap();

            if !val.trim().is_empty() && chat.borrow().get_title() != val {
                chat.borrow_mut().set_title(val.clone());
                chat.borrow().save_and_forget();
            }

            self.transition_title_state(cx)
        }

        if self.button(cx, ids!(cancel)).clicked(actions) {
            self.transition_title_state(cx)
        }
    }
}

impl ChatHistoryCardRef {
    pub fn set_chat_id(&mut self, id: ChatId) -> Result<(), &'static str> {
        let Some(mut inner) = self.borrow_mut() else {
            return Err("Widget not found in the document");
        };

        inner.set_chat_id(id);
        Ok(())
    }
}

#[derive(Clone, Default, Eq, Hash, PartialEq, Debug)]
pub enum ChatHistoryCardAction {
    #[default]
    None,
    ActivateTitleEdition(ChatId),
    MenuClosed(ChatId),
    DeleteChatOptionSelected(ChatId),
}
