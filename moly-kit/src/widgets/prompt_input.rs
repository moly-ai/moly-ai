use makepad_widgets::command_text_input::CommandTextInput;
use makepad_widgets::defer_with_redraw::DeferWithRedraw;
use makepad_widgets::*;
use std::cell::{Ref, RefMut};

#[allow(unused)]
use crate::{
    aitk::protocol::*,
    utils::makepad::events::EventExt,
    widgets::attachment_list::{AttachmentListRef, AttachmentListWidgetExt},
};

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    let SubmitButton = Button {
        width: 28
        height: 28
        padding: Inset{right: 2}
        margin: Inset{bottom: 2}

        draw_icon: {
            color: #xfff
        }

        draw_bg: {
            fn get_color(self) -> vec4 {
                if self.enabled == 0.0 {
                    return #xD0D5DD
                }
                return #x000
            }

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let center = self.rect_size * 0.5
                let radius = min(self.rect_size.x, self.rect_size.y) * 0.5

                sdf.circle(center.x, center.y, radius)
                sdf.fill_keep(self.get_color())

                return sdf.result
            }
        }
        icon_walk: {
            width: 12
            height: 12
            margin: Inset{top: 0 left: 2}
        }
    }

    let AttachButton = Button {
        visible: false
        text: "\u{f0c6}"
        width: Fit
        height: Fit
        padding: Inset{left: 8 right: 8 top: 6 bottom: 6}
        draw_text: {
            text_style +: theme.font_icons {
                font_size: 13.
            }
            color: #x333
            color_hover: #x111
            color_focus: #x111
            color_down: #x000
        }
        draw_bg: {
            color_down: #x0000
            radius: 7.
            border_size: 0.
            color_hover: #xf2
        }
    }

    let AudioButton = Button {
        visible: false
        width: 28 height: 28
        text: "\u{f095}"
        draw_text: {
            text_style +: theme.font_icons {
                font_size: 13.
            }
            color: #x333
            color_hover: #x111
            color_focus: #x111
            color_down: #x000
        }
        draw_bg: {
            color_down: #x0000
            radius: 7.
            border_size: 0.
        }
    }

    let SttButton = Button {
        visible: false
        width: 28 height: 28
        text: "\u{f130}"
        draw_text: {
            text_style +: theme.font_icons {
                font_size: 13.
            }
            color: #x333
            color_hover: #x111
            color_focus: #x111
            color_down: #x000
        }
        draw_bg: {
            color_down: #x0000
            radius: 7.
            border_size: 0.
        }
    }

    let SendControls = View {
        width: Fit height: Fit
        align: Align{x: 0.5 y: 0.5}
        spacing: 10
        stt := SttButton {}
        audio := AudioButton {}
        submit := SubmitButton {}
    }

    mod.widgets.PromptInputBase =
        #(PromptInput::register_widget(vm))

    mod.widgets.PromptInput =
        set_type_default() do mod.widgets.PromptInputBase
            CommandTextInput {
        send_icon: crate_resource("self://resources/send.svg")
        stop_icon: crate_resource("self://resources/stop.svg")
        height: Fit
        persistent: {
            height: Fit
            padding: Inset{top: 10 bottom: 10 left: 10 right: 10}
            draw_bg: {
                color: #xfff
                radius: 10.0
                border_color: #xD0D5DD
                border_size: 1.0
            }
            top: {
                height: Fit
                attachments := DenseAttachmentList {
                    wrapper: {}
                }
            }
            center: {
                height: Fit
                text_input: {
                    height: Fit
                    width: Fill
                    empty_text: "Start typing..."
                    draw_bg: {
                        pixel: fn() {
                            return vec4(0.)
                        }
                    }
                    draw_text: {
                        color: #x000
                        color_hover: #x000
                        color_focus: #x000
                        color_empty: #x98A2B3
                        color_empty_focus: #x98A2B3
                        text_style +: {font_size: 11}
                    }
                    draw_selection: {
                        color: #xd9e7e9
                        color_hover: #xd9e7e9
                        color_focus: #xd9e7e9
                    }
                    draw_cursor: {
                        color: #x000
                    }
                }
                right: {
                    // In mobile, show the send controls here
                }
            }
            bottom: {
                height: Fit
                left := View {
                    width: Fit height: Fit
                    align: Align{x: 0.0 y: 0.5}
                    attach := AttachButton {}
                    model_selector := ModelSelector {}
                }
                width: Fill height: Fit
                separator := View {width: Fill height: 1}
                SendControls {}
            }
        }
    }
}

/// Whether the submit button should send a message or stop streaming.
#[derive(Default, Copy, Clone, PartialEq)]
pub enum Task {
    #[default]
    Send,
    Stop,
}

/// Whether the prompt input accepts user interaction.
#[derive(Default, Copy, Clone, PartialEq)]
pub enum Interactivity {
    #[default]
    Enabled,
    Disabled,
}

/// A prepared text input for conversation with bots.
///
/// This is mostly a dummy widget. Prefer using and adapting
/// [`crate::widgets::chat::Chat`] instead.
#[derive(Script, Widget)]
pub struct PromptInput {
    #[deref]
    deref: CommandTextInput,

    /// Icon used when the task is set to [`Task::Send`].
    #[live]
    pub send_icon: Option<ScriptHandleRef>,

    /// Icon used when the task is set to [`Task::Stop`].
    #[live]
    pub stop_icon: Option<ScriptHandleRef>,

    /// Whether this widget should send a message or stop streaming.
    #[rust]
    pub task: Task,

    /// Whether this widget should be interactive.
    #[rust]
    pub interactivity: Interactivity,

    /// Capabilities of the currently selected bot.
    #[rust]
    pub bot_capabilities: Option<BotCapabilities>,
}

impl ScriptHook for PromptInput {
    #[allow(unused)]
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        // We can't call update_button_visibility here because we don't
        // have Cx. The visibility will be updated on the first draw or
        // when capabilities are set.
    }
}

impl Widget for PromptInput {
    fn set_text(&mut self, cx: &mut Cx, v: &str) {
        self.deref.set_text(cx, v);
    }

    fn text(&self) -> String {
        self.deref.text()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope);
        self.ui_runner().handle(cx, event, scope, self);

        if self.button(cx, ids!(attach)).clicked(event.actions()) {
            let ui = self.ui_runner();
            Attachment::pick_multiple(move |result| match result {
                Ok(attachments) => {
                    ui.defer_with_redraw(move |me: &mut PromptInput, cx, _| {
                        let mut list = me.attachment_list_ref(cx);
                        list.write().attachments.extend(attachments);
                        list.write().on_tap(move |list, index| {
                            list.attachments.remove(index);
                        });
                    });
                }
                Err(_) => {}
            });
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let mut button = self.button(cx, ids!(submit));

        match self.task {
            Task::Send => {
                let icon = self.send_icon.clone();
                script_apply_eval!(cx, button, {
                    draw_icon: {
                        svg: #(icon)
                    }
                });
            }
            Task::Stop => {
                let icon = self.stop_icon.clone();
                script_apply_eval!(cx, button, {
                    draw_icon: {
                        svg: #(icon)
                    }
                });
            }
        }

        match self.interactivity {
            Interactivity::Enabled => {
                script_apply_eval!(cx, button, {
                    draw_bg: {
                        enabled: 1.0
                    }
                });
                button.set_enabled(cx, true);
            }
            Interactivity::Disabled => {
                script_apply_eval!(cx, button, {
                    draw_bg: {
                        enabled: 0.0
                    }
                });
                button.set_enabled(cx, false);
            }
        }

        self.deref.draw_walk(cx, scope, walk)
    }
}

impl PromptInput {
    /// Resets this prompt input, erasing text and removing attachments.
    ///
    /// Shadows the [`CommandTextInput::reset`] method.
    pub fn reset(&mut self, cx: &mut Cx) {
        self.deref.reset(cx);
        self.attachment_list_ref(cx).write().attachments.clear();
    }

    /// Returns whether the submit button or the return key was pressed.
    ///
    /// To know what the submission means, check [`Self::task`].
    pub fn submitted(&self, cx: &Cx, actions: &Actions) -> bool {
        let submit = self.button(cx, ids!(submit));
        let input = self.text_input_ref(cx);
        (submit.clicked(actions) || input.returned(actions).is_some())
            && self.interactivity == Interactivity::Enabled
    }

    /// Returns whether the call/audio button was pressed.
    pub fn call_pressed(&self, cx: &Cx, actions: &Actions) -> bool {
        self.button(cx, ids!(audio)).clicked(actions)
    }

    /// Returns whether the STT button was pressed.
    pub fn stt_pressed(&self, cx: &Cx, actions: &Actions) -> bool {
        self.button(cx, ids!(stt)).clicked(actions)
    }

    /// Returns whether [`Self::task`] is [`Task::Send`].
    pub fn has_send_task(&self) -> bool {
        self.task == Task::Send
    }

    /// Returns whether [`Self::task`] is [`Task::Stop`].
    pub fn has_stop_task(&self) -> bool {
        self.task == Task::Stop
    }

    /// Allows submission.
    pub fn enable(&mut self) {
        self.interactivity = Interactivity::Enabled;
    }

    /// Disallows submission.
    pub fn disable(&mut self) {
        self.interactivity = Interactivity::Disabled;
    }

    /// Sets the task to [`Task::Send`].
    pub fn set_send(&mut self) {
        self.task = Task::Send;
    }

    /// Sets the task to [`Task::Stop`].
    pub fn set_stop(&mut self) {
        self.task = Task::Stop;
    }

    pub(crate) fn attachment_list_ref(&self, cx: &Cx) -> AttachmentListRef {
        self.attachment_list(cx, ids!(attachments))
    }

    /// Sets the chat controller for the model selector.
    pub fn set_chat_controller(
        &mut self,
        cx: &Cx,
        controller: Option<
            std::sync::Arc<std::sync::Mutex<crate::aitk::controllers::chat::ChatController>>,
        >,
    ) {
        if let Some(mut inner) = self
            .widget(cx, ids!(model_selector))
            .borrow_mut::<crate::widgets::model_selector::ModelSelector>()
        {
            inner.chat_controller = controller;
        }
    }

    /// Sets the capabilities of the currently selected bot.
    pub fn set_bot_capabilities(&mut self, cx: &mut Cx, capabilities: Option<BotCapabilities>) {
        self.bot_capabilities = capabilities;
        self.update_button_visibility(cx);
    }

    /// Sets visibility of the STT button.
    pub fn set_stt_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.button(cx, ids!(stt)).set_visible(cx, visible);
    }

    fn update_button_visibility(&mut self, cx: &mut Cx) {
        let supports_attachments = self
            .bot_capabilities
            .as_ref()
            .map(|caps| caps.has_capability(&BotCapability::AttachmentInput))
            .unwrap_or(false);

        let supports_realtime = self
            .bot_capabilities
            .as_ref()
            .map(|caps| caps.has_capability(&BotCapability::AudioCall))
            .unwrap_or(false);

        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "linux",
            target_arch = "wasm32"
        ))]
        self.button(cx, ids!(attach))
            .set_visible(cx, supports_attachments);

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "linux",
            target_arch = "wasm32"
        )))]
        self.button(cx, ids!(attach)).set_visible(cx, false);

        #[cfg(not(target_arch = "wasm32"))]
        self.button(cx, ids!(audio))
            .set_visible(cx, supports_realtime);

        self.button(cx, ids!(submit))
            .set_visible(cx, !supports_realtime);

        if supports_realtime {
            self.interactivity = Interactivity::Disabled;
            self.text_input_ref(cx).set_is_read_only(cx, true);
            self.text_input_ref(cx).set_empty_text(
                cx,
                "For realtime models, use the audio feature ->".to_string(),
            );
            self.redraw(cx);
        } else {
            self.interactivity = Interactivity::Enabled;
            self.text_input_ref(cx).set_is_read_only(cx, false);
            self.text_input_ref(cx).set_text(cx, "");
            self.redraw(cx);
        }
    }
}

impl PromptInputRef {
    /// Immutable access to the underlying [`PromptInput`].
    ///
    /// Panics if the widget reference is empty or already borrowed.
    pub fn read(&self) -> Ref<'_, PromptInput> {
        self.borrow().unwrap()
    }

    /// Mutable access to the underlying [`PromptInput`].
    ///
    /// Panics if the widget reference is empty or already borrowed.
    pub fn write(&mut self) -> RefMut<'_, PromptInput> {
        self.borrow_mut().unwrap()
    }

    /// Immutable reader to the underlying [`PromptInput`].
    ///
    /// Panics if the widget reference is empty or already borrowed.
    pub fn read_with<R>(&self, f: impl FnOnce(&PromptInput) -> R) -> R {
        f(&*self.read())
    }

    /// Mutable writer to the underlying [`PromptInput`].
    ///
    /// Panics if the widget reference is empty or already borrowed.
    pub fn write_with<R>(&mut self, f: impl FnOnce(&mut PromptInput) -> R) -> R {
        f(&mut *self.write())
    }
}
