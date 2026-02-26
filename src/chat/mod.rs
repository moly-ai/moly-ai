pub mod chat_history;
pub mod chat_history_card;
pub mod chat_history_card_options;
pub mod chat_history_panel;
pub mod chat_params;
pub mod chat_screen;
pub mod chat_screen_mobile;
pub mod chat_view;
pub mod chats_deck;
pub mod deep_inquire_content;
pub mod deep_inquire_stages;
pub mod delete_chat_modal;
pub mod entity_button;
pub mod model_info;
pub mod moly_bot_filter;
pub mod shared;

use makepad_widgets::ScriptVm;

pub fn script_mod(vm: &mut ScriptVm) {
    // Note: shared and entity_button are registered early in app.rs
    // (before landing::script_mod) because model_list uses EntityButton.
    deep_inquire_stages::script_mod(vm);
    deep_inquire_content::script_mod(vm);
    delete_chat_modal::script_mod(vm);
    chat_history_card_options::script_mod(vm);
    chat_history_card::script_mod(vm);
    chat_history::script_mod(vm);
    chat_history_panel::script_mod(vm);
    chat_params::script_mod(vm);
    chat_view::script_mod(vm);
    chats_deck::script_mod(vm);
    model_info::script_mod(vm);
    chat_screen_mobile::script_mod(vm);
    chat_screen::script_mod(vm);
}
