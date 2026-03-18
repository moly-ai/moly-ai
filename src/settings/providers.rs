use crate::{
    data::{
        providers::{Provider, ProviderConnectionStatus},
        store::{Store, normalize_provider_name},
    },
    settings::sync_modal::{SyncModalAction, SyncModalWidgetExt},
};
use makepad_widgets::*;
use moly_kit::prelude::*;

use super::{
    add_provider_modal::AddProviderModalAction,
    provider_view::ProviderViewAction,
    utilities_modal::UtilitiesModalAction,
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_SUCCESS =
        crate_resource("self://resources/images/circle_check_icon.png")
    let ICON_LOADER =
        crate_resource("self://resources/images/loader_icon.png")
    let ICON_FAILURE =
        crate_resource("self://resources/images/refresh_error_icon.png")

    let ICON_OPENAI =
        crate_resource("self://resources/images/providers/openai.png")
    let ICON_GEMINI =
        crate_resource("self://resources/images/providers/gemini.png")
    let ICON_SILICONFLOW =
        crate_resource("self://resources/images/providers/siliconflow.png")
    let ICON_OPENROUTER =
        crate_resource("self://resources/images/providers/openrouter.png")
    let ICON_MOLYSERVER =
        crate_resource("self://resources/images/providers/molyserver.png")
    let ICON_DEEPSEEK =
        crate_resource("self://resources/images/providers/deepseek.png")
    let ICON_OLLAMA =
        crate_resource("self://resources/images/providers/ollama.png")
    let ICON_ANTHROPIC =
        crate_resource("self://resources/images/providers/anthropic.png")
    let ICON_OPENCLAW =
        crate_resource("self://resources/images/providers/openclaw.png")

    let ConnectionActionButton = View {
        visible: false
        cursor: MouseCursor.Hand
        width: Fit
        height: Fit

        icon := Image {
            width: 22
            height: 22
            draw_bg +: {
                tint_color: instance(#B42318)

                get_color_scale_pan: fn(scale: vec2 pan: vec2) -> vec4 {
                    let tex_color = sample2d(self.image self.pos * scale + pan).xyzw
                    return vec4(
                        self.tint_color.rgb * tex_color.a
                        tex_color.a
                    )
                }
            }
        }
    }

    mod.widgets.ProviderItemBase = #(ProviderItem::register_widget(vm))
    let ProviderItem =
        mod.widgets.ProviderItemBase {
        width: Fill
        height: 40
        flow: Overlay
        show_bg: true
        draw_bg +: {
            border_radius: 5
        }
        padding: Inset { left: 20 }
        align: Align { x: 0.0 y: 0.5 }

        main_view := View {
            cursor: MouseCursor.Hand
            padding: 8
            align: Align { x: 0.0 y: 0.5 }
            spacing: 20
            flow: Right

            provider_icon := View {
                width: Fit
                height: Fit
                image_wrapper := View {
                    width: Fit
                    height: Fit
                    provider_icon_image := Image {
                        width: 25
                        height: 25
                    }
                    visible: true
                }

                label_wrapper := RoundedView {
                    width: 25
                    height: 25
                    visible: false
                    show_bg: true
                    draw_bg +: {
                        color: #344054
                        border_radius: 6
                    }
                    align: Align { x: 0.5 y: 0.5 }

                    initial_label := Label {
                        draw_text +: {
                            text_style: BOLD_FONT { font_size: 12 }
                            color: #f
                        }
                    }
                }
            }

            View {
                flow: Right
                width: Fill
                height: Fill
                spacing: 20
                align: Align { x: 0.0 y: 0.5 }

                provider_name_label := Label {
                    flow: Right
                    width: Fill
                    draw_text +: {
                        text_style: REGULAR_FONT { font_size: 11 }
                        color: #000
                    }
                }

                status_view := RoundedView {
                    align: Align { x: 0.5 y: 0.5 }
                    show_bg: true
                    width: Fit
                    height: Fit
                    padding: Inset {
                        left: 8 right: 8 bottom: 5 top: 5
                    }
                    margin: Inset { right: 10 }
                    draw_bg +: {
                        border_radius: 5
                        color: #9FD5C7
                        border_color: #357852
                        border_size: 1.2
                    }
                    status_label := Label {
                        text: "ON"
                        draw_text +: {
                            text_style: BOLD_FONT { font_size: 7 }
                            color: #043b1c
                        }
                    }
                }
            }
        }
    }

    mod.widgets.ProvidersBase = #(Providers::register_widget(vm))
    mod.widgets.Providers =
        set_type_default() do mod.widgets.ProvidersBase {
        width: 300
        height: Fill
        flow: Down
        spacing: 10
        padding: Inset { left: 10 right: 10 }
        providers_list := PortalList {
            width: Fill
            height: Fill
            provider_item := ProviderItem {}
        }

        add_provider_button := RoundedShadowView {
            cursor: MouseCursor.Hand
            margin: Inset {
                left: 10 right: 10 bottom: 0 top: 10
            }
            width: Fill
            height: Fit
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
                text: "+ Add a Custom Provider"
                draw_text +: {
                    text_style: REGULAR_FONT { font_size: 11 }
                    color: #000
                }
            }
        }

        open_sync_button := RoundedShadowView {
            cursor: MouseCursor.Hand
            margin: Inset { left: 10 right: 10 bottom: 0 }
            width: Fill
            height: Fit
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
                text: "Sync Settings"
                draw_text +: {
                    text_style: REGULAR_FONT { font_size: 11 }
                    color: #000
                }
            }
        }

        utilities_button := RoundedShadowView {
            cursor: MouseCursor.Hand
            margin: Inset { left: 10 right: 10 bottom: 20 }
            width: Fill
            height: Fit
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
                text: "Utilities"
                draw_text +: {
                    text_style: REGULAR_FONT { font_size: 11 }
                    color: #000
                }
            }
        }

        View {
            width: Fill
            height: Fit
            flow: Overlay

            add_provider_modal := MolyModal {
                content +: {
                    add_provider_modal_inner := AddProviderModal {}
                }
            }

            sync_modal := MolyModal {
                content +: {
                    sync_modal_inner := SyncModal {}
                }
            }

            utilities_modal := MolyModal {
                content +: {
                    utilities_modal_inner := UtilitiesModal {}
                }
            }
        }
    }
}

#[derive(Widget, Script)]
struct Providers {
    #[deref]
    view: View,

    #[rust]
    provider_icon_paths: Vec<String>,
    #[rust]
    selected_provider_id: Option<String>,

    #[rust]
    initialized: bool,
}

impl ScriptHook for Providers {
    fn on_after_apply(
        &mut self,
        _vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let names = [
            "openai", "gemini", "siliconflow", "openrouter",
            "molyserver", "deepseek", "ollama", "anthropic", "openclaw",
        ];
        self.provider_icon_paths = names
            .iter()
            .map(|name| {
                format!(
                    "{}/resources/images/providers/{}.png",
                    manifest_dir, name
                )
            })
            .collect();
    }
}

impl Widget for Providers {
    fn handle_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
    ) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);

        if !self.initialized {
            if cx.display_context.is_desktop() {
                self.initialized = true;
                let default_provider_id = "anthropic".to_string();
                self.selected_provider_id =
                    Some(default_provider_id.clone());

                cx.action(
                    ConnectionSettingsAction::ProviderSelected(
                        default_provider_id,
                    ),
                );
            }
        }

        let store = scope.data.get_mut::<Store>().unwrap();
        if store.provider_icons.is_empty() {
            store.provider_icons = self.provider_icon_paths.clone();
        }
    }

    fn draw_walk(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        walk: Walk,
    ) -> DrawStep {
        let store = scope.data.get::<Store>().unwrap();

        let mut all_providers: Vec<Provider> =
            store.chats.providers.values().cloned().collect();
        all_providers.sort_by(|a, b| a.name.cmp(&b.name));

        let entries_count = all_providers.len();

        while let Some(item) =
            self.view.draw_walk(cx, scope, walk).step()
        {
            if let Some(mut list) = item.as_portal_list().borrow_mut()
            {
                list.set_item_range(cx, 0, entries_count);
                while let Some(item_id) =
                    list.next_visible_item(cx)
                {
                    if item_id < entries_count {
                        let template = live_id!(provider_item);
                        let item = list.item(cx, item_id, template);

                        if item_id == 0 {
                            item.view(cx, ids!(separator))
                                .set_visible(cx, false);
                        }

                        let provider =
                            all_providers[item_id].clone();
                        let icon =
                            self.get_provider_icon(cx, &provider);
                        let is_selected =
                            self.selected_provider_id
                                == Some(provider.id.clone());
                        item.as_provider_item().set_provider(
                            cx,
                            provider,
                            icon,
                            is_selected,
                        );
                        item.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl Providers {
    fn get_provider_icon(
        &self,
        _cx: &Cx,
        provider: &Provider,
    ) -> Option<String> {
        let base_name = normalize_provider_name(&provider.name);

        self.provider_icon_paths
            .iter()
            .find(|path| {
                path.to_lowercase()
                    .contains(&base_name.to_lowercase())
            })
            .cloned()
    }
}

impl WidgetMatchEvent for Providers {
    fn handle_actions(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        scope: &mut Scope,
    ) {
        if let Some(fu) =
            self.view(cx, ids!(add_provider_button)).finger_up(actions)
            && fu.was_tap()
        {
            let modal = self.moly_modal(cx, ids!(add_provider_modal));
            modal.open_as_dialog(cx);
        }

        if let Some(fu) =
            self.view(cx, ids!(open_sync_button)).finger_up(actions)
            && fu.was_tap()
        {
            let modal = self.moly_modal(cx, ids!(sync_modal));
            modal.open_as_dialog(cx);
        }

        if let Some(fu) =
            self.view(cx, ids!(utilities_button)).finger_up(actions)
            && fu.was_tap()
        {
            let modal = self.moly_modal(cx, ids!(utilities_modal));
            modal.open_as_dialog(cx);
        }

        for action in actions {
            if let ConnectionSettingsAction::ProviderSelected(
                provider_id,
            ) = action.cast()
            {
                self.selected_provider_id = Some(provider_id);
            }

            if let AddProviderModalAction::ModalDismissed =
                action.cast()
            {
                self.moly_modal(cx, ids!(add_provider_modal)).close(cx);
                self.redraw(cx);
            }

            if let SyncModalAction::ModalDismissed = action.cast() {
                self.moly_modal(cx, ids!(sync_modal)).close(cx);
                self.redraw(cx);
            }

            if let UtilitiesModalAction::ModalDismissed =
                action.cast()
            {
                self.moly_modal(cx, ids!(utilities_modal)).close(cx);
                self.redraw(cx);
            }

            if self
                .moly_modal(cx, ids!(sync_modal))
                .dismissed(actions)
            {
                self.sync_modal(cx, ids!(sync_modal_inner))
                    .reset_state(cx);
            }

            if let ProviderViewAction::ProviderRemoved =
                action.cast()
            {
                let store = scope.data.get::<Store>().unwrap();
                if let Some(first_provider) =
                    store.chats.providers.values().next()
                {
                    self.selected_provider_id =
                        Some(first_provider.id.clone());
                    cx.action(
                        ConnectionSettingsAction::ProviderSelected(
                            first_provider.id.clone(),
                        ),
                    );
                }
                self.redraw(cx);
            }
        }
    }
}

#[derive(Widget, ScriptHook, Script)]
struct ProviderItem {
    #[deref]
    view: View,

    #[rust]
    provider: Provider,
}

impl Widget for ProviderItem {
    fn handle_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
    ) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        walk: Walk,
    ) -> DrawStep {
        self.label(cx, ids!(provider_name_label))
            .set_text(cx, &self.provider.name);

        let connection_status = self.provider.connection_status.clone();
        self.update_connection_status(cx, &connection_status);

        self.view(cx, ids!(status_view)).set_visible(
            cx,
            connection_status
                == ProviderConnectionStatus::Connected
                && self.provider.enabled,
        );

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ProviderItem {
    fn handle_actions(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        _scope: &mut Scope,
    ) {
        if let Some(finger_up) =
            self.view(cx, ids!(main_view)).finger_up(actions)
        {
            if finger_up.was_tap() {
                cx.action(
                    ConnectionSettingsAction::ProviderSelected(
                        self.provider.id.clone(),
                    ),
                );
            }
        }
    }
}

impl ProviderItem {
    fn update_connection_status(
        &mut self,
        cx: &mut Cx,
        connection_status: &ProviderConnectionStatus,
    ) {
        self.view(cx, ids!(connection_status_success)).set_visible(
            cx,
            *connection_status
                == ProviderConnectionStatus::Connected,
        );
        self.view(cx, ids!(connection_status_failure)).set_visible(
            cx,
            *connection_status
                == ProviderConnectionStatus::Disconnected,
        );
        self.view(cx, ids!(connection_status_loading)).set_visible(
            cx,
            *connection_status
                == ProviderConnectionStatus::Connecting,
        );
    }
}

impl ProviderItemRef {
    fn set_provider(
        &mut self,
        cx: &mut Cx,
        provider: Provider,
        icon_path: Option<String>,
        is_selected: bool,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.provider = provider.clone();

        if let Some(icon) = icon_path {
            inner.view(cx, ids!(image_wrapper)).set_visible(cx, true);
            let image = inner.image(cx, ids!(provider_icon_image));
            let _ = image
                .load_image_file_by_path(cx, icon.as_ref());

            let label_view =
                inner.view(cx, ids!(provider_icon_label));
            label_view.set_visible(cx, false);
        } else {
            inner.view(cx, ids!(image_wrapper)).set_visible(cx, false);

            let label_view = inner.view(cx, ids!(label_wrapper));
            label_view.set_visible(cx, true);

            let first_char = provider
                .name
                .chars()
                .next()
                .map(|c| c.to_uppercase().to_string())
                .unwrap_or_default();

            label_view
                .label(cx, ids!(initial_label))
                .set_text(cx, &first_char);
        }

        if is_selected && cx.display_context.is_desktop() {
            let color = vec4(0.918, 0.926, 0.937, 1.0); // #EAECEF
            script_apply_eval!(cx, inner.view, {
                draw_bg +: { color: #(color) }
            });
        } else {
            let color = vec4(0.976, 0.976, 0.976, 1.0); // #f9f9f9
            script_apply_eval!(cx, inner.view, {
                draw_bg +: { color: #(color) }
            });
        }
    }
}

#[derive(Clone, Default, Debug)]
pub enum ConnectionSettingsAction {
    #[default]
    None,
    ProviderSelected(String),
}
