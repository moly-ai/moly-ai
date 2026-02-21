//! Widgets provided by this crate. You can import this in your DSL.
//!
//! Note: Some widgets may depend on certain feature flags.

mod attachment_list;
mod attachment_view;
mod attachment_viewer_modal;
mod avatar;
mod chat_line;
mod citation;
mod image_view;
mod message_loading;
mod message_thinking_block;
mod model_selector_item;
mod slot;
mod standard_message_content;
mod theme_moly_kit_light;

// Note: Many of these widgets are not ready to be public, or they are not
// intended for public use. However, we must expose them for things related to
// Makepad, like DSL querying and overriding.
// TODO: See if overriding can be done in DSLs without making the Rust struct public.
// and if we can work with `apply_over`s with generic queries instead of the specific
// widget ones.

pub mod chat;
pub mod citation_list;
pub mod message_markdown;
pub mod messages;
pub mod model_selector;
pub mod model_selector_list;
pub mod moly_modal;
pub mod prompt_input;
pub mod realtime;
pub mod stt_input;

pub fn script_mod(vm: &mut makepad_widgets::ScriptVm) {
    theme_moly_kit_light::script_mod(vm);
    image_view::script_mod(vm);
    attachment_view::script_mod(vm);
    moly_modal::script_mod(vm);
    attachment_viewer_modal::script_mod(vm);
    attachment_list::script_mod(vm);
    citation::script_mod(vm);
    citation_list::script_mod(vm);
    makepad_code_editor::script_mod(vm);
    message_markdown::script_mod(vm);
    message_loading::script_mod(vm);
    avatar::script_mod(vm);
    slot::script_mod(vm);
    standard_message_content::script_mod(vm);
    chat_line::script_mod(vm);
    messages::script_mod(vm);
    stt_input::script_mod(vm);
    prompt_input::script_mod(vm);
    model_selector_item::script_mod(vm);
    model_selector_list::script_mod(vm);
    model_selector::script_mod(vm);
    chat::script_mod(vm);
    realtime::script_mod(vm);
    message_thinking_block::script_mod(vm);
}
