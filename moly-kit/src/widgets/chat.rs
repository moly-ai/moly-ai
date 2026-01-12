use makepad_widgets::*;
use std::cell::{Ref, RefMut};
use std::sync::{Arc, Mutex};

use crate::aitk::protocol::BotClient;
use crate::aitk::utils::tool::display_name_from_namespaced;
use crate::prelude::*;
use crate::utils::makepad::events::EventExt;

#[derive(Clone)]
pub struct SttUtility {
    pub client: Box<dyn BotClient>,
    pub bot_id: BotId,
}

live_design!(
    use link::theme::*;
    use link::widgets::*;
    use link::moly_kit_theme::*;
    use link::shaders::*;

    use crate::widgets::messages::*;
    use crate::widgets::prompt_input::*;
    use crate::widgets::moly_modal::*;
    use crate::widgets::realtime::*;

    pub Chat = {{Chat}} <RoundedView> {
        flow: Down,
        messages = <Messages> {}
        prompt = <PromptInput> {}

        <View> {
            width: Fill, height: Fit
            flow: Overlay

            audio_modal = <MolyModal> {
                dismiss_on_focus_lost: false
                content: <RealtimeContent> {}
            }
        }
    }
);

/// A batteries-included chat to to implement chatbots.
#[derive(Live, LiveHook, Widget)]
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

    #[rust]
    pub stt_utility: Option<SttUtility>,

    #[rust]
    is_recording_stt: bool,

    #[rust]
    recorded_stt_audio: Option<Arc<Mutex<SttAudioData>>>,
}

#[derive(Default)]
struct SttAudioData {
    data: Vec<f32>,
    sample_rate: Option<f64>,
}

impl Widget for Chat {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
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
        self.handle_realtime(cx);
        self.handle_modal_dismissal(cx, event);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.prompt_input_ref()
            .write()
            .set_stt_visible(cx, self.stt_utility.is_some());

        self.deref.draw_walk(cx, scope, walk)
    }
}

impl Chat {
    /// Getter to the underlying [PromptInputRef] independent of its id.
    pub fn prompt_input_ref(&self) -> PromptInputRef {
        self.prompt_input(ids!(prompt))
    }

    /// Getter to the underlying [MessagesRef] independent of its id.
    pub fn messages_ref(&self) -> MessagesRef {
        self.messages(ids!(messages))
    }

    fn handle_prompt_input(&mut self, cx: &mut Cx, event: &Event) {
        if self.prompt_input_ref().read().submitted(event.actions()) {
            self.handle_submit(cx);
        }

        if self.prompt_input_ref().read().call_pressed(event.actions()) {
            self.handle_call(cx);
        }

        if self.prompt_input_ref().read().stt_pressed(event.actions()) {
            self.handle_stt(cx);
        }
    }

    fn handle_stt(&mut self, cx: &mut Cx) {
        if self.stt_utility.is_none() {
            return;
        }

        if self.is_recording_stt {
            self.stop_stt_recording(cx);
        } else {
            self.start_stt_recording(cx);
        }
    }

    fn start_stt_recording(&mut self, cx: &mut Cx) {
        self.is_recording_stt = true;
        // Optionally update UI state here (e.g. prompt input icon state)

        if self.recorded_stt_audio.is_none() {
            self.recorded_stt_audio = Some(Arc::new(Mutex::new(SttAudioData::default())));
        }

        // Reset buffer
        if let Some(arc) = &self.recorded_stt_audio {
            if let Ok(mut buffer) = arc.lock() {
                buffer.data.clear();
                buffer.sample_rate = None;
            }

            let buffer_clone = arc.clone();
            cx.audio_input(0, move |info, input_buffer| {
                let channel = input_buffer.channel(0); // Mono

                if let Ok(mut recorded) = buffer_clone.try_lock() {
                    if recorded.sample_rate.is_none() {
                        recorded.sample_rate = Some(info.sample_rate);
                    }
                    recorded.data.extend_from_slice(channel);
                }
            });
        }
    }

    fn stop_stt_recording(&mut self, cx: &mut Cx) {
        self.is_recording_stt = false;
        // Stop recording
        cx.audio_input(0, |_, _| {});

        if let Some(buffer_arc) = self.recorded_stt_audio.clone() {
            self.process_stt_audio(cx, buffer_arc);
        }
    }

    fn process_stt_audio(&mut self, _cx: &mut Cx, buffer_arc: Arc<Mutex<SttAudioData>>) {
        if let Some(utility) = &self.stt_utility {
            let mut client = utility.client.clone();
            let bot_id = utility.bot_id.clone();
            let ui = self.ui_runner();

            let (samples, sample_rate) = {
                let guard = buffer_arc.lock().unwrap();
                (guard.data.clone(), guard.sample_rate)
            };

            if samples.is_empty() {
                return;
            }

            let sample_rate = sample_rate.unwrap_or(48000.0) as u32;
            let wav_bytes = crate::utils::audio::build_wav(&samples, sample_rate, 1);

            let attachment = Attachment::from_bytes(
                "recording.wav".to_string(),
                Some("audio/wav".to_string()),
                &wav_bytes,
            );

            let message = Message {
                from: EntityId::User,
                content: MessageContent {
                    attachments: vec![attachment],
                    ..Default::default()
                },
                ..Default::default()
            };

            crate::aitk::utils::asynchronous::spawn(async move {
                use futures::{StreamExt, pin_mut};
                let stream = client.send(&bot_id, &[message], &[]);

                let filtered = stream
                    .filter_map(|r| async move { r.value().map(|c| c.text.clone()) })
                    .filter(|text| futures::future::ready(!text.is_empty()));
                pin_mut!(filtered);
                let text = filtered.next().await;

                if let Some(text) = text {
                    ui.defer_with_redraw(move |me, cx, _| {
                        me.prompt_input_ref().set_text(cx, &text);
                    });
                }
            });
        }
    }

    fn handle_realtime(&mut self, _cx: &mut Cx) {
        if self.realtime(ids!(realtime)).connection_requested()
            && self
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

    fn handle_modal_dismissal(&mut self, cx: &mut Cx, event: &Event) {
        // Check if the modal should be dismissed
        for action in event.actions() {
            if let RealtimeModalAction::DismissModal = action.cast() {
                self.moly_modal(ids!(audio_modal)).close(cx);
            }
        }

        // Check if the audio modal was dismissed
        if self
            .moly_modal(ids!(audio_modal))
            .dismissed(event.actions())
        {
            // Collect conversation messages from the realtime widget before resetting
            let mut conversation_messages =
                self.realtime(ids!(realtime)).take_conversation_messages();

            // Reset realtime widget state for cleanup
            self.realtime(ids!(realtime)).reset_state(cx);

            // Add conversation messages to chat history preserving order
            if !conversation_messages.is_empty() {
                let chat_controller = self.chat_controller.clone().unwrap();

                // Get current messages and append the new conversation messages
                let mut all_messages = chat_controller.lock().unwrap().state().messages.clone();

                // Add a system message before and after the conversation, informing
                // that a voice call happened.
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

                self.messages_ref().write().instant_scroll_to_bottom(cx);
            }
        }
    }

    fn handle_capabilities(&mut self, cx: &mut Cx) {
        let capabilities = self.chat_controller.as_ref().and_then(|controller| {
            let lock = controller.lock().unwrap();
            let bot_id = lock.state().bot_id.as_ref()?;
            lock.state()
                .get_bot(bot_id)
                .map(|bot| bot.capabilities.clone())
        });

        self.prompt_input_ref()
            .write()
            .set_bot_capabilities(cx, capabilities);
    }

    fn handle_messages(&mut self, cx: &mut Cx, event: &Event) {
        for action in event.actions() {
            let Some(action) = action.as_widget_action() else {
                continue;
            };

            if action.widget_uid != self.messages_ref().widget_uid() {
                continue;
            }

            let chat_controller = self.chat_controller.clone().unwrap();

            match action.cast::<MessagesAction>() {
                MessagesAction::Delete(index) => chat_controller
                    .lock()
                    .unwrap()
                    .dispatch_mutation(VecMutation::<Message>::RemoveOne(index)),
                MessagesAction::Copy(index) => {
                    let lock = chat_controller.lock().unwrap();
                    let text = &lock.state().messages[index].content.text;
                    cx.copy_to_clipboard(text);
                }
                MessagesAction::EditSave(index) => {
                    let text = self
                        .messages_ref()
                        .read()
                        .current_editor_text()
                        .expect("no editor text");

                    self.messages_ref()
                        .write()
                        .set_message_editor_visibility(index, false);

                    let mut lock = chat_controller.lock().unwrap();

                    let mutation =
                        VecMutation::update_with(&lock.state().messages, index, |message| {
                            message.update_content(move |content| {
                                content.text = text;
                            });
                        });

                    lock.dispatch_mutation(mutation);
                }
                MessagesAction::EditRegenerate(index) => {
                    let mut messages =
                        chat_controller.lock().unwrap().state().messages[0..=index].to_vec();

                    let text = self
                        .messages_ref()
                        .read()
                        .current_editor_text()
                        .expect("no editor text");

                    self.messages_ref()
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
                        .map(|c| c.lock().unwrap().state().bot_id.is_some())
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

                    let mut updated_message = lock.state().messages[index].clone();

                    for tool_call in &mut updated_message.content.tool_calls {
                        tool_call.permission_status = ToolCallPermissionStatus::Approved;
                    }

                    lock.dispatch_mutation(VecMutation::Update(index, updated_message));

                    let tools = lock.state().messages[index].content.tool_calls.clone();
                    let bot_id = lock.state().bot_id.clone();
                    lock.dispatch_task(ChatTask::Execute(tools, bot_id));
                }
                MessagesAction::ToolDeny(index) => {
                    let mut lock = chat_controller.lock().unwrap();

                    let mut updated_message = lock.state().messages[index].clone();

                    updated_message.update_content(|content| {
                        for tool_call in &mut content.tool_calls {
                            tool_call.permission_status = ToolCallPermissionStatus::Denied;
                        }
                    });

                    lock.dispatch_mutation(VecMutation::Update(index, updated_message));

                    // Create synthetic tool results indicating denial to maintain conversation flow
                    let tool_results: Vec<ToolResult> = lock.state().messages[index]
                        .content
                        .tool_calls
                        .iter()
                        .map(|tc| {
                            let display_name = display_name_from_namespaced(&tc.name);
                            ToolResult {
                                tool_call_id: tc.id.clone(),
                                content: format!(
                                    "Tool execution was denied by the user. Tool '{}' was not executed.",
                                    display_name
                                ),
                                is_error: true,
                            }
                        })
                        .collect();

                    // Add tool result message with denial results
                    lock.dispatch_mutation(VecMutation::Push(Message {
                        from: EntityId::Tool,
                        content: MessageContent {
                            text: "🚫 Tool execution was denied by the user.".to_string(),
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
        let mut prompt = self.prompt_input_ref();
        let chat_controller = self.chat_controller.clone().unwrap();

        if prompt.read().has_send_task()
            && self
                .chat_controller
                .as_ref()
                .map(|c| c.lock().unwrap().state().bot_id.is_some())
                .unwrap_or(false)
        {
            let text = prompt.text();
            let attachments = prompt
                .read()
                .attachment_list_ref()
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
        // Use the standard send mechanism which will return the upgrade
        // The upgrade message will be processed in the plugin.
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

    pub fn set_chat_controller(
        &mut self,
        _cx: &mut Cx,
        chat_controller: Option<Arc<Mutex<ChatController>>>,
    ) {
        if self.chat_controller.as_ref().map(Arc::as_ptr)
            == chat_controller.as_ref().map(Arc::as_ptr)
        {
            return;
        }

        self.unlink_current_controller();
        self.chat_controller = chat_controller;

        self.messages_ref().write().chat_controller = self.chat_controller.clone();
        self.realtime(ids!(realtime))
            .set_chat_controller(self.chat_controller.clone());
        self.prompt_input_ref()
            .write()
            .set_chat_controller(self.chat_controller.clone());

        if let Some(controller) = self.chat_controller.as_ref() {
            let mut guard = controller.lock().unwrap();

            let plugin = Plugin::new(self.ui_runner());
            self.plugin_id = Some(guard.append_plugin(plugin));
        }
    }

    pub fn chat_controller(&self) -> Option<&Arc<Mutex<ChatController>>> {
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
        self.prompt_input_ref().write().set_stop();
        self.messages_ref().write().animated_scroll_to_bottom(cx);
        self.redraw(cx);
    }

    fn handle_streaming_end(&mut self, cx: &mut Cx) {
        self.prompt_input_ref().write().set_send();
        self.redraw(cx);
    }
}

// TODO: Since `ChatRef` is generated by a macro, I can't document this to give
// these functions better visibility from the module view.
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
    pub fn write_with<R>(&mut self, f: impl FnOnce(&mut Chat) -> R) -> R {
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
    fn on_state_ready(&mut self, _state: &ChatState, mutations: &[ChatStateMutation]) {
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
                        // Check if currently selected bot is still in the list
                        if let Some(controller) = &chat.chat_controller {
                            let mut lock = controller.lock().unwrap();
                            if let Some(bot_id) = lock.state().bot_id.clone() {
                                let bot_still_available =
                                    lock.state().bots.iter().any(|b| &b.id == &bot_id);
                                if !bot_still_available {
                                    // Selected bot was removed/disabled - clear selection
                                    lock.dispatch_mutation(ChatStateMutation::SetBotId(None));
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

    fn on_upgrade(&mut self, upgrade: Upgrade, bot_id: &BotId) -> Option<Upgrade> {
        match upgrade {
            Upgrade::Realtime(channel) => {
                let entity_id = EntityId::Bot(bot_id.clone());
                self.ui.defer(move |me, cx, _| {
                    me.handle_streaming_end(cx);

                    // Set up the realtime channel in the UI
                    let mut realtime = me.realtime(ids!(realtime));
                    realtime.set_bot_entity_id(cx, entity_id);
                    realtime.set_realtime_channel(channel.clone());

                    let modal = me.moly_modal(ids!(audio_modal));
                    modal.open_as_dialog(cx);
                });
                None
            }
            #[allow(unreachable_patterns)]
            upgrade => Some(upgrade),
        }
    }
}
