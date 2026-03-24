use crate::{
    data::downloads::download::DownloadFileAction,
    shared::{
        actions::DownloadAction,
        utils::{format_model_downloaded_size, format_model_size},
    },
};
use makepad_widgets::*;
use moly_protocol::data::{FileId, PendingDownload, PendingDownloadsStatus};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_PAUSE = crate_resource("self://resources/icons/pause_download.svg")
    let ICON_CANCEL = crate_resource("self://resources/icons/cancel_download.svg")
    let ICON_PLAY = crate_resource("self://resources/icons/play_download.svg")
    let ICON_RETRY = crate_resource("self://resources/icons/retry.svg")

    let ModelAttributeTag = RoundedView {
        width: Fit
        height: Fit
        padding: Inset {top: 6 bottom: 6 left: 10 right: 10}

        spacing: 5
        draw_bg +: {
            border_radius: 2.0
        }

        caption := Label {
            draw_text +: {
                text_style: theme.font_regular {font_size: 9}
                color: #fff
            }
        }
    }

    let Information = View {
        width: Fill
        height: Fit
        flow: Right
        spacing: 12
        margin: Inset {right: 60}

        align: Align {x: 0.0 y: 0.5}

        architecture_tag := ModelAttributeTag {
            caption +: {
                text: "StableLM"
            }
            draw_bg +: {
                color: #A44EBB
            }
        }

        params_size_tag := ModelAttributeTag {
            caption +: {
                text: "3B"
            }
            draw_bg +: {
                color: #44899A
            }
        }

        filename := Label {
            draw_text +: {
                text_style: theme.font_regular {font_size: 10}
                color: #000
            }
            text: "Stable-code-instruct-3b-Q8_0.gguf"
        }
    }

    let Progress = View {
        width: Fill
        height: Fit
        spacing: 8

        flow: Down

        View {
            width: Fill
            height: Fit

            flow: Right

            progress := Label {
                draw_text +: {
                    text_style: theme.font_bold {font_size: 9}
                    color: #099250
                }
                text: "Downloading 9.7%"
            }
            View { width: Fill height: 1 }
            downloaded_size := Label {
                draw_text +: {
                    text_style: theme.font_regular {font_size: 9}
                    color: #667085
                }
                text: "288.55 MB / 2.97 GB | 10.59 MB/s "
            }
        }

        View {
            width: Fill
            height: 12

            flow: Overlay

            RoundedView {
                width: 600
                height: Fill
                draw_bg +: {
                    color: #D9D9D9
                    border_radius: 2.0
                }
            }

            progress_bar := RoundedView {
                width: 0
                height: Fill
                draw_bg +: {
                    color: #099250
                    border_radius: 2.0
                }
            }
        }
    }

    let ActionButton = MolyButton {
        width: 40
        height: 40

        draw_bg +: {
            border_color_1: #EAECF0
            border_size: 1.0
            color: #fff
            color_hover: #E2F1F1
            border_radius: 2.0
        }

        draw_icon +: {
            color: #667085
        }
    }

    let Actions = View {
        width: Fill
        height: Fit
        flow: Right
        spacing: 12

        align: Align {x: 0.5 y: 0.5}

        pause_button := ActionButton {
            draw_icon +: {
                svg: (ICON_PAUSE)
            }
            icon_walk +: { margin: Inset { left: 6 } }
        }

        play_button := ActionButton {
            draw_icon +: {
                svg: (ICON_PLAY)
            }
            icon_walk +: { margin: Inset { left: 6 } }
        }

        retry_button := ActionButton {
            draw_icon +: {
                svg: (ICON_RETRY)
            }
        }

        cancel_button := ActionButton {
            draw_icon +: {
                svg: (ICON_CANCEL)
            }
            icon_walk +: { margin: 0 }
        }
    }

    mod.widgets.DownloadItemBase = #(DownloadItem::register_widget(vm))
    mod.widgets.DownloadItem = set_type_default() do mod.widgets.DownloadItemBase {
        ..mod.widgets.RoundedView
        width: Fill
        height: Fit

        show_bg: true
        padding: 20
        margin: Inset {bottom: 16}
        spacing: 30
        align: Align {x: 0.0 y: 0.5}

        cursor: Default

        draw_bg +: {
            border_color: #EAECF0
            border_size: 1.0
            border_radius: 3.0
            color: #fff
        }

        Information {}
        Progress {}
        Actions {}
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct DownloadItem {
    #[deref]
    view: View,

    #[rust]
    file_id: Option<FileId>,
}

impl Widget for DownloadItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let download = scope.data.get::<PendingDownload>().unwrap();
        self.file_id = Some(download.file.id.clone());

        self.label(cx, ids!(filename))
            .set_text(cx, download.file.name.as_str());

        self.label(cx, ids!(architecture_tag.caption))
            .set_text(cx, download.model.architecture.as_str());

        self.label(cx, ids!(params_size_tag.caption))
            .set_text(cx, &&download.model.requires.as_str());

        let progress_bar_width = download.progress * 6.0; // 6.0 = 600px / 100%
        let mut label = self.label(cx, ids!(progress));
        match download.status {
            PendingDownloadsStatus::Initializing => {
                let downloading_color = vec4(0.035, 0.572, 0.314, 1.0); //#099250

                label.set_text(cx, &format!("Downloading {:.1}%", download.progress));
                script_apply_eval!(cx, label, {
                    draw_text +: { color: #(downloading_color) }
                });

                let mut progress_bar = self.view(cx, ids!(progress_bar));
                script_apply_eval!(cx, progress_bar, {
                    width: #(progress_bar_width)
                    draw_bg +: { color: #(downloading_color) }
                });

                self.button(cx, ids!(pause_button)).set_visible(cx, false);
                self.button(cx, ids!(play_button)).set_visible(cx, false);
                self.button(cx, ids!(retry_button)).set_visible(cx, false);
                self.button(cx, ids!(cancel_button)).set_visible(cx, false);
            }
            PendingDownloadsStatus::Downloading => {
                let downloading_color = vec4(0.035, 0.572, 0.314, 1.0); //#099250

                label.set_text(cx, &format!("Downloading {:.1}%", download.progress));
                script_apply_eval!(cx, label, {
                    draw_text +: { color: #(downloading_color) }
                });

                let mut progress_bar = self.view(cx, ids!(progress_bar));
                script_apply_eval!(cx, progress_bar, {
                    width: #(progress_bar_width)
                    draw_bg +: { color: #(downloading_color) }
                });

                self.button(cx, ids!(pause_button)).set_visible(cx, true);
                self.button(cx, ids!(play_button)).set_visible(cx, false);
                self.button(cx, ids!(retry_button)).set_visible(cx, false);
                self.button(cx, ids!(cancel_button)).set_visible(cx, true);
            }
            PendingDownloadsStatus::Paused => {
                let paused_color = vec4(0.4, 0.44, 0.52, 1.0); //#667085

                label.set_text(cx, &format!("Paused {:.1}%", download.progress));
                script_apply_eval!(cx, label, {
                    draw_text +: { color: #(paused_color) }
                });

                let mut progress_bar = self.view(cx, ids!(progress_bar));
                script_apply_eval!(cx, progress_bar, {
                    width: #(progress_bar_width)
                    draw_bg +: { color: #(paused_color) }
                });

                self.button(cx, ids!(pause_button)).set_visible(cx, false);
                self.button(cx, ids!(play_button)).set_visible(cx, true);
                self.button(cx, ids!(retry_button)).set_visible(cx, false);
                self.button(cx, ids!(cancel_button)).set_visible(cx, true);
            }
            PendingDownloadsStatus::Error => {
                let failed_color = vec4(0.7, 0.11, 0.09, 1.0); // #B42318

                label.set_text(cx, &format!("Error {:.1}%", download.progress));
                script_apply_eval!(cx, label, {
                    draw_text +: { color: #(failed_color) }
                });

                let mut progress_bar = self.view(cx, ids!(progress_bar));
                script_apply_eval!(cx, progress_bar, {
                    width: #(progress_bar_width)
                    draw_bg +: { color: #(failed_color) }
                });

                self.button(cx, ids!(pause_button)).set_visible(cx, false);
                self.button(cx, ids!(play_button)).set_visible(cx, false);
                self.button(cx, ids!(retry_button)).set_visible(cx, true);
                self.button(cx, ids!(cancel_button)).set_visible(cx, true);
            }
        }

        let total_size = format_model_size(&download.file.size).unwrap_or("-".to_string());
        let downloaded_size = format_model_downloaded_size(&download.file.size, download.progress)
            .unwrap_or("-".to_string());

        self.label(cx, ids!(downloaded_size))
            .set_text(cx, &format!("{} / {}", downloaded_size, total_size));

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for DownloadItem {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        for actions in actions {
            if let Some(action) = actions.downcast_ref::<DownloadFileAction>() {
                if self.file_id.as_ref() == Some(&action.file_id) {
                    self.redraw(cx);
                }
            }
        }

        for button_id in [ids!(play_button), ids!(retry_button)] {
            if self.button(cx, button_id).clicked(&actions) {
                let Some(file_id) = &self.file_id else { return };
                cx.action(DownloadAction::Play(file_id.clone()));
            }
        }

        if self.button(cx, ids!(pause_button)).clicked(&actions) {
            let Some(file_id) = &self.file_id else { return };
            cx.action(DownloadAction::Pause(file_id.clone()));
        }

        if self.button(cx, ids!(cancel_button)).clicked(&actions) {
            let Some(file_id) = &self.file_id else { return };
            cx.action(DownloadAction::Cancel(file_id.clone()));
        }
    }
}
