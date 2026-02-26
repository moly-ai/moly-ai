use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use crate.ui.*

    load_all_resources() do #(App::script_component(vm)) {
        ui: mod.widgets.Ui {}
    }
}

#[derive(Script, ScriptHook)]
struct App {
    #[live]
    ui: WidgetRef,
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        makepad_widgets::script_mod(vm);
        moly_kit::widgets::script_mod(vm);
        crate::meta::script_mod(vm);
        crate::demo_chat::script_mod(vm);
        crate::ui::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }
}

app_main!(App);
