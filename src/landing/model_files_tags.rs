use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ModelFilesListLabel = RoundedView {
        width: Fit
        height: Fit
        padding: Inset {top: 6 bottom: 6 left: 10 right: 10}

        draw_bg +: {
            border_radius: uniform(2.0)
            color: #E6F1EC
        }

        label := Label {
            draw_text +: {
                text_style: theme.font_regular {font_size: 9}
                color: #1C1917
            }
        }
    }

    mod.widgets.ModelFilesTagsBase = #(ModelFilesTags::register_widget(vm))
    mod.widgets.ModelFilesTags = set_type_default() do mod.widgets.ModelFilesTagsBase {
        width: Fit
        height: Fit
        flow: Right
        spacing: 5

        template: ModelFilesListLabel {}
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ModelFilesTags {
    #[uid]
    uid: WidgetUid,

    #[redraw]
    #[rust]
    area: Area,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    #[live]
    template: Option<ScriptObjectRef>,

    #[rust]
    items: ComponentMap<LiveId, WidgetRef>,
}

impl Widget for ModelFilesTags {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        for (_id, item) in self.items.iter_mut() {
            item.handle_event(cx, event, scope);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        for (_id, item) in self.items.iter_mut() {
            let _ = item.draw_walk(cx, scope, walk);
        }
        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

impl ModelFilesTagsRef {
    pub fn set_tags(&self, cx: &mut Cx, tags: &Vec<String>) {
        let Some(mut tags_widget) = self.borrow_mut() else {
            return;
        };
        tags_widget.items.clear();
        for (i, tag) in tags.iter().enumerate() {
            let item_id = LiveId(i as u64).into();
            let item_widget = {
                let template = tags_widget.template.clone();
                cx.with_vm(|vm| {
                    let obj = template.as_object().expect("template not set");
                    WidgetRef::script_from_value(vm, obj.into())
                })
            };
            item_widget.label(cx, ids!(label)).set_text(cx, tag);
            tags_widget.items.insert(item_id, item_widget);
        }
    }
}
