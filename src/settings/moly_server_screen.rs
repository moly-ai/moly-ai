use makepad_widgets::*;

use crate::{app::NavigationAction, data::store::Store};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_DISCOVER =
        crate_resource("self://resources/icons/discover.svg")
    let ICON_MY_MODELS =
        crate_resource("self://resources/icons/my_models.svg")
    let ICON_CLOUD =
        crate_resource("self://resources/icons/cloud.svg")
    let ICON_RETRY =
        crate_resource("self://resources/icons/retry.svg")

    let SUBSIDEBAR_BG_COLOR = (MAIN_BG_COLOR)
    let SUBSIDEBAR_FONT_COLOR = #x2C3E50
    let SUBSIDEBAR_FONT_COLOR_HOVER = #x2C3E50
    let SUBSIDEBAR_FONT_COLOR_SELECTED = #344054

    let SubSidebarMenuButton = SidebarMenuButton {
        width: Fill
        height: Fit
        padding: Inset { top: 8 bottom: 8 left: 15 }
        flow: Right
        align: Align { x: 0.0 y: 0.5 }

        icon_walk +: { margin: 0 width: 22 height: 22 }

        draw_bg +: {
            border_size: uniform(0.0)
            border_radius: uniform(2.5)

            color: uniform(#xf9f9f9)
            color_hover: uniform(#xebedee)
            color_active: uniform(#xebedee)
        }

        draw_text +: {
            color: (SUBSIDEBAR_FONT_COLOR)
            color_hover: (SUBSIDEBAR_FONT_COLOR_HOVER)
            color_active: (SUBSIDEBAR_FONT_COLOR_SELECTED)

            text_style: REGULAR_FONT { font_size: 10 }
        }

        draw_icon +: {
            color: #x2C3E50
            color_hover: #x2C3E50
            color_active: #x344054
        }
    }

    let MolyServerNotAccesible = View {
        visible: false
        padding: Inset { left: 30 top: 40 }
        spacing: 50
        flow: Down

        header := View {
            height: Fit
            flow: Down
            spacing: 40
            Label {
                draw_text +: {
                    text_style: BOLD_FONT { font_size: 25 }
                    color: #000
                }
                text: "MolyServer (disconnected)"
            }

            Label {
                width: Fill
                height: Fit
                draw_text +: {
                    text_style +: { font_size: 12 }
                    color: #000
                }
                text: "MolyServer is a local HTTP server that powers the Moly app by providing capabilities for searching, downloading, and running local LLMs).\nYou can install MolyServer by following the instructions in https://github.com/moly-ai/moly-local."
            }
        }

        View {
            height: Fit
            width: Fill
            spacing: 20
            flow: Down
            Label {
                width: Fit
                height: Fit
                draw_text +: {
                    text_style: BOLD_FONT { font_size: 12 }
                    color: #000
                }
                text: "We could not reach the server.\nPlease make sure it is running and that MolyServer is enabled in the provider settings."
            }

            View {
                width: Fill
                height: Fit
                spacing: 8
                go_to_providers := MolyButton {
                    draw_bg +: {
                        color: (CTA_BUTTON_COLOR)
                        border_size: 0
                    }
                    draw_icon +: {
                        svg: ICON_CLOUD
                    }
                    draw_text +: {
                        text_style: BOLD_FONT { font_size: 10 }
                    }
                    text: "Go to Providers"
                }
                refresh := MolyButton {
                    draw_bg +: {
                        color: (CTA_BUTTON_COLOR)
                        border_size: 0
                    }
                    draw_icon +: {
                        svg: ICON_RETRY
                    }
                    draw_text +: {
                        text_style: BOLD_FONT { font_size: 10 }
                    }
                    text: "Refresh"
                }
            }
        }
    }

    mod.widgets.MolyServerScreenBase = #(MolyServerScreen::register_widget(vm))
    mod.widgets.MolyServerScreen =
        set_type_default() do mod.widgets.MolyServerScreenBase {
        main_content := View {
            visible: false
            menu := RoundedView {
                width: 130
                height: Fill
                flow: Down
                padding: Inset {
                    top: 50 bottom: 20 left: 5 right: 8
                }

                show_bg: true
                draw_bg +: {
                    color: (SUBSIDEBAR_BG_COLOR)
                    border_radius: uniform(0.0)
                }

                discover_tab := SubSidebarMenuButton {
                    text: "Discover"
                    draw_icon +: {
                        svg: ICON_DISCOVER
                    }
                }
                my_models_tab := SubSidebarMenuButton {
                    text: "My Models"
                    draw_icon +: {
                        svg: ICON_MY_MODELS
                    }
                }
            }

            right_border := SolidView {
                width: 1.6
                height: Fill
                margin: Inset { top: 15 bottom: 15 }
                draw_bg +: {
                    color: #xeaeaea
                }
            }

            pages := View {
                discover_frame := View {
                    visible: true
                    LandingScreen {}
                }
                my_models_frame := View {
                    visible: false
                    MyModelsScreen {}
                }
            }
        }

        server_not_accessible := MolyServerNotAccesible {}
    }
}

#[derive(Widget, Script)]
pub struct MolyServerScreen {
    #[deref]
    view: View,
}

impl ScriptHook for MolyServerScreen {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        vm.with_cx_mut(|cx| {
            if let Some(mut rb) = self
                .view
                .radio_button(cx, ids!(menu.discover_tab))
                .borrow_mut()
            {
                rb.animator_play(cx, ids!(active.on));
            }
        });
    }
}

impl Widget for MolyServerScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get_mut::<Store>().unwrap();
        if store.is_moly_server_connected() {
            self.view(cx, ids!(server_not_accessible))
                .set_visible(cx, false);
            self.view(cx, ids!(main_content)).set_visible(cx, true);
        } else {
            self.view(cx, ids!(server_not_accessible))
                .set_visible(cx, true);
            self.view(cx, ids!(main_content)).set_visible(cx, false);
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for MolyServerScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let selected_index = self
            .radio_button_set(cx, ids_array!(menu.discover_tab, menu.my_models_tab))
            .selected(cx, actions);

        let discover_frame = self.view(cx, ids!(pages.discover_frame));
        let my_models_frame = self.view(cx, ids!(pages.my_models_frame));

        match selected_index {
            Some(0) => {
                discover_frame.set_visible(cx, true);
                my_models_frame.set_visible(cx, false);
                self.redraw(cx);
            }
            Some(1) => {
                discover_frame.set_visible(cx, false);
                my_models_frame.set_visible(cx, true);
                self.redraw(cx);
            }
            _ => (),
        }

        if self.button(cx, ids!(go_to_providers)).clicked(actions) {
            cx.action(NavigationAction::NavigateToProviders);
        }
        if self.button(cx, ids!(refresh)).clicked(actions) {
            let store = scope.data.get_mut::<Store>().unwrap();
            store.sync_with_moly_server();
        }
    }
}
