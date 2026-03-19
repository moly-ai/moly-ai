use makepad_widgets::*;
use moly_kit::prelude::*;

use crate::data::{
    providers::{Provider, ProviderBot, ProviderConnectionStatus, ProviderType},
    store::Store,
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let REFRESH_ICON =
        crate_resource("self://resources/images/refresh_icon.png")

    let IconButton = Button {
        width: Fit
        height: Fit
        draw_text +: {
            text_style: theme.font_icons {
                font_size: 14.
            }
            color: #5
            color_hover: #2
            color_focus: #2
            color_down: #5
        }
        draw_bg +: {
            color_down: #0000
            border_radius: 7.
            border_size: 0.
        }
    }

    let FormGroup = View {
        flow: Down
        height: Fit
    }

    mod.widgets.ModelEntryBase = #(ModelEntry::register_widget(vm))
    let ModelEntry = mod.widgets.ModelEntryBase {
        align: Align { x: 0.5 y: 0.5 }
        width: Fill
        height: 50
        flow: Down
        separator := View {
            height: 1
            show_bg: true
            draw_bg +: {
                color: #D9D9D9
            }
        }

        content := View {
            flow: Right
            width: Fill
            height: Fill
            align: Align { x: 0.5 y: 0.5 }
            model_name := Label {
                text: "Model Name"
                draw_text +: {
                    text_style: REGULAR_FONT { font_size: 11 }
                    color: #000
                }
            }

            vertical_filler := View {
                width: Fill
                height: 1
            }

            enabled_toggle := View {
                flow: Right
                height: Fit
                width: Fill
                align: Align { x: 1.0 y: 0.5 }
                spacing: 20
                enabled_switch := MolySwitch {
                    animator: Animator {
                        selected: {
                            default: @on
                        }
                    }
                }
            }
        }
    }

    let HeaderEntry = View {
        width: Fill
        height: Fit
        flow: Down
        padding: Inset { top: 10 }

        label := Label {
            draw_text +: {
                text_style: BOLD_FONT { font_size: 13.5 }
                color: #555
            }
        }

        separator := View {
            margin: Inset { top: 10 }
            height: 1
            show_bg: true
            draw_bg +: {
                color: #D9D9D9
            }
        }
    }

    mod.widgets.ProviderViewBase = #(ProviderView::register_widget(vm))
    mod.widgets.ProviderView =
        set_type_default() do mod.widgets.ProviderViewBase {
        width: Fill
        height: Fill
        show_bg: true
        draw_bg +: {
            color: (MAIN_BG_COLOR_DARK)
            border_radius: 4.5
            shadow_color: uniform(#0002)
            shadow_radius: 8.0
            shadow_offset: vec2(0.0, -1.5)
        }

        content := ScrollYView {
            flow: Down
            height: Fill
            padding: 0
            scroll_bars: ScrollBars {
                scroll_bar_y: ScrollBar {
                    bar_size: 7.
                    draw_bg +: {
                        color: #d5d4d4
                        color_hover: #b8b8b8
                        color_drag: #a8a8a8
                    }
                }
            }

            FormGroup {
                flow: Right
                View {
                    flow: Down
                    width: Fit
                    height: Fit
                    name := Label {
                        draw_text +: {
                            text_style: BOLD_FONT { font_size: 15 }
                            color: #000
                        }
                    }

                    View {
                        width: Fit
                        height: Fit
                        margin: Inset { top: 10 }
                        Label {
                            text: "Type:"
                            draw_text +: {
                                text_style: BOLD_FONT { font_size: 11 }
                                color: #000
                            }
                        }
                        provider_type := Label {
                            margin: Inset { left: 4 }
                            draw_text +: {
                                text_style +: { font_size: 11 }
                                color: #000
                            }
                        }
                    }
                }

                View { width: Fill height: 0 }

                View {
                    margin: Inset { top: 10 }
                    align: Align { x: 0.5 y: 0.5 }
                    width: Fit
                    height: Fit
                    flow: Right
                    refresh_button := View {
                        visible: false
                        cursor: MouseCursor.Hand
                        width: Fit
                        height: Fit

                        icon := Image {
                            width: 22
                            height: 22
                            src: (REFRESH_ICON)
                        }
                    }
                    provider_enabled_switch := MolySwitch {
                        margin: Inset { left: 10 }
                        animator: Animator {
                            selected: {
                                default: @on
                            }
                        }
                    }
                }
            }

            separator := View {
                margin: Inset { top: 15 }
                height: 1
                show_bg: true
                draw_bg +: {
                    color: #D9D9D9
                }
            }

            FormGroup {
                margin: Inset { top: 15 }
                Label {
                    text: "API Host"
                    draw_text +: {
                        text_style: BOLD_FONT { font_size: 12 }
                        color: #000
                    }
                }

                View {
                    width: Fill
                    height: 35
                    api_host := MolyTextInput {
                        width: Fill
                        height: 30
                        empty_text: "https://some-api.com/v1"
                        draw_text +: {
                            text_style: REGULAR_FONT { font_size: 12 }
                            color: #000
                        }
                        is_multiline: false
                        input_mode: Url
                        autocorrect: Disabled
                        autocapitalize: None
                        return_key_type: Go
                    }
                }
            }

            FormGroup {
                margin: Inset { top: 10 }
                Label {
                    text: "API Key"
                    draw_text +: {
                        text_style: BOLD_FONT { font_size: 12 }
                        color: #000
                    }
                }

                View {
                    align: Align { x: 0.0 y: 0.5 }
                    width: Fill
                    height: 35
                    api_key := MolyTextInput {
                        empty_text: ""
                        width: Fill
                        height: 30
                        draw_text +: {
                            text_style: REGULAR_FONT {
                                font_size: 12
                            }
                            color: #000
                        }
                        is_password: true
                        is_multiline: false
                    }

                    toggle_key_visibility := IconButton {
                        text: "\u{f06e}"
                    }
                }
                View {
                    margin: Inset { top: 10 }
                    width: Fill
                    height: Fit
                    align: Align { x: 0.0 y: 0.5 }
                    connection_status := Label {
                        draw_text +: {
                            text_style: BOLD_FONT { font_size: 10 }
                            color: #000
                        }
                    }
                }
            }

            system_prompt_group := FormGroup {
                margin: Inset { top: 10 }
                height: Fit
                visible: false
                Label {
                    text: "System Prompt"
                    draw_text +: {
                        text_style: BOLD_FONT { font_size: 12 }
                        color: #000
                    }
                }

                View {
                    height: 85
                    scroll_bars: ScrollBars {
                        show_scroll_x: false
                        show_scroll_y: true
                        scroll_bar_y: ScrollBar {
                            draw_bg +: {
                                color: #D9
                                color_hover: #888
                                color_drag: #777
                            }
                        }
                    }
                    system_prompt := MolyTextInput {
                        width: Fill
                        height: Fit
                        empty_text: "Optional: enter a custom system prompt.\nWhen using a custom prompt, we recommend including the language you'd like to be greeted on, knowledge cutoff, and tool usage eagerness.\nMoly automatically appends useful context to your prompt, like the time of day."
                        draw_text +: {
                            text_style: REGULAR_FONT { font_size: 11 }
                        }
                    }
                }
            }

            save_provider := MolyButton {
                margin: Inset { top: 10 }
                width: Fit
                height: 30
                padding: Inset {
                    left: 20 right: 20 top: 0 bottom: 0
                }
                text: "Save"
                draw_bg +: {
                    color: (CTA_BUTTON_COLOR)
                    border_size: 0
                }
            }

            provider_features_group := View {
                width: Fill
                height: Fit
                flow: Down

                tools_form_group := FormGroup {
                    margin: Inset { top: 10 }
                    visible: false
                    height: Fit

                    View {
                        margin: Inset { top: 10 }
                        width: Fill
                        height: 1
                        show_bg: true
                        draw_bg +: {
                            color: #D9D9D9
                        }
                    }

                    Label {
                        margin: Inset { top: 10 }
                        text: "MCP Configuration"
                        draw_text +: {
                            text_style: BOLD_FONT { font_size: 12 }
                            color: #000
                        }
                    }

                    View {
                        margin: Inset { top: 10 }
                        flow: Right
                        width: Fit
                        height: Fit
                        align: Align { x: 0.5 y: 0.5 }
                        Label {
                            text: "Enable Tools"
                            draw_text +: {
                                text_style +: { font_size: 12 }
                                color: #000
                            }
                        }

                        provider_tools_switch := MolySwitch {
                            margin: Inset { left: 10 }
                            animator: Animator {
                                selected: {
                                    default: @on
                                }
                            }
                        }
                    }

                    View {
                        margin: Inset { top: 10 }
                        width: Fill
                        height: 1
                        show_bg: true
                        draw_bg +: {
                            color: #D9D9D9
                        }
                    }
                }

                models_label := Label {
                    margin: Inset { top: 10 }
                    text: "Models"
                    draw_text +: {
                        text_style: BOLD_FONT { font_size: 12 }
                        color: #000
                    }
                }

                View {
                    margin: Inset { top: 10 }
                    width: Fill
                    height: Fit
                    model_search_input := MolyTextInput {
                        width: Fill
                        height: 30
                        empty_text: "Search models..."
                        draw_text +: {
                            text_style: REGULAR_FONT { font_size: 12 }
                            color: #000
                        }
                    }
                }

                models_list := FlatList {
                    margin: Inset { top: 10 }
                    width: Fill
                    height: Fit
                    flow: Down
                    grab_key_focus: true
                    drag_scrolling: true

                    model_entry := ModelEntry {}
                    header_entry := HeaderEntry {}
                }

                show_others_button := MolyButton {
                    margin: Inset { top: 10 }
                    visible: false
                    padding: Inset {
                        top: 6 bottom: 6 left: 12 right: 12
                    }
                    text: "Show potentially unsupported models"
                    draw_bg +: {
                        color: (TRANSPARENT)
                        border_color_1: #xe17100
                        border_size: 1.0
                    }
                    draw_text +: {
                        text_style: REGULAR_FONT { font_size: 11 }
                        color: #xe17100
                    }
                }
            }

            remove_provider_view := View {
                margin: Inset { top: 10 }
                width: Fill
                height: Fit
                align: Align { x: 1.0 y: 0.5 }
                remove_provider_button := MolyButton {
                    padding: Inset {
                        left: 20 right: 20 top: 10 bottom: 10
                    }
                    width: Fit
                    height: Fit
                    text: "Remove Provider"
                    draw_text +: {
                        text_style: BOLD_FONT { font_size: 10 }
                    }
                    draw_bg +: {
                        color: #B4605A
                        border_size: 0
                    }
                }
            }

            View { height: 10 }
        }
    }
}

#[derive(Widget, ScriptHook, Script)]
struct ProviderView {
    #[deref]
    view: View,

    #[rust]
    provider: Provider,

    #[rust]
    initialized: bool,

    #[rust]
    showing_others: bool,
}

impl Widget for ProviderView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get_mut::<Store>().unwrap();
        let mut models = store.chats.get_provider_models(&self.provider.id);

        let has_models = !models.is_empty();

        let provider_has_recommended = models.iter().any(|m| m.is_recommended);

        let search_term = self
            .text_input(cx, ids!(model_search_input))
            .text()
            .to_lowercase();
        if !search_term.is_empty() {
            models.retain(|m| m.name.to_lowercase().contains(&search_term));
        }

        models.sort_by(|a, b| {
            if a.is_recommended != b.is_recommended {
                return b.is_recommended.cmp(&a.is_recommended);
            }
            a.name.cmp(&b.name)
        });

        let (recommended, mut others): (Vec<_>, Vec<_>) =
            models.into_iter().partition(|m| m.is_recommended);

        let mut show_others_button = false;

        if provider_has_recommended && !self.showing_others {
            if !others.is_empty() {
                show_others_button = true;
                others.clear();
            }
        }

        enum DisplayItem {
            Header(String),
            Bot(ProviderBot),
        }

        let mut display_items = Vec::new();

        let show_headers = !recommended.is_empty() && !others.is_empty();

        if !recommended.is_empty() {
            if show_headers {
                display_items.push(DisplayItem::Header("Recommended".to_string()));
            }
            for model in recommended {
                display_items.push(DisplayItem::Bot(model));
            }
        }

        if !others.is_empty() {
            if show_headers {
                display_items.push(DisplayItem::Header("Unknown".to_string()));
            }
            for model in others {
                display_items.push(DisplayItem::Bot(model));
            }
        }

        let provider = store.chats.providers.get(&self.provider.id).cloned();

        if let Some(provider) = provider {
            if !self.initialized {
                self.provider = provider;
                self.initialized = true;
            } else {
                self.provider.connection_status = provider.connection_status;
            }
        }

        self.update_connection_status(cx);

        if self.provider.enabled {
            self.view(cx, ids!(refresh_button)).set_visible(cx, true);
        } else {
            self.view(cx, ids!(refresh_button)).set_visible(cx, false);
        }

        let show_models = has_models && self.provider.provider_type != ProviderType::OpenClaw;
        self.view(cx, ids!(provider_features_group))
            .set_visible(cx, show_models);

        self.button(cx, ids!(show_others_button))
            .set_visible(cx, show_others_button);

        let content_padding: f64 = if cx.display_context.is_desktop() {
            25.0
        } else {
            5.0
        };

        let mut content_view = self.view(cx, ids!(content));
        script_apply_eval!(cx, content_view, { padding: #(content_padding) });

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_flat_list().borrow_mut() {
                let mut previous_was_header = false;
                for (idx, display_item) in display_items.iter().enumerate() {
                    match display_item {
                        DisplayItem::Header(text) => {
                            let item_id = LiveId::from_str(&text);
                            if let Some(item) = list.item(cx, item_id, live_id!(header_entry)) {
                                item.label(cx, ids!(label)).set_text(cx, text);
                                item.draw_all(cx, scope);
                            }
                            previous_was_header = true;
                        }
                        DisplayItem::Bot(bot) => {
                            let item_id = LiveId::from_str(&bot.name);
                            if let Some(item) = list.item(cx, item_id, live_id!(model_entry)) {
                                let show_separator = idx > 0 && !previous_was_header;
                                item.view(cx, ids!(separator))
                                    .set_visible(cx, show_separator);

                                item.label(cx, ids!(model_name))
                                    .set_text(cx, &bot.human_readable_name());
                                item.check_box(cx, ids!(enabled_switch))
                                    .set_active(cx, bot.enabled && self.provider.enabled);

                                item.as_model_entry().set_model_name(&bot.name);
                                item.as_model_entry().set_model_id(&bot.id.to_string());
                                item.draw_all(cx, scope);
                            }
                            previous_was_header = false;
                        }
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl ProviderView {
    fn update_connection_status(&mut self, cx: &mut Cx) {
        let mut connection_status_label = self.label(cx, ids!(connection_status));
        connection_status_label.set_text(cx, &self.provider.connection_status.to_human_readable());
        let text_color = match &self.provider.connection_status {
            ProviderConnectionStatus::Connected => vec4(0.0, 0.576, 0.314, 1.0),
            ProviderConnectionStatus::Disconnected => vec4(0.0, 0.0, 0.0, 1.0),
            ProviderConnectionStatus::Connecting => vec4(0.5, 0.5, 0.5, 1.0),
            ProviderConnectionStatus::Error(_error) => vec4(1.0, 0.0, 0.0, 1.0),
        };
        script_apply_eval!(cx, connection_status_label, {
            draw_text +: {
                color: #(text_color)
            }
        });
    }
}

impl WidgetMatchEvent for ProviderView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let store = scope.data.get_mut::<Store>().unwrap();

        if self.button(cx, ids!(show_others_button)).clicked(actions) {
            self.showing_others = true;
            self.redraw(cx);
        }

        let provider_enabled_switch = self.check_box(cx, ids!(provider_enabled_switch));
        if let Some(enabled) = provider_enabled_switch.changed(actions) {
            self.provider.enabled = enabled;
            store.insert_or_update_provider(&self.provider);
            self.redraw(cx);
        }

        let provider_tools_switch = self.check_box(cx, ids!(provider_tools_switch));
        if let Some(tools_enabled) = provider_tools_switch.changed(actions) {
            self.provider.tools_enabled = tools_enabled;
            store.insert_or_update_provider(&self.provider);
            self.redraw(cx);
        }

        for action in actions {
            if let Some(action) = action.downcast_ref::<ModelEntryAction>() {
                match action {
                    ModelEntryAction::ModelEnabledChanged(model_name, model_id, enabled) => {
                        store.preferences.update_model_status(
                            &self.provider.id,
                            model_name,
                            *enabled,
                        );

                        if let Some(model) =
                            store.chats.available_bots.get_mut(&BotId::new(model_id))
                        {
                            model.enabled = *enabled;
                        } else {
                            ::log::warn!(
                                "Toggling model status: Bot with id {} and name {} not found in available_bots",
                                model_id,
                                model_name
                            );
                        }
                        store.reload_bot_context();
                        self.redraw(cx);
                    }
                    _ => {}
                }
            }
        }

        if self.button(cx, ids!(save_provider)).clicked(actions) {
            self.provider.url = self
                .view
                .text_input(cx, ids!(api_host))
                .text()
                .trim()
                .to_string();
            let api_key = self
                .view
                .text_input(cx, ids!(api_key))
                .text()
                .trim()
                .to_string();
            if api_key.is_empty() {
                self.provider.api_key = None;
            } else {
                self.provider.api_key = Some(api_key);
            }

            if self.provider.provider_type == ProviderType::OpenAiRealtime {
                let system_prompt = self
                    .view
                    .text_input(cx, ids!(system_prompt))
                    .text()
                    .trim()
                    .to_string();
                if system_prompt.is_empty() {
                    self.provider.system_prompt = None;
                } else {
                    self.provider.system_prompt = Some(system_prompt);
                }
            }

            self.provider.enabled = true;
            self.provider.connection_status = ProviderConnectionStatus::Connecting;
            self.check_box(cx, ids!(provider_enabled_switch))
                .set_active(cx, true);

            store.insert_or_update_provider(&self.provider);

            self.update_connection_status(cx);
            self.redraw(cx);
        }

        if let Some(_fe) = self.view(cx, ids!(refresh_button)).finger_up(actions) {
            self.provider.connection_status = ProviderConnectionStatus::Connecting;

            store.insert_or_update_provider(&self.provider);

            self.update_connection_status(cx);
            self.redraw(cx);
        }

        if self
            .button(cx, ids!(remove_provider_button))
            .clicked(actions)
        {
            store.remove_provider(&self.provider.id);
            cx.action(ProviderViewAction::ProviderRemoved);
            self.redraw(cx);
        }

        if self
            .button(cx, ids!(toggle_key_visibility))
            .clicked(actions)
        {
            let api_key_input = self.text_input(cx, ids!(api_key));
            api_key_input.set_is_password(cx, !api_key_input.is_password());
            if api_key_input.is_password() {
                self.button(cx, ids!(toggle_key_visibility))
                    .set_text(cx, "\u{f070}");
            } else {
                self.button(cx, ids!(toggle_key_visibility))
                    .set_text(cx, "\u{f06e}");
            }
            self.redraw(cx);
        }
    }
}

impl ProviderViewRef {
    pub fn set_provider(&mut self, cx: &mut Cx, provider: &Provider) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.provider = provider.clone();
            inner
                .text_input(cx, ids!(model_search_input))
                .set_text(cx, "");
            inner.showing_others = false;

            let api_key_input = inner.text_input(cx, ids!(api_key));
            if let Some(api_key) = &provider.api_key {
                api_key_input.set_text(cx, &api_key);
            } else {
                api_key_input.set_text(cx, "");
            }

            inner
                .text_input(cx, ids!(api_host))
                .set_text(cx, &provider.url);
            inner.label(cx, ids!(name)).set_text(cx, &provider.name);
            inner
                .label(cx, ids!(provider_type))
                .set_text(cx, &provider.provider_type.to_human_readable());
            inner
                .check_box(cx, ids!(provider_enabled_switch))
                .set_active(cx, provider.enabled);
            inner
                .check_box(cx, ids!(provider_tools_switch))
                .set_active(cx, provider.tools_enabled);

            if provider.provider_type == ProviderType::OpenAiRealtime {
                inner
                    .view(cx, ids!(system_prompt_group))
                    .set_visible(cx, true);
                if let Some(system_prompt) = &provider.system_prompt {
                    inner
                        .text_input(cx, ids!(system_prompt))
                        .set_text(cx, &system_prompt);
                } else {
                    inner.text_input(cx, ids!(system_prompt)).set_text(cx, "");
                }
            } else {
                inner
                    .view(cx, ids!(system_prompt_group))
                    .set_visible(cx, false);
            }

            if provider.provider_type == ProviderType::OpenAiRealtime
                || provider.provider_type == ProviderType::OpenAi
            {
                inner.view(cx, ids!(tools_form_group)).set_visible(cx, true);
            } else {
                inner
                    .view(cx, ids!(tools_form_group))
                    .set_visible(cx, false);
            }

            if provider.was_customly_added {
                inner
                    .view(cx, ids!(remove_provider_view))
                    .set_visible(cx, true);
            } else {
                inner
                    .view(cx, ids!(remove_provider_view))
                    .set_visible(cx, false);
            }

            inner.view.redraw(cx);
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ProviderViewAction {
    #[default]
    None,
    ProviderRemoved,
}

#[derive(Script, ScriptHook, Widget)]
struct ModelEntry {
    #[deref]
    view: View,

    #[rust]
    model_name: String,

    #[rust]
    model_id: String,
}

impl Widget for ModelEntry {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let is_desktop = cx.display_context.is_desktop();
        if is_desktop {
            script_apply_eval!(cx, self.view, { height: 60 });
        } else {
            script_apply_eval!(cx, self.view, { height: 80 });
        }

        let mut model_name = self.view.label(cx, ids!(model_name));
        if is_desktop {
            let fit = Size::fit();
            script_apply_eval!(cx, model_name, { width: #(fit) });
        } else {
            script_apply_eval!(cx, model_name, { width: 200 });
        }

        self.view
            .view(cx, ids!(vertical_filler))
            .set_visible(cx, is_desktop);

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ModelEntry {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let enabled_switch = self.check_box(cx, ids!(enabled_switch));
        if let Some(change) = enabled_switch.changed(actions) {
            cx.action(ModelEntryAction::ModelEnabledChanged(
                self.model_name.clone(),
                self.model_id.clone(),
                change,
            ));
            self.redraw(cx);
        }
    }
}

impl ModelEntryRef {
    pub fn set_model_name(&mut self, name: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.model_name = name.to_string();
        }
    }

    pub fn set_model_id(&mut self, id: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.model_id = id.to_string();
        }
    }
}

#[derive(Clone, Debug, Default)]
enum ModelEntryAction {
    #[default]
    None,
    ModelEnabledChanged(String, String, bool),
}
