use crate::shared::utils::hugging_face_model_url;
use makepad_widgets::*;

use super::downloaded_files_row::DownloadedFilesRowProps;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let MolyHtml = Html {
        font_color: #000
        draw_block +: {
            code_color: (#EAECF0)
        }
        font_size: 10
        code_layout: Layout { padding: 15 }
    }

    mod.widgets.ModelInfoModalBase = #(ModelInfoModal::register_widget(vm))
    mod.widgets.ModelInfoModal =
        set_type_default() do mod.widgets.ModelInfoModalBase {
        width: Fit
        height: Fit

        wrapper := RoundedView {
            flow: Down
            width: 800
            height: Fit
            padding: Inset { top: 44 right: 30 bottom: 30 left: 50 }
            spacing: 5

            show_bg: true
            draw_bg +: {
                color: #fff
                border_radius: 3
            }

            View {
                width: Fill
                height: Fit
                flow: Right

                padding: Inset { top: 6 bottom: 20 }

                title := View {
                    width: Fit
                    height: Fit

                    filename := Label {
                        draw_text +: {
                            text_style: BOLD_FONT { font_size: 13 }
                            color: #000
                        }
                    }
                }

                filler_x := View { width: Fill height: Fit }

                close_button := MolyButton {
                    width: Fit
                    height: Fit
                    margin: Inset { top: -6 }

                    draw_icon +: {
                        svg: ICON_CLOSE
                        get_color: fn() -> vec4 {
                            return #000
                        }
                    }
                    icon_walk +: { width: 12 height: 12 }
                }
            }

            file_dir := View {
                width: Fill
                height: Fit
                flow: Down
                spacing: 8
                align: Align { x: 0.0 y: 0.6 }

                Label {
                    text: "Read from"
                    draw_text +: {
                        text_style: REGULAR_FONT { font_size: 10 }
                        color: #344054
                    }
                }
                path := MolyHtml {
                    width: Fill
                    font_size: 10
                    code_layout: Layout { padding: 9 }
                }
            }

            body := View {
                width: Fill
                height: Fit
                flow: Down
                spacing: 20

                metadata := MolyHtml {}
                actions := View {
                    width: Fill height: Fit
                    flow: Right
                    align: Align { x: 0.0 y: 0.5 }
                    spacing: 20

                    copy_button := MolyButton {
                        width: Fit
                        height: Fit
                        padding: Inset {
                            top: 10 bottom: 10 left: 14 right: 14
                        }
                        spacing: 10

                        draw_icon +: {
                            svg: ICON_COPY
                            get_color: fn() -> vec4 {
                                return #x0
                            }
                        }
                        icon_walk +: { width: 14 height: 14 }

                        draw_bg +: {
                            border_radius: instance(2.0)
                            border_color_1: #D0D5DD
                            border_size: 1.2
                            color: #EDFCF2
                        }

                        text: "Copy to Clipboard"
                        draw_text +: {
                            text_style: REGULAR_FONT {
                                font_size: 10
                            }
                            color: #x0
                        }
                    }
                    external_link := MolyButton {
                        width: Fit
                        height: Fit
                        padding: Inset {
                            top: 10 bottom: 10 left: 14 right: 14
                        }

                        draw_bg +: {
                            border_radius: instance(2.0)
                            border_color_1: #D0D5DD
                            border_size: 1.2
                            color: #F5FEFF
                        }

                        text: "Model Card on Hugging Face"
                        draw_text +: {
                            text_style: REGULAR_FONT {
                                font_size: 10
                            }
                            color: #x0
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ModelInfoModalAction {
    #[default]
    None,
    ModalDismissed,
}

#[derive(Script, ScriptHook, Widget)]
pub struct ModelInfoModal {
    #[deref]
    view: View,
    #[rust]
    model_id: String,
    #[rust]
    stringified_model_data: String,
}

impl Widget for ModelInfoModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let props = scope.props.get::<DownloadedFilesRowProps>().unwrap();
        let downloaded_file = &props.downloaded_file;

        self.model_id = downloaded_file.model.id.clone();

        self.label(cx, ids!(title.filename))
            .set_text(cx, &downloaded_file.file.name);

        if let Some(path) = &downloaded_file.file.downloaded_path {
            self.html(cx, ids!(file_dir.path))
                .set_text(cx, &format!("<pre>{}</pre>", path));
        } else {
            self.view(cx, ids!(file_dir)).set_visible(cx, false);
        }

        self.stringified_model_data = serde_json::to_string_pretty(&downloaded_file.model)
            .expect("Could not serialize model data into json");
        let metadata = format!("<pre>{}</pre>", self.stringified_model_data);

        self.html(cx, ids!(wrapper.body.metadata))
            .set_text(cx, &metadata);

        self.view
            .draw_walk(cx, scope, walk.with_abs_pos(DVec2 { x: 0., y: 0. }))
    }
}

impl WidgetMatchEvent for ModelInfoModal {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(cx, ids!(close_button)).clicked(actions) {
            cx.action(ModelInfoModalAction::ModalDismissed);
        }

        if self
            .button(cx, ids!(wrapper.body.actions.copy_button))
            .clicked(actions)
        {
            cx.copy_to_clipboard(&self.stringified_model_data);
        }

        if self
            .button(cx, ids!(wrapper.body.actions.external_link))
            .clicked(actions)
        {
            let model_url = hugging_face_model_url(&self.model_id);
            if let Err(e) = robius_open::Uri::new(&model_url).open() {
                error!("Error opening URL: {:?}", e);
            }
        }
    }
}
