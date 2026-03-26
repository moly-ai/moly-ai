use std::sync::{Arc, Mutex};

use makepad_widgets::defer_with_redraw::DeferWithRedraw;
use makepad_widgets::*;
use moly_kit::prelude::*;

const OPEN_AI_KEY: Option<&str> = option_env!("OPEN_AI_KEY");
const OPEN_ROUTER_KEY: Option<&str> = option_env!("OPEN_ROUTER_KEY");
const SILICON_FLOW_KEY: Option<&str> = option_env!("SILICON_FLOW_KEY");

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.MolyMiniBase = #(MolyMini::register_widget(vm))

    load_all_resources() do #(App::script_component(vm)) {
        ui: Root {
            main_window := Window {
                window +: { inner_size: vec2(800 600) title: "moly-mini" }
                pass +: { clear_color: #xfff }
                body +: {
                    moly_mini := mod.widgets.MolyMiniBase {
                        width: Fill
                        height: Fill
                        flow: Down
                        padding: Inset {
                            top: 40 left: 12 right: 12 bottom: 12
                        }
                        chat := Chat {}
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        makepad_widgets::script_mod(vm);
        moly_kit::widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

#[derive(Script, ScriptHook, Widget)]
struct MolyMini {
    #[deref]
    view: View,

    #[rust]
    controller: Option<Arc<Mutex<ChatController>>>,
}

impl Widget for MolyMini {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.ui_runner().handle(cx, event, scope, self);
        self.view.handle_event(cx, event, scope);

        if let Event::Startup = event {
            self.setup(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MolyMini {
    fn setup(&mut self, cx: &mut Cx) {
        let client = Self::build_client();

        let controller = ChatController::builder()
            .with_basic_spawner()
            .with_client(client)
            .with_plugin_prepend(AutoSelectPlugin {
                ui: self.ui_runner(),
                initialized: false,
            })
            .build_arc();

        controller.lock().unwrap().dispatch_task(ChatTask::Load);

        self.controller = Some(controller.clone());
        self.chat(cx, ids!(chat))
            .write()
            .set_chat_controller(cx, Some(controller));
    }

    fn build_client() -> RouterClient {
        let client = RouterClient::new();

        let ollama = OpenAiClient::new("http://localhost:11434/v1".into());
        client.insert_client("ollama", Box::new(ollama));

        if let Some(key) = OPEN_AI_KEY {
            let mut openai = OpenAiClient::new("https://api.openai.com/v1".into());
            let _ = openai.set_key(key);
            client.insert_client("open_ai", Box::new(openai));
        }

        if let Some(key) = OPEN_ROUTER_KEY {
            let mut open_router = OpenAiClient::new("https://openrouter.ai/api/v1".into());
            let _ = open_router.set_key(key);
            client.insert_client("open_router", Box::new(open_router));
        }

        if let Some(key) = SILICON_FLOW_KEY {
            let mut siliconflow = OpenAiClient::new("https://api.siliconflow.cn/api/v1".into());
            let _ = siliconflow.set_key(key);
            client.insert_client("silicon_flow", Box::new(siliconflow));
        }

        client
    }

    fn select_first_bot(&mut self, bots: &[Bot]) {
        let Some(controller) = &self.controller else {
            return;
        };
        if let Some(bot) = bots.first() {
            controller
                .lock()
                .unwrap()
                .dispatch_mutation(ChatStateMutation::SetBotId(Some(bot.id.clone())));
        } else {
            eprintln!("No models available, check your API keys.");
        }
    }
}

/// Selects the first available bot once models are loaded.
struct AutoSelectPlugin {
    ui: UiRunner<MolyMini>,
    initialized: bool,
}

impl ChatControllerPlugin for AutoSelectPlugin {
    fn on_state_ready(&mut self, state: &ChatState, _mutations: &[ChatStateMutation]) {
        if self.initialized || state.bots.is_empty() {
            return;
        }

        let bots = state.bots.clone();
        self.ui.defer_with_redraw(move |widget, _cx, _scope| {
            widget.select_first_bot(&bots);
        });
        self.initialized = true;
    }
}
