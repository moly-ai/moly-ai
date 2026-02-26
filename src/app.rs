use crate::data::capture::register_capture_manager;
use crate::data::downloads::download::DownloadFileAction;
use crate::data::downloads::DownloadPendingNotification;
use crate::data::moly_client::MolyClientAction;
use crate::data::store::*;
use crate::landing::model_files_item::ModelFileItemAction;
use crate::my_models::delete_model_modal::DeleteModelModalAction;
use crate::shared::actions::{ChatAction, DownloadAction};
use crate::shared::download_notification_popup::{
    DownloadNotificationPopupAction, DownloadNotificationPopupRef,
    DownloadNotificationPopupWidgetRefExt, DownloadResult,
};
use crate::shared::moly_server_popup::MolyServerPopupAction;
use crate::shared::popup_notification::PopupNotificationWidgetRefExt;
use moly_protocol::data::{File, FileId};

use makepad_widgets::*;
use markdown::MarkdownAction;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_CHAT = crate_resource("self://resources/icons/chat.svg")
    let ICON_LOCAL = crate_resource("self://resources/icons/local.svg")
    let ICON_MCP = crate_resource("self://resources/icons/mcp.svg")
    let ICON_CLOUD = crate_resource("self://resources/icons/cloud.svg")
    let ICON_MOLYSERVER =
        crate_resource("self://resources/images/providers/molyserver.png")

    let ApplicationPages = RoundedShadowView {
        width: Fill height: Fill
        margin: Inset {top: 12 right: 12 bottom: 12}
        padding: 3
        flow: Overlay

        show_bg: true
        draw_bg +: {
            color: (MAIN_BG_COLOR)
            border_radius: instance(4.5)
            shadow_color: uniform(#x0003)
            shadow_radius: 15.0
            shadow_offset: vec2(0.0 -1.5)
        }

        chat_frame := ChatScreen {visible: true}
        moly_server_frame := MolyServerScreen {visible: false}
        mcp_frame := McpScreen {visible: false}
        providers_frame := ProvidersScreen {visible: false}
    }

    let SidebarMenu = RoundedView {
        width: 90 height: Fill
        flow: Down spacing: 15.0
        padding: Inset { top: 40 bottom: 20 left: 0 right: 0 }

        align: Align {x: 0.5 y: 0.5}

        show_bg: true
        draw_bg +: {
            color: (SIDEBAR_BG_COLOR)
            border_radius: instance(0.0)
        }

        logo := View {
            width: Fit height: Fit
            margin: Inset {bottom: 5}
            Image {
                width: 50 height: 50
                src: (ICON_MOLYSERVER)
            }
        }

        seprator := View {
            width: Fill height: 1.6
            margin: Inset {left: 15 right: 15 bottom: 10}
            show_bg: true
            draw_bg +: {
                color: #xdadada
            }
        }

        chat_tab := SidebarMenuButton {
            animator: Animator {active: {default: @on}}
            text: "Chat"
            draw_icon +: {
                svg: (ICON_CHAT)
            }
        }
        moly_server_tab := SidebarMenuButton {
            text: "MolyServer"
            draw_icon +: {
                svg: (ICON_LOCAL)
            }
        }
        HorizontalFiller {}

        mcp_tab_container := View {
            width: Fit height: Fit
            visible: false

            mcp_tab := SidebarMenuButton {
                text: "MCP"
                draw_icon +: {
                    svg: (ICON_MCP)
                }
            }
        }

        providers_tab := SidebarMenuButton {
            text: "Providers"
            draw_icon +: {
                svg: (ICON_CLOUD)
            }
        }
    }

    startup() do #(App::script_component(vm)) {
        ui: Root {
            main_window := Window {
                window +: {inner_size: vec2(1440 1024) title: "Moly"}
                pass +: {clear_color: #xfff}

                caption_bar +: {
                    caption_label := View {}
                    windows_buttons := View {
                        visible: false
                        width: Fit height: Fit
                        min := MolyDesktopButton {
                            draw_bg +: {button_type: DesktopButtonType.WindowsMin}
                        }
                        max := MolyDesktopButton {
                            draw_bg +: {button_type: DesktopButtonType.WindowsMax}
                        }
                        close := MolyDesktopButton {
                            draw_bg +: {button_type: DesktopButtonType.WindowsClose}
                        }
                    }
                }

                body +: {
                    keyboard_min_shift: 75
                    flow: Overlay
                    width: Fill
                    height: Fill
                    padding: 0

                    loading_view := View {
                        align: Align {x: 0.5 y: 0.5}
                        flow: Down spacing: 20
                        Image {
                            width: 100 height: 100
                            src: (ICON_MOLYSERVER)
                        }
                        Label {
                            text: "Loading..."
                            draw_text +: {
                                text_style: theme.font_bold { font_size: 12 }
                                color: #x444
                            }
                        }
                    }

                    root := #(MolyRoot::register_widget(vm)) {
                        width: Fill
                        height: Fill
                        show_bg: true
                        draw_bg +: {
                            color: (MAIN_BG_COLOR_DARK)
                        }

                        root_adaptive_view := AdaptiveView {
                            Mobile +: {
                                application_pages := ApplicationPages {
                                    margin: 0
                                }
                            }

                            Desktop +: {
                                sidebar_menu := SidebarMenu {}
                                application_pages := ApplicationPages {}
                            }
                        }
                    }

                    download_popup := PopupNotification {
                        content +: {
                            popup_download_notification :=
                                DownloadNotificationPopup {}
                        }
                    }

                    moly_server_popup := PopupNotification {
                        content +: {
                            popup_moly_server := MolyServerPopup {}
                        }
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    pub ui: WidgetRef,

    #[rust]
    pub store: Option<Store>,

    #[rust]
    timer: Timer,

    #[rust]
    download_retry_attempts: usize,

    #[rust]
    file_id: Option<FileId>,
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        makepad_widgets::script_mod(vm);
        moly_kit::widgets::script_mod(vm);

        crate::shared::script_mod(vm);
        // entity_button (and its dep chat::shared) must be registered before
        // landing, which uses EntityButton in model_list.
        crate::chat::shared::script_mod(vm);
        crate::chat::entity_button::script_mod(vm);
        crate::landing::script_mod(vm);
        crate::my_models::script_mod(vm);
        crate::settings::script_mod(vm);
        crate::mcp::script_mod(vm);
        crate::chat::script_mod(vm);

        App::from_script_mod(vm, self::script_mod)
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui_runner()
            .handle(cx, event, &mut Scope::empty(), self);

        if let Event::Startup = event {
            // Prevent rendering the ui before the store is initialized.
            self.ui.view(cx, ids!(body)).set_visible(cx, false);
            register_capture_manager();

            #[cfg(any(target_os = "android", target_os = "ios"))]
            // Initialize filesystem with the data directory if available,
            // required for mobile platforms.
            if let Some(data_dir) = cx.get_data_dir() {
                // Ensure the data directory exists
                let path = std::path::PathBuf::from(data_dir.clone());
                let _ = std::fs::create_dir_all(path.clone());
                if path.exists() {
                    crate::shared::utils::filesystem::init_cx_data_dir(path);
                } else {
                    panic!("Failed to create data directory: {}", data_dir);
                }
            }

            Store::load_into_app();
        }

        // If the store is not loaded, do not continue with
        // store-dependent logic. However, we still want the window to
        // handle Makepad events (e.g. window initialization events,
        // platform context changes, etc.)
        let Some(store) = self.store.as_mut() else {
            self.ui.handle_event(cx, event, &mut Scope::empty());
            return;
        };

        self.ui.view(cx, ids!(loading_view)).set_visible(cx, false);

        // It triggers when the timer expires.
        if self.timer.is_event(event).is_some() {
            if let Some(file_id) = &self.file_id {
                let (model, file) = store.get_model_and_file_download(&file_id);
                store.downloads.download_file(model, file);
                self.ui.redraw(cx);
            }
        }

        let scope = &mut Scope::with_data(store);
        self.ui.handle_event(cx, event, scope);
        self.match_event(cx, event);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut navigate_to_chat = false;
        let mut navigate_to_moly_server = false;
        let mut navigate_to_providers = false;
        let mut navigate_to_mcp = false;

        let radio_button_set;
        // Only show the MCP tab in native builds
        #[cfg(not(target_arch = "wasm32"))]
        {
            radio_button_set = self.ui.radio_button_set(
                cx,
                ids_array!(
                    sidebar_menu.chat_tab,
                    sidebar_menu.moly_server_tab,
                    sidebar_menu.mcp_tab,
                    sidebar_menu.providers_tab,
                ),
            );
            self.ui
                .view(cx, ids!(sidebar_menu.mcp_tab_container))
                .set_visible(cx, true);
        }

        #[cfg(target_arch = "wasm32")]
        {
            radio_button_set = self.ui.radio_button_set(
                cx,
                ids_array!(
                    sidebar_menu.chat_tab,
                    sidebar_menu.moly_server_tab,
                    sidebar_menu.providers_tab,
                ),
            );
            self.ui
                .view(cx, ids!(sidebar_menu.mcp_tab_container))
                .set_visible(cx, false);
        }

        if let Some(selected_tab) = radio_button_set.selected(cx, actions) {
            #[cfg(not(target_arch = "wasm32"))]
            match selected_tab {
                0 => navigate_to_chat = true,
                1 => navigate_to_moly_server = true,
                2 => navigate_to_mcp = true,
                3 => navigate_to_providers = true,
                _ => {}
            }

            #[cfg(target_arch = "wasm32")]
            match selected_tab {
                0 => navigate_to_chat = true,
                1 => navigate_to_moly_server = true,
                2 => navigate_to_providers = true,
                _ => {}
            }
        }

        for action in actions.iter() {
            if let MarkdownAction::LinkNavigated(url) = action.as_widget_action().cast() {
                let _ = robius_open::Uri::new(&url).open();
            }

            self.store.as_mut().unwrap().handle_action(action);

            if let Some(_) = action.downcast_ref::<DownloadFileAction>() {
                self.notify_downloaded_files(cx);
            }

            let store = self.store.as_mut().unwrap();

            match action.cast() {
                StoreAction::Search(keywords) => {
                    store.search.load_search_results(keywords);
                }
                StoreAction::ResetSearch => {
                    store.search.load_featured_models();
                }
                StoreAction::Sort(criteria) => {
                    store.search.sort_models(criteria);
                }
                _ => {}
            }

            match action.cast() {
                ModelFileItemAction::Download(file_id) => {
                    let (model, file) = store.get_model_and_file_download(&file_id);
                    store.downloads.download_file(model, file);
                    self.ui.redraw(cx);
                }
                _ => {}
            }

            match action.cast() {
                DownloadAction::Play(file_id) => {
                    let (model, file) = store.get_model_and_file_download(&file_id);
                    store.downloads.download_file(model, file);
                    self.ui.redraw(cx);
                }
                DownloadAction::Pause(file_id) => {
                    store.downloads.pause_download_file(&file_id);
                    self.ui.redraw(cx);
                }
                DownloadAction::Cancel(file_id) => {
                    store.downloads.cancel_download_file(&file_id);
                    self.ui.redraw(cx);
                }
                _ => {}
            }

            if let ChatAction::Start(_) = action.cast() {
                let chat_radio_button = self.ui.radio_button(cx, ids!(chat_tab));
                chat_radio_button.select(cx, &mut Scope::empty());
            }

            if let NavigationAction::NavigateToMyModels = action.cast() {
                let my_models_radio_button = self.ui.radio_button(cx, ids!(my_models_tab));
                my_models_radio_button.select(cx, &mut Scope::empty());
            }

            if let NavigationAction::NavigateToProviders = action.cast() {
                let providers_radio_button = self.ui.radio_button(cx, ids!(providers_tab));
                providers_radio_button.select(cx, &mut Scope::empty());
                navigate_to_providers = true;
            }

            store.handle_provider_connection_action(action.cast());
            // redraw the UI to reflect the connection status
            self.ui.redraw(cx);

            if matches!(
                action.cast(),
                DownloadNotificationPopupAction::ActionLinkClicked
                    | DownloadNotificationPopupAction::CloseButtonClicked
            ) {
                self.ui
                    .popup_notification(cx, ids!(download_popup))
                    .close(cx);
            }

            if let MolyClientAction::ServerUnreachable = action.cast() {
                self.ui
                    .popup_notification(cx, ids!(moly_server_popup))
                    .open(cx);
            }

            if let MolyServerPopupAction::CloseButtonClicked = action.cast() {
                self.ui
                    .popup_notification(cx, ids!(moly_server_popup))
                    .close(cx);
            }
        }

        // Handle navigation after processing all actions
        if navigate_to_providers {
            self.navigate_to(cx, ids!(application_pages.providers_frame));
        } else if navigate_to_chat {
            self.navigate_to(cx, ids!(application_pages.chat_frame));
        } else if navigate_to_moly_server {
            self.navigate_to(cx, ids!(application_pages.moly_server_frame));
        } else if navigate_to_mcp {
            self.navigate_to(cx, ids!(application_pages.mcp_frame));
        }
    }
}

impl App {
    fn notify_downloaded_files(&mut self, cx: &mut Cx) {
        let store = self.store.as_mut().unwrap();
        if let Some(notification) = store.downloads.next_download_notification() {
            let mut popup = self
                .ui
                .download_notification_popup(cx, ids!(popup_download_notification));

            match notification {
                DownloadPendingNotification::DownloadedFile(file) => {
                    popup.set_data(cx, &file, DownloadResult::Success);
                    cx.action(DeleteModelModalAction::ModelDeleted);
                }
                DownloadPendingNotification::DownloadErrored(file) => {
                    self.file_id = Some((file.id).clone());
                    self.start_retry_timeout(cx, popup, file);
                }
            }

            self.ui
                .popup_notification(cx, ids!(download_popup))
                .open(cx);
        }
    }

    fn start_retry_timeout(
        &mut self,
        cx: &mut Cx,
        mut popup: DownloadNotificationPopupRef,
        file: File,
    ) {
        match self.download_retry_attempts {
            0 => {
                self.timer = cx.start_timeout(15.0);
                self.download_retry_attempts += 1;
                popup.set_retry_data(cx);
            }
            1 => {
                self.timer = cx.start_timeout(30.0);
                self.download_retry_attempts += 1;
                popup.set_retry_data(cx);
            }
            2 => {
                self.timer = cx.start_timeout(60.0);
                self.download_retry_attempts += 1;
                popup.set_retry_data(cx);
            }
            _ => {
                popup.set_data(cx, &file, DownloadResult::Failure);
                self.download_retry_attempts = 0;
            }
        }
    }

    fn navigate_to(&mut self, cx: &mut Cx, id: &[LiveId]) {
        let providers_id = ids!(application_pages.providers_frame);
        let chat_id = ids!(application_pages.chat_frame);
        let moly_server_id = ids!(application_pages.moly_server_frame);
        let mcp_id = ids!(application_pages.mcp_frame);

        if id != providers_id {
            self.ui.widget(cx, providers_id).set_visible(cx, false);
        }

        if id != chat_id {
            self.ui.widget(cx, chat_id).set_visible(cx, false);
        }

        if id != moly_server_id {
            self.ui.widget(cx, moly_server_id).set_visible(cx, false);
        }

        if id != mcp_id {
            self.ui.widget(cx, mcp_id).set_visible(cx, false);
        }

        self.ui.widget(cx, id).set_visible(cx, true);
    }
}

#[derive(Clone, Default, Debug)]
pub enum NavigationAction {
    NavigateToProviders,
    NavigateToMyModels,
    #[default]
    None,
}

/// Workaround to switch between sync and async code in `Store`.
pub fn app_runner() -> UiRunner<App> {
    // `0` is reserved for whatever implements `AppMain`.
    UiRunner::new(0)
}

/// A wrapper around the main Moly view, used to prevent draw/events
/// from being propagated to all of Moly if the store is not loaded.
#[derive(Script, ScriptHook, Widget)]
pub struct MolyRoot {
    #[deref]
    view: View,
}

impl Widget for MolyRoot {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if scope.data.get::<Store>().is_none() {
            return DrawStep::done();
        }
        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if scope.data.get::<Store>().is_none() {
            return;
        }
        self.view.handle_event(cx, event, scope);
    }
}
