use makepad_widgets::*;

use crate::data::providers::ProviderBot;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ChatModelAvatar = RoundedView {
        width: 24
        height: 24

        show_bg: true
        draw_bg +: {
            color: #37567d
            border_radius: 6
        }

        align: Align {x: 0.5 y: 0.5}

        avatar_label := Label {
            width: Fit
            height: Fit
            draw_text +: {
                text_style: BOLD_FONT {font_size: 8}
                color: #fff
            }
            text: "P"
        }
    }

    mod.widgets.ChatAgentAvatarBase = #(ChatAgentAvatar::register_widget(vm))
    mod.widgets.ChatAgentAvatar = set_type_default() do mod.widgets.ChatAgentAvatarBase {
        reasoner_agent_icon: crate_resource("self://resources/images/reasoner_agent_icon.png")
        width: Fit
        height: Fit
        image := Image { width: 24 height: 24 }
    }
}

#[derive(Script, Widget, ScriptHook)]
pub struct ChatAgentAvatar {
    #[deref]
    view: View,

    #[live]
    reasoner_agent_icon: Option<ScriptHandleRef>,

    #[rust]
    pending_image_update: Option<ScriptHandleRef>,
}

impl Widget for ChatAgentAvatar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(dep) = self.pending_image_update.take() {
            script_apply_eval!(cx, self.view, {
                image +: {
                    src: #(dep)
                }
            })
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl ChatAgentAvatar {
    pub fn set_bot(&mut self, _agent: &ProviderBot) {
        let dep = self.reasoner_agent_icon.clone();

        self.pending_image_update = dep;
    }
}

impl ChatAgentAvatarRef {
    pub fn set_bot(&mut self, agent: &ProviderBot) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_bot(agent);
        }
    }

    pub fn set_visible(&mut self, visible: bool) -> () {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.visible = visible;
        }
    }
}
