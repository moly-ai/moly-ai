use makepad_widgets::Cx;
use moly_kit::aitk::utils::asynchronous::spawn;
use moly_kit::prelude::*;

use crate::data::providers::ProviderId;

use super::providers::{Provider, ProviderBot, ProviderFetchModelsResult, ProviderType};

/// Fetches models for a provider using the appropriate MolyKit client
pub fn fetch_models_for_provider(provider: &Provider) {
    let provider_id = provider.id.clone();
    let url = provider.url.clone();
    let api_key = provider.api_key.clone();

    match provider.provider_type {
        ProviderType::OpenAi | ProviderType::MolyServer | ProviderType::MoFa => {
            fetch_models_with_client(
                provider_id.clone(),
                move || {
                    let mut client = OpenAiClient::new(url);
                    if let Some(key) = api_key {
                        let _ = client.set_key(&key);
                    }
                    Box::new(client)
                },
                move |bot| ProviderBot {
                    id: bot.id.clone(),
                    name: bot.name.clone(),
                    description: format!("Model from {}", provider_id),
                    provider_id: provider_id.clone(),
                    enabled: true,
                    is_recommended: false,
                },
                Some(should_include_model),
            );
        }
        ProviderType::OpenAiImage => {
            fetch_models_with_client(
                provider_id.clone(),
                move || {
                    let client_url = url.trim_start_matches('#').to_string();
                    let mut client = OpenAiImageClient::new(client_url);
                    if let Some(key) = api_key {
                        let _ = client.set_key(&key);
                    }
                    Box::new(client)
                },
                move |bot| ProviderBot {
                    id: bot.id.clone(),
                    name: bot.name.clone(),
                    description: "OpenAI Image Generation Model".to_string(),
                    provider_id: provider_id.clone(),
                    enabled: true,
                    is_recommended: false,
                },
                None,
            );
        }
        ProviderType::OpenAiRealtime => {
            fetch_models_with_client(
                provider_id.clone(),
                move || {
                    let client_url = url.trim_start_matches('#').to_string();
                    let mut client = OpenAiRealtimeClient::new(client_url);
                    if let Some(key) = api_key {
                        let _ = client.set_key(&key);
                    }
                    Box::new(client)
                },
                move |bot| ProviderBot {
                    id: bot.id.clone(),
                    name: bot.name.clone(),
                    description: "OpenAI Realtime Model".to_string(),
                    provider_id: provider_id.clone(),
                    enabled: true,
                    is_recommended: false,
                },
                None,
            );
        }
        ProviderType::DeepInquire => {
            fetch_models_with_client(
                provider_id.clone(),
                move || {
                    let mut client = crate::data::deep_inquire_client::DeepInquireClient::new(url);
                    if let Some(key) = api_key {
                        let _ = client.set_key(&key);
                    }
                    Box::new(client)
                },
                move |bot| ProviderBot {
                    id: bot.id.clone(),
                    name: bot.name.clone(),
                    description: "A search assistant".to_string(),
                    provider_id: provider_id.clone(),
                    enabled: true,
                    is_recommended: false,
                },
                None,
            );
        }
    }
}

/// Generic function to fetch models using any BotClient implementation
fn fetch_models_with_client<F, M>(
    provider_id: ProviderId,
    client_factory: F,
    map_bot: M,
    filter: Option<fn(&str) -> bool>,
) where
    F: FnOnce() -> Box<dyn BotClient> + Send + 'static,
    M: Fn(Bot) -> ProviderBot + Send + 'static,
{
    spawn(async move {
        let mut client = client_factory();

        match client.bots().await.into_result() {
            Ok(bots) => {
                let models: Vec<ProviderBot> = bots
                    .into_iter()
                    .filter(|bot| filter.map_or(true, |f| f(&bot.name)))
                    .map(|bot| Bot {
                        // The client Moly interacts with in the `Store` is a `RouterClient`.
                        // This module is creating specific clients to obtain the bots that will
                        // end up becoming `ProviderBot`s as expected by Moly.
                        // So for now, let's ensure here that ids match.
                        id: RouterClient::prefix(&provider_id, &bot.id),
                        ..bot
                    })
                    .map(map_bot)
                    .collect();

                Cx::post_action(ProviderFetchModelsResult::Success(provider_id, models));
            }
            Err(errors) => {
                let error = if errors.is_empty() {
                    ClientError::new(
                        ClientErrorKind::Unknown,
                        "An error occurred, but no details were provided".to_string(),
                    )
                } else {
                    errors[0].clone()
                };
                Cx::post_action(ProviderFetchModelsResult::Failure(provider_id, error));
            }
        }
    });
}

/// Filter out non-chat models for OpenAI-compatible providers
pub fn should_include_model(model_id: &str) -> bool {
    // Filter out non-chat models
    if model_id.contains("dall-e")
        || model_id.contains("whisper")
        || model_id.contains("tts")
        || model_id.contains("davinci")
        || model_id.contains("audio")
        || model_id.contains("babbage")
        || model_id.contains("moderation")
        || model_id.contains("embedding")
    {
        return false;
    }
    true
}
