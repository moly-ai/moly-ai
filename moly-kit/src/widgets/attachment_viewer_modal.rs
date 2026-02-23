use makepad_widgets::*;

use crate::aitk::protocol::*;
use crate::utils::makepad::events::EventExt;
use crate::widgets::attachment_view::AttachmentViewWidgetExt;
use crate::widgets::moly_modal::{MolyModalRef, MolyModalWidgetExt};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.AttachmentViewerModal =
        #(AttachmentViewerModal::register_widget(vm)) View {
        flow: Overlay
        width: 0
        height: 0
        modal := MolyModal {
            content +: {
                width: Fill
                height: Fill
                View {
                    flow: Down
                    padding: 16
                    spacing: 16
                    View {
                        height: Fit
                        align: Align { x: 1 }
                        spacing: 4
                        save := Button { text: "Save" }
                        close := Button { text: "X" }
                    }
                    attachment := AttachmentView {}
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct AttachmentViewerModal {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    deref: View,
}

impl Widget for AttachmentViewerModal {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope);

        if self.button(cx, ids!(modal.save)).clicked(event.actions()) {
            self.attachment_view(cx, ids!(attachment))
                .borrow()
                .unwrap()
                .get_attachment()
                .save();
        }

        if self.button(cx, ids!(modal.close)).clicked(event.actions()) {
            self.close(cx)
        }
    }
}

impl AttachmentViewerModal {
    pub fn open(&mut self, cx: &mut Cx, attachment: Attachment) {
        self.modal_ref(cx).open(cx);
        self.attachment_view(cx, ids!(attachment))
            .borrow_mut()
            .unwrap()
            .set_attachment(cx, attachment);
    }

    pub fn close(&mut self, cx: &mut Cx) {
        self.modal_ref(cx).close(cx);
    }

    fn modal_ref(&self, cx: &Cx) -> MolyModalRef {
        self.moly_modal(cx, ids!(modal))
    }
}
