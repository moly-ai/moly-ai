//! The avatar of a bot in a chat message.

use crate::aitk::protocol::*;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.AvatarBase = #(Avatar::register_widget(vm))

    mod.widgets.Avatar = set_type_default() do mod.widgets.AvatarBase {
        height: Fit
        width: Fit
        grapheme := RoundedView {
            visible: false
            width: 24
            height: 24

            show_bg: true
            draw_bg +: {
                color: #x37567d
                radius: 3
            }

            align: Align{x: 0.5 y: 0.5}

            label := Label {
                width: Fit
                height: Fit
                draw_text +: {
                    text_style +: theme.font_bold {font_size: 8.5}
                    color: #fff
                }
                text: "P"
            }
        }

        dependency := RoundedView {
            width: 28
            height: 28
            visible: false

            show_bg: true
            draw_bg +: {
                radius: 2
            }

            image := Image {
                width: 28
                height: 28
            }
        }
    }
}

#[derive(Script, Widget, ScriptHook)]
pub struct Avatar {
    #[deref]
    deref: View,

    #[rust]
    pub avatar: Option<EntityAvatar>,
}

impl Widget for Avatar {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(avatar) = &self.avatar {
            match avatar {
                EntityAvatar::Text(grapheme) => {
                    self.view(cx, ids!(grapheme)).set_visible(cx, true);
                    self.view(cx, ids!(dependency)).set_visible(cx, false);
                    self.label(cx, ids!(label)).set_text(cx, grapheme);
                }
                EntityAvatar::Image(path) => {
                    self.view(cx, ids!(dependency)).set_visible(cx, true);
                    self.view(cx, ids!(grapheme)).set_visible(cx, false);
                    let _ = self
                        .image(cx, ids!(image))
                        .load_image_dep_by_path(cx, path)
                        .or_else(|_| {
                            self.image(cx, ids!(image))
                                .load_image_file_by_path(cx, path.as_ref())
                        });
                }
            }
        }

        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope)
    }
}
