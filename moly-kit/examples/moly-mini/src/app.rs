use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use crate.ui.*

    mod.widgets.AppBase = #(App::register_widget(vm))
    mod.widgets.App = set_type_default() do mod.widgets.AppBase {
        ui: mod.widgets.Ui {}
    }
}

#[derive(Script, ScriptHook)]
struct App {
    #[script]
    ui: WidgetRef,
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl ScriptRegister for App {
    fn script_register(vm: &mut ScriptVm) {
        makepad_widgets::script_mod(vm);
        moly_kit::widgets::script_mod(vm);
        crate::meta::script_mod(vm);
        crate::demo_chat::script_mod(vm);
        crate::ui::script_mod(vm);
    }
}

app_main!(App);
