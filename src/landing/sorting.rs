use crate::data::{search::SortCriteria, store::StoreAction};
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ModelsDropDown = DropDown {
        width: Fit
        height: Fit
        padding: Inset {top: 20.0 right: 10.0 bottom: 20.0 left: 16.0}

        popup_menu_position: BelowInput

        draw_text: {
            text_style: theme.font_bold { font_size: 9 }
            get_color: fn() -> vec4 {
                return mix(
                    #000
                    #000
                    self.focus
                )
            }
        }

        popup_menu: {
            width: 220

            draw_bg: {
                color: #fff
                border_size: 1.5
                //border_color_1: #EAECF0
                border_radius: 4.0
            }

            menu_item: {
                width: Fill
                height: Fit

                padding: Inset {left: 20 top: 15 bottom: 15 right: 20}

                draw_bg: {
                    color: #fff
                    color_active: #xeee9

                    pixel: fn() -> vec4 {
                        let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                        sdf.clear(mix(
                            self.color
                            self.color_active
                            self.hover
                        ))

                        let sz = 3.
                        let dx = 1.6
                        let c = vec2(0.9 * self.rect_size.x 0.5 * self.rect_size.y)
                        sdf.move_to(c.x - sz + dx * 0.5 c.y - sz + dx)
                        sdf.line_to(c.x c.y + sz)
                        sdf.line_to(c.x + sz * 2.0 c.y - sz * 2.0)
                        sdf.stroke(mix(#0000 #0 self.active) 1.0)

                        return sdf.result
                    }
                }

                draw_text: {
                    text_style: theme.font_bold { font_size: 9 }
                    active: instance(0.0)
                    hover: instance(0.0)
                    get_color: fn() -> vec4 {
                        return #000
                    }
                }
            }
        }

        draw_bg: {
            open: instance(0.0)

            get_bg: fn(inout sdf: Sdf2d) {
                sdf.box(
                    2
                    2
                    self.rect_size.x - 4
                    self.rect_size.y - 4
                    4.0
                )
                sdf.stroke_keep(#EAECF0 2.)
                sdf.fill(#fff)
            }

            pixel: fn() -> vec4 {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                self.get_bg(sdf)

                let c = vec2(self.rect_size.x - 20.0 self.rect_size.y * 0.5)
                let sz = 2.5

                if self.open < 0.5 {
                    sdf.move_to(c.x - sz * 2.0 c.y - sz)
                    sdf.line_to(c.x c.y + sz)
                    sdf.line_to(c.x + sz * 2.0 c.y - sz)
                }
                else {
                    sdf.move_to(c.x - sz * 2.0 c.y + sz)
                    sdf.line_to(c.x c.y - sz)
                    sdf.line_to(c.x + sz * 2.0 c.y + sz)
                }
                sdf.stroke(#666 1.0)

                return sdf.result
            }
        }
    }

    mod.widgets.Sorting = #(Sorting::register_widget(vm)) ViewBase {
        width: Fit
        height: Fit
        align: Align {x: 0.5 y: 0.5}

        Label {
            draw_text: {
                text_style: theme.font_regular {font_size: 9}
                color: #667085
            }
            text: "SORT BY"
        }

        options := ModelsDropDown {
            width: 220
            height: Fit

            margin: Inset {left: 20 right: 40}

            labels: ["Most Downloads", "Least Downloads", "Most Likes", "Least Likes"]
            values: [MostDownloads, LeastDownloads, MostLikes, LeastLikes]
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct Sorting {
    #[deref]
    view: View,
}

impl Widget for Sorting {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for Sorting {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if let Some(item_selected) = self.drop_down(cx, ids!(options)).selected(&actions) {
            // TODO Check if we can use liveids instead of item index
            let criteria = match item_selected {
                0 => SortCriteria::MostDownloads,
                1 => SortCriteria::LeastDownloads,
                2 => SortCriteria::MostLikes,
                3 => SortCriteria::LeastLikes,
                4_usize.. => panic!(),
            };

            cx.action(StoreAction::Sort(criteria));
        }
    }
}

impl SortingRef {
    pub fn _set_visible(&self, cx: &mut Cx, visible: bool) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.view.set_visible(cx, visible);
    }

    pub fn set_selected_item(&self, cx: &mut Cx, criteria: SortCriteria) {
        let Some(inner) = self.borrow_mut() else {
            return;
        };
        let criteria_id = match criteria {
            SortCriteria::MostDownloads => 0,
            SortCriteria::LeastDownloads => 1,
            SortCriteria::MostLikes => 2,
            SortCriteria::LeastLikes => 3,
        };
        inner
            .drop_down(cx, ids!(options))
            .set_selected_item(cx, criteria_id);
    }
}
