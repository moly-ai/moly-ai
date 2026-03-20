use super::chat_history_card::ChatHistoryCardWidgetRefExt;
use crate::chat::entity_button::EntityButtonWidgetRefExt;
use crate::data::chats::chat::ChatId;
use crate::data::store::Store;
use crate::shared::actions::ChatAction;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let HeadingLabel = Label {
        margin: Inset {left: 4 bottom: 4}
        draw_text +: {
            text_style: BOLD_FONT {font_size: 10.5}
            color: #3
        }
    }

    let NoAgentsWarning = Label {
        margin: Inset {left: 4 bottom: 4}
        width: Fill
        draw_text +: {
            text_style +: {font_size: 8.5}
            color: #3
        }
    }

    mod.widgets.ChatHistoryBase = #(ChatHistory::register_widget(vm))
    mod.widgets.ChatHistory = set_type_default() do mod.widgets.ChatHistoryBase {
        width: Fill height: Fill
        show_bg: true
        draw_bg +: {
            color: (MAIN_BG_COLOR)
            pixel: fn() {
                return Pal.premul(self.color)
            }
        }
        padding: Inset {left: 10 right: 10}

        list := PortalList {
            drag_scrolling: false
            AgentHeading := HeadingLabel { text: "AGENTS" }
            NoAgentsWarning := NoAgentsWarning {}
            Agent := mod.widgets.EntityButton {
                server_url_visible: true
            }
            ChatsHeading := HeadingLabel { text: "CHATS" margin: Inset {top: 10} }
            ChatHistoryCard := mod.widgets.ChatHistoryCard {
                cursor: Default
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ChatHistory {
    #[deref]
    deref: View,
}

impl Widget for ChatHistory {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get_mut::<Store>().unwrap();

        enum Item<'a> {
            ChatsHeader,
            ChatButton(&'a ChatId),
        }

        let mut items: Vec<Item> = Vec::new();

        items.push(Item::ChatsHeader);

        let mut chat_ids = store
            .chats
            .saved_chats
            .iter()
            .map(|c| c.borrow().id)
            .collect::<Vec<_>>();

        chat_ids.sort_by(|a, b| b.cmp(a));

        items.extend(chat_ids.iter().map(Item::ChatButton));

        while let Some(view_item) = self.deref.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = view_item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, items.len());
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id >= items.len() {
                        continue;
                    }

                    match &items[item_id] {
                        Item::ChatsHeader => {
                            let item = list.item(cx, item_id, id!(ChatsHeading));
                            item.draw_all(cx, scope);
                        }
                        Item::ChatButton(chat_id) => {
                            let mut item = list
                                .item(cx, item_id, id!(ChatHistoryCard))
                                .as_chat_history_card();
                            let _ = item.set_chat_id(**chat_id);
                            item.draw_all(cx, scope);
                        }
                    }
                }
            }
        }

        DrawStep::done()
    }
}

impl WidgetMatchEvent for ChatHistory {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let clicked_entity_button = self
            .portal_list(cx, ids!(list))
            .items_with_actions(actions)
            .iter()
            .map(|(_, item)| item.as_entity_button())
            .find(|eb| eb.clicked(actions));

        if let Some(entity_button) = clicked_entity_button {
            let bot_id = entity_button.get_bot_id();
            if let Some(bot_id) = bot_id {
                cx.action(ChatAction::Start(bot_id));
            }
        }
    }
}
