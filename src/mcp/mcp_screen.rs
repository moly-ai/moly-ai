use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    use mod.widgets.McpServers

    mod.widgets.McpScreen = McpScreen {
        #(McpScreen::register_widget(vm))
        width: Fill, height: Fill
        spacing: 20
        flow: Down

        header := View {
            height: Fit
            spacing: 20
            flow: Down

            padding: Inset { left: 30 top: 40 }
            Label {
                draw_text: {
                    text_style: BOLD_FONT { font_size: 25 }
                    color: #000
                }
                text: "MCP Servers"
            }

            Label {
                draw_text: {
                    text_style: BOLD_FONT { font_size: 12 }
                    color: #000
                }
                text: "Manage MCP servers and tools"
            }
        }

        mcp_servers := McpServers {}
    }
}

#[derive(Widget, ScriptHook, Script)]
pub struct McpScreen {
    #[deref]
    view: View,
}

impl Widget for McpScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for McpScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let stack_navigation = self.stack_navigation(ids!(navigation));
        stack_navigation.handle_stack_view_actions(cx, actions);
    }
}
