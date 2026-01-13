use makepad_widgets::*;

use crate::data::preferences::SttUtilityPreferences;
use crate::data::store::Store;

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

    UTILITIES_MODAL_WIDTH = 500
    UTILITIES_MODAL_HEIGHT = 500

    FormGroup = <View> {
        flow: Down
        height: Fit
        spacing: 10
        align: {x: 0.0, y: 0.5}
    }

    ModalTextInput = <MolyTextInput> {
        draw_bg: {
            border_size: 1.0
            border_color: #ddd
        }
        draw_text: {
            text_style: <REGULAR_FONT>{font_size: 12},
            color: #000
            color_hover: #000
            color_focus: #000
            color_empty: #98A2B3
            color_empty_focus: #98A2B3
        }
        width: Fill, height: Fit
    }

    ModalLabel = <Label> {
        draw_text: {
            text_style: <REGULAR_FONT>{font_size: 12},
            color: #000
        }
    }

    pub UtilitiesModal = {{UtilitiesModal}} <RoundedView> {
        flow: Down
        width: (UTILITIES_MODAL_WIDTH)
        height: (UTILITIES_MODAL_HEIGHT)
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
                input = <ModalTextInput> {
                    url_input = <ModalTextInput> {
                        empty_text: "https://api.openai.com/v1"
                    }
                }
            }

            api_key_group = <FormGroup> {
                label = <ModalLabel> {
                    text: "API Key (optional)"
                }
                input = <ModalTextInput> {
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
                input = <ModalTextInput> {
                    model_input = <ModalTextInput> {
                        empty_text: "whisper-1"
                    }
                }
            }

            save_button = <MolyButton> {
                width: Fill, height: Fit
                margin: {top: 10}
                padding: {top: 14, bottom: 14, left: 10, right: 10}

                draw_bg: {
                    instance background_color: #099250
                    instance border_color: #099250
                    instance border_width: 1.2
                    instance radius: 3.0

                    fn get_bg_color(self) -> vec4 {
                        return self.background_color
                    }

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                        sdf.box(
                            1,
                            1,
                            self.rect_size.x - 2,
                            self.rect_size.y - 2,
                            self.radius
                        )
                        sdf.fill_keep(self.get_bg_color())
                        sdf.stroke(
                            self.border_color,
                            self.border_width
                        )
                        return sdf.result
                    }
                }

                draw_text: {
                    text_style: <REGULAR_FONT>{font_size: 10},
                    fn get_color(self) -> vec4 {
                        return #fff
                    }
                }

                text: "Save"
            }
        }
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

        draw_select: {
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
}

#[derive(Live, Widget, LiveHook)]
pub struct UtilitiesModal {
    #[deref]
    view: View,

    #[rust]
    initialized: bool,
}

impl Widget for UtilitiesModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.initialized {
            self.initialized = true;
            self.load_settings(scope, cx);
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for UtilitiesModal {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if self.button(ids!(close_button)).clicked(actions) {
            cx.action(UtilitiesModalAction::ModalDismissed);
        }

        if self.button(ids!(save_button)).clicked(actions) {
            self.save_settings(scope, cx);
            cx.action(UtilitiesModalAction::ModalDismissed);
        }
    }
}

impl UtilitiesModal {
    fn load_settings(&mut self, scope: &mut Scope, cx: &mut Cx) {
        let store = scope.data.get_mut::<Store>().unwrap();
        let stt_prefs = store.preferences.get_stt_utility();

        self.check_box(ids!(enabled_toggle))
            .set_active(cx, stt_prefs.enabled);

        self.text_input(ids!(url_input))
            .set_text(cx, &stt_prefs.url);

        if let Some(ref api_key) = stt_prefs.api_key {
            self.text_input(ids!(api_key_input)).set_text(cx, api_key);
        }

        self.text_input(ids!(model_input))
            .set_text(cx, &stt_prefs.model_name);
    }

    fn save_settings(&mut self, scope: &mut Scope, cx: &mut Cx) {
        let enabled = self.check_box(ids!(enabled_toggle)).active(cx);
        let url = self.text_input(ids!(url_input)).text();
        let api_key_text = self.text_input(ids!(api_key_input)).text();
        let api_key = if api_key_text.is_empty() {
            None
        } else {
            Some(api_key_text)
        };
        let model_name = self.text_input(ids!(model_input)).text();

        let config = SttUtilityPreferences {
            enabled,
            url,
            api_key,
            model_name,
        };

        let store = scope.data.get_mut::<Store>().unwrap();
        store.preferences.update_stt_utility(config);
    }
}
