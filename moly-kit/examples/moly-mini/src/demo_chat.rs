use std::sync::{Arc, Mutex};

use makepad_widgets::*;

use moly_kit::aitk::utils::asynchronous::spawn;
use moly_kit::prelude::*;

const OPEN_AI_KEY: Option<&str> = option_env!("OPEN_AI_KEY");
const OPEN_AI_STT_KEY: Option<&str> = option_env!("OPEN_AI_STT_KEY");
const OPEN_AI_IMAGE_KEY: Option<&str> = option_env!("OPEN_AI_IMAGE_KEY");
const OPEN_AI_REALTIME_KEY: Option<&str> = option_env!("OPEN_AI_REALTIME_KEY");
const OPEN_ROUTER_KEY: Option<&str> = option_env!("OPEN_ROUTER_KEY");
const SILICON_FLOW_KEY: Option<&str> = option_env!("SILICON_FLOW_KEY");

live_design!(
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use moly_kit::widgets::chat::Chat;
    use crate::bot_selector::*;

    pub DemoChat = {{DemoChat}} {
        flow: Down,
        padding: 12,
        spacing: 12,

        chat = <Chat> { }
    }
);

#[derive(Live, Widget)]
pub struct DemoChat {
    #[deref]
    deref: View,

    #[rust]
    pub controller: Option<Arc<Mutex<ChatController>>>,
}

impl Widget for DemoChat {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.ui_runner().handle(cx, event, scope, self);
        self.deref.handle_event(cx, event, scope);

        let Event::Actions(_actions) = event else {
            return;
        };
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }
}

impl LiveHook for DemoChat {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        // Setup some hooks as an example of how to use them.
        self.setup_chat_hooks();
        self.setup_chat_controller(cx);
    }
}

impl DemoChat {
    fn fill_selector(&mut self, _cx: &mut Cx, bots: Vec<Bot>) {
        let mut controller = self.controller.as_ref().unwrap().lock().unwrap();

        // Select the first available bot
        if let Some(bot) = bots.first() {
            controller.dispatch_mutation(ChatStateMutation::SetBotId(Some(bot.id.clone())));
        } else {
            eprintln!("No models available, check your API keys.");
        }
    }

    fn setup_chat_hooks(&self) {
        // self.chat(ids!(chat)).write_with(|chat| {
        //     chat.set_hook_before(|group, chat, cx| {
        //         let mut abort = false;

        //         for task in group.iter_mut() {
        //             if let ChatTask::CopyMessage(index) = task {
        //                 abort = true;

        //                 let text = chat.messages_ref().read_with(|messages| {
        //                     let text = &messages.messages[*index].content.text;
        //                     format!("You copied the following text from Moly (mini): {}", text)
        //                 });

        //                 cx.copy_to_clipboard(&text);
        //             }

        //             if let ChatTask::UpdateMessage(_index, message) = task {
        //                 message.content.text =
        //                     message.content.text.replace("ello", "3110 (hooked)");

        //                 if message.content.text.contains("bad word") {
        //                     abort = true;
        //                 }
        //             }
        //         }

        //         if abort {
        //             group.clear();
        //         }
        //     });

        //     chat.set_hook_after(|group, _, _| {
        //         for task in group.iter() {
        //             if let ChatTask::UpdateMessage(_index, message) = task {
        //                 log!("Message updated after hook: {:?}", message.content);
        //             }
        //         }
        //     });
        // });
    }

    fn setup_chat_controller(&mut self, cx: &mut Cx) {
        let client = {
            let mut client = RouterClient::new();

            let tester = TesterClient;
            client.insert_client("tester", Box::new(tester));

            let ollama = OpenAiClient::new("http://localhost:11434/v1".into());
            client.insert_client("ollama", Box::new(ollama));

            if let Some(key) = OPEN_AI_IMAGE_KEY {
                let mut openai_image = OpenAiImageClient::new("https://api.openai.com/v1".into());
                let _ = openai_image.set_key(key);
                client.insert_client("open_ai_image", Box::new(openai_image));
            }

            if let Some(key) = OPEN_AI_REALTIME_KEY {
                let mut openai_realtime =
                    OpenAiRealtimeClient::new("wss://api.openai.com/v1/realtime".into());
                let _ = openai_realtime.set_key(key);
                client.insert_client("open_ai_realtime", Box::new(openai_realtime));
            }

            // Only add OpenAI client if API key is present
            if let Some(key) = OPEN_AI_KEY {
                let openai_url = "https://api.openai.com/v1";
                let mut openai = OpenAiClient::new(openai_url.into());
                let _ = openai.set_key(key);
                client.insert_client("open_ai", Box::new(openai));
            }

            // Only add OpenRouter client if API key is present
            if let Some(key) = OPEN_ROUTER_KEY {
                let open_router_url = "https://openrouter.ai/api/v1";
                let mut open_router = OpenAiClient::new(open_router_url.into());
                let _ = open_router.set_key(key);
                client.insert_client("open_router", Box::new(open_router));
            }

            // Only add SiliconFlow client if API key is present
            if let Some(key) = SILICON_FLOW_KEY {
                let siliconflow_url = "https://api.siliconflow.cn/api/v1";
                let mut siliconflow = OpenAiClient::new(siliconflow_url.into());
                let _ = siliconflow.set_key(key);
                client.insert_client("silicon_flow", Box::new(siliconflow));
            }

            client
        };

        // Create MCP manager and configure playwright tool
        let tool_manager = {
            let manager = McpManagerClient::new();

            // Configure playwright tool
            let playwright_transport = {
                let mut command = tokio::process::Command::new("zsh");
                command.arg("/Users/wyeworks/mcp/scripts/playwright.sh");
                McpTransport::Stdio(command)
            };

            let manager_clone = manager.clone();
            spawn(async move {
                if let Err(e) = manager_clone
                    .add_server("playwright", playwright_transport)
                    .await
                {
                    eprintln!("Failed to add playwright server: {}", e);
                }
            });

            manager
        };

        let controller = ChatController::builder()
            .with_basic_spawner()
            .with_client(client)
            .with_tool_manager(tool_manager)
            .with_plugin_prepend(Plugin {
                ui: self.ui_runner(),
                initialized: false,
            })
            .build_arc();

        controller.lock().unwrap().dispatch_task(ChatTask::Load);

        self.controller = Some(controller.clone());
        let mut chat = self.chat(ids!(chat));
        chat.write().set_chat_controller(cx, Some(controller));

        if let Some(key) = OPEN_AI_STT_KEY {
            let mut client = OpenAiSttClient::new("https://api.openai.com/v1".to_string());
            let _ = client.set_key(key);
            chat.write().set_stt_utility(Some(SttUtility {
                client: Box::new(client),
                bot_id: BotId::new("gpt-4o-transcribe"),
            }));
        }
    }
}

struct Plugin {
    ui: UiRunner<DemoChat>,
    initialized: bool,
}

impl ChatControllerPlugin for Plugin {
    fn on_state_ready(&mut self, state: &ChatState, _mutations: &[ChatStateMutation]) {
        self.init(state);
    }
}

impl Plugin {
    fn init(&mut self, state: &ChatState) {
        if self.initialized {
            return;
        }

        if !state.bots.is_empty() {
            let bots = state.bots.clone();
            self.ui.defer_with_redraw(move |widget, cx, _scope| {
                widget.fill_selector(cx, bots);
            });

            self.initialized = true;
            // TODO: Unsuscribe?
        }
    }
}
