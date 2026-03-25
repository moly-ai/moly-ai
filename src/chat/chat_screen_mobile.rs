use makepad_widgets::*;
use moly_kit::prelude::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_NEW_CHAT = crate_resource("self://resources/icons/new_chat.svg")

    let SettingsMenu = MolyModal {
        align: Align {x: 0.0 y: 0.0}
        bg_view +: {
            visible: false
        }
        content +: {
            width: Fit height: Fit
            RoundedView {
                draw_bg +: {
                    color: #fff
                    border_radius: 5.0
                    border_size: 1.0
                    border_color: #xD0D5DD
                }
                width: 150 height: Fit
                align: Align {x: 0.5 y: 0.5}
                flow: Down
                spacing: 10
                padding: Inset {
                    left: 10 right: 10 top: 20 bottom: 20
                }
                go_to_providers := View {
                    align: Align {x: 0.5 y: 0.5}
                    width: Fill height: Fit
                    cursor: MouseCursor.Hand
                    Label {
                        text: "Providers"
                        draw_text +: {
                            color: #x0
                        }
                    }
                }

                separator := SolidView {
                    width: Fill height: 0.5
                    draw_bg +: {color: #xd3d3d3}
                    margin: Inset {left: 10 right: 10}
                }

                go_to_mcp := View {
                    align: Align {x: 0.5 y: 0.5}
                    width: Fill height: Fit
                    cursor: MouseCursor.Hand
                    Label {
                        text: "MCP Servers"
                        draw_text +: {
                            color: #x0
                        }
                    }
                }
            }
        }
    }

    let HEADER_HEIGHT = 100
    let MolyNavigationView = StackNavigationView {
        width: Fill height: Fill
        draw_bg +: { color: (MAIN_BG_COLOR) }
        header +: {
            height: (HEADER_HEIGHT)
            content +: {
                padding: Inset {top: 10}
                align: Align {y: 0.5}
                button_container +: {
                    align: Align {y: 0.5}
                    padding: Inset {left: 16}
                    left_button +: {
                        height: Fit
                        icon_walk +: {width: 12 height: Fit}
                        draw_icon +: {
                            brightness: 0.0
                        }
                    }
                }
                title_container +: {
                    title +: {
                        draw_text +: {
                            text_style: theme.font_bold {font_size: 14}
                            color: #x0
                        }
                    }
                }
            }
        }
        body +: { margin: Inset {top: (HEADER_HEIGHT) }}
    }

    mod.widgets.ChatScreenMobileBase = #(ChatScreenMobile::register_widget(vm))
    mod.widgets.ChatScreenMobile =
        set_type_default() do mod.widgets.ChatScreenMobileBase {
        width: Fill height: Fill
        flow: Overlay
        margin: Inset { top: 40 }

        navigation := StackNavigation {
            width: Fill height: Fill
            root_view +: {
                width: Fill height: Fill
                flow: Overlay
                menu_toggle := View {
                    margin: Inset {top: 10 left: 20}
                    width: Fit height: Fit
                    cursor: MouseCursor.Hand
                    IconSet {
                        text: "\u{f0c9}"
                        draw_text +: {
                            color: #x0
                            text_style +: { font_size: 18.0 }
                        }
                    }
                }

                CachedWidget {
                    chats_deck := ChatsDeck {}
                }
            }

            history_navigation_view := MolyNavigationView {
                header +: {
                    content +: {
                        title_container +: {
                            title +: {
                                text: "Chat History"
                            }
                        }
                        settings_button := View {
                            margin: Inset {left: 100}
                            align: Align {x: 1.0 y: 0.5}
                            width: Fill height: Fit
                            margin: Inset {right: 15}
                            cursor: MouseCursor.Hand
                            IconSet {
                                text: "\u{f013}"
                                draw_text +: {
                                    color: #x333
                                    text_style +: { font_size: 18.0 }
                                }
                            }
                        }
                        settings_menu := SettingsMenu {}
                    }
                }
                body +: {
                    flow: Overlay
                    chat_history := ChatHistory {
                        width: Fill height: Fill
                    }
                    align: Align { x: 0.95 y: 0.95 }
                    RoundedView {
                        show_bg: true
                        draw_bg +: {
                            color: #x0
                            border_radius: uniform(5.0)
                        }
                        width: Fit height: Fit

                        align: Align { x: 0.5 y: 0.5 }
                        new_chat_button := MolyButton {
                            width: Fit height: Fit
                            padding: Inset {
                                left: 10 right: 10 top: 10 bottom: 10
                            }
                            icon_walk +: {
                                margin: Inset { top: -1 }
                                width: 18 height: 18
                            }
                            text: "New Chat"
                            draw_text +: {
                                color: #xf
                                text_style +: { font_size: 12.0 }
                            }
                            draw_icon +: {
                                svg: (ICON_NEW_CHAT)
                                color: #xf
                            }
                        }
                    }
                }
            }

            mcp_navigation_view := MolyNavigationView {
                header +: {
                    content +: {
                        title_container +: {
                            title +: {
                                text: "MCP Servers"
                            }
                        }
                    }
                }
                body +: {
                    mcp_screen := mod.widgets.McpScreen {
                        width: Fill height: Fill
                        header +: { visible: false }
                    }
                }
            }

            providers_navigation_view := MolyNavigationView {
                header +: {
                    content +: {
                        title_container +: {
                            title +: {
                                text: "Providers"
                            }
                        }
                    }
                }
                body +: {
                    providers := mod.widgets.ProvidersScreen {
                        width: Fill height: Fill
                        header +: { visible: false }
                    }
                }
            }

            provider_navigation_view := MolyNavigationView {
                header +: {
                    content +: {
                        title_container +: {
                            title +: {
                                text: "Provider Settings"
                            }
                        }
                    }
                }
                body +: {
                    padding: Inset {top: 10}
                    provider_view := mod.widgets.ProviderView {
                        width: Fill height: Fill
                        padding: Inset {
                            left: 20 right: 20 top: 30 bottom: 30
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ChatScreenMobile {
    #[deref]
    view: View,
}

impl Widget for ChatScreenMobile {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ChatScreenMobile {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let stack_navigation = self.stack_navigation(cx, ids!(navigation));
        stack_navigation.handle_stack_view_actions(cx, actions);

        // Menu Toggle
        if let Some(_evt) = self.view(cx, ids!(menu_toggle)).finger_down(actions) {
            stack_navigation.push(cx, id!(history_navigation_view));
        }

        let modal = self.moly_modal(cx, ids!(settings_menu));

        // Settings Menu
        if let Some(_evt) = self.view(cx, ids!(settings_button)).finger_down(actions) {
            let parent_view_width = self
                .stack_navigation_view(cx, ids!(history_navigation_view))
                .area()
                .rect(cx)
                .size
                .x;

            let button_rect = self.view(cx, ids!(settings_button)).area().rect(cx);
            let coords = dvec2(
                parent_view_width - 170.0,
                button_rect.pos.y + button_rect.size.y,
            );

            let margin = Inset {
                left: coords.x,
                top: coords.y,
                right: 0.0,
                bottom: 0.0,
            };
            let mut content = self.view(cx, ids!(settings_menu.content));
            script_apply_eval!(cx, content, { margin: #(margin) });
            modal.open(cx);
        }

        // Go to Providers
        if let Some(_evt) = self.view(cx, ids!(go_to_providers)).finger_down(actions) {
            modal.close(cx);
            stack_navigation.push(cx, id!(providers_navigation_view));
        }

        // Go to MCP Servers
        if let Some(_evt) = self.view(cx, ids!(go_to_mcp)).finger_down(actions) {
            modal.close(cx);
            stack_navigation.push(cx, id!(mcp_navigation_view));
        }
    }
}
