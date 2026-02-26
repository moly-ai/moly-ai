use crate::demo_chat::DemoChatWidgetExt;
use makepad_widgets::*;
use moly_kit::prelude::*;

script_mod! {
    use mod.prelude.widgets.*

    use crate.demo_chat.*

    mod.widgets.UiBase = #(Ui::register_widget(vm))
    mod.widgets.Ui = set_type_default() do mod.widgets.UiBase {
        align: Align { x: 0.5 y: 0.5 }
        pass: Pass { clear_color: #xfff }

        body := View {
            padding: Inset { top: 40 }
            chat_2 := mod.widgets.DemoChat {}
        }
    }
}

#[derive(Script, Widget)]
pub struct Ui {
    #[deref]
    deref: Window,
}

impl Widget for Ui {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope);

        if let Event::Startup = event {
            let bot_id = BotId::new("unknown_bot");

            let messages = std::iter::repeat([
                Message {
                    from: EntityId::User,
                    content: MessageContent {
                        text: "Hello".to_string(),
                        citations: vec![],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Message {
                    from: EntityId::Bot(bot_id),
                    content: MessageContent {
                        text: "World".to_string(),
                        attachments: vec![
                            Attachment::from_bytes(
                                "text.txt".into(),
                                Some("text/plain".into()),
                                b"Hello, world!",
                            ),
                            Attachment::from_bytes(
                                "image.png".into(),
                                Some("image/png".into()),
                                include_bytes!(
                                    "../../../../packaging/Moly macOS dmg background.png"
                                ),
                            ),
                        ],
                        citations: vec![
                            "https://github.com/ZhangHanDong/url-preview/issues/2".to_string(),
                            "https://3.basecamp.com/5400951/buckets/28531977/messages/8467029657"
                                .to_string(),
                            "https://en.wikipedia.org/wiki/ICO_(file_format)".to_string(),
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ])
            .take(1)
            .flatten()
            .collect::<Vec<_>>();
        }
    }
}

impl ScriptHook for Ui {}
