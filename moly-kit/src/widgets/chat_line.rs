use makepad_widgets::*;

use crate::{
    MolyModalWidgetExt,
    utils::makepad::{events::EventExt, hits::HitExt},
};

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use link::moly_kit_theme::*;
    use link::shaders::*;

    use crate::widgets::standard_message_content::*;
    use crate::widgets::message_loading::*;
    use crate::widgets::avatar::*;
    use crate::widgets::slot::*;
    use crate::widgets::moly_modal::*;

    Sender = <View> {
        height: Fit,
        spacing: 10,
        margin: {bottom: 8},
        align: {y: 0.5}
        avatar = <Avatar> {}
        name = <Label> {
            padding: 0
            draw_text:{
                text_style: <THEME_FONT_BOLD>{font_size: 11},
                color: #000
            }
        }
    }

    ActionButton = <Button> {
        width: Fit
        height: Fit
        padding: { top: 12, right: 12, bottom: 12, left: 12}
        align: {x: 0.0, y: 0.5}

        draw_bg: {
            border_size: 0,
            border_radius: 0
        }

        icon_walk: {width: 12, height: 12}
        draw_icon: {
            fn get_color(self) -> vec4 {
                return #000;
            }
        }

        draw_text: {
            text_style: {font_size: 9},
            fn get_color(self) -> vec4 {
                return #000;
            }
        }
    }

    EditActionButton = <Button> {
        padding: {left: 10, right: 10, top: 4, bottom: 4},
        draw_text: {
            color: #000
            color_hover: #000
            color_focus: #000
        }
    }

    EditActions = <View> {
        height: Fit,
        align: {y: 0.5},
        spacing: 5
        save = <EditActionButton> { text: "save" }
        save_and_regenerate = <EditActionButton> { text: "save and regenerate" }
        cancel = <EditActionButton> { text: "cancel" }
    }

    Editor = <View> {
        height: Fit,
        input = <TextInput> {
            padding: {top: 8, bottom: 8, left: 10, right: 10}
            width: Fill,
            empty_text: "\n",
            draw_bg: {
                color: #fff,
                border_radius: 5.0,
                border_size: 0.0,
                color_focus: #fff
            }

            draw_selection: {
                uniform color: #eee
                uniform color_hover: #ddd
                uniform color_focus: #ddd
            }

            draw_text: {
                color: #x0
                uniform color_hover: #x0
                uniform color_focus: #x0
            }
        }
    }

    pub ChatLine = {{ChatLine}} <RoundedView> {
        flow: Down,
        height: Fit,
        margin: {left: 10, right: 10}
        message_section = <RoundedView> {
            flow: Down,
            height: Fit,
            sender = <Sender> {}
            content_section = <View> {
                height: Fit,
                margin: { left: 32 }
                content = <Slot> { default: <StandardMessageContent> {} }
            }
            editor = <Editor> { margin: { left: 32 }, visible: false }
        }
        actions_section = <View> {
            flow: Overlay,
            height: Fit,
            margin: {left: 32},
            edit_actions = <EditActions> { visible: false }
            actions_modal = <MolyModal> {
                content: <RoundedView> {
                    width: 100,
                    height: Fit,
                    flow: Down,

                    draw_bg: {
                        color: #fff,
                        border_size: 1.0,
                        border_color: #D0D5DD,
                    }

                    copy = <ActionButton> {
                        width: Fill,
                        text: "Copy"
                        draw_icon: {
                            svg_file: dep("crate://self/resources/copy.svg")
                        }
                    }

                    edit = <ActionButton> {
                        width: Fill,
                        text: "Edit"
                        draw_icon: {
                            svg_file: dep("crate://self/resources/edit.svg")
                        }
                    }

                    delete = <ActionButton> {
                        width: Fill,
                        text: "Delete"
                        draw_icon: {
                            svg_file: dep("crate://self/resources/delete.svg")
                            fn get_color(self) -> vec4 {
                                return #B42318;
                            }
                        }
                        draw_text: {
                            fn get_color(self) -> vec4 {
                                return #B42318;
                            }
                        }
                    }
                }
            }
        }
    }

    pub UserLine = <ChatLine> {
        message_section = {
            sender = {
                avatar = {
                    grapheme = {
                        draw_bg: {
                            color: #008F7E
                        }
                    }
                }
            }
        }
    }

    pub BotLine = <ChatLine> {}

    pub LoadingLine = <BotLine> {
        message_section = {
            content_section = <View> {
                height: Fit,
                padding: {left: 32}
                loading = <MessageLoading> {}
            }
        }
    }

    // Note: For now, let's use bot's apparence for app messages.
    // Idea: With the current design, this can be something centered and fit
    // up to the fill size. If we drop the current design and simplify it, we could
    // just use the bot's design for all messages.
    pub AppLine = <BotLine> {
        margin: {left: 0}
        message_section = {
            padding: {left: 12, right: 12, top: 12, bottom: 0}
            draw_bg: {
                border_color: #344054
                border_size: 1.2
                border_radius: 8.0
            }
            sender = {
                margin: {bottom: 5}
                avatar = {
                    grapheme = {draw_bg: {color: #344054}}
                }
                name = {text: "Application"}
            }
        }
        actions_section = {
            margin: {left: 32}
        }
    }

    pub ErrorLine = <AppLine> {
        message_section = {
            draw_bg: {color: #f003}

            sender = {
                avatar = {
                    grapheme = {draw_bg: {color: #f003}}
                }
            }
        }
    }

    pub SystemLine = <AppLine> {
        message_section = {
            draw_bg: {color: #e3f2fd}

            sender = {
                avatar = {
                    grapheme = {draw_bg: {color: #1976d2}}
                }
                name = {text: "System"}
            }
        }
    }

    ToolApprovalButton = <Button> {
        padding: {left: 15, right: 15, top: 8, bottom: 8},
        draw_text: {
            text_style: <THEME_FONT_BOLD>{font_size: 10},
            color: #fff
            color_hover: #fff
            color_focus: #fff
        }
    }

    ToolApprovalActions = <View> {
        width: Fill, height: Fit,
        align: {y: 0.5},
        spacing: 5,
        padding: {bottom: 8}
        approve = <ToolApprovalButton> {
            text: "Approve",
            draw_bg: {color: #4CAF50, color_hover: #45a049}
        }
        deny = <ToolApprovalButton> {
            text: "Deny",
            draw_bg: {color: #f44336, color_hover: #d32f2f}
        }
    }

    // Line for tool permission requests (from assistant asking to use a tool)
    pub ToolRequestLine = <AppLine> {
        message_section = {
            draw_bg: {color: #fff3e0}

            sender = {
                avatar = {
                    grapheme = {draw_bg: {color: #ff9800}}
                }
            }
            content_section = {
                flow: Down
                tool_actions = <ToolApprovalActions> { visible: false }
                status_view = <View> {
                    visible: false
                    width: Fill, height: Fit,
                    align: {x: 1.0, y: 0.5}
                    padding: {bottom: 8, right: 10}
                    approved_status = <Label> {
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{font_size: 11},
                            color: #000
                        }
                    }
                }
            }
        }
    }

    // Line for tool execution results (EntityId::Tool)
    pub ToolResultLine = <AppLine> {
        message_section = {
            draw_bg: {color: #cfe4ff}

            sender = {
                avatar = {
                    grapheme = {draw_bg: {color: #1a5b9c}}
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, DefaultNone)]
pub enum ChatLineAction {
    Copy,
    Edit,
    Delete,
    Save,
    SaveAndRegenerate,
    EditCancel,
    ToolApprove,
    ToolDeny,
    EditorChange(String),
    None,
}

#[derive(Live, Widget, LiveHook)]
pub struct ChatLine {
    #[deref]
    deref: View,
}

impl Widget for ChatLine {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope);
        let actions = event.actions();

        if self.button(ids!(copy)).clicked(actions) {
            self.moly_modal(ids!(actions_modal)).close(cx);
            cx.widget_action(self.widget_uid(), &scope.path, ChatLineAction::Copy);
        }

        if self.button(ids!(edit)).clicked(actions) {
            self.moly_modal(ids!(actions_modal)).close(cx);
            cx.widget_action(self.widget_uid(), &scope.path, ChatLineAction::Edit);
        }

        if self.button(ids!(delete)).clicked(actions) {
            self.moly_modal(ids!(actions_modal)).close(cx);
            cx.widget_action(self.widget_uid(), &scope.path, ChatLineAction::Delete);
        }

        if self.button(ids!(edit_actions.save)).clicked(actions) {
            println!("Save clicked");
            cx.widget_action(self.widget_uid(), &scope.path, ChatLineAction::Save);
        }

        if self
            .button(ids!(edit_actions.save_and_regenerate))
            .clicked(actions)
        {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ChatLineAction::SaveAndRegenerate,
            );
        }

        if self.button(ids!(edit_actions.cancel)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, ChatLineAction::EditCancel);
        }

        if self.button(ids!(approve)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, ChatLineAction::ToolApprove);
        }

        if self.button(ids!(deny)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, ChatLineAction::ToolDeny);
        }

        if let Some(change) = self.text_input(ids!(input)).changed(actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ChatLineAction::EditorChange(change),
            );
        }

        if let Some(pos) = event.hits(cx, self.area()).secondary_pointer_action_pos() {
            self.moly_modal(ids!(actions_modal)).open_as_popup(cx, pos);
        }
    }
}
