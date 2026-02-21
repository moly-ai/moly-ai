use makepad_widgets::ScriptVm;

pub mod actions;
pub mod bot_context;
pub mod desktop_buttons;
pub mod download_notification_popup;
pub mod external_link;
pub mod list;
pub mod meta;
pub mod moly_server_popup;
pub mod popup_notification;
pub mod resource_imports;
pub mod styles;
pub mod tooltip;
pub mod utils;
pub mod widgets;

pub fn script_mod(vm: &mut ScriptVm) {
    meta::script_mod(vm);
    list::script_mod(vm);
    styles::script_mod(vm);
    resource_imports::script_mod(vm);
    widgets::script_mod(vm);
    popup_notification::script_mod(vm);
    external_link::script_mod(vm);
    download_notification_popup::script_mod(vm);
    tooltip::script_mod(vm);
    desktop_buttons::script_mod(vm);
    moly_server_popup::script_mod(vm);
}
