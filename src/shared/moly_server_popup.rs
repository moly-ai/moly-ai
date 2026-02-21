use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ERROR_ICON = crate_resource("self://resources/images/failure_icon.png")

    let MolyServerPopupDialog = RoundedView {
        width: 350
        height: Fit
        margin: Inset { top: 20 right: 20 }
        padding: Inset { top: 20 right: 20 bottom: 20 left: 20 }
        spacing: 15

        show_bg: true
        draw_bg +: {
            color: #fff
            border_radius: instance(4.0)
            pixel: fn() -> vec4 {
                let border_color = #d4
                let border_width = 1
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let body = #fff

                sdf.box(
                    1.
                    1.
                    self.rect_size.x - 2.0
                    self.rect_size.y - 2.0
                    self.border_radius
                )
                sdf.fill_keep(body)

                sdf.stroke(
                    border_color
                    border_width
                )
                return sdf.result
            }
        }
    }

    let NetworkErrorCloseButton = MolyButton {
        width: Fit
        height: Fit

        margin: Inset { top: -8 }

        draw_icon +: {
            svg: ICON_CLOSE
            get_color: fn() -> vec4 {
                return #000
            }
        }
        icon_walk: Walk { width: 10 height: 10 }
    }

    let NetworkErrorIcon = View {
        width: Fit
        height: Fit
        margin: Inset { top: -10 left: -10 }
        error_icon := View {
            width: Fit
            height: Fit
            Image {
                source: ERROR_ICON
                width: 35
                height: 35
            }
        }
    }

    let NetworkErrorContent = View {
        width: Fill
        height: Fit
        flow: Down
        spacing: 10

        title := Label {
            draw_text +: {
                text_style: BOLD_FONT { font_size: 9 }
                word: Wrap
                color: #000
            }
            text: "Network Connection Error"
        }

        message := Label {
            width: Fill
            draw_text +: {
                text_style: REGULAR_FONT { font_size: 9 }
                word: Wrap
                color: #000
            }
            text: "Connection with MolySever interrupted.\nPlease check that the server is running and try again."
        }
    }

    mod.widgets.MolyServerPopup =
        #(MolyServerPopup::register_widget(vm)) ViewBase {
        width: Fit
        height: Fit

        MolyServerPopupDialog {
            NetworkErrorIcon {}
            NetworkErrorContent {}
            close_button := NetworkErrorCloseButton {}
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum MolyServerPopupAction {
    #[default]
    None,
    CloseButtonClicked,
}

#[derive(Script, ScriptHook, Widget)]
pub struct MolyServerPopup {
    #[deref]
    view: View,

    #[layout]
    layout: Layout,
}

impl Widget for MolyServerPopup {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self
            .view
            .draw_walk(cx, scope, walk.with_abs_pos(DVec2 { x: 0., y: 0. }));

        DrawStep::done()
    }
}

impl WidgetMatchEvent for MolyServerPopup {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(ids!(close_button)).clicked(actions) {
            cx.action(MolyServerPopupAction::CloseButtonClicked);
        }
    }
}
