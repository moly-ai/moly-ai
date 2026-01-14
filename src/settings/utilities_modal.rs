use crate::data::store::Store;
use crate::shared::utils::version::{Pull, Version};
use makepad_widgets::*;

#[derive(Clone, DefaultNone, Debug)]
pub enum UtilitiesModalAction {
    ModalDismissed,
    None,
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;

    ICON_CLOSE = dep("crate://self/resources/icons/close.svg")

    ModalLabel = <Label> {
        width: Fill, height: Fit
        draw_text: {
            wrap: Word
            text_style: <REGULAR_FONT>{font_size: 9},
            color: #999
        }
    }

    ModalTextInput = <TextInput> {
        width: Fill, height: Fit

        draw_bg: {
            color: #fff
            instance radius: 2.0
            instance border_width: 0.0
            instance border_color: #D0D5DD
            instance inset: vec4(0.0, 0.0, 0.0, 0.0)

            fn get_color(self) -> vec4 {
                return self.color
            }

            fn get_border_color(self) -> vec4 {
                return self.border_color
            }

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.box(
                    self.inset.x + self.border_width,
                    self.inset.y + self.border_width,
                    self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
                    self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
                    max(1.0, self.radius)
                )
                sdf.fill_keep(self.get_color())
                sdf.stroke(
                    self.get_border_color(),
                    self.border_width
                )
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <REGULAR_FONT>{font_size: 10},

            fn get_color(self) -> vec4 {
                return #000
            }
        }

        draw_cursor: {
            instance focus: 0.0
            uniform border_radius: 0.5
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.,
                    0.,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.border_radius
                )
                sdf.fill(mix(#fff, #bbb, self.focus));
                return sdf.result
            }
        }

        draw_selection: {
            instance hover: 0.0
            instance focus: 0.0
            uniform border_radius: 2.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.,
                    0.,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.border_radius
                )
                sdf.fill(mix(#eee, #ddd, self.focus));
                return sdf.result
            }
        }

        padding: {top: 11, right: 10, bottom: 11, left: 10}
    }

    FormGroup = <View> {
        width: Fill, height: Fit
        flow: Down
        spacing: 5

        label = <ModalLabel> {}
        input = <View> {
            width: Fill, height: Fit
        }
    }

    pub UtilitiesModal = {{UtilitiesModal}} <RoundedView> {
        flow: Down
        width: 500
        height: Fit
        show_bg: true
        draw_bg: {
            color: #fff
        }

        padding: 25
        spacing: 20

        draw_bg: {
            border_radius: 3.0
        }

        header = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 10
            align: {x: 0.0, y: 0.5}

            title = <View> {
                width: Fill, height: Fit

                title_label = <Label> {
                    width: Fill, height: Fit
                    draw_text: {
                        wrap: Word
                        text_style: <BOLD_FONT>{font_size: 13},
                        color: #000
                    }
                    text: "Utilities"
                }
            }

            close_button = <MolyButton> {
                width: Fit, height: Fit
                icon_walk: {width: 14, height: Fit}
                draw_icon: {
                    svg_file: (ICON_CLOSE),
                    fn get_color(self) -> vec4 {
                        return #000;
                    }
                }
                draw_bg: {
                    instance pressed: 0.0
                    instance hover: 0.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        return sdf.result
                    }
                }
            }
        }

        body = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 20

            <Label> {
                width: Fill, height: Fit
                draw_text: {
                    wrap: Word
                    text_style: <BOLD_FONT>{font_size: 11},
                    color: #666
                }
                text: "Speech to Text (STT)"
            }

            enabled_toggle = <CheckBox> {
                width: Fill, height: Fit
                label_walk: {margin: {left: 20}}
                // draw_check: {
                //     check_type: Toggle,
                //     color_active: #0
                // }

                draw_text: {
                    text_style: <REGULAR_FONT>{font_size: 10},
                    fn get_color(self) -> vec4 {
                        return #000
                    }
                }
                text: "Enable STT"
            }

            url_group = <FormGroup> {
                label = <ModalLabel> {
                    text: "API Host"
                }
                input = {
                    url_input2 = <ModalTextInput> {
                        empty_text: "https://api.openai.com/v1"
                    }
                }
            }

            api_key_group = <FormGroup> {
                label = <ModalLabel> {
                    text: "API Key (optional)"
                }
                input = {
                    api_key_input = <ModalTextInput> {
                        is_password: true
                        empty_text: "sk-..."
                    }
                }
            }

            model_group = <FormGroup> {
                label = <ModalLabel> {
                    text: "Model Name"
                }
                input = {
                    model_input = <ModalTextInput> {
                        empty_text: "whisper-1"
                    }
                }
            }
        }
    }

}

#[derive(Live, Widget, LiveHook)]
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
        if self.button(ids!(close_button)).clicked(actions) {
            cx.action(UtilitiesModalAction::ModalDismissed);
        }

        let store = scope.data.get_mut::<Store>().unwrap();
        let stt_config = &mut store.preferences.stt_config;

        if let Some(value) = self.check_box(ids!(enabled_toggle)).changed(actions) {
            stt_config.update_and_notify(|config| {
                config.enabled = value;
            });
        }

        if let Some(value) = self.text_input(ids!(url_input2)).changed(actions) {
            ::log::debug!("STT URL changed to {}", value);
            stt_config.update_and_notify(|config| {
                config.url = value;
            });
        }

        if let Some(value) = self.text_input(ids!(api_key_input)).changed(actions) {
            stt_config.update_and_notify(|config| {
                config.api_key = if value.is_empty() { None } else { Some(value) };
            });
        }

        if let Some(value) = self.text_input(ids!(model_input)).changed(actions) {
            stt_config.update_and_notify(|config| {
                config.model_name = value;
            });
        }
    }
}

impl UtilitiesModal {
    fn pull(&mut self, cx: &mut Cx, scope: &mut Scope) {
        let store = scope.data.get_mut::<Store>().unwrap();
        if let Some(stt_config) = self.stt_config.pull(&store.preferences.stt_config) {
            self.check_box(ids!(enabled_toggle))
                .set_active(cx, stt_config.enabled);

            self.text_input(ids!(url_input2))
                .set_text(cx, &stt_config.url);

            if let Some(ref api_key) = stt_config.api_key {
                self.text_input(ids!(api_key_input)).set_text(cx, api_key);
            }

            self.text_input(ids!(model_input))
                .set_text(cx, &stt_config.model_name);

            self.redraw(cx);
        }
    }
}
