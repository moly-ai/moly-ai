use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    use moly_widgets::theme::*;
    use moly_widgets::button::*;
    use crate::button_showcase::*;
    use crate::card_showcase::*;
    use crate::switch_showcase::*;
    use crate::text_input_showcase::*;
    use crate::theme_showcase::*;

    SidebarButton = <FilledNeutralButton> {
        width: Fill,
    }

    pub Ui = {{Ui}} {
        width: Fill,
        height: Fill,
        flow: Right,
        show_bg: true,
        draw_bg: { color: (COLOR_BG) }

        // ── Sidebar ─────────────────────────────────────────────
        <View> {
            width: 180,
            height: Fill,
            flow: Down,
            spacing: 8,
            padding: 12,
            show_bg: true,
            draw_bg: { color: (COLOR_SURFACE) }

            <Label> {
                width: Fill,
                padding: { bottom: 8 }
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 12 }
                    color: (COLOR_TEXT)
                }
                text: "Moly Zoo"
            }

            theme_button = <SidebarButton> { text: "Theme" }
            buttons_button = <SidebarButton> { text: "Buttons" }
            cards_button = <SidebarButton> { text: "Cards" }
            switches_button = <SidebarButton> { text: "Switches" }
            text_inputs_button = <SidebarButton> { text: "Text Inputs" }
        }

        // ── Content ─────────────────────────────────────────────
        <View> {
            width: Fill,
            height: Fill,

            theme_view = <ScrollYView> {
                width: Fill,
                height: Fill,
                visible: true,
                <ThemeShowcase> {}
            }

            buttons_view = <ScrollYView> {
                width: Fill,
                height: Fill,
                visible: false,
                <ButtonShowcase> {}
            }

            cards_view = <ScrollYView> {
                width: Fill,
                height: Fill,
                visible: false,
                <CardShowcase> {}
            }

            switches_view = <ScrollYView> {
                width: Fill,
                height: Fill,
                visible: false,
                <SwitchShowcase> {}
            }

            text_inputs_view = <ScrollYView> {
                width: Fill,
                height: Fill,
                visible: false,
                <TextInputShowcase> {}
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct Ui {
    #[deref]
    deref: View,
}

impl LiveHook for Ui {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.widget(ids!(disabled_primary)).set_disabled(cx, true);
        self.widget(ids!(disabled_neutral)).set_disabled(cx, true);
        self.widget(ids!(disabled_danger)).set_disabled(cx, true);
        self.widget(ids!(disabled_warning)).set_disabled(cx, true);
        self.widget(ids!(disabled_transparent))
            .set_disabled(cx, true);
    }
}

impl Widget for Ui {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }
}

impl WidgetMatchEvent for Ui {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(ids!(theme_button)).clicked(actions) {
            self.view(ids!(theme_view)).set_visible(cx, true);
            self.view(ids!(buttons_view)).set_visible(cx, false);
            self.view(ids!(cards_view)).set_visible(cx, false);
            self.view(ids!(switches_view)).set_visible(cx, false);
            self.view(ids!(text_inputs_view)).set_visible(cx, false);
            self.redraw(cx);
        }

        if self.button(ids!(buttons_button)).clicked(actions) {
            self.view(ids!(theme_view)).set_visible(cx, false);
            self.view(ids!(buttons_view)).set_visible(cx, true);
            self.view(ids!(cards_view)).set_visible(cx, false);
            self.view(ids!(switches_view)).set_visible(cx, false);
            self.view(ids!(text_inputs_view)).set_visible(cx, false);
            self.redraw(cx);
        }

        if self.button(ids!(cards_button)).clicked(actions) {
            self.view(ids!(theme_view)).set_visible(cx, false);
            self.view(ids!(buttons_view)).set_visible(cx, false);
            self.view(ids!(cards_view)).set_visible(cx, true);
            self.view(ids!(switches_view)).set_visible(cx, false);
            self.view(ids!(text_inputs_view)).set_visible(cx, false);
            self.redraw(cx);
        }

        if self.button(ids!(switches_button)).clicked(actions) {
            self.view(ids!(theme_view)).set_visible(cx, false);
            self.view(ids!(buttons_view)).set_visible(cx, false);
            self.view(ids!(cards_view)).set_visible(cx, false);
            self.view(ids!(switches_view)).set_visible(cx, true);
            self.view(ids!(text_inputs_view)).set_visible(cx, false);
            self.redraw(cx);
        }

        if self.button(ids!(text_inputs_button)).clicked(actions) {
            self.view(ids!(theme_view)).set_visible(cx, false);
            self.view(ids!(buttons_view)).set_visible(cx, false);
            self.view(ids!(cards_view)).set_visible(cx, false);
            self.view(ids!(switches_view)).set_visible(cx, false);
            self.view(ids!(text_inputs_view)).set_visible(cx, true);
            self.redraw(cx);
        }
    }
}
