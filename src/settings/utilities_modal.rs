use crate::data::store::Store;
use crate::shared::utils::version::{Pull, Version};
use makepad_widgets::*;

#[derive(Clone, Debug, Default)]
pub enum UtilitiesModalAction {
    ModalDismissed,
    #[default]
    None,
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let IconButton = Button {
        width: Fit
        height: Fit
        draw_text +: {
            text_style: theme.font_icons {
                font_size: 14.
            }
            color: #5
            color_hover: #2
            color_focus: #2
            color_down: #5
        }
        draw_bg +: {
            color_down: #0000
            border_radius: 7.
            border_size: 0.
        }
    }

    let FormGroup = View {
        width: Fill
        height: Fit
        flow: Down
        spacing: 5

        label := Label {
            width: Fill
            height: Fit
            draw_text +: {
                text_style: REGULAR_FONT { font_size: 9 }
                color: #999
            }
        }
        input := View {
            width: Fill
            height: Fit
        }
    }

    mod.widgets.UtilitiesModalBase = #(UtilitiesModal::register_widget(vm))
    mod.widgets.UtilitiesModal =
        set_type_default() do mod.widgets.UtilitiesModalBase {
        ..mod.widgets.RoundedView
        flow: Down
        width: 500
        height: Fit
        show_bg: true
        draw_bg +: {
            color: #fff
            border_radius: 3.0
        }

        padding: 25
        spacing: 10

        header := View {
            width: Fill
            height: Fit
            flow: Right
            spacing: 10
            align: Align { x: 0.0 y: 0.5 }

            title := View {
                width: Fill
                height: Fit

                title_label := Label {
                    width: Fill
                    height: Fit
                    draw_text +: {
                        text_style: BOLD_FONT { font_size: 13 }
                        color: #000
                    }
                    text: "Utilities"
                }
            }

            close_button := MolyButton {
                width: Fit
                height: Fit
                icon_walk +: { width: 14 height: Fit }
                draw_icon +: {
                    svg: ICON_CLOSE
                    get_color: fn() -> vec4 {
                        return #000
                    }
                }
            }
        }

        body := View {
            width: Fill
            height: Fit
            flow: Down
            spacing: 10

            Label {
                width: Fill
                height: Fit
                draw_text +: {
                    text_style: BOLD_FONT { font_size: 11 }
                    color: #666
                }
                text: "Speech to Text (STT)"
            }

            View {
                width: Fill
                height: Fit
                flow: Right
                align: Align { x: 0.0 y: 0.5 }
                spacing: 10

                Label {
                    width: Fit
                    height: Fit
                    text: "Enable STT"
                    draw_text +: {
                        text_style: REGULAR_FONT { font_size: 10 }
                        color: #000
                    }
                }

                enabled_toggle := MolySwitch {}
            }

            url_group := FormGroup {
                label := {
                    text: "API Host"
                }
                input := {
                    url_input := MolyTextInput {
                        width: Fill
                        height: Fit
                        empty_text: "https://api.openai.com/v1"
                        padding: Inset { top: 10 bottom: 10 left: 10 right: 10 }
                        draw_bg +: {
                            color: #fff
                            border_size: 1.0
                            border_color_1: #D0D5DD
                            border_radius: 2.0
                        }
                        draw_text +: {
                            text_style: REGULAR_FONT { font_size: 10 }
                            color: #000
                        }
                    }
                }
            }

            api_key_group := FormGroup {
                label := {
                    text: "API Key (optional)"
                }
                input := {
                    flow: Right
                    spacing: 5
                    align: Align { x: 0.0 y: 0.5 }

                    api_key_input := MolyTextInput {
                        width: Fill
                        height: 30
                        is_password: true
                        empty_text: "sk-..."
                        padding: Inset { top: 6 bottom: 6 left: 10 right: 10 }
                        draw_bg +: {
                            color: #fff
                            border_size: 1.0
                            border_color_1: #D0D5DD
                            border_radius: 2.0
                        }
                        draw_text +: {
                            text_style: REGULAR_FONT { font_size: 10 }
                            color: #000
                        }
                    }

                    toggle_key_visibility := IconButton {
                        text: "" // fa-eye
                    }
                }
            }

            model_group := FormGroup {
                label := {
                    text: "Model Name"
                }
                input := {
                    model_input := MolyTextInput {
                        width: Fill
                        height: Fit
                        empty_text: "whisper-1"
                        padding: Inset { top: 10 bottom: 10 left: 10 right: 10 }
                        draw_bg +: {
                            color: #fff
                            border_size: 1.0
                            border_color_1: #D0D5DD
                            border_radius: 2.0
                        }
                        draw_text +: {
                            text_style: REGULAR_FONT { font_size: 10 }
                            color: #000
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct UtilitiesModal {
    #[deref]
    view: View,

    #[rust]
    stt_config: Option<Version>,
}

impl Widget for UtilitiesModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
        self.pull(cx, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for UtilitiesModal {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if self.button(cx, ids!(close_button)).clicked(actions) {
            cx.action(UtilitiesModalAction::ModalDismissed);
        }

        let store = scope.data.get_mut::<Store>().unwrap();
        let prefs = &mut store.preferences;

        if let Some(value) = self.check_box(cx, ids!(enabled_toggle)).changed(actions) {
            prefs.update_stt_config(|config| {
                config.enabled = value;
            });
        }

        if let Some(value) = self.text_input(cx, ids!(url_input)).changed(actions) {
            prefs.update_stt_config(|config| {
                config.url = value;
            });
        }

        if let Some(value) = self.text_input(cx, ids!(api_key_input)).changed(actions) {
            prefs.update_stt_config(|config| {
                config.api_key = value;
            });
        }

        if self
            .button(cx, ids!(toggle_key_visibility))
            .clicked(actions)
        {
            let api_key_input = self.text_input(cx, ids!(api_key_input));
            api_key_input.set_is_password(cx, !api_key_input.is_password());
            if api_key_input.is_password() {
                self.button(cx, ids!(toggle_key_visibility))
                    .set_text(cx, ""); // fa-eye-slash
            } else {
                self.button(cx, ids!(toggle_key_visibility))
                    .set_text(cx, ""); // fa-eye
            }
            self.redraw(cx);
        }

        if let Some(value) = self.text_input(cx, ids!(model_input)).changed(actions) {
            prefs.update_stt_config(|config| {
                config.model_name = value;
            });
        }
    }
}

impl UtilitiesModal {
    fn pull(&mut self, cx: &mut Cx, scope: &mut Scope) {
        let store = scope.data.get_mut::<Store>().unwrap();
        if let Some(stt_config) = self.stt_config.pull(store.preferences.stt_config()) {
            self.check_box(cx, ids!(enabled_toggle))
                .set_active(cx, stt_config.enabled);

            self.text_input(cx, ids!(url_input))
                .set_text(cx, &stt_config.url);

            self.text_input(cx, ids!(api_key_input))
                .set_text(cx, &stt_config.api_key);

            self.text_input(cx, ids!(model_input))
                .set_text(cx, &stt_config.model_name);

            self.redraw(cx);
        }
    }
}
