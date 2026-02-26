use crate::data::store::Store;
use crate::settings::sync_modal::SyncModalAction;
use makepad_code_editor::code_editor::{CodeEditorAction, KeepCursorInView};
use makepad_code_editor::decoration::DecorationSet;
use makepad_code_editor::{CodeDocument, CodeEditor, CodeSession};

use makepad_widgets::*;

use crate::data::mcp_servers::McpServersConfig;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*
    mod.widgets.MolyCodeViewBase = #(MolyCodeView::register_widget(vm))
    let MolyCodeView = mod.widgets.MolyCodeViewBase {
        editor := CodeEditor {
            pad_left_top: vec2(0.0 -0.0)
            height: Fit
            empty_page_at_end: false
            read_only: true
            show_gutter: false
        }
    }

    let McpCodeView = MolyCodeView {
        editor +: {
            read_only: false
            margin: Inset { top: -2 bottom: 2 }
            pad_left_top: vec2(10.0 10.0)
            width: Fill
            height: Fill
            draw_bg +: { color: #1d2330 }
            draw_text +: {
                text_style +: {
                    font_size: 9
                }
            }

            token_colors +: {
                whitespace: #a8b5d1
                delimiter: #a8b5d1
                delimiter_highlight: #xc5cee0
                error_decoration: #f44747
                warning_decoration: #cd9731

                unknown: #a8b5d1
                branch_keyword: #xd2a6ef
                constant: #ffd9af
                identifier: #a8b5d1
                loop_keyword: #xd2a6ef
                number: #ffd9af
                other_keyword: #xd2a6ef
                punctuator: #a8b5d1
                string: #58ffc7
                function: #82aaff
                typename: #fcf9c3
                comment: #506686
            }
        }
    }

    let ServersEditorWrapper = View {
        AdaptiveView {
            Desktop +: {
                mcp_code_view := McpCodeView {}
            }
            Mobile +: {
                mcp_code_view := MolyTextInput {
                    width: Fill, height: Fill
                }
            }
        }
    }

    let ServersEditor = View {
        width: Fill, height: Fill
        flow: Down
        padding: Inset { left: 20 right: 20 bottom: 5 }
        align: Align { x: 1.0 }

        View {
            width: Fill, height: Fill
            padding: Inset { left: 0 right: 25 top: 8 bottom: 8 }
            servers_editor_wrapper := ServersEditorWrapper {}
        }

        View {
            width: Fill, height: Fit
            align: Align { x: 1.0 y: 0.5 }
            padding: Inset { left: 0 right: 15 top: 8 bottom: 8 }

            save_button := RoundedShadowView {
                cursor: MouseCursor.Hand
                margin: Inset {
                    left: 10 right: 10 bottom: 0 top: 0
                }
                width: Fit, height: Fit
                align: Align { x: 0.5 y: 0.5 }
                padding: Inset {
                    left: 30 right: 30 bottom: 15 top: 15
                }
                draw_bg +: {
                    color: (MAIN_BG_COLOR)
                    border_radius: 4.5
                    shadow_color: uniform(#0002)
                    shadow_radius: 8.0
                    shadow_offset: vec2(0.0 -1.5)
                }
                Label {
                    text: "Save and restart servers"
                    draw_text +: {
                        text_style: REGULAR_FONT { font_size: 11 }
                        color: #000
                    }
                }
            }
        }
    }

    let SaveStatus = View {
        width: Fill, height: Fit
        padding: Inset { left: 10 right: 20 top: 8 bottom: 8 }
        save_status := Label {
            draw_text +: {
                text_style: BOLD_FONT { font_size: 10 }
                color: #000
            }
        }
    }

    let Instructions = View {
        width: 600, height: Fit
        flow: Down, spacing: 10
        instructions := Label {
            width: Fill, height: Fit
            text: "Add new servers by editing the list under 'servers'. You can copy paste you configuration from other applications like Clade Desktop or VSCode.
You can also add an \"enabled\": false flag to disable a specific server."
            draw_text +: {
                text_style: REGULAR_FONT { font_size: 11 }
                color: #000
            }
        }
    }

    let ToggleMCPWrapper = View {
        width: Fit, height: Fit
        spacing: 12
        align: Align { x: 0.5 y: 0.5 }
        Label {
            text: "Enable MCP Servers"
            draw_text +: {
                text_style: BOLD_FONT { font_size: 11 }
                color: #000
            }
        }

        servers_enabled_switch := MolySwitch {
            animator: Animator {
                selected: {
                    default: @on
                }
            }
        }
    }

    let DangerousModeWrapper = View {
        width: Fill, height: Fit
        flow: Down, spacing: 8
        align: Align { x: 0.0 y: 0.5 }

        View {
            width: Fill, height: Fit
            spacing: 12
            align: Align { x: 0.0 y: 0.5 }

            Label {
                text: "⚠️ Dangerous Mode"
                draw_text +: {
                    text_style: BOLD_FONT { font_size: 11 }
                    color: #xFF3333
                }
            }

            dangerous_mode_switch := MolySwitch {
                animator: Animator {
                    selected: {
                        default: @off
                    }
                }
            }
        }

        Label {
            text: "WARNING: This mode automatically approves ALL tool calls without asking for permission.
Only enable if you trust all configured MCP servers completely."
            draw_text +: {
                text_style: REGULAR_FONT { font_size: 11 }
                color: #xFF6666
            }
            width: Fill
        }
    }

    mod.widgets.McpServersBase = #(McpServers::register_widget(vm))
    mod.widgets.McpServers = set_type_default() do mod.widgets.McpServersBase {
        AdaptiveView {
            Desktop +: {
                width: Fill, height: Fill
                flow: Right
                ServersEditor { width: 600 }
                View {
                    flow: Down, spacing: 10
                    margin: Inset { top: 10 }
                    width: Fill, height: Fill
                    ToggleMCPWrapper {}
                    Instructions {}
                    DangerousModeWrapper {}
                    SaveStatus {}
                }
            }
            Mobile +: {
                ScrollYView {
                    flow: Down
                    padding: Inset { left: 10 }
                    ToggleMCPWrapper {}
                    Instructions {
                        width: Fill
                        padding: Inset { left: 0 }
                        Label {
                            width: Fill
                            text: "Note that only HTTP/SSE servers are supported on mobile devices"
                            draw_text +: {
                                text_style: BOLD_FONT { font_size: 11 }
                                color: #xFFA000
                            }
                        }
                    }
                    DangerousModeWrapper {}
                    ServersEditor { width: Fill }
                    SaveStatus {}
                }
            }
        }
    }
}

#[derive(Widget, Script)]
struct McpServers {
    #[deref]
    view: View,

    #[rust]
    mcp_servers_config: McpServersConfig,

    #[rust]
    initialized: bool,
}

impl ScriptHook for McpServers {
    fn on_after_new(&mut self, _vm: &mut ScriptVm) {
        self.mcp_servers_config = McpServersConfig::create_sample();
    }
}

impl Widget for McpServers {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);

        let editor = self.widget(cx, ids!(mcp_code_view));

        if !self.initialized || editor.text().is_empty() {
            self.initialized = true;
            let store = scope.data.get::<Store>().unwrap();
            let mut config = store.get_mcp_servers_config().clone();

            config.enabled = store.preferences.get_mcp_servers_enabled();

            config.dangerous_mode_enabled =
                store.preferences.get_mcp_servers_dangerous_mode_enabled();

            self.set_mcp_servers_config(cx, config);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl McpServers {
    fn set_mcp_servers_config(&mut self, cx: &mut Cx, config: McpServersConfig) {
        self.mcp_servers_config = config;
        let display_json = self
            .mcp_servers_config
            .to_json()
            .unwrap_or_else(|_| "{}".to_string());

        self.widget(cx, ids!(mcp_code_view))
            .set_text(cx, &display_json);

        self.check_box(cx, ids!(servers_enabled_switch))
            .set_active(cx, self.mcp_servers_config.enabled);

        self.check_box(cx, ids!(dangerous_mode_switch))
            .set_active(cx, self.mcp_servers_config.dangerous_mode_enabled);
    }
}

impl WidgetMatchEvent for McpServers {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if self
            .view(cx, ids!(save_button))
            .finger_up(actions)
            .is_some()
        {
            let json_text = self.widget(cx, ids!(mcp_code_view)).text();

            match McpServersConfig::from_json(&json_text) {
                Ok(config) => {
                    let store = scope.data.get_mut::<Store>().unwrap();

                    match store.update_mcp_servers_from_json(&json_text) {
                        Ok(()) => {
                            store.set_mcp_servers_enabled(config.enabled);

                            self.set_mcp_servers_config(cx, config);

                            self.label(cx, ids!(save_status)).set_text(cx, "");
                            self.redraw(cx);
                        }
                        Err(e) => {
                            self.label(cx, ids!(save_status))
                                .set_text(cx, &format!("{}", e));
                            self.redraw(cx);
                        }
                    }
                }
                Err(e) => {
                    self.label(cx, ids!(save_status))
                        .set_text(cx, &format!("Invalid JSON: {}", e));
                    self.redraw(cx);
                }
            }
        }

        let servers_enabled_switch = self.check_box(cx, ids!(servers_enabled_switch));
        if let Some(enabled) = servers_enabled_switch.changed(actions) {
            self.mcp_servers_config.enabled = enabled;

            let display_json = self
                .mcp_servers_config
                .to_json()
                .unwrap_or_else(|_| "{}".to_string());
            self.widget(cx, ids!(mcp_code_view))
                .set_text(cx, &display_json);

            let store = scope.data.get_mut::<Store>().unwrap();
            store.set_mcp_servers_enabled(enabled);
            self.redraw(cx);
        }

        let dangerous_mode_switch = self.check_box(cx, ids!(dangerous_mode_switch));
        if let Some(enabled) = dangerous_mode_switch.changed(actions) {
            self.mcp_servers_config.dangerous_mode_enabled = enabled;

            let display_json = self
                .mcp_servers_config
                .to_json()
                .unwrap_or_else(|_| "{}".to_string());
            self.widget(cx, ids!(mcp_code_view))
                .set_text(cx, &display_json);

            let store = scope.data.get_mut::<Store>().unwrap();
            store.set_mcp_servers_dangerous_mode_enabled(enabled);
            self.redraw(cx);
        }

        for action in actions {
            if let SyncModalAction::McpServersUpdated = action.cast() {
                let store = scope.data.get_mut::<Store>().unwrap();
                self.set_mcp_servers_config(cx, store.get_mcp_servers_config().clone());
                self.redraw(cx);
            }
        }
    }
}

/// Moly's version of Makepad's CodeView (broken upstream)
#[derive(Script, ScriptHook, WidgetRegister, WidgetRef)]
pub struct MolyCodeView {
    #[live]
    pub editor: CodeEditor,
    #[rust]
    pub session: Option<CodeSession>,
    #[live(false)]
    keep_cursor_at_end: bool,

    #[live]
    text: ArcStringMut,
}

impl MolyCodeView {
    pub fn lazy_init_session(&mut self) {
        if self.session.is_none() {
            let dec = DecorationSet::new();
            let doc = CodeDocument::new(self.text.as_ref().into(), dec);
            self.session = Some(CodeSession::new(doc));
            self.session.as_mut().unwrap().handle_changes();
            if self.keep_cursor_at_end {
                self.session.as_mut().unwrap().set_cursor_at_file_end();
                self.editor.keep_cursor_in_view = KeepCursorInView::Once
            }
        }
    }
}

impl WidgetNode for MolyCodeView {
    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.editor.walk(cx)
    }
    fn area(&self) -> Area {
        self.editor.area()
    }
    fn redraw(&mut self, cx: &mut Cx) {
        self.editor.redraw(cx)
    }
    fn find_widgets_from_point(&self, cx: &Cx, point: DVec2, found: &mut dyn FnMut(&WidgetRef)) {
        self.editor.find_widgets_from_point(cx, point, found)
    }
    fn visible(&self) -> bool {
        self.editor.visible()
    }
    fn set_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.editor.set_visible(cx, visible)
    }
}

impl Widget for MolyCodeView {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.lazy_init_session();
        let session = self.session.as_mut().unwrap();

        self.editor.draw_walk_editor(cx, session, walk);

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.lazy_init_session();
        let session = self.session.as_mut().unwrap();
        for action in self
            .editor
            .handle_event(cx, event, &mut Scope::empty(), session)
        {
            session.handle_changes();

            match action {
                CodeEditorAction::TextDidChange => {
                    let document_text = session.document().as_text().to_string();
                    if self.text.as_ref() != &document_text {
                        self.text.as_mut_empty().clear();
                        self.text.as_mut_empty().push_str(&document_text);
                    }
                }
                _ => {}
            }
        }
    }

    fn text(&self) -> String {
        if let Some(session) = &self.session {
            session.document().as_text().to_string()
        } else {
            self.text.as_ref().to_string()
        }
    }

    fn set_text(&mut self, cx: &mut Cx, v: &str) {
        let current_text = if let Some(session) = &self.session {
            session.document().as_text().to_string()
        } else {
            self.text.as_ref().to_string()
        };

        if current_text != v {
            self.text.as_mut_empty().clear();
            self.text.as_mut_empty().push_str(v);

            if let Some(session) = &mut self.session {
                session.document().replace(v.into());
                session.handle_changes();
            }

            self.redraw(cx);
        }
    }
}
