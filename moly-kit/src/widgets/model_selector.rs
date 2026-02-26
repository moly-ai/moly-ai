use makepad_widgets::*;
use std::sync::{Arc, Mutex};

use crate::{
    aitk::{
        controllers::chat::{ChatController, ChatStateMutation},
        protocol::*,
    },
    utils::makepad::events::EventExt,
    widgets::{
        model_selector_item::ModelSelectorItemAction,
        model_selector_list::ModelSelectorList,
        moly_modal::MolyModalWidgetExt,
    },
};

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    let ICON_DROP = crate_resource("self://resources/drop_icon.png")

    let ModelSelectorButton = Button {
        width: Fit
        height: Fit
        padding: Inset{left: 8 right: 8 top: 6 bottom: 6}

        draw_bg: {
            color_down: #x0000
            radius: 7.
            border_size: 0.
            color_hover: #xf2
        }

        draw_text: {
            text_style +: theme.font_regular {
                font_size: 11.
            }
            color: #x222
            color_hover: #x111
            color_focus: #x111
            color_down: #x000
        }
    }

    let ModelSelectorOptions = RoundedShadowView {
        width: Fill height: Fit
        padding: 8
        flow: Down
        spacing: 8

        show_bg: true
        draw_bg: {
            color: #xf9
            radius: 6.0
            shadow_color: uniform(#x0002)
            shadow_radius: 9.0
            shadow_offset: vec2(0.0 -2.0)
        }

        search_container := RoundedView {
            width: Fill height: Fit
            show_bg: true
            padding: Inset{top: 4 bottom: 4 left: 8 right: 8}
            spacing: 8
            align: Align{x: 0.0 y: 0.5}
            draw_bg: {
                radius: 6.0
                border_color: #xD0D5DD
                border_size: 1.0
                color: #xfff
            }

            search_input := TextInput {
                width: Fill height: Fit
                draw_bg: {
                    pixel: fn() {
                        return vec4(0.)
                    }
                }
                draw_text: {
                    text_style +: theme.font_regular {font_size: 11}
                    color: #x000
                    color_hover: #x98A2B3
                    color_focus: #x000
                    color_empty: #x98A2B3
                    color_empty_focus: #x98A2B3
                    color_empty_hover: #x98A2B3
                }
                draw_cursor: {
                    color: #x000
                }
                empty_text: "Search models"
            }
        }

        list_container := ScrollYView {
            width: Fill
            height: 200
            scroll_bars: {
                scroll_bar_y: {
                    drag_scrolling: true
                    draw_bg: {
                        color: #xD9
                        color_hover: #x888
                        color_drag: #x777
                    }
                }
            }

            list := ModelSelectorList {}
        }
    }

    mod.widgets.ModelSelectorBase =
        #(ModelSelector::register_widget(vm))

    mod.widgets.ModelSelector =
        set_type_default() do mod.widgets.ModelSelectorBase View {
        width: Fit height: Fit
        flow: Overlay

        button := ModelSelectorButton {
            text: "Loading model..."
        }

        modal := MolyModal {
            dismiss_on_focus_lost: true
            bg_view: {
                visible: false
            }
            align: Align{x: 0.0 y: 0.0}

            content: View {
                width: 400
                height: Fit
                padding: Inset{
                    top: 20 left: 10 right: 10 bottom: 20
                }
                options := ModelSelectorOptions {}
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ModelSelector {
    #[deref]
    view: View,

    #[rust]
    pub chat_controller: Option<Arc<Mutex<ChatController>>>,

    #[rust]
    pub open: bool,
}

impl Widget for ModelSelector {
    fn handle_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
    ) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);

        if self.button(cx, ids!(button)).clicked(event.actions()) {
            if !self.open {
                self.open_modal(cx);
            } else {
                self.close_modal(cx);
            }
        }

        if self.moly_modal(cx, ids!(modal)).dismissed(event.actions()) {
            self.close_modal(cx);
            self.clear_search(cx);
            self.button(cx, ids!(button)).reset_hover(cx);
        }

        if self.open && !cx.display_context.is_desktop() {
            if let Hit::FingerUp(fe) = event
                .hits(cx, self.view(cx, ids!(modal.bg_view)).area())
            {
                if fe.was_tap() {
                    self.close_modal(cx);
                    self.clear_search(cx);
                    self.button(cx, ids!(button)).reset_hover(cx);
                }
            }
        }
    }

    fn draw_walk(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        walk: Walk,
    ) -> DrawStep {
        let (bots, selected_bot_id) =
            if let Some(chat_controller) = &self.chat_controller {
                let state =
                    chat_controller.lock().unwrap().state().clone();
                (state.bots, state.bot_id)
            } else {
                (Vec::new(), None)
            };

        if bots.is_empty() {
            self.button(cx, ids!(button))
                .set_text(cx, "No models available");
            self.button(cx, ids!(button)).set_enabled(cx, false);
        } else {
            self.button(cx, ids!(button)).set_enabled(cx, true);

            if let Some(bot_id) = &selected_bot_id {
                if let Some(bot) =
                    bots.iter().find(|b| &b.id == bot_id)
                {
                    self.button(cx, ids!(button))
                        .set_text(cx, &bot.name);
                } else {
                    self.button(cx, ids!(button))
                        .set_text(cx, "Choose an AI assistant");
                }
            } else {
                self.button(cx, ids!(button))
                    .set_text(cx, "Choose an AI assistant");
            }
        }

        if let Some(controller) = &self.chat_controller
            && let Some(mut list) = self
                .widget(ids!(options.list_container.list))
                .borrow_mut::<ModelSelectorList>()
            && Arc::as_ptr(controller)
                != list
                    .chat_controller
                    .as_ref()
                    .map(Arc::as_ptr)
                    .unwrap_or(std::ptr::null())
        {
            list.chat_controller = Some(controller.clone());
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ModelSelector {
    fn handle_actions(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        _scope: &mut Scope,
    ) {
        if let Some(text) = self
            .text_input(cx, ids!(options.search_container.search_input))
            .changed(actions)
        {
            if let Some(mut list) = self
                .widget(ids!(options.list_container.list))
                .borrow_mut::<ModelSelectorList>()
            {
                list.search_filter = text;
                list.items.clear();
                list.total_height = None;
            }
        }

        let list_widget =
            self.widget(ids!(options.list_container.list));
        for action in actions {
            let Some(action) = action.as_widget_action() else {
                continue;
            };

            if action.widget_uid != list_widget.widget_uid() {
                continue;
            }

            match action.cast() {
                ModelSelectorItemAction::BotSelected(bot_id) => {
                    if let Some(controller) =
                        &self.chat_controller
                    {
                        controller
                            .lock()
                            .unwrap()
                            .dispatch_mutation(
                                ChatStateMutation::SetBotId(
                                    Some(bot_id),
                                ),
                            );
                    }

                    self.button(cx, ids!(button)).reset_hover(cx);
                    self.close_modal(cx);
                    self.clear_search(cx);
                    self.redraw(cx);
                }
                _ => {}
            }
        }
    }
}

impl ModelSelector {
    fn open_modal(&mut self, cx: &mut Cx) {
        self.open = true;

        let button_rect =
            self.button(cx, ids!(button)).area().rect(cx);

        const LIST_HEIGHT: f64 = 200.0;
        const SEARCH_HEIGHT: f64 = 40.0;
        const PADDING_HEIGHT: f64 = 68.0;
        const MODAL_CONTENT_HEIGHT: f64 =
            LIST_HEIGHT + SEARCH_HEIGHT + PADDING_HEIGHT;
        const GAP: f64 = 25.0;

        let modal_x;
        let modal_y;
        let mut bg_view_visible = false;

        if cx.display_context.is_desktop() {
            modal_x = button_rect.pos.x - GAP;
            modal_y = button_rect.pos.y
                - MODAL_CONTENT_HEIGHT
                - GAP
                - 5.0;
        } else {
            modal_x = 0.0;
            modal_y = cx.display_context.screen_size.y
                - MODAL_CONTENT_HEIGHT
                - 5.0;
            bg_view_visible = true;
        }

        let modal = self.moly_modal(cx, ids!(modal));

        let bg_visible = bg_view_visible;
        let margin = Inset::new(
            modal_y as f32,
            0.0,
            0.0,
            modal_x as f32,
        );
        script_apply_eval!(cx, modal, {
            bg_view: {
                visible: #(bg_visible)
            }
            content: {
                margin: #(margin)
            }
        });

        if !cx.display_context.is_desktop() {
            script_apply_eval!(cx, modal, {
                dismiss_on_focus_lost: false
                content: {
                    width: Fill
                    padding: 0
                }
            });
        } else {
            script_apply_eval!(cx, modal, {
                content: {
                    width: 400
                    padding: Inset{
                        top: 20 left: 10 right: 10 bottom: 20
                    }
                }
            });
        }

        modal.open(cx);
    }

    fn close_modal(&mut self, cx: &mut Cx) {
        self.open = false;
        self.moly_modal(cx, ids!(modal)).close(cx);
    }

    fn clear_search(&mut self, cx: &mut Cx) {
        if let Some(mut list) = self
            .widget(ids!(options.list_container.list))
            .borrow_mut::<ModelSelectorList>()
        {
            list.search_filter.clear();
            list.items.clear();
            list.total_height = None;
        }
        self.text_input(cx, ids!(options.search_container.search_input))
            .set_text(cx, "");
        self.redraw(cx);
    }
}

impl ModelSelectorRef {
    /// Sets the chat controller for populating the bot list.
    pub fn set_chat_controller(
        &mut self,
        controller: Option<Arc<Mutex<ChatController>>>,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.chat_controller = controller;
        }
    }

    /// Sets a custom grouping function for organizing bots in the list.
    ///
    /// By default, bots are grouped by their provider (extracted from
    /// BotId). Applications can provide a custom grouping function to add
    /// provider icons, custom display names, or different grouping logic.
    pub fn set_grouping<F>(&mut self, grouping: F)
    where
        F: Fn(&Bot) -> BotGroup + 'static,
    {
        if let Some(inner) = self.borrow_mut() {
            if let Some(mut list) = inner
                .widget(ids!(options.list_container.list))
                .borrow_mut::<ModelSelectorList>()
            {
                list.grouping = Box::new(grouping);
            }
        }
    }
}

/// Default grouping: groups all bots under "All" category.
pub fn default_grouping(bot: &Bot) -> BotGroup {
    BotGroup {
        id: "all".to_string(),
        label: "All".to_string(),
        icon: Some(bot.avatar.clone()),
    }
}

/// Defines how a bot should be grouped in the model selector.
///
/// This struct is returned by the grouping function to specify:
/// - A unique group identifier for deduplication and sorting
/// - A display label shown in the group header
/// - An optional icon displayed next to the group label
#[derive(Clone, Debug)]
pub struct BotGroup {
    /// Unique identifier for the group (used for deduplication and sorting)
    pub id: String,
    /// Display name shown in the group header
    pub label: String,
    /// Optional icon displayed next to the group label
    pub icon: Option<EntityAvatar>,
}
