use super::model_selector_item::{ModelSelectorItemAction, ModelSelectorItemWidgetRefExt};
use crate::{
    aitk::{controllers::chat::ChatController, protocol::*},
    widgets::model_selector::{default_grouping, BotGroup},
};
use makepad_widgets::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// We need a type alias, so Makepad's `#[rust(...)]` macro attribute works.
type ErasedGroupingClosure = Box<dyn Fn(&Bot) -> BotGroup>;

/// Trait for filtering which bots to show in the model selector.
pub trait BotFilter {
    /// Returns whether the given bot should be shown in the list.
    fn should_show(&self, bot: &Bot) -> bool;
}

script_mod! {
    use mod.prelude.widgets.*

    ICON_SIZE = 25.0

    mod.widgets.ModelSelectorList = ModelSelectorList {
        width: Fill, height: Fit
        flow: Down

        item_template := ModelSelectorItem {}

        section_label_template := View {
            width: Fill, height: Fit
            padding: Inset { left: 14, top: 6, bottom: 4 }
            align: Align { x: 0.0, y: 0.5 }
            spacing: 4

            icon_view := View {
                width: Fit, height: Fit
                visible: false
                icon_image := Image {
                    width: (ICON_SIZE), height: (ICON_SIZE)
                }
            }

            icon_fallback_view := RoundedView {
                width: (ICON_SIZE), height: (ICON_SIZE)
                visible: false
                show_bg: true
                draw_bg +: {
                    color: #344054
                    border_radius: 6.0
                }
                align: Align { x: 0.5, y: 0.5 }

                icon_fallback_label := Label {
                    draw_text: {
                        text_style: theme.font_bold { font_size: 13.0 }
                        color: #fff
                    }
                }
            }

            label := Label {
                draw_text: {
                    text_style: theme.font_bold { font_size: 10.0 }
                    color: #989898
                }
            }
        }
    }
}

#[derive(Script, Widget)]
pub struct ModelSelectorList {
    #[redraw]
    #[rust]
    area: Area,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    // Templates collected from DSL in on_after_apply
    #[rust]
    templates: HashMap<LiveId, ScriptObjectRef>,

    #[rust]
    pub items: ComponentMap<LiveId, WidgetRef>,

    #[rust]
    pub search_filter: String,

    #[rust]
    pub total_height: Option<f64>,

    #[rust]
    pub chat_controller: Option<Arc<Mutex<ChatController>>>,

    #[rust(Box::new(default_grouping) as ErasedGroupingClosure)]
    pub grouping: ErasedGroupingClosure,

    #[rust]
    pub filter: Option<Box<dyn BotFilter>>,
}

impl ScriptHook for ModelSelectorList {
    fn on_before_apply(
        &mut self,
        _vm: &mut ScriptVm,
        apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        if apply.is_reload() {
            self.templates.clear();
        }
    }

    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        apply: &Apply,
        _scope: &mut Scope,
        value: ScriptValue,
    ) {
        if !apply.is_eval() {
            if let Some(obj) = value.as_object() {
                vm.vec_with(obj, |vm, vec| {
                    for kv in vec {
                        if let Some(id) = kv.key.as_id() {
                            if let Some(template_obj) = kv.value.as_object() {
                                self.templates
                                    .insert(id, vm.bx.heap.new_object_ref(template_obj));
                            }
                        }
                    }
                });
            }
        }
    }
}

impl Widget for ModelSelectorList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Forward events to child items
        for (_, item) in self.items.iter_mut() {
            item.handle_event(cx, event, scope);
        }

        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);

        // Get bots and selected_bot_id from chat controller
        let (bots, selected_bot_id) = if let Some(chat_controller) = &self.chat_controller {
            {
                let chat_controller = chat_controller.lock().unwrap();
                let state = chat_controller.state();
                (state.bots.clone(), state.bot_id.clone())
            }
        } else {
            (Vec::new(), None)
        };

        self.draw_items(cx, &bots, selected_bot_id.as_ref());

        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

impl WidgetMatchEvent for ModelSelectorList {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        // Catch widget actions from child items and re-fire with our widget_uid.
        // This bubbles the action up to the parent ModelSelector.
        for action in actions {
            let Some(widget_action) = action.as_widget_action() else {
                continue;
            };

            if let ModelSelectorItemAction::BotSelected(bot_id) = widget_action.cast() {
                cx.widget_action(
                    self.widget_uid(),
                    ModelSelectorItemAction::BotSelected(bot_id),
                );
            }
        }
    }
}

impl ModelSelectorList {
    fn create_widget_from_template(
        templates: &HashMap<LiveId, ScriptObjectRef>,
        cx: &mut Cx,
        template_name: LiveId,
    ) -> WidgetRef {
        let template_ref = templates.get(&template_name).expect("template not found");
        let template_value: ScriptValue = template_ref.as_object().into();
        cx.with_vm(|vm| WidgetRef::script_from_value(vm, template_value))
    }

    fn draw_items(&mut self, cx: &mut Cx2d, bots: &[Bot], selected_bot_id: Option<&BotId>) {
        let mut total_height = 0.0;

        // Filter bots based on search
        let terms = self
            .search_filter
            .split_whitespace()
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<_>>();

        let filtered_bots: Vec<&Bot> = bots
            .iter()
            .filter(|bot| {
                // Filter by search terms
                let matches_search = if terms.is_empty() {
                    true
                } else {
                    let name = bot.name.to_ascii_lowercase();
                    let id = bot.id.as_str().to_ascii_lowercase();
                    terms.iter().all(|t| name.contains(t) || id.contains(t))
                };

                // Filter by custom filter function (if provided)
                let passes_filter = self.filter.as_ref().map_or(true, |f| f.should_show(bot));

                matches_search && passes_filter
            })
            .collect();

        // Group bots by their group ID
        let mut groups: HashMap<String, ((String, Option<EntityAvatar>), Vec<&Bot>)> =
            HashMap::new();
        for bot in filtered_bots {
            let group = (self.grouping)(bot);
            groups
                .entry(group.id)
                .or_insert_with(|| ((group.label, group.icon), Vec::new()))
                .1
                .push(bot);
        }

        // Sort groups alphabetically by group ID
        let mut group_list: Vec<_> = groups.into_iter().collect();
        group_list.sort_by(|(a_id, _), (b_id, _)| a_id.cmp(b_id));

        for (group_id, ((group_label, group_icon), mut group_bots)) in group_list {
            // Render section header
            let section_id = LiveId::from_str(&format!("section_{}", group_id));
            let templates = &self.templates;
            let section_label = self.items.get_or_insert(cx, section_id, |cx| {
                Self::create_widget_from_template(templates, cx, id!(section_label_template))
            });

            section_label
                .label(cx, ids!(label))
                .set_text(cx, &group_label);

            match group_icon
                .or_else(|| EntityAvatar::from_first_grapheme(&group_label.to_uppercase()))
                .unwrap_or_else(|| EntityAvatar::Text("?".into()))
            {
                EntityAvatar::Image(image) => {
                    section_label
                        .view(cx, ids!(icon_fallback_view))
                        .set_visible(cx, false);
                    section_label
                        .view(cx, ids!(icon_view))
                        .set_visible(cx, true);
                    let _ = section_label
                        .image(cx, ids!(icon_image))
                        .load_image_dep_by_path(cx, image.as_str())
                        .or_else(|_| {
                            section_label
                                .image(cx, ids!(icon_image))
                                .load_image_file_by_path(cx, image.as_ref())
                        });
                }
                EntityAvatar::Text(text) => {
                    section_label
                        .view(cx, ids!(icon_view))
                        .set_visible(cx, false);
                    section_label
                        .view(cx, ids!(icon_fallback_view))
                        .set_visible(cx, true);
                    section_label
                        .label(cx, ids!(icon_fallback_label))
                        .set_text(cx, &text);
                }
            }

            let _ = section_label.draw_all(cx, &mut Scope::empty());
            total_height += section_label.area().rect(cx).size.y;

            // Sort bots within group by name
            group_bots.sort_by(|a, b| a.name.cmp(&b.name));

            // Render bot items in this group
            for bot in group_bots {
                let item_id = LiveId::from_str(bot.id.as_str());

                let templates = &self.templates;
                let item_widget = self.items.get_or_insert(cx, item_id, |cx| {
                    Self::create_widget_from_template(templates, cx, id!(item_template))
                });

                let mut item = item_widget.as_model_selector_item();
                item.set_bot(bot.clone());

                let is_selected = selected_bot_id == Some(&bot.id);
                item.set_selected(is_selected);

                let _ = item_widget.draw_all(cx, &mut Scope::empty());
                total_height += item_widget.area().rect(cx).size.y;
            }
        }

        self.total_height = Some(total_height);
    }
}

impl ModelSelectorListRef {
    /// Returns the total computed height of all items.
    pub fn get_height(&self) -> f64 {
        if let Some(inner) = self.borrow() {
            inner.total_height.unwrap_or(0.0)
        } else {
            0.0
        }
    }

    /// Sets the search filter and resets the item list.
    pub fn set_search_filter(&mut self, cx: &mut Cx, filter: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.search_filter = filter.to_string();
            inner.items.clear();
            inner.total_height = None;
            inner.redraw(cx);
        }
    }

    /// Clears the search filter.
    pub fn clear_search_filter(&mut self, cx: &mut Cx) {
        self.set_search_filter(cx, "");
    }

    /// Sets the chat controller used to retrieve bot data.
    pub fn set_chat_controller(&mut self, controller: Option<Arc<Mutex<ChatController>>>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.chat_controller = controller;
        }
    }

    /// Sets a custom grouping function for organizing bots.
    pub fn set_grouping<F>(&mut self, grouping: F)
    where
        F: Fn(&Bot) -> BotGroup + 'static,
    {
        if let Some(mut inner) = self.borrow_mut() {
            inner.grouping = Box::new(grouping);
        }
    }
}
