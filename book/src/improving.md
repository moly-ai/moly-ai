# Multiple Providers and Dynamic Models

The [Quickstart](quickstart.md) used a single client with a hardcoded model. This
chapter builds on that to show how to support multiple providers, discover models at
runtime, and use plugins for automation.

The patterns shown here match what the
[moly-mini](https://github.com/moly-ai/moly-ai/tree/main/moly-kit/examples/moly-mini)
example does.

## RouterClient

aitk's `RouterClient` aggregates multiple `BotClient` implementations into one,
routing requests based on a prefix in the `BotId`. See the
[aitk Router Client documentation](https://moly-ai.github.io/aitk/clients/router.html)
for full details.

Instead of creating a single `OpenAiClient`, build a `RouterClient` with multiple
sub-clients:

```rust
fn build_client() -> RouterClient {
    let client = RouterClient::new();

    // Ollama runs locally, no key needed.
    let ollama = OpenAiClient::new("http://localhost:11434/v1".into());
    client.insert_client("ollama", Box::new(ollama));

    // Add OpenAI if a key is available.
    if let Some(key) = option_env!("OPEN_AI_KEY") {
        let mut openai = OpenAiClient::new("https://api.openai.com/v1".into());
        let _ = openai.set_key(key);
        client.insert_client("open_ai", Box::new(openai));
    }

    // Add more providers following the same pattern.

    client
}
```

## Loading models dynamically

Instead of hardcoding a `BotId`, ask the controller to fetch available models from
all configured providers:

```rust
let controller = ChatController::builder()
    .with_client(build_client())
    .with_basic_spawner()
    .build_arc();

controller.lock().unwrap().dispatch_task(ChatTask::Load);
```

`ChatTask::Load` triggers an async call to `bots()` on the client. When it completes,
the controller's `state().bots` is populated with all available models (prefixed by
their router key, e.g. `ollama/llama3`, `open_ai/gpt-4.1`).

The `Chat` widget includes a built-in `ModelSelector` that lets the user pick from
the loaded models. Once the user selects a model, the widget sets the `BotId` on the
controller automatically.

## Plugins

aitk's `ChatControllerPlugin` trait lets you hook into state changes and task
execution. Plugins form a composable pipeline -- they observe mutations, react to
state transitions, and can intercept tasks before they execute. See the
[aitk Chat App documentation](https://moly-ai.github.io/aitk/chat-app/simple.html)
for more information.

In our example, we'll use a plugin to auto-select a model once they finish loading,
so the user can start chatting immediately without manual selection.

The plugin uses Makepad's `DeferWithRedraw` trait, which requires an explicit import:

```rust
use makepad_widgets::defer_with_redraw::DeferWithRedraw;
```

```rust
struct AutoSelectPlugin {
    ui: UiRunner<MyChat>,
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
```

The plugin watches for state changes. Once `state.bots` is populated (meaning the
`Load` task completed), it defers a UI action via Makepad's `UiRunner` to select the
first bot. `UiRunner` is Makepad's mechanism for safely deferring work back to the
widget that owns the runner.

The selection itself dispatches a mutation on the controller:

```rust
impl MyChat {
    fn select_first_bot(&mut self, bots: &[Bot]) {
        let Some(controller) = &self.controller else {
            return;
        };
        if let Some(bot) = bots.first() {
            controller
                .lock()
                .unwrap()
                .dispatch_mutation(ChatStateMutation::SetBotId(Some(bot.id.clone())));
        }
    }
}
```

## Putting it together

Register the plugin when building the controller, and wire everything in
`on_after_new`. Since the plugin uses `UiRunner`, you also need to add
`self.ui_runner().handle(...)` to your widget's `handle_event`:

```rust
impl Widget for MyChat {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.ui_runner().handle(cx, event, scope, self);
        self.view.handle_event(cx, event, scope);
    }

    // draw_walk unchanged...
}

impl ScriptHook for MyChat {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        let controller = ChatController::builder()
            .with_basic_spawner()
            .with_client(build_client())
            .with_plugin_prepend(AutoSelectPlugin {
                ui: self.ui_runner(),
                initialized: false,
            })
            .build_arc();

        controller.lock().unwrap().dispatch_task(ChatTask::Load);

        self.controller = Some(controller.clone());
        vm.with_cx_mut(|cx| {
            self.chat(cx, ids!(chat))
                .write()
                .set_chat_controller(cx, Some(controller));
        });
    }
}
```
