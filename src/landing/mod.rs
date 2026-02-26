pub mod download_item;
pub mod downloads;
pub mod landing_screen;
pub mod model_card;
pub mod model_files;
pub mod model_files_item;
pub mod model_files_list;
pub mod model_files_tags;
pub mod model_list;
pub mod search_bar;
pub mod search_loading;
pub mod shared;
pub mod sorting;

use makepad_widgets::ScriptVm;

pub fn script_mod(vm: &mut ScriptVm) {
    shared::script_mod(vm);
    model_files_tags::script_mod(vm);
    model_files_item::script_mod(vm);
    model_files_list::script_mod(vm);
    model_files::script_mod(vm);
    sorting::script_mod(vm);
    search_loading::script_mod(vm);
    download_item::script_mod(vm);
    downloads::script_mod(vm);
    model_card::script_mod(vm);
    model_list::script_mod(vm);
    search_bar::script_mod(vm);
    landing_screen::script_mod(vm);
}
