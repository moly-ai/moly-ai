use makepad_widgets::*;

use crate::{
    utils::makepad::{events::EventExt, hits::HitExt},
    widgets::moly_modal::{MolyModalRef, MolyModalWidgetExt},
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let Sender = View {
        height: Fit,
        spacing: 10,
        margin: Inset { bottom: 8 },
        align: Align { y: 0.5 }
        avatar := Avatar {}
        name := Label {
            padding: 0
            draw_text +: {
                text_style: theme.font_bold { font_size: 11 },
                color: #000
            }
        }
    }

    let ActionButton = Button {
        width: Fit
        height: Fit
        padding: Inset { top: 12, right: 12, bottom: 12, left: 12 }
        margin: 0
        align: Align { x: 0.0, y: 0.5 }
        draw_bg +: {
            pixel: fn() -> vec4 {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let color = mix(#xF2F4F700 #xEAECEF88 self.hover);
                let color = mix(color #xEAECEFFF self.down);

                sdf.box(0.0 0.0 self.rect_size.x self.rect_size.y 2.5);
                sdf.fill_keep(color);

                return sdf.result;
            }
        }

        icon_walk +: { width: 12, height: 12 }
        draw_icon +: {
            get_color: fn() -> vec4 {
                return #000;
            }
        }

        draw_text +: {
            text_style +: { font_size: 9 },
            get_color: fn() -> vec4 {
                return #000;
            }
        }
    }

    let EditActionButton = Button {
        padding: Inset { left: 10, right: 10, top: 4, bottom: 4 },
        draw_text +: {
            color: #000
            color_hover: #000
            color_focus: #000
        }
    }

    let EditActions = View {
        height: Fit,
        align: Align { y: 0.5 },
        spacing: 5
        save := EditActionButton { text: "save" }
        save_and_regenerate := EditActionButton { text: "save and regenerate" }
        cancel := EditActionButton { text: "cancel" }
    }

    let Editor = View {
        height: Fit,
        input := TextInput {
            padding: Inset { top: 8, bottom: 8, left: 10, right: 10 }
            width: Fill,
            empty_text: "\n",
            draw_bg +: {
                color: #fff,
                border_radius: 5.0,
                border_size: 0.0,
                color_focus: #fff
            }

            draw_selection +: {
                color: uniform(#xeee)
                color_hover: uniform(#xddd)
                color_focus: uniform(#xddd)
            }

            draw_text +: {
                color: #x0
                color_hover: uniform(#x0)
                color_focus: uniform(#x0)
            }
        }
    }

    mod.widgets.ChatLineBase = #(ChatLine::register_widget(vm))
    mod.widgets.ChatLine = set_type_default() do mod.widgets.ChatLineBase {
        flow: Down,
        height: Fit,
        padding: 10,
        show_bg: true,
        draw_bg +: {
            hover: instance(0.0)
            down: instance(0.0)

            pixel: fn() -> vec4 {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let color = mix(#xF2F4F700 #xEAECEF88 self.hover);
                let color = mix(color #xEAECEFFF self.down);

                sdf.box(0.0 0.0 self.rect_size.x self.rect_size.y 2.5);
                sdf.fill_keep(color);

                return sdf.result;
            }
        }

        message_section := RoundedView {
            flow: Down,
            height: Fit,
            sender := Sender {}
            content_section := View {
                height: Fit,
                margin: Inset { left: 32 }
                content := Slot { default: StandardMessageContent {} }
            }
            editor := Editor { margin: Inset { left: 32 }, visible: false }
        }
        actions_section := View {
            flow: Overlay,
            height: Fit,
            margin: Inset { left: 32 },
            edit_actions := EditActions { visible: false }
            actions_modal := MolyModal {
                content: RoundedView {
                    width: 100,
                    height: Fit,
                    flow: Down,

                    draw_bg +: {
                        color: #fff,
                        border_size: 1.0,
                        border_color: #xD0D5DD,
                    }

                    copy := ActionButton {
                        width: Fill,
                        text: "Copy"
                        draw_icon +: {
                            svg: crate_resource("self://resources/copy.svg")
                        }
                    }

                    edit := ActionButton {
                        width: Fill,
                        text: "Edit"
                        draw_icon +: {
                            svg: crate_resource("self://resources/edit.svg")
                        }
                    }

                    delete := ActionButton {
                        width: Fill,
                        text: "Delete"
                        draw_icon +: {
                            svg: crate_resource("self://resources/delete.svg")
                            get_color: fn() -> vec4 {
                                return #xB42318;
                            }
                        }
                        draw_text +: {
                            get_color: fn() -> vec4 {
                                return #xB42318;
                            }
                        }
                    }
                }
            }
        }
        animator: Animator {
            hover: {
                default: @off
                off: AnimatorState {
                    from: { all: Forward { duration: 0.15 } }
                    apply: {
                        draw_bg: { hover: 0.0 }
                    }
                }
                on: AnimatorState {
                    from: { all: Snap }
                    apply: {
                        draw_bg: { hover: 1.0 }
                    }
                }
            }
            down: {
                default: @off
                off: AnimatorState {
                    from: { all: Forward { duration: 0.5 } }
                    ease: OutExp
                    apply: {
                        draw_bg: { down: 0.0 }
                    }
                }
                on: AnimatorState {
                    ease: OutExp
                    from: {
                        all: Forward { duration: 0.2 }
                    }
                    apply: {
                        draw_bg: { down: 1.0 }
                    }
                }
            }
        }
    }

    mod.widgets.UserLine = mod.widgets.ChatLine {
        message_section +: {
            sender +: {
                avatar +: {
                    grapheme +: {
                        draw_bg +: {
                            color: #x008F7E
                        }
                    }
                }
            }
        }
    }

    mod.widgets.BotLine = mod.widgets.ChatLine {}

    mod.widgets.LoadingLine = mod.widgets.BotLine {
        message_section +: {
            content_section := View {
                height: Fit,
                padding: Inset { left: 32 }
                loading := MessageLoading {}
            }
        }
    }

    // Note: For now, let's use bot's apparence for app messages.
    mod.widgets.AppLine = mod.widgets.BotLine {
        margin: Inset { left: 0 }
        message_section +: {
            padding: Inset { left: 12, right: 12, top: 12, bottom: 0 }
            draw_bg +: {
                border_color: #344054
                border_size: 1.2
                border_radius: 8.0
            }
            sender +: {
                margin: Inset { bottom: 5 }
                avatar +: {
                    grapheme +: { draw_bg +: { color: #344054 } }
                }
                name +: { text: "Application" }
            }
        }
        actions_section +: {
            margin: Inset { left: 32 }
        }
    }

    mod.widgets.ErrorLine = mod.widgets.AppLine {
        message_section +: {
            draw_bg +: { color: #xf003 }

            sender +: {
                avatar +: {
                    grapheme +: { draw_bg +: { color: #xf003 } }
                }
            }
            content_section +: {
                flow: Down,
                padding: Inset { bottom: 10 },
                error_details_section := View {
                    flow: Down,
                    width: Fill,
                    height: Fit,
                    visible: false,

                    error_note := Label {
                        width: Fill,
                        margin: Inset { top: 6 },
                        draw_text +: {
                            text_style: theme.font_italic { font_size: 10 },
                            color: #555,
                        }
                    }
                    error_details_toggle := View {
                        width: Fit,
                        height: Fit,
                        cursor: MouseCursor.Hand,
                        margin: Inset { top: 6 },
                        toggle_label := Label {
                            text: "Show details",
                            draw_text +: {
                                text_style +: { font_size: 9.5 },
                                color: #x1a5b9c,
                            }
                        }
                    }
                    error_details := RoundedView {
                        width: Fill,
                        height: Fit,
                        visible: false,
                        margin: Inset { top: 4 },
                        padding: 8,
                        draw_bg +: {
                            color: #x0001,
                            border_radius: 4.0,
                        }
                        details_text := Label {
                            width: Fill,
                            draw_text +: {
                                text_style +: { font_size: 9 },
                                color: #333,
                            }
                        }
                    }
                }
            }
        }
    }

    mod.widgets.SystemLine = mod.widgets.AppLine {
        message_section +: {
            draw_bg +: { color: #xe3f2fd }

            sender +: {
                avatar +: {
                    grapheme +: { draw_bg +: { color: #x1976d2 } }
                }
                name +: { text: "System" }
            }
        }
    }

    let ToolApprovalButton = Button {
        padding: Inset { left: 15, right: 15, top: 8, bottom: 8 },
        draw_text +: {
            text_style: theme.font_bold { font_size: 10 },
            color: #fff
            color_hover: #fff
            color_focus: #fff
        }
    }

    let ToolApprovalActions = View {
        width: Fill, height: Fit,
        align: Align { y: 0.5 },
        spacing: 5,
        padding: Inset { bottom: 8 }
        approve := ToolApprovalButton {
            text: "Approve",
            draw_bg +: { color: #x4CAF50, color_hover: #x45a049 }
        }
        deny := ToolApprovalButton {
            text: "Deny",
            draw_bg +: { color: #xf44336, color_hover: #xd32f2f }
        }
    }

    mod.widgets.ToolRequestLine = mod.widgets.AppLine {
        message_section +: {
            draw_bg +: { color: #xfff3e0 }

            sender +: {
                avatar +: {
                    grapheme +: { draw_bg +: { color: #xff9800 } }
                }
            }
            content_section +: {
                flow: Down
                tool_actions := ToolApprovalActions { visible: false }
                status_view := View {
                    visible: false
                    width: Fill, height: Fit,
                    align: Align { x: 1.0, y: 0.5 }
                    padding: Inset { bottom: 8, right: 10 }
                    approved_status := Label {
                        draw_text +: {
                            text_style: theme.font_bold { font_size: 11 },
                            color: #000
                        }
                    }
                }
            }
        }
    }

    mod.widgets.ToolResultLine = mod.widgets.AppLine {
        message_section +: {
            draw_bg +: { color: #xcfe4ff }

            sender +: {
                avatar +: {
                    grapheme +: { draw_bg +: { color: #x1a5b9c } }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ChatLineAction {
    Copy,
    Edit,
    Delete,
    Save,
    SaveAndRegenerate,
    EditCancel,
    ToolApprove,
    ToolDeny,
    EditorChanged,
    ErrorDetailsToggle,
    #[default]
    None,
}

#[derive(Script, ScriptHook, Widget, Animator)]
pub struct ChatLine {
    #[deref]
    deref: View,
    #[source]
    source: ScriptObjectRef,
    #[apply_default]
    animator: Animator,
}

impl Widget for ChatLine {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope);
        let actions = event.actions();

        if self.copy_ref(cx).clicked(actions) {
            self.actions_modal_ref(cx).close(cx);
            cx.widget_action(self.widget_uid(), ChatLineAction::Copy);
        }

        if self.edit_ref(cx).clicked(actions) {
            self.actions_modal_ref(cx).close(cx);
            cx.widget_action(self.widget_uid(), ChatLineAction::Edit);
        }

        if self.delete_ref(cx).clicked(actions) {
            self.actions_modal_ref(cx).close(cx);
            cx.widget_action(self.widget_uid(), ChatLineAction::Delete);
        }

        if self.save_ref(cx).clicked(actions) {
            cx.widget_action(self.widget_uid(), ChatLineAction::Save);
        }

        if self.save_and_regenerate_ref(cx).clicked(actions) {
            cx.widget_action(self.widget_uid(), ChatLineAction::SaveAndRegenerate);
        }

        if self.cancel_ref(cx).clicked(actions) {
            cx.widget_action(self.widget_uid(), ChatLineAction::EditCancel);
        }

        if self.approve_ref(cx).clicked(actions) {
            cx.widget_action(self.widget_uid(), ChatLineAction::ToolApprove);
        }

        if self.deny_ref(cx).clicked(actions) {
            cx.widget_action(self.widget_uid(), ChatLineAction::ToolDeny);
        }

        if self.input_ref(cx).changed(actions).is_some() {
            cx.widget_action(self.widget_uid(), ChatLineAction::EditorChanged);
        }

        if self
            .view(cx, ids!(error_details_toggle))
            .finger_up(&actions)
            .is_some()
        {
            cx.widget_action(self.widget_uid(), ChatLineAction::ErrorDetailsToggle);
        }

        if let Some(pos) = event.hits(cx, self.area()).secondary_pointer_action_pos() {
            self.dismiss_all_hovers(cx);
            self.actions_modal_ref(cx).open_as_popup(cx, pos);
        }
    }
}

impl ChatLine {
    fn copy_ref(&self, cx: &mut Cx) -> ButtonRef {
        self.button(cx, ids!(copy))
    }

    fn edit_ref(&self, cx: &mut Cx) -> ButtonRef {
        self.button(cx, ids!(edit))
    }

    fn delete_ref(&self, cx: &mut Cx) -> ButtonRef {
        self.button(cx, ids!(delete))
    }

    fn approve_ref(&self, cx: &mut Cx) -> ButtonRef {
        self.button(cx, ids!(approve))
    }

    fn deny_ref(&self, cx: &mut Cx) -> ButtonRef {
        self.button(cx, ids!(deny))
    }

    fn input_ref(&self, cx: &mut Cx) -> TextInputRef {
        self.text_input(cx, ids!(input))
    }

    fn save_ref(&self, cx: &mut Cx) -> ButtonRef {
        self.button(cx, ids!(edit_actions.save))
    }

    fn save_and_regenerate_ref(&self, cx: &mut Cx) -> ButtonRef {
        self.button(cx, ids!(edit_actions.save_and_regenerate))
    }

    fn cancel_ref(&self, cx: &mut Cx) -> ButtonRef {
        self.button(cx, ids!(edit_actions.cancel))
    }

    fn actions_modal_ref(&self, cx: &mut Cx) -> MolyModalRef {
        self.moly_modal(cx, ids!(actions_modal))
    }

    fn dismiss_all_hovers(&mut self, cx: &mut Cx) {
        self.animator_cut(cx, ids!(hover.off));
        self.animator_cut(cx, ids!(down.off));
        self.copy_ref(cx).reset_hover(cx);
        self.edit_ref(cx).reset_hover(cx);
        self.delete_ref(cx).reset_hover(cx);
    }
}
