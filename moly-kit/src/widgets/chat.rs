use makepad_widgets::*;
use makepad_widgets::defer_with_redraw::DeferWithRedraw;
use std::cell::{Ref, RefMut};
use std::sync::{Arc, Mutex};

use crate::aitk::utils::tool::display_name_from_namespaced;
use crate::prelude::*;
use crate::utils::makepad::events::EventExt;
use crate::widgets::stt_input::*;

// Re-export type needed to configure STT.
pub use crate::widgets::stt_input::SttUtility;

script_mod!(
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.Chat = #(Chat::register_widget(vm)) RoundedView {
        flow: Down
        messages := Messages {}
        prompt := PromptInput {}
        stt_input := SttInput { visible: false }

        View {
            width: Fill, height: Fit
            flow: Overlay

            audio_modal := MolyModal {
                dismiss_on_focus_lost: false
                content +: mod.widgets.RealtimeContent {}
            }
        }
    }
);

/// A batteries-included chat to implement chatbots.
#[derive(Script, ScriptHook, Widget)]
pub struct Chat {
    #[deref]
    deref: View,

    #[rust]
    chat_controller: Option<Arc<Mutex<ChatController>>>,

    /// Toggles response streaming on or off. Default is on.
    // TODO: Implement this.
    #[live(true)]
    pub stream: bool,

    #[rust]
    plugin_id: Option<ChatControllerPluginRegistrationId>,
}

impl Widget for Chat {
    fn handle_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
    ) {
        // Handle audio devices setup
        if let Event::AudioDevices(devices) = event {
            let input = devices.default_input();
            if !input.is_empty() {
                cx.use_audio_inputs(&input);
            }
        }

        self.ui_runner().handle(cx, event, scope, self);
        self.deref.handle_event(cx, event, scope);

        self.handle_messages(cx, event);
        self.handle_prompt_input(cx, event);
        self.handle_stt_input_actions(cx, event);
        self.handle_realtime(cx);
        self.handle_modal_dismissal(cx, event);
    }

    fn draw_walk(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        walk: Walk,
    ) -> DrawStep {
        let has_stt =
            self.stt_input_ref(cx).read().stt_utility().is_some();
        self.prompt_input_ref(cx).write().set_stt_visible(cx, has_stt);

        self.deref.draw_walk(cx, scope, walk)
    }
}

impl Chat {
    /// Getter to the underlying [PromptInputRef] independent of its id.
    pub fn prompt_input_ref(&self, cx: &Cx) -> PromptInputRef {
        self.prompt_input(cx, ids!(prompt))
    }

    /// Getter to the underlying [MessagesRef] independent of its id.
    pub fn messages_ref(&self, cx: &Cx) -> MessagesRef {
        self.messages(cx, ids!(messages))
    }

    /// Getter to the underlying [SttInputRef] independent of its id.
    pub fn stt_input_ref(&self, cx: &Cx) -> SttInputRef {
        self.stt_input(cx, ids!(stt_input))
    }

    /// Configures the STT utility to be used for speech-to-text.
    pub fn set_stt_utility(&mut self, cx: &Cx, utility: Option<SttUtility>) {
        self.stt_input_ref(cx).write().set_stt_utility(utility);
    }

    /// Returns the current STT utility, if any, as a clone.
    pub fn stt_utility(&self, cx: &Cx) -> Option<SttUtility> {
        self.stt_input_ref(cx).read().stt_utility().cloned()
    }

    fn handle_prompt_input(&mut self, cx: &mut Cx, event: &Event) {
        let submitted =
            self.prompt_input_ref(cx).read().submitted(cx, event.actions());
        if submitted {
            self.handle_submit(cx);
        }

        let call_pressed =
            self.prompt_input_ref(cx).read().call_pressed(cx, event.actions());
        if call_pressed {
            self.handle_call(cx);
        }

        let stt_pressed =
            self.prompt_input_ref(cx).read().stt_pressed(cx, event.actions());
        if stt_pressed {
            self.prompt_input_ref(cx).set_visible(cx, false);
            self.stt_input_ref(cx).set_visible(cx, true);
            self.stt_input_ref(cx).write().start_recording(cx);
            self.redraw(cx);
        }
    }

    fn handle_stt_input_actions(&mut self, cx: &mut Cx, event: &Event) {
        let transcription = self
            .stt_input_ref(cx)
            .read()
            .transcribed(event.actions());

        if let Some(transcription) = transcription {
            self.stt_input_ref(cx).set_visible(cx, false);
            self.prompt_input_ref(cx).set_visible(cx, true);

            let mut text = self.prompt_input_ref(cx).text();
            if let Some(last) = text.as_bytes().last()
                && *last != b' '
            {
                text.push(' ');
            }
            text.push_str(&transcription);
            self.prompt_input_ref(cx).set_text(cx, &text);

            self.prompt_input_ref(cx).redraw(cx);
        }

        let cancelled =
            self.stt_input_ref(cx).read().cancelled(event.actions());
        if cancelled {
            self.stt_input_ref(cx).set_visible(cx, false);
            self.prompt_input_ref(cx).set_visible(cx, true);
            self.prompt_input_ref(cx).redraw(cx);
        }
    }

    fn handle_realtime(&mut self, cx: &mut Cx) {
        if self.realtime(cx, ids!(realtime)).connection_requested()
            && self
                .chat_controller
                .as_ref()
                .map(|c| {
                    c.lock().unwrap().state().bot_id.is_some()
                })
                .unwrap_or(false)
        {
            self.chat_controller
                .as_mut()
                .unwrap()
                .lock()
                .unwrap()
                .dispatch_task(ChatTask::Send);
        }
    }

    fn handle_modal_dismissal(&mut self, cx: &mut Cx, event: &Event) {
        for action in event.actions() {
            if let RealtimeModalAction::DismissModal = action.cast() {
                self.moly_modal(cx, ids!(audio_modal)).close(cx);
            }
        }

        if self
            .moly_modal(cx, ids!(audio_modal))
            .dismissed(event.actions())
        {
            let mut conversation_messages = self
                .realtime(cx, ids!(realtime))
                .take_conversation_messages();

            self.realtime(cx, ids!(realtime)).reset_state(cx);

            if !conversation_messages.is_empty() {
                let chat_controller =
                    self.chat_controller.clone().unwrap();

                let mut all_messages = chat_controller
                    .lock()
                    .unwrap()
                    .state()
                    .messages
                    .clone();

                let system_message = Message {
                    from: EntityId::App,
                    content: MessageContent {
                        text: "Voice call started.".to_string(),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                conversation_messages.insert(0, system_message);

                let system_message = Message {
                    from: EntityId::App,
                    content: MessageContent {
                        text: "Voice call ended.".to_string(),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                conversation_messages.push(system_message);

                all_messages.extend(conversation_messages);
                chat_controller
                    .lock()
                    .unwrap()
                    .dispatch_mutation(VecMutation::Set(all_messages));

                self.messages_ref(cx)
                    .write()
                    .instant_scroll_to_bottom(cx);
            }
        }
    }

    fn handle_capabilities(&mut self, cx: &mut Cx) {
        let capabilities =
            self.chat_controller.as_ref().and_then(|controller| {
                let lock = controller.lock().unwrap();
                let bot_id = lock.state().bot_id.as_ref()?;
                lock.state()
                    .get_bot(bot_id)
                    .map(|bot| bot.capabilities.clone())
            });

        self.prompt_input_ref(cx)
            .write()
            .set_bot_capabilities(cx, capabilities);
    }

    fn handle_messages(&mut self, cx: &mut Cx, event: &Event) {
        for action in event.actions() {
            let Some(action) = action.as_widget_action() else {
                continue;
            };

            if action.widget_uid != self.messages_ref(cx).widget_uid() {
                continue;
            }

            let chat_controller =
                self.chat_controller.clone().unwrap();

            match action.cast::<MessagesAction>() {
                MessagesAction::Delete(index) => chat_controller
                    .lock()
                    .unwrap()
                    .dispatch_mutation(
                        VecMutation::<Message>::RemoveOne(index),
                    ),
                MessagesAction::Copy(index) => {
                    let lock = chat_controller.lock().unwrap();
                    let text =
                        &lock.state().messages[index].content.text;
                    cx.copy_to_clipboard(text);
                }
                MessagesAction::EditSave(index) => {
                    let text = self
                        .messages_ref(cx)
                        .read()
                        .current_editor_text(cx)
                        .expect("no editor text");

                    self.messages_ref(cx)
                        .write()
                        .set_message_editor_visibility(index, false);

                    let mut lock = chat_controller.lock().unwrap();

                    let mutation = VecMutation::update_with(
                        &lock.state().messages,
                        index,
                        |message| {
                            message.update_content(move |content| {
                                content.text = text;
                            });
                        },
                    );

                    lock.dispatch_mutation(mutation);
                }
                MessagesAction::EditRegenerate(index) => {
                    let mut messages = chat_controller
                        .lock()
                        .unwrap()
                        .state()
                        .messages[0..=index]
                        .to_vec();

                    let text = self
                        .messages_ref(cx)
                        .read()
                        .current_editor_text(cx)
                        .expect("no editor text");

                    self.messages_ref(cx)
                        .write()
                        .set_message_editor_visibility(index, false);

                    messages[index].update_content(|content| {
                        content.text = text;
                    });

                    chat_controller
                        .lock()
                        .unwrap()
                        .dispatch_mutation(VecMutation::Set(messages));

                    if self
                        .chat_controller
                        .as_ref()
                        .map(|c| {
                            c.lock()
                                .unwrap()
                                .state()
                                .bot_id
                                .is_some()
                        })
                        .unwrap_or(false)
                    {
                        chat_controller
                            .lock()
                            .unwrap()
                            .dispatch_task(ChatTask::Send);
                    }
                }
                MessagesAction::ToolApprove(index) => {
                    let mut lock = chat_controller.lock().unwrap();

                    let mut updated_message =
                        lock.state().messages[index].clone();

                    for tool_call in
                        &mut updated_message.content.tool_calls
                    {
                        tool_call.permission_status =
                            ToolCallPermissionStatus::Approved;
                    }

                    lock.dispatch_mutation(VecMutation::Update(
                        index,
                        updated_message,
                    ));

                    let tools = lock.state().messages[index]
                        .content
                        .tool_calls
                        .clone();
                    let bot_id = lock.state().bot_id.clone();
                    lock.dispatch_task(ChatTask::Execute(
                        tools, bot_id,
                    ));
                }
                MessagesAction::ToolDeny(index) => {
                    let mut lock = chat_controller.lock().unwrap();

                    let mut updated_message =
                        lock.state().messages[index].clone();

                    updated_message.update_content(|content| {
                        for tool_call in &mut content.tool_calls {
                            tool_call.permission_status =
                                ToolCallPermissionStatus::Denied;
                        }
                    });

                    lock.dispatch_mutation(VecMutation::Update(
                        index,
                        updated_message,
                    ));

                    let tool_results: Vec<ToolResult> = lock
                        .state()
                        .messages[index]
                        .content
                        .tool_calls
                        .iter()
                        .map(|tc| {
                            let display_name =
                                display_name_from_namespaced(&tc.name);
                            ToolResult {
                                tool_call_id: tc.id.clone(),
                                content: format!(
                                    "Tool execution was denied by the user. \
                                     Tool '{}' was not executed.",
                                    display_name
                                ),
                                is_error: true,
                            }
                        })
                        .collect();

                    lock.dispatch_mutation(VecMutation::Push(Message {
                        from: EntityId::Tool,
                        content: MessageContent {
                            text: "\u{1f6ab} Tool execution was denied \
                                   by the user."
                                .to_string(),
                            tool_results,
                            ..Default::default()
                        },
                        ..Default::default()
                    }));
                }
                MessagesAction::None => {}
            }
        }
    }

    fn handle_submit(&mut self, cx: &mut Cx) {
        let mut prompt = self.prompt_input_ref(cx);
        let chat_controller = self.chat_controller.clone().unwrap();

        if prompt.read().has_send_task()
            && self
                .chat_controller
                .as_ref()
                .map(|c| {
                    c.lock().unwrap().state().bot_id.is_some()
                })
                .unwrap_or(false)
        {
            let text = prompt.text();
            let attachments = prompt
                .read()
                .attachment_list_ref(cx)
                .read()
                .attachments
                .clone();

            if !text.is_empty() || !attachments.is_empty() {
                chat_controller
                    .lock()
                    .unwrap()
                    .dispatch_mutation(VecMutation::Push(Message {
                        from: EntityId::User,
                        content: MessageContent {
                            text,
                            attachments,
                            ..Default::default()
                        },
                        ..Default::default()
                    }));
            }

            prompt.write().reset(cx);
            chat_controller
                .lock()
                .unwrap()
                .dispatch_task(ChatTask::Send);
        } else if prompt.read().has_stop_task() {
            chat_controller
                .lock()
                .unwrap()
                .dispatch_task(ChatTask::Stop);
        }
    }

    fn handle_call(&mut self, _cx: &mut Cx) {
        if self
            .chat_controller
            .as_ref()
            .map(|c| c.lock().unwrap().state().bot_id.is_some())
            .unwrap_or(false)
        {
            self.chat_controller
                .as_mut()
                .unwrap()
                .lock()
                .unwrap()
                .dispatch_task(ChatTask::Send);
        }
    }

    /// Returns true if the chat is currently streaming.
    pub fn is_streaming(&self) -> bool {
        self.chat_controller
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .state()
            .is_streaming
    }

    /// Sets the chat controller for this chat widget.
    pub fn set_chat_controller(
        &mut self,
        cx: &mut Cx,
        chat_controller: Option<Arc<Mutex<ChatController>>>,
    ) {
        if self.chat_controller.as_ref().map(Arc::as_ptr)
            == chat_controller.as_ref().map(Arc::as_ptr)
        {
            return;
        }

        self.unlink_current_controller();
        self.chat_controller = chat_controller;

        self.messages_ref(cx).write().chat_controller =
            self.chat_controller.clone();
        self.realtime(cx, ids!(realtime))
            .set_chat_controller(self.chat_controller.clone());
        self.prompt_input_ref(cx)
            .write()
            .set_chat_controller(cx, self.chat_controller.clone());

        if let Some(controller) = self.chat_controller.as_ref() {
            let mut guard = controller.lock().unwrap();

            let plugin = Plugin::new(self.ui_runner());
            self.plugin_id = Some(guard.append_plugin(plugin));
        }
    }

    /// Returns a reference to the chat controller, if set.
    pub fn chat_controller(
        &self,
    ) -> Option<&Arc<Mutex<ChatController>>> {
        self.chat_controller.as_ref()
    }

    fn unlink_current_controller(&mut self) {
        if let Some(plugin_id) = self.plugin_id {
            if let Some(controller) = self.chat_controller.as_ref() {
                controller.lock().unwrap().remove_plugin(plugin_id);
            }
        }

        self.chat_controller = None;
        self.plugin_id = None;
    }

    fn handle_streaming_start(&mut self, cx: &mut Cx) {
        self.prompt_input_ref(cx).write().set_stop();
        self.messages_ref(cx).write().animated_scroll_to_bottom(cx);
        self.redraw(cx);
    }

    fn handle_streaming_end(&mut self, cx: &mut Cx) {
        self.prompt_input_ref(cx).write().set_send();
        self.redraw(cx);
    }
}

// TODO: Since `ChatRef` is generated by a macro, I can't document this
// to give these functions better visibility from the module view.
impl ChatRef {
    /// Immutable access to the underlying [Chat].
    ///
    /// Panics if the widget reference is empty or if it's already borrowed.
    pub fn read(&self) -> Ref<'_, Chat> {
        self.borrow().unwrap()
    }

    /// Mutable access to the underlying [Chat].
    ///
    /// Panics if the widget reference is empty or if it's already borrowed.
    pub fn write(&mut self) -> RefMut<'_, Chat> {
        self.borrow_mut().unwrap()
    }

    /// Immutable reader to the underlying [Chat].
    ///
    /// Panics if the widget reference is empty or if it's already borrowed.
    pub fn read_with<R>(&self, f: impl FnOnce(&Chat) -> R) -> R {
        f(&*self.read())
    }

    /// Mutable writer to the underlying [Chat].
    ///
    /// Panics if the widget reference is empty or if it's already borrowed.
    pub fn write_with<R>(
        &mut self,
        f: impl FnOnce(&mut Chat) -> R,
    ) -> R {
        f(&mut *self.write())
    }
}

impl Drop for Chat {
    fn drop(&mut self) {
        self.unlink_current_controller();
    }
}

struct Plugin {
    ui: UiRunner<Chat>,
}

impl Plugin {
    fn new(ui: UiRunner<Chat>) -> Self {
        Self { ui }
    }
}

impl ChatControllerPlugin for Plugin {
    fn on_state_ready(
        &mut self,
        _state: &ChatState,
        mutations: &[ChatStateMutation],
    ) {
        for mutation in mutations {
            match mutation {
                ChatStateMutation::SetIsStreaming(true) => {
                    self.ui.defer(|chat, cx, _| {
                        chat.handle_streaming_start(cx);
                    });
                }
                ChatStateMutation::SetIsStreaming(false) => {
                    self.ui.defer(|chat, cx, _| {
                        chat.handle_streaming_end(cx);
                    });
                }
                ChatStateMutation::MutateBots(_) => {
                    self.ui.defer(|chat, cx, _| {
                        if let Some(controller) =
                            &chat.chat_controller
                        {
                            let mut lock =
                                controller.lock().unwrap();
                            if let Some(bot_id) =
                                lock.state().bot_id.clone()
                            {
                                let bot_still_available = lock
                                    .state()
                                    .bots
                                    .iter()
                                    .any(|b| &b.id == &bot_id);
                                if !bot_still_available {
                                    lock.dispatch_mutation(
                                        ChatStateMutation::SetBotId(
                                            None,
                                        ),
                                    );
                                }
                            }
                        }

                        chat.handle_capabilities(cx);
                    });
                }
                ChatStateMutation::SetBotId(_bot_id) => {
                    self.ui.defer(move |chat, cx, _| {
                        chat.handle_capabilities(cx);
                    });
                }
                _ => {}
            }
        }

        // Always redraw on state change.
        self.ui.defer_with_redraw(move |_, _, _| {});
    }

    fn on_upgrade(
        &mut self,
        upgrade: Upgrade,
        bot_id: &BotId,
    ) -> Option<Upgrade> {
        match upgrade {
            Upgrade::Realtime(channel) => {
                let entity_id = EntityId::Bot(bot_id.clone());
                self.ui.defer(move |me, cx, _| {
                    me.handle_streaming_end(cx);

                    let mut realtime = me.realtime(cx, ids!(realtime));
                    realtime.set_bot_entity_id(cx, entity_id);
                    realtime
                        .set_realtime_channel(channel.clone());

                    let modal = me.moly_modal(cx, ids!(audio_modal));
                    modal.open_as_dialog(cx);
                });
                None
            }
            #[allow(unreachable_patterns)]
            upgrade => Some(upgrade),
        }
    }
}
