use crate::data::store::Store;
use makepad_widgets::*;

#[derive(Clone, DefaultNone, Debug)]
pub enum MemoryModalAction {
    ModalDismissed,
    None,
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;

    ICON_CLOSE = dep("crate://self/resources/icons/close.svg")

    pub MemoryModal = {{MemoryModal}} <RoundedView> {
        flow: Down
        width: 500
        height: Fit
        show_bg: true
        draw_bg: {
            color: #fff
            border_radius: 3.0
        }

        padding: 25
        spacing: 10

        header = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 10
            align: {x: 0.0, y: 0.5}

            title = <View> {
                width: Fill, height: Fit

                title_label = <Label> {
                    width: Fill, height: Fit
                    draw_text: {
                        wrap: Word
                        text_style: <BOLD_FONT>{font_size: 13},
                        color: #000
                    }
                    text: "Manage Memories"
                }
            }

            close_button = <MolyButton> {
                width: Fit, height: Fit
                icon_walk: {width: 14, height: Fit}
                draw_icon: {
                    svg_file: (ICON_CLOSE),
                    fn get_color(self) -> vec4 {
                        return #000;
                    }
                }
            }
        }

        <Label> {
            width: Fill, height: Fit
            draw_text: {
                wrap: Word
                text_style: <REGULAR_FONT>{font_size: 9},
                color: #999
            }
            text: "Facts the AI has saved about you. These are injected into every conversation."
        }

        list = <PortalList> {
            width: Fill
            height: 300
            flow: Down
            scroll_bar: { bar_size: 0.0 }

            MemoryEntry = <RoundedView> {
                width: Fill, height: Fit
                flow: Right
                spacing: 8
                align: {x: 0.0, y: 0.5}
                padding: {top: 8, bottom: 8, left: 10, right: 10}
                margin: {bottom: 4}
                show_bg: true
                draw_bg: {
                    color: #FAFAFA
                    border_radius: 3.0
                }

                content_label = <Label> {
                    width: Fill, height: Fit
                    draw_text: {
                        wrap: Word
                        text_style: <REGULAR_FONT>{font_size: 9.5},
                        color: #333
                    }
                }

                delete_button = <MolyButton> {
                    width: Fit, height: Fit
                    padding: {top: 4, bottom: 4, left: 8, right: 8}
                    text: "Delete"
                    draw_text: {
                        text_style: <REGULAR_FONT>{font_size: 8},
                        color: #d32f2f
                    }
                    draw_bg: {
                        color: #0000
                        border_size: 0.
                    }
                }
            }
        }

        empty_state = <View> {
            width: Fill, height: Fit
            padding: {top: 20, bottom: 20}
            align: {x: 0.5, y: 0.5}

            <Label> {
                width: Fit, height: Fit
                draw_text: {
                    text_style: <REGULAR_FONT>{font_size: 10},
                    color: #999
                }
                text: "No memories saved yet. The AI will save memories as you chat."
            }
        }

        footer = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 10
            align: {x: 1.0, y: 0.5}
            margin: {top: 5}

            clear_all_button = <MolyButton> {
                width: Fit, height: Fit
                padding: {top: 6, bottom: 6, left: 12, right: 12}
                text: "Clear All"
                draw_text: {
                    text_style: <REGULAR_FONT>{font_size: 9},
                    color: #d32f2f
                }
                draw_bg: {
                    color: #FFF
                    border_radius: 3.0
                    border_size: 1.0
                    border_color: #d32f2f
                }
            }
        }
    }
}

#[derive(Live, Widget, LiveHook)]
pub struct MemoryModal {
    #[deref]
    view: View,
}

impl Widget for MemoryModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get_mut::<Store>().unwrap();
        let memories = store.memory_store.memories();
        let is_empty = memories.is_empty();

        self.view(ids!(list)).set_visible(cx, !is_empty);
        self.view(ids!(empty_state)).set_visible(cx, is_empty);
        self.view(ids!(footer)).set_visible(cx, !is_empty);

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, memories.len());
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id >= memories.len() {
                        continue;
                    }
                    let item = list.item(cx, item_id, live_id!(MemoryEntry));
                    item.label(ids!(content_label))
                        .set_text(cx, &memories[item_id].content);
                    item.draw_all(cx, scope);
                }
            }
        }

        DrawStep::done()
    }
}

impl WidgetMatchEvent for MemoryModal {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if self.button(ids!(close_button)).clicked(actions) {
            cx.action(MemoryModalAction::ModalDismissed);
        }

        if self.button(ids!(clear_all_button)).clicked(actions) {
            let store = scope.data.get_mut::<Store>().unwrap();
            store.memory_store.clear();
            self.redraw(cx);
        }

        // Handle individual delete buttons from portal list items.
        let store = scope.data.get_mut::<Store>().unwrap();
        let list = self.portal_list(ids!(list));
        let memories = store.memory_store.memories();

        let deleted_index = list
            .items_with_actions(actions)
            .iter()
            .find(|(_, item)| item.button(ids!(delete_button)).clicked(actions))
            .map(|(index, _)| *index);

        if let Some(index) = deleted_index {
            if let Some(memory) = memories.get(index) {
                store.memory_store.remove(&memory.id);
                self.redraw(cx);
            }
        }
    }
}
