use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::resource_imports::*;
    use crate::shared::widgets::MolyButton;

    UPDATE_ICON = dep("crate://self/resources/images/failure_icon.png")

    UpdatePopupDialog = <RoundedView> {
        width: 350
        height: Fit
        margin: {top: 20, right: 20}
        padding: {top: 20, right: 20 bottom: 20 left: 20}
        spacing: 15

        show_bg: true
        draw_bg: {
            color: #fff
            instance border_radius: 4.0
            fn pixel(self) -> vec4 {
                let border_color = #d4;
                let border_width = 1;
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let body = #fff

                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    self.border_radius
                )
                sdf.fill_keep(body)

                sdf.stroke(
                    border_color,
                    border_width
                )
                return sdf.result
            }
        }
    }

    UpdateCloseButton = <MolyButton> {
        width: Fit,
        height: Fit,

        margin: {top: -8}

        draw_icon: {
            svg_file: (ICON_CLOSE),
            fn get_color(self) -> vec4 {
                return #000;
            }
        }
        icon_walk: {width: 10, height: 10}
    }

    UpdateIcon = <View> {
        width: Fit,
        height: Fit,
        margin: {top: -10, left: -10}
        update_icon = <View> {
            width: Fit,
            height: Fit,
            <Image> {
                source: (UPDATE_ICON),
                width: 35,
                height: 35,
            }
        }
    }

    UpdateContent = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 10

        title = <Label> {
            draw_text:{
                text_style: <BOLD_FONT>{font_size: 9},
                word: Wrap,
                color: #000
            }
            text: "New update available"
        }

        summary = <Label> {
            width: Fill,
            draw_text:{
                text_style: <REGULAR_FONT>{font_size: 9},
                word: Wrap,
                color: #000
            }
            text: ""
        }

        actions = <View> {
            width: Fit,
            height: Fit,
            spacing: 10,

            download_button = <MolyButton> {
                text: "Download update"
                draw_text: {
                    color: #000
                    text_style: <BOLD_FONT>{font_size: 9}
                }
            }

            later_button = <MolyButton> {
                text: "Later"
                draw_text: {
                    color: #000
                    text_style: <BOLD_FONT>{font_size: 9}
                }
            }
        }
    }

    pub UpdateNotificationPopup = {{UpdateNotificationPopup}} {
        width: Fit
        height: Fit

        <UpdatePopupDialog> {
            <UpdateIcon> {}
            <UpdateContent> {}
            close_button = <UpdateCloseButton> {}
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum UpdateNotificationPopupAction {
    None,
    CloseButtonClicked,
    OpenReleasePage,
}

#[derive(Live, LiveHook, Widget)]
pub struct UpdateNotificationPopup {
    #[deref]
    view: View,

    #[layout]
    layout: Layout,

    #[rust]
    version: Option<String>,
}

impl Widget for UpdateNotificationPopup {
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

impl WidgetMatchEvent for UpdateNotificationPopup {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(ids!(close_button)).clicked(actions) {
            cx.action(UpdateNotificationPopupAction::CloseButtonClicked);
        }

        if self.button(ids!(download_button)).clicked(actions) {
            cx.action(UpdateNotificationPopupAction::OpenReleasePage);
        }

        if self.button(ids!(later_button)).clicked(actions) {
            cx.action(UpdateNotificationPopupAction::CloseButtonClicked);
        }
    }
}

impl UpdateNotificationPopup {
    pub fn set_version(&mut self, cx: &mut Cx, version: &str) {
        self.version = Some(version.to_string());
        let summary = match &self.version {
            Some(v) => format!("Version {v} is available. Download the latest release to update."),
            None => "A new version is available.".to_string(),
        };

        self.label(ids!(summary)).set_text(cx, &summary);
    }
}

impl UpdateNotificationPopupRef {
    pub fn set_version(&mut self, cx: &mut Cx, version: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_version(cx, version);
        }
    }
}
