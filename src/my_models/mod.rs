pub mod delete_model_modal;
pub mod downloaded_files_row;
pub mod downloaded_files_table;
pub mod model_info_modal;
pub mod my_models_screen;

use makepad_widgets::ScriptVm;

pub fn script_mod(vm: &mut ScriptVm) {
    my_models_screen::script_mod(vm);
    downloaded_files_table::script_mod(vm);
    downloaded_files_row::script_mod(vm);
    delete_model_modal::script_mod(vm);
    model_info_modal::script_mod(vm);
}
