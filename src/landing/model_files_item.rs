use makepad_widgets::*;
use moly_protocol::data::{File, FileId, PendingDownloadsStatus};

use super::model_files_tags::ModelFilesTagsWidgetExt;
use crate::{
    data::{
        downloads::download::DownloadFileAction,
        store::{FileWithDownloadInfo, Store},
    },
    shared::{
        actions::{ChatAction, DownloadAction},
        utils::format_model_size,
    },
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_DOWNLOAD = crate_resource("self://resources/icons/download.svg")
    let START_CHAT = crate_resource("self://resources/icons/start_chat.svg")

    let ICON_PAUSE = crate_resource("self://resources/icons/pause_download.svg")
    let ICON_CANCEL = crate_resource("self://resources/icons/cancel_download.svg")
    let ICON_PLAY = crate_resource("self://resources/icons/play_download.svg")
    let ICON_RETRY = crate_resource("self://resources/icons/retry.svg")

    mod.widgets.ModelFilesRow = RoundedYView {
        width: Fill
        height: Fit

        show_bg: true
        draw_bg +: {
            color: #00f
            border_radius: vec2(1.0 1.0)
        }

        cell1 := View { width: Fill height: 56 padding: 10 align: Align {x: 0.0 y: 0.5} }
        cell2 := View { width: 140 height: 56 padding: 10 align: Align {x: 0.0 y: 0.5} }
        cell3 := View { width: 340 height: 56 padding: 10 align: Align {x: 0.0 y: 0.5} }
        cell4 := View { width: 250 height: 56 padding: 10 align: Align {x: 0.0 y: 0.5} }
    }

    let ModelCardButton = MolyButton {
        width: 140
        height: 32
        draw_text +: {
            color: #5B6B7D
            text_style: theme.font_bold { font_size: 9}
        }
    }

    let DownloadButton = ModelCardButton {
        draw_bg +: { color: (CTA_BUTTON_COLOR) border_size: 0.0 }
        text: "Download"
        draw_text +: {
            color: (MAIN_BG_COLOR)
        }
        draw_icon +: {
            svg: (ICON_DOWNLOAD)
        }
    }

    let StartChatButton = ModelCardButton {
        draw_bg +: { color: #fff color_hover: #7697E4 border_color_1: (CTA_BUTTON_COLOR) border_size: 1 }
        text: "Chat with Model"
        draw_text +: {
            color: (CTA_BUTTON_COLOR)
        }
        draw_icon +: {
            svg: (START_CHAT)
            color: (CTA_BUTTON_COLOR)
        }
    }

    let DownloadPendingButton = MolyButton {
        width: 25
        height: 25
        padding: 4
        draw_icon +: {
            get_color: fn() -> vec4 {
                return #667085;
            }
        }
    }

    let DownloadPendingControls = View {
        align: Align {y: 0.5}
        spacing: 8
        progress_bar := View {
            width: 74
            height: 12
            flow: Overlay

            RoundedView {
                height: Fill
                draw_bg +: {
                    color: #D9D9D9
                    border_radius: 2.5
                }
            }

            progress_fill := RoundedView {
                width: 0
                height: Fill
                draw_bg +: {
                    border_radius: 2.5
                }
            }
        }
        progress_text_layout := View {
            width: 40
            align: Align {x: 1 y: 0.5}
            progress_text := Label {
                text: "0%"
                draw_text +: {
                    text_style: theme.font_bold {font_size: 9}
                }
            }
        }

        resume_download_button := DownloadPendingButton {
            icon_walk +: { margin: Inset { left: 4 } }
            draw_icon +: {
                svg: (ICON_PLAY)
            }
        }
        retry_download_button := DownloadPendingButton {
            draw_icon +: {
                svg: (ICON_RETRY)
            }
        }
        pause_download_button := DownloadPendingButton {
            icon_walk +: { margin: Inset { left: 4 } }
            draw_icon +: {
                svg: (ICON_PAUSE)
            }
        }
        cancel_download_button := DownloadPendingButton {
            draw_icon +: {
                svg: (ICON_CANCEL)
            }
        }
    }

    mod.widgets.ModelFilesItemBase = #(ModelFilesItem::register_widget(vm))
    mod.widgets.ModelFilesItem = set_type_default() do mod.widgets.ModelFilesItemBase {
        show_bg: true
        draw_bg +: {
            color: #f
        }

        cell1 := View {
            width: Fill height: 56 padding: 10 align: Align {x: 0.0 y: 0.5}
            spacing: 10
            filename := Label {
                draw_text +: {
                    text_style: theme.font_bold {font_size: 9}
                    color: #000
                }
            }
        }

        cell2 := View {
            width: 140 height: 56 padding: 10 align: Align {x: 0.0 y: 0.5}
            full_size := Label {
                draw_text +: {
                    text_style: theme.font_regular {font_size: 9}
                    color: #000
                }
            }
        }

        cell3 := View {
            width: 340 height: 56 padding: 10 align: Align {x: 0.0 y: 0.5}
            spacing: 6
            quantization_tag := RoundedView {
                width: Fit
                height: Fit
                padding: Inset {top: 6 bottom: 6 left: 10 right: 10}

                draw_bg +: {
                    border_radius: instance(2.0)
                    border_color: #B4B4B4
                    border_size: 0.5
                    color: #FFF
                }

                quantization := Label {
                    draw_text +: {
                        text_style: theme.font_regular {font_size: 9}
                        color: #000
                    }
                }
            }
            tags := ModelFilesTags {}
        }

        cell4 := View {
            width: 250 height: 56 padding: 10 align: Align {x: 0.0 y: 0.5}
            download_button := DownloadButton { visible: false }
            start_chat_button := StartChatButton { visible: false }
            download_pending_controls := DownloadPendingControls { visible: false }
        }
    }
}

#[derive(Clone, Default, Debug)]
pub enum ModelFileItemAction {
    Download(FileId),
    #[default]
    None,
}

#[derive(Script, ScriptHook, Widget)]
pub struct ModelFilesItem {
    #[deref]
    view: View,

    #[rust]
    file_id: Option<FileId>,
}

impl Widget for ModelFilesItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let files_info = &scope.props.get::<FileWithDownloadInfo>().unwrap();
        let filename = &files_info.file.name;
        let size = format_model_size(&files_info.file.size).unwrap_or("-".to_string());
        let quantization = &files_info.file.quantization;

        self.label(cx, ids!(cell1.filename)).set_text(cx, filename);
        self.label(cx, ids!(cell2.full_size)).set_text(cx, &size);
        self.label(cx, ids!(cell3.quantization_tag.quantization))
            .set_text(cx, quantization);

        if let Some(download) = &files_info.download {
            let progress = format!("{:.1}%", download.progress);
            let progress_fill_max = 74.0;
            let progress_fill = download.progress * progress_fill_max / 100.0;

            let is_resume_download_visible =
                matches!(download.status, PendingDownloadsStatus::Paused);
            let is_pause_download_visible =
                matches!(download.status, PendingDownloadsStatus::Downloading);
            let is_retry_download_visible =
                matches!(download.status, PendingDownloadsStatus::Error);
            let is_cancel_download_visible =
                !matches!(download.status, PendingDownloadsStatus::Initializing);

            let status_color = match download.status {
                PendingDownloadsStatus::Downloading | PendingDownloadsStatus::Initializing => {
                    vec3(0.035, 0.572, 0.314)
                }
                PendingDownloadsStatus::Paused => vec3(0.4, 0.44, 0.52),
                PendingDownloadsStatus::Error => vec3(0.7, 0.11, 0.09),
            };

            let pending = ids!(cell4.download_pending_controls);
            self.view(cx, pending).set_visible(cx, true);

            let mut progress_text = self.label(
                cx,
                ids!(
                    cell4
                        .download_pending_controls
                        .progress_text_layout
                        .progress_text
                ),
            );
            progress_text.set_text(cx, &progress);
            script_apply_eval!(cx, progress_text, {
                draw_text +: { color: #(status_color) }
            });

            let mut progress_fill_view = self.view(
                cx,
                ids!(cell4.download_pending_controls.progress_bar.progress_fill),
            );
            script_apply_eval!(cx, progress_fill_view, {
                width: #(progress_fill)
                draw_bg +: { color: #(status_color) }
            });

            self.view(
                cx,
                ids!(cell4.download_pending_controls.resume_download_button),
            )
            .set_visible(cx, is_resume_download_visible);
            self.view(
                cx,
                ids!(cell4.download_pending_controls.retry_download_button),
            )
            .set_visible(cx, is_retry_download_visible);
            self.view(
                cx,
                ids!(cell4.download_pending_controls.pause_download_button),
            )
            .set_visible(cx, is_pause_download_visible);
            self.view(
                cx,
                ids!(cell4.download_pending_controls.cancel_download_button),
            )
            .set_visible(cx, is_cancel_download_visible);

            self.view(cx, ids!(cell4.start_chat_button))
                .set_visible(cx, false);
            self.view(cx, ids!(cell4.download_button))
                .set_visible(cx, false);
        } else if files_info.file.downloaded {
            self.view(cx, ids!(cell4.download_pending_controls))
                .set_visible(cx, false);
            self.view(cx, ids!(cell4.start_chat_button))
                .set_visible(cx, true);
            self.view(cx, ids!(cell4.download_button))
                .set_visible(cx, false);
        } else {
            self.view(cx, ids!(cell4.download_pending_controls))
                .set_visible(cx, false);
            self.view(cx, ids!(cell4.start_chat_button))
                .set_visible(cx, false);
            self.view(cx, ids!(cell4.download_button))
                .set_visible(cx, true);
        };

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ModelFilesItem {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        for actions in actions {
            if let Some(action) = actions.downcast_ref::<DownloadFileAction>() {
                if self.file_id.as_ref() == Some(&action.file_id) {
                    self.redraw(cx);
                }
            }
        }

        let Some(file_id) = self.file_id.clone() else {
            return;
        };

        if self.button(cx, ids!(download_button)).clicked(&actions) {
            cx.action(ModelFileItemAction::Download(file_id.clone()));
        }

        if self.button(cx, ids!(start_chat_button)).clicked(&actions) {
            let store = scope.data.get_mut::<Store>().unwrap();
            let bot_id = store.chats.get_bot_id_by_file_id(&file_id);
            if let Some(bot_id) = bot_id {
                cx.action(ChatAction::Start(bot_id));
            }
        }

        if [ids!(resume_download_button), ids!(retry_download_button)]
            .iter()
            .any(|id| self.button(cx, *id).clicked(&actions))
        {
            cx.action(DownloadAction::Play(file_id.clone()));
        }

        if self
            .button(cx, ids!(pause_download_button))
            .clicked(&actions)
        {
            cx.action(DownloadAction::Pause(file_id.clone()));
        }

        if self
            .button(cx, ids!(cancel_download_button))
            .clicked(&actions)
        {
            cx.action(DownloadAction::Cancel(file_id.clone()));
        }
    }
}

impl ModelFilesItemRef {
    pub fn set_file(&mut self, cx: &mut Cx, file: File) {
        let Some(mut item_widget) = self.borrow_mut() else {
            return;
        };

        item_widget.file_id = Some(file.id.clone());

        item_widget
            .model_files_tags(cx, ids!(tags))
            .set_tags(cx, &file.tags);
    }
}
