use crate::{aitk::protocol::*, widgets::attachment_view::AttachmentViewWidgetRefExt};
use makepad_widgets::defer_with_redraw::DeferWithRedraw;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    let ITEM_HEIGHT = 200.0
    let ITEM_WIDTH = 200.0
    let ITEM_RADIUS = 8.0

    let DENSE_ITEM_HEIGHT = 100.0
    let DENSE_ITEM_WIDTH = 100.0
    let DENSE_ITEM_RADIUS = 6.0

    mod.widgets.ItemViewBase =
        #(ItemView::register_widget(vm))

    mod.widgets.ItemView =
        set_type_default() do mod.widgets.ItemViewBase RoundedView {
        height: 200.0
        width: 200.0
        margin: Inset{right: 4}
        cursor: MouseCursor.Hand
        draw_bg: {
            radius: 8.0
            border_color: #xD0D5DD
            border_size: 1.0
        }
    }

    mod.widgets.AttachmentListBase =
        #(AttachmentList::register_widget(vm))

    mod.widgets.AttachmentList =
        set_type_default() do mod.widgets.AttachmentListBase {
        height: Fit
        // The wrapper is just to control visibility. If we put this in the
        // main widget, `draw_walk` will not run at all, making visibility
        // binding harder.
        wrapper := View {
            visible: false
            height: Fit
            list := PortalList {
                flow: Right
                height: 200.0
                scroll_bar: {bar_size: 0.0}

                File: ItemView {
                    preview_wrapper := CachedRoundedView {
                        draw_bg: {
                            radius: 8.0
                        }
                        preview := AttachmentView {
                            image_wrapper: {
                                image: {contain: false}
                            }
                            tag_wrapper: {visible: true}
                        }
                    }
                }
            }
        }
    }

    mod.widgets.DenseAttachmentList = AttachmentList {
        wrapper: {
            list: {
                height: 100.0
                File: {
                    height: 100.0
                    width: 100.0
                    draw_bg: {
                        radius: 6.0
                    }
                    preview_wrapper: {
                        draw_bg: {
                            radius: 6.0
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, Widget, ScriptHook)]
pub struct AttachmentList {
    #[deref]
    deref: View,

    #[rust]
    pub attachments: Vec<Attachment>,

    #[rust]
    pub on_tap: Option<Box<dyn FnMut(&mut AttachmentList, usize) + 'static>>,
}

impl Widget for AttachmentList {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view(cx, ids!(wrapper))
            .set_visible(cx, !self.attachments.is_empty());

        let attachments_count = self.attachments.len();
        let list = self.portal_list(cx, ids!(list));
        while let Some(widget) = self.deref.draw_walk(cx, scope, walk).step() {
            if widget.widget_uid() == list.widget_uid() {
                let mut list = list.borrow_mut().unwrap();
                list.set_item_range(cx, 0, attachments_count);
                while let Some(index) = list.next_visible_item(cx) {
                    if index >= attachments_count {
                        continue;
                    }

                    let attachment = &self.attachments[index];
                    let item = list.item(cx, index, live_id!(File));

                    item.attachment_view(cx, ids!(preview))
                        .borrow_mut()
                        .unwrap()
                        .set_attachment(cx, attachment.clone());

                    let ui = self.ui_runner();
                    item.as_item_view().borrow_mut().unwrap().on_tap = Some(Box::new(move || {
                        ui.defer_with_redraw(move |me: &mut AttachmentList, _, _| {
                            if let Some(mut on_tap) = me.on_tap.take() {
                                on_tap(me, index);
                                me.on_tap = Some(on_tap);
                            }
                        });
                    }));

                    item.draw_all_unscoped(cx);
                }
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.ui_runner().handle(cx, event, scope, self);
        self.deref.handle_event(cx, event, scope)
    }
}

impl AttachmentList {
    /// Sets a callback invoked when an attachment item is tapped.
    pub fn on_tap<F>(&mut self, f: F)
    where
        F: FnMut(&mut AttachmentList, usize) + 'static,
    {
        self.on_tap = Some(Box::new(f));
    }
}

impl AttachmentListRef {
    /// Immutable access to the underlying [`AttachmentList`].
    ///
    /// Panics if the widget reference is empty or if it's already borrowed.
    pub fn read(&self) -> std::cell::Ref<'_, AttachmentList> {
        self.borrow().unwrap()
    }

    /// Mutable access to the underlying [`AttachmentList`].
    ///
    /// Panics if the widget reference is empty or if it's already borrowed.
    pub fn write(&mut self) -> std::cell::RefMut<'_, AttachmentList> {
        self.borrow_mut().unwrap()
    }
}

#[derive(Script, Widget, ScriptHook)]
struct ItemView {
    #[deref]
    deref: View,

    #[rust]
    on_tap: Option<Box<dyn FnMut() + 'static>>,
}

impl Widget for ItemView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope);
        if let Hit::FingerUp(fu) = event.hits(cx, self.area()) {
            if fu.was_tap() {
                if let Some(on_tap) = &mut self.on_tap {
                    on_tap();
                }
            }
        }
    }
}
