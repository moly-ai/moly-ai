use makepad_widgets::*;
use moly_protocol::data::{File, FileId};

use crate::{app::NavigationAction, shared::actions::DownloadAction};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let SUCCESS_ICON = crate_resource("self://resources/images/success_icon.png")
    let FAILURE_ICON = crate_resource("self://resources/images/failure_icon.png")

    let PRIMARY_LINK_FONT_COLOR = #x0E7090
    let SECONDARY_LINK_FONT_COLOR = #667085

    let PopupActionLink = LinkLabel {
        width: Fit
        margin: 2
        draw_text +: {
            text_style: BOLD_FONT { font_size: 9 }
            get_color: fn() -> vec4 {
                return mix(
                    mix(
                        PRIMARY_LINK_FONT_COLOR
                        PRIMARY_LINK_FONT_COLOR
                        self.hover
                    )
                    PRIMARY_LINK_FONT_COLOR
                    self.down
                )
            }
        }
    }

    let PopupSecondaryActionLink = LinkLabel {
        width: Fit
        margin: 2
        draw_text +: {
            text_style: BOLD_FONT { font_size: 9 }
            get_color: fn() -> vec4 {
                return mix(
                    mix(
                        SECONDARY_LINK_FONT_COLOR
                        SECONDARY_LINK_FONT_COLOR
                        self.hover
                    )
                    SECONDARY_LINK_FONT_COLOR
                    self.down
                )
            }
        }
    }

    let PopupDialog = RoundedView {
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
                let border_size = 1
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
                    border_size
                )
                return sdf.result
            }
        }
    }

    let PopupCloseButton = MolyButton {
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

    let NotificationIcons = View {
        width: Fit
        height: Fit
        margin: Inset { top: -10 left: -10 }
        success_icon := View {
            width: Fit
            height: Fit
            Image {
                source: SUCCESS_ICON
                width: 35
                height: 35
            }
        }
        failure_icon := View {
            visible: false
            width: Fit
            height: Fit
            Image {
                source: FAILURE_ICON
                width: 35
                height: 35
            }
        }
    }

    let NotificationContent = View {
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
            text: "Model Downloaded Successfully"
        }

        summary := Label {
            width: Fill
            draw_text +: {
                text_style: REGULAR_FONT { font_size: 9 }
                word: Wrap
                color: #000
            }
            text: ""
        }

        success_actions := View {
            width: Fit
            height: Fit
            view_in_my_models_link := PopupActionLink {
                text: "View in My Models"
            }
        }

        failure_actions := View {
            width: Fit
            height: Fit
            spacing: 10

            retry_link := PopupActionLink {
                text: "Retry"
            }

            cancel_link := PopupSecondaryActionLink {
                text: "Cancel"
            }
        }
    }

    mod.widgets.DownloadNotificationPopup =
        #(DownloadNotificationPopup::register_widget(vm)) ViewBase {
        width: Fit
        height: Fit

        PopupDialog {
            NotificationIcons {}
            NotificationContent {}
            close_button := PopupCloseButton {}
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum DownloadNotificationPopupAction {
    #[default]
    None,
    CloseButtonClicked,
    ActionLinkClicked,
}

#[derive(Default)]
pub enum DownloadResult {
    #[default]
    Success,
    Failure,
}

#[derive(Script, ScriptHook, Widget)]
pub struct DownloadNotificationPopup {
    #[deref]
    view: View,
    #[layout]
    layout: Layout,

    #[rust]
    download_result: DownloadResult,
    #[rust]
    file_id: Option<FileId>,
    #[rust]
    filename: String,
    #[rust]
    count: usize,
}

impl Widget for DownloadNotificationPopup {
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

impl WidgetMatchEvent for DownloadNotificationPopup {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(cx, ids!(close_button)).clicked(actions) {
            cx.action(DownloadNotificationPopupAction::CloseButtonClicked);
        }

        if self
            .link_label(cx, ids!(view_in_my_models_link))
            .clicked(actions)
        {
            cx.action(NavigationAction::NavigateToMyModels);
            cx.action(DownloadNotificationPopupAction::ActionLinkClicked);
        }

        if self.link_label(cx, ids!(retry_link)).clicked(actions) {
            let Some(file_id) = &self.file_id else { return };
            cx.action(DownloadAction::Play(file_id.clone()));
            cx.action(DownloadNotificationPopupAction::ActionLinkClicked);
        }

        if self.link_label(cx, ids!(cancel_link)).clicked(actions) {
            let Some(file_id) = &self.file_id else { return };
            cx.action(DownloadAction::Cancel(file_id.clone()));
            cx.action(DownloadNotificationPopupAction::ActionLinkClicked);
        }
    }
}

impl DownloadNotificationPopup {
    /// Updates the popup content based on the current download result.
    pub fn update_content(&mut self, cx: &mut Cx) {
        match self.download_result {
            DownloadResult::Success => self.show_success_content(cx),
            DownloadResult::Failure => self.show_failure_content(cx),
        }
    }

    fn show_success_content(&mut self, cx: &mut Cx) {
        self.view(cx, ids!(success_icon)).set_visible(cx, true);
        self.view(cx, ids!(failure_icon)).set_visible(cx, false);

        self.view(cx, ids!(success_actions)).set_visible(cx, true);
        self.view(cx, ids!(failure_actions)).set_visible(cx, false);

        self.label(cx, ids!(title))
            .set_text(cx, "Model Downloaded Successfully");

        self.label(cx, ids!(summary))
            .set_text(cx, &format!("{} successfuly downloaded.", &self.filename));
    }

    fn show_failure_content(&mut self, cx: &mut Cx) {
        self.view(cx, ids!(success_icon)).set_visible(cx, false);
        self.view(cx, ids!(failure_icon)).set_visible(cx, true);

        self.view(cx, ids!(success_actions)).set_visible(cx, false);
        self.view(cx, ids!(failure_actions)).set_visible(cx, true);

        self.label(cx, ids!(title))
            .set_text(cx, "Errors while downloading models");

        self.label(cx, ids!(summary)).set_text(
            cx,
            &format!(
                "{} encountered some errors when downloading.",
                &self.filename
            ),
        );
    }

    /// Shows retry content with escalating delay messages.
    pub fn show_retry_content(&mut self, cx: &mut Cx) {
        let content = self.label(cx, ids!(summary));
        self.view(cx, ids!(success_icon)).set_visible(cx, false);
        self.view(cx, ids!(failure_icon)).set_visible(cx, true);

        self.view(cx, ids!(success_actions)).set_visible(cx, false);
        self.view(cx, ids!(failure_actions)).set_visible(cx, false);

        self.label(cx, ids!(title)).set_text(cx, "Retry");

        match self.count {
            0 => {
                content.set_text(cx, "Download interrupted. Will resume in 15 seconds.");
                self.count += 1;
            }
            1 => {
                content.set_text(cx, "Download interrupted. Will resume in 30 seconds.");
                self.count += 1;
            }
            2 => {
                content.set_text(cx, "Download interrupted. Will resume in 60 seconds.");
                self.count += 1;
            }
            _ => {
                self.count = 0;
            }
        }
    }
}

impl DownloadNotificationPopupRef {
    /// Sets data and updates the popup content.
    pub fn set_data(&mut self, cx: &mut Cx, file: &File, download_result: DownloadResult) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.file_id = Some(file.id.clone());
            inner.filename = file.name.clone();
            inner.download_result = download_result;

            inner.update_content(cx);
        }
    }

    /// Sets the popup to show retry content.
    pub fn set_retry_data(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show_retry_content(cx);
        }
    }
}
