use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::ui::*;

    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                window: { title: "Moly Zoo" },
                body = <Ui> {}
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        moly_widgets::live_design(cx);
        crate::button_showcase::live_design(cx);
        crate::card_showcase::live_design(cx);
        crate::switch_showcase::live_design(cx);
        crate::text_input_showcase::live_design(cx);
        crate::theme_showcase::live_design(cx);
        crate::ui::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
