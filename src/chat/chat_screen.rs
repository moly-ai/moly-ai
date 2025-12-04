use makepad_widgets::*;
use moly_kit::ai_kit::utils::asynchronous::spawn;
use moly_kit::prelude::*;

use std::collections::HashMap;

use crate::data::bot_fetcher::should_include_model;
use crate::data::deep_inquire_client::DeepInquireClient;
use crate::data::providers::{Provider, ProviderBot, ProviderId, ProviderType};
use crate::data::store::Store;
use crate::data::supported_providers::{self, SupportedProvider};
use crate::settings::provider_view::ProviderViewWidgetExt;
use crate::settings::providers::ConnectionSettingsAction;
use crate::shared::actions::ChatAction;
use crate::shared::bot_context::BotContext;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::chat::chat_history_panel::ChatHistoryPanel;
    use crate::chat::chat_screen_mobile::ChatScreenMobile;
    use crate::chat::chats_deck::ChatsDeck;

    pub ChatScreen = {{ChatScreen}} {
        width: Fill, height: Fill
        spacing: 10

        adaptive_view = <AdaptiveView> {
            Mobile = {
                <ChatScreenMobile> {}
            }

            Desktop = {
                <View> {
                    width: Fit, height: Fill
                    chat_history_panel = <ChatHistoryPanel> {}
                }

                <CachedWidget> {
                    chats_deck = <ChatsDeck> {}
                }
            }
        }

        // TODO: Add chat params back in, only when the model is a local model (MolyServer)
        // currenlty MolyKit does not support chat params
        //
        // <View> {
        //     width: Fit,
        //     height: Fill,
        //
        //     chat_params = <ChatParams> {}
        // }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ChatScreen {
    #[deref]
    view: View,

    #[rust(true)]
    first_render: bool,

    #[rust]
    creating_bot_context: bool,
}

impl Widget for ChatScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.ui_runner().handle(cx, event, scope, self);

        // TODO This check is actually copied from Makepad view.rs file
        // It's not clear why it's needed here, but without this line
        // the "View all files" link in Discover section does not work after visiting the chat screen
        if self.visible || !event.requires_visibility() {
            self.view.handle_event(cx, event, scope);
        }

        let store = scope.data.get_mut::<Store>().unwrap();

        let should_recreate_bot_context = store.bot_context.is_none();

        if (self.first_render || should_recreate_bot_context) && !self.creating_bot_context {
            self.create_bot_context(cx, scope);
            self.first_render = false;
        }

        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ChatScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if self.button(ids!(new_chat_button)).clicked(&actions) {
            cx.action(ChatAction::StartWithoutEntity);
            self.stack_navigation(ids!(navigation)).pop_to_root(cx);
            self.redraw(cx);
        }

        for action in actions {
            if let ChatAction::ChatSelected(_chat_id) = action.cast() {
                self.stack_navigation(ids!(navigation)).pop_to_root(cx);
                self.redraw(cx);
            }

            if let ConnectionSettingsAction::ProviderSelected(provider_id) = action.cast() {
                self.stack_navigation(ids!(navigation))
                    .push(cx, live_id!(provider_navigation_view));

                let provider = scope
                    .data
                    .get_mut::<Store>()
                    .unwrap()
                    .chats
                    .providers
                    .get(&provider_id);
                if let Some(provider) = provider {
                    self.view
                        .provider_view(ids!(provider_view))
                        .set_provider(cx, provider);
                } else {
                    eprintln!("Provider not found: {}", provider_id);
                }

                self.redraw(cx);
            }
        }
    }
}

impl ChatScreen {
    fn create_bot_context(&mut self, _cx: &mut Cx, scope: &mut Scope) {
        let store = scope.data.get_mut::<Store>().unwrap();

        let multi_client = {
            let mut multi_client = MultiClient::new();
            let supported_providers_list = supported_providers::load_supported_providers();

            let available_bots = store.chats.available_bots.clone();
            let providers = store.chats.providers.clone();

            // Filter enabled providers upfront and check credentials
            for (_key, provider) in store
                .chats
                .providers
                .iter()
                .filter(|(_, p)| p.enabled && has_valid_credentials(p))
            {
                let client: Option<Box<dyn BotClient>> = match provider.provider_type {
                    ProviderType::OpenAi | ProviderType::MolyServer | ProviderType::MoFa => {
                        create_openai_client(
                            provider,
                            &supported_providers_list,
                            &available_bots,
                            &providers,
                            &store,
                            ClientFilter::ChatModels,
                        )
                    }
                    ProviderType::OpenAiImage => create_openai_image_client(
                        provider,
                        &supported_providers_list,
                        &available_bots,
                        &providers,
                        &store,
                    ),
                    ProviderType::OpenAiRealtime => create_openai_realtime_client(provider),
                    ProviderType::DeepInquire => create_deep_inquire_client(
                        provider,
                        &supported_providers_list,
                        &available_bots,
                        &providers,
                        &store,
                    ),
                };

                if let Some(client) = client {
                    multi_client.add_client(client);
                }
            }

            multi_client
        };

        let mut context: BotContext = multi_client.into();
        let tool_manager = store.create_and_load_mcp_tool_manager();
        tool_manager
            .set_dangerous_mode_enabled(store.preferences.get_mcp_servers_dangerous_mode_enabled());
        context.set_tool_manager(tool_manager);

        store.bot_context = Some(context.clone());

        self.creating_bot_context = true;

        let ui = self.ui_runner();
        spawn(async move {
            let _ = context.load().await;
            ui.defer_with_redraw(move |me, _cx, _scope| {
                me.creating_bot_context = false;
            });
        });
    }
}

type ProviderMap = HashMap<ProviderId, Provider>;
type BotMap = HashMap<BotId, ProviderBot>;

// Helper types and functions for client creation

#[derive(Clone)]
enum ClientFilter {
    // Apply should_include_model filter
    ChatModels,
    // Check bot.enabled in available_bots
    BotEnabled,
    // No extra filtering beyond provider enabled
    None,
}

fn is_localhost(url: &str) -> bool {
    url.contains("localhost") || url.contains("127.0.0.1")
}

fn has_valid_credentials(provider: &Provider) -> bool {
    match provider.provider_type {
        ProviderType::OpenAi | ProviderType::MolyServer | ProviderType::OpenAiRealtime => {
            provider.api_key.is_some() || is_localhost(&provider.url)
        }
        ProviderType::MoFa | ProviderType::OpenAiImage | ProviderType::DeepInquire => true,
    }
}

fn apply_icon(bots: &mut Vec<Bot>, icon_opt: &Option<LiveDependency>) {
    if let Some(icon) = icon_opt {
        for bot in bots.iter_mut() {
            bot.avatar = EntityAvatar::Image(icon.as_str().to_string());
        }
    }
}

fn apply_bot_filters(
    bots: &mut Vec<Bot>,
    available_bots: &BotMap,
    providers: &ProviderMap,
    filter: &ClientFilter,
    supported_models: &Option<Vec<String>>,
) {
    // Filter by provider/bot enabled status
    if !available_bots.is_empty() {
        bots.retain(|bot| {
            if let Some(provider_bot) = available_bots.get(&bot.id) {
                let provider_enabled = providers
                    .get(&provider_bot.provider_id)
                    .map_or(false, |p| p.enabled);

                match filter {
                    ClientFilter::BotEnabled => provider_bot.enabled && provider_enabled,
                    _ => provider_enabled,
                }
            } else {
                // Bot not in available_bots yet, let it through
                true
            }
        });
    }

    // Apply filter type
    if matches!(filter, ClientFilter::ChatModels) {
        bots.retain(|bot| should_include_model(&bot.name));
    }

    // Apply supported models whitelist
    if let Some(models) = supported_models {
        bots.retain(|bot| models.contains(&bot.name));
    }
}

fn setup_map_client<C: BotClient + 'static>(
    map_client: &mut MapClient<C>,
    provider: &Provider,
    supported_providers_list: &[SupportedProvider],
    available_bots: &BotMap,
    providers: &ProviderMap,
    store: &Store,
    filter: ClientFilter,
) {
    let supported_models = supported_providers_list
        .iter()
        .find(|sp| sp.id == provider.id)
        .and_then(|sp| sp.supported_models.clone());

    let icon_opt = store.get_provider_icon(&provider.name);
    let available_bots = available_bots.clone();
    let providers = providers.clone();

    map_client.set_map_bots(move |mut bots| {
        apply_bot_filters(
            &mut bots,
            &available_bots,
            &providers,
            &filter,
            &supported_models,
        );
        apply_icon(&mut bots, &icon_opt);
        bots
    });
}

fn create_openai_client(
    provider: &Provider,
    supported_providers_list: &[SupportedProvider],
    available_bots: &BotMap,
    providers: &ProviderMap,
    store: &Store,
    filter: ClientFilter,
) -> Option<Box<dyn BotClient>> {
    let mut client = OpenAiClient::new(provider.url.clone());

    if let Some(key) = provider.api_key.as_ref() {
        if let Err(e) = client.set_key(key) {
            eprintln!("Failed to set API key for {}: {}", provider.name, e);
            return None;
        }
    }
    client.set_tools_enabled(provider.tools_enabled);

    let mut map_client = MapClient::from(client);

    setup_map_client(
        &mut map_client,
        provider,
        supported_providers_list,
        available_bots,
        providers,
        store,
        filter,
    );

    Some(Box::new(map_client))
}

fn create_openai_image_client(
    provider: &Provider,
    supported_providers_list: &[SupportedProvider],
    available_bots: &BotMap,
    providers: &ProviderMap,
    store: &Store,
) -> Option<Box<dyn BotClient>> {
    let client_url = provider.url.trim_start_matches('#').to_string();
    let mut client = OpenAiImageClient::new(client_url);

    if let Some(key) = provider.api_key.as_ref() {
        if let Err(e) = client.set_key(key) {
            eprintln!("Failed to set API key for {}: {}", provider.name, e);
            return None;
        }
    }

    let mut map_client = MapClient::from(client);

    setup_map_client(
        &mut map_client,
        provider,
        supported_providers_list,
        available_bots,
        providers,
        store,
        ClientFilter::BotEnabled,
    );

    Some(Box::new(map_client))
}

fn create_openai_realtime_client(provider: &Provider) -> Option<Box<dyn BotClient>> {
    let client_url = provider.url.trim_start_matches('#').to_string();
    let mut client = OpenAiRealtimeClient::new(client_url);

    if let Some(key) = provider.api_key.as_ref() {
        if let Err(e) = client.set_key(key) {
            eprintln!("Failed to set API key for {}: {}", provider.name, e);
            return None;
        }
    }
    if let Some(prompt) = provider.system_prompt.as_ref() {
        if let Err(e) = client.set_system_prompt(prompt) {
            eprintln!("Failed to set system prompt for {}: {}", provider.name, e);
            return None;
        }
    }
    client.set_tools_enabled(provider.tools_enabled);

    Some(Box::new(client))
}

fn create_deep_inquire_client(
    provider: &Provider,
    supported_providers_list: &[SupportedProvider],
    available_bots: &BotMap,
    providers: &ProviderMap,
    store: &Store,
) -> Option<Box<dyn BotClient>> {
    let mut client = DeepInquireClient::new(provider.url.clone());

    if let Some(key) = provider.api_key.as_ref() {
        if let Err(e) = client.set_key(key) {
            eprintln!("Failed to set API key for {}: {}", provider.name, e);
            return None;
        }
    }

    let mut map_client = MapClient::from(client);

    setup_map_client(
        &mut map_client,
        provider,
        supported_providers_list,
        available_bots,
        providers,
        store,
        ClientFilter::None,
    );

    Some(Box::new(map_client))
}
