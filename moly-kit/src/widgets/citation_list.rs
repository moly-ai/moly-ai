use makepad_widgets::*;

use super::citation::CitationWidgetRefExt;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.CitationListBase = #(CitationList::register_widget(vm))
    mod.widgets.CitationList = set_type_default() do mod.widgets.CitationListBase {
        width: Fill
        height: Fit
        list := PortalList {
            flow: Right
            width: Fill
            // Fit doesn't work here.
            height: 50
            Citation := Citation {
                // spacing on parent doesn't work
                margin: Inset { right: 8 }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct CitationList {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    deref: View,

    #[rust]
    pub urls: Vec<String>,
}

impl Widget for CitationList {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let list_uid = self.portal_list(cx, ids!(list)).widget_uid();
        while let Some(widget) = self.deref.draw_walk(cx, scope, walk).step() {
            if widget.widget_uid() == list_uid {
                self.draw_list(cx, &mut *widget.as_portal_list().borrow_mut().unwrap());
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope)
    }
}

impl CitationList {
    fn draw_list(&mut self, cx: &mut Cx2d, list: &mut PortalList) {
        list.set_item_range(cx, 0, self.urls.len());
        while let Some(index) = list.next_visible_item(cx) {
            if index >= self.urls.len() {
                continue;
            }

            let item = list.item(cx, index, id!(Citation));
            item.as_citation()
                .borrow_mut()
                .unwrap()
                .set_url_once(cx, self.urls[index].clone());
            item.draw_all(cx, &mut Scope::empty());
        }
    }
}
