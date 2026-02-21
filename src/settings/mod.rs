pub mod add_provider_modal;
pub mod moly_server_screen;
pub mod provider_view;
pub mod providers;
pub mod providers_screen;
pub mod sync_modal;
pub mod utilities_modal;
use makepad_widgets::ScriptVm;

pub fn script_mod(vm: &mut ScriptVm) {
    providers_screen::script_mod(vm);
    moly_server_screen::script_mod(vm);
    provider_view::script_mod(vm);
    providers::script_mod(vm);
    add_provider_modal::script_mod(vm);
    sync_modal::script_mod(vm);
    utilities_modal::script_mod(vm);
}
