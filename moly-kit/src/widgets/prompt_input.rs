use makepad_widgets::*;
use std::cell::{Ref, RefMut};

#[allow(unused)]
use crate::{
    aitk::protocol::*,
    utils::makepad::events::EventExt,
    widgets::attachment_list::{AttachmentListRef, AttachmentListWidgetExt},
};

script_mod! {
    use mod.prelude.widgets.*

    let SubmitButton = Button {
        width: 28,
        height: 28,
        padding: Inset { right: 2 },
        margin: Inset { bottom: 2 },

        draw_icon +: {
            color: #fff
        }

        draw_bg +: {
            get_color: fn() {
                if self.enabled == 0.0 {
                    return #xD0D5DD;
                }
                return #000;
            }

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let center = self.rect_size * 0.5
                let radius = min(self.rect_size.x self.rect_size.y) * 0.5

                sdf.circle(center.x center.y radius)
                sdf.fill_keep(self.get_color())

                return sdf.result
            }
        }
        icon_walk: Walk {
            width: 12,
            height: 12
            margin: Inset { top: 0, left: 2 },
        }
    }

    let AttachButton = Button {
        visible: false
        text: "",
        width: Fit,
        height: Fit,
        padding: Inset { left: 8, right: 8, top: 6, bottom: 6 }
        draw_text +: {
            text_style: theme.font_icons { font_size: 13. }
            color: #333,
            color_hover: #111,
            color_focus: #111
            color_down: #000
        }
        draw_bg +: {
            color_down: #0000
            border_radius: 7.
            border_size: 0.
            color_hover: #xf2
        }
    }

    let AudioButton = Button {
        visible: false
        width: 28, height: 28
        text: ""
        draw_text +: {
            text_style: theme.font_icons { font_size: 13. }
            color: #333,
            color_hover: #111,
            color_focus: #111
            color_down: #000
        }
        draw_bg +: {
            color_down: #0000
            border_radius: 7.
            border_size: 0.
        }
    }

    let SttButton = Button {
        visible: false
        width: 28, height: 28
        text: ""
        draw_text +: {
            text_style: theme.font_icons { font_size: 13. }
            color: #333,
            color_hover: #111,
            color_focus: #111
            color_down: #000
        }
        draw_bg +: {
            color_down: #0000
            border_radius: 7.
            border_size: 0.
        }
    }

    let SendControls = View {
        width: Fit, height: Fit
        align: Align { x: 0.5, y: 0.5 }
        spacing: 10
        stt := SttButton {}
        audio := AudioButton {}
        submit := SubmitButton {}
    }

    mod.widgets.PromptInput = #(PromptInput::register_widget(vm)) CommandTextInput {
        send_icon: crate_resource("self://resources/send.svg"),
        stop_icon: crate_resource("self://resources/stop.svg"),
        height: Fit { max: 350 }
        persistent := {
            height: Fit
            padding: Inset { top: 10, bottom: 10, left: 10, right: 10 }
            draw_bg +: {
                color: #fff,
                border_radius: 10.0,
                border_color: #xD0D5DD,
                border_size: 1.0,
            }
            top := {
                height: Fit
                attachments := DenseAttachmentList {
                    wrapper := {}
                }
            }
            center := {
                height: Fit
                text_input := {
                    height: Fit {
                        min: 35
                        max: 180
                    }
                    width: Fill
                    empty_text: "Start typing...",
                    draw_bg +: {
                        pixel: fn() {
                            return vec4(0.)
                        }
                    }
                    draw_text +: {
                        color: #000
                        color_hover: #000
                        color_focus: #000
                        color_empty: #x98A2B3
                        color_empty_focus: #x98A2B3
                        text_style: { font_size: 11 }
                    }
                    draw_selection +: {
                        color: #xd9e7e9
                        color_hover: #xd9e7e9
                        color_focus: #xd9e7e9
                    }
                    draw_cursor +: {
                        color: #000
                    }
                }
                right := {
                    // In mobile, show the send controls here, right to the input
                }
            }
            bottom := {
                height: Fit
                left := View {
                    width: Fit, height: Fit
                    align: Align { x: 0.0, y: 0.5 }
                    attach := AttachButton {}
                    model_selector := ModelSelector {}
                }
                width: Fill, height: Fit
                separator := View { width: Fill, height: 1 }
                SendControls {}
            }
        }
    }
}

#[derive(Default, Copy, Clone, PartialEq)]
pub enum Task {
    #[default]
    Send,
    Stop,
}

#[derive(Default, Copy, Clone, PartialEq)]
pub enum Interactivity {
    #[default]
    Enabled,
    Disabled,
}

/// A prepared text input for conversation with bots.
///
/// This is mostly a dummy widget. Prefer using and adapting [crate::widgets::chat::Chat] instead.
#[derive(Script, Widget)]
pub struct PromptInput {
    #[deref]
    deref: CommandTextInput,

    /// Icon used by this widget when the task is set to [Task::Send].
    #[live]
    pub send_icon: Option<ScriptHandleRef>,

    /// Icon used by this widget when the task is set to [Task::Stop].
    #[live]
    pub stop_icon: Option<ScriptHandleRef>,

    /// If this widget should provoke sending a message or stopping the current response.
    #[rust]
    pub task: Task,

    /// If this widget should be interactive or not.
    #[rust]
    pub interactivity: Interactivity,

    /// Capabilities of the currently selected bot
    #[rust]
    pub bot_capabilities: Option<BotCapabilities>,
}

impl ScriptHook for PromptInput {
    fn on_after_new(&mut self, _vm: &mut ScriptVm) {
        // Cannot call update_button_visibility here because we don't have cx.
        // It will be called later when bot capabilities are set.
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

        if self.button(ids!(attach)).clicked(event.actions()) {
            let ui = self.ui_runner();
            Attachment::pick_multiple(move |result| match result {
                Ok(attachments) => {
                    ui.defer_with_redraw(move |me, _, _| {
                        let mut list = me.attachment_list_ref();
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
        let button = self.button(ids!(submit));

        match self.task {
            Task::Send => {
                if let Some(icon) = &self.send_icon {
                    let icon = icon.clone();
                    script_apply_eval!(cx, button, {
                        draw_icon: { svg: #(icon) }
                    });
                }
            }
            Task::Stop => {
                if let Some(icon) = &self.stop_icon {
                    let icon = icon.clone();
                    script_apply_eval!(cx, button, {
                        draw_icon: { svg: #(icon) }
                    });
                }
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
    /// Reset this prompt input erasing text, removing attachments, etc.
    ///
    /// Shadows the [`CommandTextInput::reset`] method.
    pub fn reset(&mut self, cx: &mut Cx) {
        self.deref.reset(cx);
        self.attachment_list_ref().write().attachments.clear();
    }

    /// Check if the submit button or the return key was pressed.
    ///
    /// Note: To know what the button submission means, check [Self::task] or
    /// the utility methods.
    pub fn submitted(&self, actions: &Actions) -> bool {
        let submit = self.button(ids!(submit));
        let input = self.text_input_ref();
        (submit.clicked(actions) || input.returned(actions).is_some())
            && self.interactivity == Interactivity::Enabled
    }

    pub fn call_pressed(&self, actions: &Actions) -> bool {
        self.button(ids!(audio)).clicked(actions)
    }

    pub fn stt_pressed(&self, actions: &Actions) -> bool {
        self.button(ids!(stt)).clicked(actions)
    }

    /// Shorthand to check if [Self::task] is set to [Task::Send].
    pub fn has_send_task(&self) -> bool {
        self.task == Task::Send
    }

    /// Shorthand to check if [Self::task] is set to [Task::Stop].
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

    /// Shorthand to set [Self::task] to [Task::Send].
    pub fn set_send(&mut self) {
        self.task = Task::Send;
    }

    /// Shorthand to set [Self::task] to [Task::Stop].
    pub fn set_stop(&mut self) {
        self.task = Task::Stop;
    }

    pub(crate) fn attachment_list_ref(&self) -> AttachmentListRef {
        self.attachment_list(ids!(attachments))
    }

    /// Set the chat controller for the model selector
    pub fn set_chat_controller(
        &mut self,
        controller: Option<
            std::sync::Arc<std::sync::Mutex<crate::aitk::controllers::chat::ChatController>>,
        >,
    ) {
        if let Some(mut inner) = self
            .widget(ids!(model_selector))
            .borrow_mut::<crate::widgets::model_selector::ModelSelector>()
        {
            inner.chat_controller = controller;
        }
    }

    /// Set the capabilities of the currently selected bot
    pub fn set_bot_capabilities(&mut self, cx: &mut Cx, capabilities: Option<BotCapabilities>) {
        self.bot_capabilities = capabilities;
        self.update_button_visibility(cx);
    }

    pub fn set_stt_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.button(ids!(stt)).set_visible(cx, visible);
    }

    /// Update button visibility based on bot capabilities
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
        self.button(ids!(attach))
            .set_visible(cx, supports_attachments);

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "linux",
            target_arch = "wasm32"
        )))]
        self.button(ids!(attach)).set_visible(cx, false);

        #[cfg(not(target_arch = "wasm32"))]
        self.button(ids!(audio)).set_visible(cx, supports_realtime);

        self.button(ids!(submit))
            .set_visible(cx, !supports_realtime);

        if supports_realtime {
            self.interactivity = Interactivity::Disabled;
            self.text_input_ref().set_is_read_only(cx, true);
            self.text_input_ref().set_empty_text(
                cx,
                "For realtime models, use the audio feature ->".to_string(),
            );
            self.redraw(cx);
        } else {
            self.interactivity = Interactivity::Enabled;
            self.text_input_ref().set_is_read_only(cx, false);
            self.text_input_ref().set_text(cx, "");
            self.redraw(cx);
        }
    }
}

impl PromptInputRef {
    /// Immutable access to the underlying [[PromptInput]].
    ///
    /// Panics if the widget reference is empty or if it's already borrowed.
    pub fn read(&self) -> Ref<'_, PromptInput> {
        self.borrow().unwrap()
    }

    /// Mutable access to the underlying [[PromptInput]].
    ///
    /// Panics if the widget reference is empty or if it's already borrowed.
    pub fn write(&mut self) -> RefMut<'_, PromptInput> {
        self.borrow_mut().unwrap()
    }

    /// Immutable reader to the underlying [[PromptInput]].
    ///
    /// Panics if the widget reference is empty or if it's already borrowed.
    pub fn read_with<R>(&self, f: impl FnOnce(&PromptInput) -> R) -> R {
        f(&*self.read())
    }

    /// Mutable writer to the underlying [[PromptInput]].
    ///
    /// Panics if the widget reference is empty or if it's already borrowed.
    pub fn write_with<R>(&mut self, f: impl FnOnce(&mut PromptInput) -> R) -> R {
        f(&mut *self.write())
    }
}
