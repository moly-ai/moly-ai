use crate::data::store::ModelWithDownloadInfo;
use crate::shared::external_link::ExternalLinkWidgetExt;
use crate::shared::utils::hugging_face_model_url;
use chrono::Utc;
use makepad_widgets::*;
use moly_kit::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_DOWNLOADS = crate_resource("self://resources/icons/downloads.svg")
    let ICON_FAVORITE = crate_resource("self://resources/icons/favorite.svg")
    let ICON_EXTERNAL_LINK = crate_resource("self://resources/icons/external_link.svg")

    let ModelHeading = View {
        flow: Down
        width: Fill
        height: Fit

        spacing: 10

        View {
            width: Fill
            height: Fit

            spacing: 10
            align: Align {x: 0.0 y: 0.5}

            View {
                width: Fit
                height: Fit
                model_name := Label {
                    draw_text: {
                        text_style: theme.font_bold {font_size: 16}
                        color: #000
                    }
                }
            }


            VerticalFiller {}

            model_like_count := ModelAttributeTag {
                width: Fit
                height: Fit

                padding: Inset {top: 6 bottom: 6 left: 6 right: 10}

                draw_bg: {
                    color: #0000
                    border_color: #98A2B3
                    border_size: 0.8
                }
                attr_name := Icon {
                    draw_icon: {
                        svg_file: (ICON_FAVORITE)
                        get_color: fn() -> vec4 {
                            return #000;
                        }
                    }
                    icon_walk: {width: 14 height: 14}
                }

                attr_value: {
                    margin: Inset {left: 5}
                    draw_text: {
                        color: #000
                        text_style: theme.font_regular {font_size: 9}
                    }
                }
            }

            model_download_count := ModelAttributeTag {
                width: Fit
                height: Fit

                padding: Inset {top: 6 bottom: 6 left: 6 right: 10}

                draw_bg: {
                    color: #0000
                    border_color: #98A2B3
                    border_size: 0.8
                }
                attr_name := Icon {
                    draw_icon: {
                        svg_file: (ICON_DOWNLOADS)
                        get_color: fn() -> vec4 {
                            return #000;
                        }
                    }
                    icon_walk: {width: 12 height: 12}
                }

                attr_value: {
                    margin: Inset {left: 5}
                    draw_text: {
                        color: #000
                        text_style: theme.font_regular {font_size: 9}
                    }
                }
            }


            View {
                width: 260
                height: Fit
                model_released_at_tag := ModelAttributeTag {
                    width: Fit
                    height: Fit

                    draw_bg: {
                        color: #0000
                        border_color: #98A2B3
                        border_size: 0.8
                    }
                    attr_name: {
                        draw_text: { color: #000 }
                        text: "Released"
                    }
                    attr_value: {
                        margin: Inset {left: 10}
                        draw_text: { color: #000 }
                    }
                }
            }
        }
        ModelAttributes {}
    }

    let ModelSummary = View {
        width: Fill
        height: Fit
        flow: Down
        spacing: 20
        padding: Inset {right: 100}

        Label {
            draw_text: {
                text_style: theme.font_bold {font_size: 11}
                color: #000
            }
            text: "Model Summary"
        }
        model_summary := Label {
            width: Fill
            draw_text: {
                text_style: theme.font_regular {font_size: 9}
                word: Wrap
                color: #000
            }
        }

        view_all_button := ModelLink {
            link: { text: "View All" }
        }
    }

    let ExternalLinkIcon = Icon {
        draw_icon: {
            svg_file: (ICON_EXTERNAL_LINK)
            get_color: fn() -> vec4 {
                return (MODEL_LINK_FONT_COLOR);
            }
        }
        icon_walk: {width: 14 height: 14}
    }

    let ModelDetails = View {
        width: 400
        height: Fit
        flow: Down
        spacing: 20

        Label {
            draw_text: {
                text_style: theme.font_bold {font_size: 11}
                color: #000
            }
            text: "Resources"
        }

        View {
            align: Align {x: 0.5 y: 1.0}
            width: Fit
            height: Fit
            author_link := ExternalLink {}
            ExternalLinkIcon {}
        }

        View {
            align: Align {x: 0.5 y: 1.0}
            width: Fit
            height: Fit
            model_hugging_face_link := ExternalLink { link: { text: "Hugging Face" } }
            ExternalLinkIcon {}
        }
    }

    let ModelInformation = View {
        width: Fill
        height: Fit
        spacing: 10
        ModelSummary {}
        ModelDetails {}
    }


    mod.widgets.ModelCardViewAllModal = #(ModelCardViewAllModal::register_widget(vm)) ViewBase {
        width: Fit
        height: Fit

        RoundedView {
            flow: Down
            width: 600
            height: Fit
            padding: Inset {top: 30 right: 30 bottom: 50 left: 50}
            spacing: 10

            show_bg: true
            draw_bg: {
                color: #fff
                border_radius: 3
            }

            View {
                width: Fill
                height: Fit
                filler_x := View {width: Fill height: Fit}

                close_button := MolyButton {
                    width: Fit
                    height: Fit

                    draw_icon: {
                        svg_file: (ICON_CLOSE)
                        color: #000
                    }
                    icon_walk: {width: 12 height: 12}
                }
            }

            View {
                width: Fill
                height: Fit
                padding: Inset {bottom: 20}

                view_all_model_name := Label {
                    draw_text: {
                        text_style: theme.font_bold {font_size: 16}
                        color: #000
                    }
                }
            }

            View {
                width: Fill
                height: Fit
                flow: Down
                spacing: 10

                Label {
                    draw_text: {
                        text_style: theme.font_bold {font_size: 9}
                        color: #000
                    }
                    text: "Model Description"
                }
                view_all_model_summary := Label {
                    width: Fill
                    draw_text: {
                        text_style: theme.font_regular {font_size: 9}
                        word: Wrap
                        color: #000
                    }
                }
            }
        }
    }


    mod.widgets.ModelCard = #(ModelCard::register_widget(vm)) ViewBase {
        width: Fill
        height: Fit

        // This is necesary because we have a Modal widget inside this widget
        flow: Overlay

        cursor: MouseCursor.Default
        padding: Inset {left: 10 top: 8 right: 10 bottom: 8}

        RoundedShadowView {
            width: Fill
            height: Fit

            draw_bg: {
                color: (MAIN_BG_COLOR_DARK)
                border_radius: 4.5
                shadow_color: uniform(#x0002)
                shadow_radius: 9.0
                shadow_offset: vec2(0.0 -2.0)
            }

            flow: Down
            padding: 30
            spacing: 20

            ModelHeading {}
            Line {}
            ModelInformation {}
            ModelFiles {}
        }

        modal := MolyModal {
            content: {
                ModelCardViewAllModal {}
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ModelCard {
    #[deref]
    view: View,

    #[rust]
    model_id: String,
}

impl Widget for ModelCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let model = &scope.data.get::<ModelWithDownloadInfo>().unwrap();

        self.model_id = model.model_id.clone();

        let name = &model.name;
        self.label(ids!(model_name)).set_text(cx, name);

        let download_count = &model.download_count;
        self.label(ids!(model_download_count.attr_value))
            .set_text(cx, &format!("{}", download_count));

        let like_count = &model.like_count;
        self.label(ids!(model_like_count.attr_value))
            .set_text(cx, &format!("{}", like_count));

        let size = &model.size;
        self.label(ids!(model_size_tag.attr_value))
            .set_text(cx, size);

        let requires = &model.requires;
        self.label(ids!(model_requires_tag.attr_value))
            .set_text(cx, requires);

        let architecture = &model.architecture;
        self.label(ids!(model_architecture_tag.attr_value))
            .set_text(cx, architecture);

        let summary = &model.summary;
        const MAX_SUMMARY_LENGTH: usize = 500;
        let trimmed_summary = if summary.len() > MAX_SUMMARY_LENGTH {
            let trimmed = summary
                .graphemes(true)
                .take(MAX_SUMMARY_LENGTH)
                .collect::<String>();
            format!("{}...", trimmed)
        } else {
            summary.to_string()
        };
        self.label(ids!(model_summary))
            .set_text(cx, &trimmed_summary);

        let author_name = &model.author.name;
        let author_url = &model.author.url;
        let mut author_external_link = self.external_link(ids!(author_link));
        author_external_link
            .link_label(ids!(link))
            .set_text(cx, author_name);
        author_external_link.set_url(author_url);

        let model_hugging_face_url = hugging_face_model_url(&model.model_id);
        let mut model_hugging_face_external_link =
            self.external_link(ids!(model_hugging_face_link));
        model_hugging_face_external_link.set_url(&model_hugging_face_url);

        let author_description = &model.author.description;
        self.label(ids!(author_description))
            .set_text(cx, author_description);

        let released_at_str = formatted_model_release_date(&model);
        self.label(ids!(model_released_at_tag.attr_value))
            .set_text(cx, &released_at_str);

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ModelCard {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.link_label(ids!(view_all_button.link)).clicked(actions) {
            self.moly_modal(ids!(modal)).open(cx);
            self.redraw(cx);
        }

        for action in actions {
            if let ModelCardViewAllModalAction::CloseButtonClicked = action.cast() {
                self.moly_modal(ids!(modal)).close(cx);
                self.redraw(cx);
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
enum ModelCardViewAllModalAction {
    #[default]
    None,
    CloseButtonClicked,
}

#[derive(Script, ScriptHook, Widget)]
pub struct ModelCardViewAllModal {
    #[deref]
    view: View,
}

impl Widget for ModelCardViewAllModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let model = &scope.data.get::<ModelWithDownloadInfo>().unwrap();

        let name = &model.name;
        self.label(ids!(view_all_model_name)).set_text(cx, name);

        let summary = &model.summary;
        self.label(ids!(view_all_model_summary))
            .set_text(cx, summary);

        self.view
            .draw_walk(cx, scope, walk.with_abs_pos(DVec2 { x: 0., y: 0. }))
    }
}

impl WidgetMatchEvent for ModelCardViewAllModal {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(ids!(close_button)).clicked(actions) {
            cx.action(ModelCardViewAllModalAction::CloseButtonClicked);
        }
    }
}

fn formatted_model_release_date(model: &ModelWithDownloadInfo) -> String {
    let released_at = model.released_at.naive_local().format("%b %-d, %C%y");
    let days_ago = (Utc::now() - model.released_at).num_days();
    format!("{} ({} days ago)", released_at, days_ago)
}
