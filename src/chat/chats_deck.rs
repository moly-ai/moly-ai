use std::collections::{HashMap, VecDeque};

use makepad_widgets::*;
use moly_kit::prelude::*;

use super::chat_view::ChatViewRef;
use crate::chat::chat_view::ChatViewWidgetRefExt;
use crate::data::capture::CaptureAction;
use crate::data::chats::chat::Chat as ChatData;
use crate::data::chats::chat::ChatId;
use crate::data::store::Store;
use crate::shared::actions::ChatAction;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ChatsDeckBase = #(ChatsDeck::register_widget(vm))
    mod.widgets.ChatsDeck = set_type_default() do mod.widgets.ChatsDeckBase {
        width: Fill height: Fill
        padding: Inset {top: 18 bottom: 0 right: 28 left: 28}

        chat_view_template := ChatView {}
    }
}

#[derive(Script, Widget)]
pub struct ChatsDeck {
    #[deref]
    view: View,

    /// All currently active chat instances, keyed by their corresponding ChatID.
    /// Each chat maintains its own instance to keep background streaming alive.
    #[rust]
    chat_view_refs: HashMap<ChatId, ChatViewRef>,

    /// LRU tracking for memory management.
    /// When we exceed MAX_CHAT_VIEWS, we evict the oldest chat (unless it's streaming).
    #[rust]
    chat_view_accessed_order: VecDeque<ChatId>,

    /// The currently visible/focused chat id.
    #[rust]
    currently_visible_chat_id: Option<ChatId>,

    /// The template for creating new chat views.
    #[live]
    chat_view_template: Option<ScriptObjectRef>,
}

impl ScriptHook for ChatsDeck {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        value: ScriptValue,
    ) {
        if let Some(obj) = value.as_object() {
            vm.vec_with(obj, |vm, vec| {
                for kv in vec {
                    if let Some(id) = kv.key.as_id() {
                        if id == id!(chat_view_template) {
                            if let Some(template_obj) = kv.value.as_object() {
                                self.chat_view_template =
                                    Some(vm.bx.heap.new_object_ref(template_obj));
                            }
                        }
                    }
                }
            });
        }
    }
}

/// The maximum number of chat views that can be kept alive at once.
/// Prevents unbounded memory growth in long-running sessions.
const MAX_CHAT_VIEWS: usize = 10;

impl Widget for ChatsDeck {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);

        // Handle events for ALL instances to keep background activity (streaming, etc.) alive
        for (_, chat_view) in self.chat_view_refs.iter_mut() {
            chat_view.handle_event(cx, event, scope);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Because chats_deck is being cached, overriding its properties in the DSL
        // does not take effect. For now we'll override them through script_apply_eval.
        // TODO: Do not use CachedWidget, create a shared structure of chat instances
        // that is shared across layouts.
        if cx.display_context.is_desktop() {
            let padding = Inset {
                top: 18.0,
                bottom: 0.0,
                right: 28.0,
                left: 28.0,
            };
            script_apply_eval!(cx, self.view, {
                padding: #(padding)
            });
        } else {
            let padding = Inset {
                top: 55.0,
                left: 0.0,
                right: 0.0,
                bottom: 0.0,
            };
            script_apply_eval!(cx, self.view, {
                padding: #(padding)
            });
        }

        cx.begin_turtle(walk, self.layout);

        // Draw only the currently visible chat
        if let Some(chat_id) = self.currently_visible_chat_id {
            if let Some(chat_view) = self.chat_view_refs.get_mut(&chat_id) {
                let _ = chat_view.draw(cx, scope);
            }
        }

        cx.end_turtle();
        DrawStep::done()
    }
}

impl WidgetMatchEvent for ChatsDeck {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let store = scope.data.get_mut::<Store>().unwrap();
        for action in actions {
            // Handle chat start
            match action.cast() {
                ChatAction::Start(bot_id) => {
                    let chat_id = store.chats.create_empty_chat(Some(bot_id.clone()));
                    let chat = store.chats.get_chat_by_id(chat_id);
                    if let Some(chat) = chat {
                        self.create_or_update_chat_view(cx, &chat.borrow());
                    }
                }
                ChatAction::StartWithoutEntity => {
                    let chat_id = store.chats.create_empty_chat(None);
                    let chat = store.chats.get_chat_by_id(chat_id);
                    if let Some(chat) = chat {
                        self.create_or_update_chat_view(cx, &chat.borrow());
                    }
                }
                _ => {}
            }

            // Handle chat selection (from chat history)
            match action.cast() {
                ChatAction::ChatSelected(chat_id) => {
                    let selected_chat = store.chats.get_chat_by_id(chat_id);

                    if let Some(chat) = selected_chat {
                        store
                            .preferences
                            .set_current_chat_model(chat.borrow().associated_bot.clone());

                        self.create_or_update_chat_view(cx, &chat.borrow());
                    }
                }
                _ => {}
            }

            // Handle Context Capture
            if let CaptureAction::Capture { event } = action.cast() {
                // Paste the captured text into the currently visible chat
                if let Some(chat_id) = self.currently_visible_chat_id {
                    if let Some(chat_view) = self.chat_view_refs.get_mut(&chat_id) {
                        chat_view
                            .prompt_input(cx, ids!(prompt))
                            .write()
                            .set_text(cx, event.contents());
                    }
                }
            }
        }
    }
}

impl ChatsDeck {
    pub fn create_or_update_chat_view(&mut self, cx: &mut Cx, chat_data: &ChatData) {
        // Check if an instance already exists for this chat
        if let Some(existing_view) = self.chat_view_refs.get_mut(&chat_data.id) {
            // Instance exists, just make it visible and focused
            self.currently_visible_chat_id = Some(chat_data.id);

            // Update focus states
            existing_view.set_focused(true);
            for (id, chat_view) in self.chat_view_refs.iter_mut() {
                if *id != chat_data.id {
                    chat_view.set_focused(false);
                }
            }

            // Update LRU access order
            self.chat_view_accessed_order
                .retain(|id| *id != chat_data.id);
            self.chat_view_accessed_order.push_back(chat_data.id);

            return; // EARLY RETURN, don't recreate!
        }

        // No existing instance, create a new one
        let chat_view = cx.with_vm(|vm| {
            let template_value: ScriptValue = self
                .chat_view_template
                .as_ref()
                .expect("chat_view_template not set")
                .as_object()
                .into();
            WidgetRef::script_from_value(vm, template_value)
        });
        let mut chat_view = chat_view.as_chat_view();

        // Initialize new instance
        chat_view.set_chat_id(chat_data.id);

        // Load messages into the controller
        chat_view
            .borrow()
            .unwrap()
            .chat_controller()
            .lock()
            .unwrap()
            .dispatch_mutation(VecMutation::Set(chat_data.messages.clone()));

        // Sync associated_bot from Store to ChatController
        if let Some(bot_id) = &chat_data.associated_bot {
            chat_view.set_bot_id(Some(bot_id.clone()));
        }

        // Set as focused
        chat_view.set_focused(true);

        // Insert into HashMap
        self.chat_view_refs.insert(chat_data.id, chat_view);
        self.currently_visible_chat_id = Some(chat_data.id);

        // Defocus other chats
        for (id, cv) in self.chat_view_refs.iter_mut() {
            if *id != chat_data.id {
                cv.set_focused(false);
            }
        }

        // Add to LRU tracking
        self.chat_view_accessed_order.push_back(chat_data.id);

        // Evict oldest instance if we exceed max
        if self.chat_view_accessed_order.len() > MAX_CHAT_VIEWS {
            let oldest_id = self.chat_view_accessed_order.pop_front().unwrap();
            if let Some(oldest_view) = self.chat_view_refs.get_mut(&oldest_id) {
                // Don't evict if currently streaming
                if !oldest_view.chat(cx, ids!(chat)).read().is_streaming() {
                    self.chat_view_refs.remove(&oldest_id);
                } else {
                    // Put back in queue if streaming
                    self.chat_view_accessed_order.push_front(oldest_id);
                }
            }
        }

        // TODO: Focus on prompt input
    }
}
