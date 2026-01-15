use makepad_widgets::*;

use crate::aitk::protocol::*;
use crate::utils::makepad::events::EventExt;
use crate::widgets::attachment_view::AttachmentViewWidgetExt;
use crate::widgets::moly_modal::{MolyModalRef, MolyModalWidgetExt};

live_design! {
    use link::theme::*;
    use link::widgets::*;

    use crate::widgets::moly_modal::*;
    use crate::widgets::attachment_view::*;

    pub AttachmentViewerModal = {{AttachmentViewerModal}} {
        flow: Overlay,
        width: 0,
        height: 0,
        modal = <MolyModal> {
            content: {
                // TODO: Using fill in the content breaks the underlying modal backdrop
                // close on click behavior.
                width: Fill,
                height: Fill,
                <View> {
                    flow: Down,
                    padding: 16,
                    spacing: 16,
                    <View> {
                        height: Fit,
                        align: {x: 1},
                        spacing: 4,
                        save = <Button> {text: "Save"}
                        close = <Button> {text: "X"}
                    }
                    attachment = <AttachmentView> {}
                    text_viewer = <View> {
                        visible: false,
                        width: Fill,
                        height: Fill,
                        text_scroll = <View> {
                            width: Fill,
                            height: Fill,
                            scroll_bars: {show_scroll_x: true, show_scroll_y: true}
                            text_content = <Label> {
                                text: "",
                                padding: 16,
                                draw_text: {
                                    color: #333,
                                    text_style: {font_size: 11, line_spacing: 1.4},
                                    wrap: Word
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget, LiveHook)]
pub struct AttachmentViewerModal {
    #[deref]
    deref: View,

    #[rust]
    current_attachment: Option<Attachment>,
}

impl Widget for AttachmentViewerModal {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope);

        if self.button(ids!(modal.save)).clicked(event.actions()) {
            if let Some(attachment) = &self.current_attachment {
                attachment.save();
            }
        }

        if self.button(ids!(modal.close)).clicked(event.actions()) {
            self.close(cx)
        }
    }
}

impl AttachmentViewerModal {
    pub fn open(&mut self, cx: &mut Cx, attachment: Attachment) {
        self.current_attachment = Some(attachment.clone());
        self.modal_ref().open(cx);

        if attachment.is_text() {
            self.view(ids!(attachment)).set_visible(cx, false);
            self.view(ids!(text_viewer)).set_visible(cx, true);
            self.load_text_content(attachment);
        } else {
            self.view(ids!(attachment)).set_visible(cx, true);
            self.view(ids!(text_viewer)).set_visible(cx, false);
            self.attachment_view(ids!(attachment))
                .borrow_mut()
                .unwrap()
                .set_attachment(cx, attachment);
        }
    }

    fn load_text_content(&mut self, attachment: Attachment) {
        let ui = self.ui_runner();
        crate::aitk::utils::asynchronous::spawn(async move {
            let Ok(content) = attachment.read().await else {
                ::log::error!("Failed to read text attachment {}", attachment.name);
                return;
            };

            let text = String::from_utf8_lossy(&content).into_owned();
            ui.defer_with_redraw(move |me, cx, _| {
                me.label(ids!(text_content)).set_text(cx, &text);
            });
        });
    }

    pub fn close(&mut self, cx: &mut Cx) {
        self.modal_ref().close(cx);
    }

    fn modal_ref(&self) -> MolyModalRef {
        self.moly_modal(ids!(modal))
    }
}
