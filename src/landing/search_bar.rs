use crate::data::search::SortCriteria;
use crate::data::store::StoreAction;
use crate::landing::sorting::SortingWidgetExt;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_SEARCH = crate_resource("self://resources/icons/search.svg")
    let ICON_CLOSE = crate_resource("self://resources/icons/close.svg")

    mod.widgets.SearchBarBase = #(SearchBar::register_widget(vm))
    mod.widgets.SearchBar = set_type_default() do mod.widgets.SearchBarBase {
        width: Fill
        height: 200

        flow: Down
        spacing: 30
        align: Align {x: 0.5 y: 0.5}

        show_bg: true

        draw_bg +: {
            color: instance((MAIN_BG_COLOR_DARK))
            color2: instance(#xa6bec6)
            get_color: fn() -> vec4 {
                let coef = self.rect_size.y / self.rect_size.x

                let distance_vec = self.pos - vec2(0.8 1.1)
                let norm_distance = length(vec2(distance_vec.x distance_vec.y * coef) * 2.2)

                if pow(norm_distance 1.4) > 1.0 {
                    return self.color
                } else {
                    return mix(self.color2 self.color pow(norm_distance 1.4))
                }
            }

            pixel: fn() -> vec4 {
                return Pal.premul(self.get_color())
            }
        }

        title := View {
            width: Fit
            height: Fit
            Label {
                draw_text +: {
                    text_style: theme.font_regular {font_size: 13}
                    color: #000
                }
                text: "Discover, download, and run local LLMs"
            }
        }

        input_container := RoundedShadowView {
            width: 800
            height: Fit

            show_bg: true
            draw_bg +: {
                color: instance((MAIN_BG_COLOR))
                border_radius: 8.5
                shadow_color: uniform(#x0001)
                shadow_radius: 8.0
                shadow_offset: vec2(0.0, -2.0)
            }

            padding: Inset {top: 3 bottom: 3 left: 20 right: 20}
            margin: Inset {left: 30 right: 30}

            spacing: 4
            align: Align {x: 0.0 y: 0.5}

            Icon {
                draw_icon +: {
                    svg: (ICON_SEARCH)
                    get_color: fn() -> vec4 {
                        return #666
                    }
                }
                icon_walk +: {width: 17 height: 17}
            }

            input := MolyTextInput {
                width: Fill
                height: Fit
                empty_text: "Search Model by Keyword"
                draw_bg +: {
                    color: (MAIN_BG_COLOR)
                }
            }

            clear_text_button := MolyButton {
                visible: false
                draw_icon +: {
                    svg: (ICON_CLOSE)
                    get_color: fn() -> vec4 {
                        return #8
                    }
                }
                icon_walk +: {width: 10 height: 10}
            }
        }

        search_sorting := View {
            visible: false
            width: 300
            height: Fit
            margin: Inset {left: 30 right: 30}
            mod.widgets.Sorting {}
        }

        animator: Animator {
            search_bar: {
                default: @expanded
                collapsed: AnimatorState {
                    redraw: true
                    from: {all: Forward {duration: 0.3}}
                    ease: ExpDecay {d1: 0.80 d2: 0.97}
                    apply: { height: 100 }
                }
                expanded: AnimatorState {
                    redraw: true
                    from: {all: Forward {duration: 0.3}}
                    ease: ExpDecay {d1: 0.80 d2: 0.97}
                    apply: { height: 200 }
                }
            }
        }
    }
}

#[derive(Animator, Script, ScriptHook, Widget)]
pub struct SearchBar {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,

    #[apply_default]
    animator: Animator,

    #[rust]
    collapsed: bool,

    #[rust]
    search_timer: Timer,

    #[live(0.3)]
    search_debounce_time: f64,
}

impl Widget for SearchBar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        if self.search_timer.is_event(event).is_some() {
            self.search_timer = Timer::default();

            let input = self.text_input(cx, ids!(input));
            let keywords = input.text();
            const MIN_SEARCH_LENGTH: usize = 2;

            if keywords.len() > MIN_SEARCH_LENGTH {
                cx.action(StoreAction::Search(keywords.to_string()));
            } else if keywords.len() == 0 {
                cx.action(StoreAction::ResetSearch);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for SearchBar {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let input = self.text_input(cx, ids!(input));
        let clear_text_button = self.button(cx, ids!(clear_text_button));

        if let Some((keywords, _)) = input.returned(actions) {
            if keywords.len() > 0 {
                cx.action(StoreAction::Search(keywords.to_string()));
            } else {
                cx.action(StoreAction::ResetSearch);
            }
        }

        if let Some(text) = input.changed(actions) {
            clear_text_button.set_visible(cx, !text.is_empty());
            cx.stop_timer(self.search_timer);
            self.search_timer = cx.start_timeout(self.search_debounce_time);
        }

        if self.button(cx, ids!(clear_text_button)).clicked(actions) {
            input.set_text(cx, "");
            clear_text_button.set_visible(cx, false);
            input.set_key_focus(cx);

            cx.action(StoreAction::ResetSearch);
        }
    }
}

impl SearchBarRef {
    pub fn collapse(&self, cx: &mut Cx, selected_sort: SortCriteria) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        if inner.collapsed {
            return;
        }
        inner.collapsed = true;

        let flow_right = Flow::default();
        let fill = Size::fill();
        script_apply_eval!(cx, inner, {
            flow: #(flow_right)
            align: {x: 0.0 y: 0.5}
            padding: {left: 20}
            spacing: 80
        });

        inner.view(cx, ids!(title)).set_visible(cx, false);

        let mut input_container = inner.view(cx, ids!(input_container));
        script_apply_eval!(cx, input_container, { width: #(fill) });

        inner.view(cx, ids!(search_sorting)).set_visible(cx, true);

        inner
            .sorting(cx, ids!(search_sorting))
            .set_selected_item(cx, selected_sort);
        inner.animator_play(cx, ids!(search_bar.collapsed));
    }

    pub fn expand(&self, cx: &mut Cx) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        if !inner.collapsed {
            return;
        }
        inner.collapsed = false;

        let flow_down = Flow::Down;
        script_apply_eval!(cx, inner, {
            flow: #(flow_down)
            align: {x: 0.5 y: 0.5}
            padding: {left: 0}
            spacing: 50
        });

        inner.view(cx, ids!(title)).set_visible(cx, true);

        let mut input_container = inner.view(cx, ids!(input_container));
        script_apply_eval!(cx, input_container, { width: 800 });

        inner.view(cx, ids!(search_sorting)).set_visible(cx, false);

        inner.animator_play(cx, ids!(search_bar.expanded));
    }
}
