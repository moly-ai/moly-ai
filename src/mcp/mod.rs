pub mod mcp_screen;
pub mod mcp_servers;

use makepad_widgets::ScriptVm;

pub fn script_mod(vm: &mut ScriptVm) {
    mcp_servers::script_mod(vm);
    mcp_screen::script_mod(vm);
}
