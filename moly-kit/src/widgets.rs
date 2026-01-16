//! Widgets provided by this crate. You can import this in your DSL.
//!
//! Note: Some widgets may depend on certain feature flags.

use makepad_widgets::*;

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

pub fn live_design(cx: &mut makepad_widgets::Cx) {
    theme_moly_kit_light::live_design(cx);
    // Link the MolyKit theme to the MolyKit-specific theme.
    // Currently we only have a light theme which we use as default.
    cx.link(live_id!(moly_kit_theme), live_id!(theme_moly_kit_light));

    math_widget::math::live_design(cx);
    image_view::live_design(cx);
    attachment_view::live_design(cx);
    moly_modal::live_design(cx);
    attachment_viewer_modal::live_design(cx);
    attachment_list::live_design(cx);
    citation::live_design(cx);
    citation_list::live_design(cx);
    makepad_code_editor::live_design(cx);
    message_markdown::live_design(cx);
    message_loading::live_design(cx);
    avatar::live_design(cx);
    slot::live_design(cx);
    standard_message_content::live_design(cx);
    chat_line::live_design(cx);
    messages::live_design(cx);
    stt_input::live_design(cx);
    prompt_input::live_design(cx);
    model_selector_item::live_design(cx);
    model_selector_list::live_design(cx);
    model_selector::live_design(cx);
    chat::live_design(cx);
    realtime::live_design(cx);
    message_thinking_block::live_design(cx);
}
