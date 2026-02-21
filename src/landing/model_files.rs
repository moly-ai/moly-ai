use crate::data::store::{ModelWithDownloadInfo, StoreAction};
use makepad_widgets::*;

use super::model_files_list::ModelFilesListWidgetExt;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_ADD = crate_resource("self://resources/icons/add.svg")
    let ICON_REMOVE = crate_resource("self://resources/icons/remove.svg")

    let ActionToggleButton = MolyRadioButtonTab {
        width: Fit
        height: 40
        padding: Inset {left: 20 top: 10 bottom: 10 right: 20}
        label_walk: { margin: 0 }
        draw_text: {
            text_style: theme.font_bold {font_size: 9}
            color_active: #475467
            color: #475467
            color_hover: #173437
        }
        draw_bg: {
            color: #D0D5DD
            color_active: #fff
            color_hover: #D0D5DD
            border_color_1: #D0D5DD
            border_size: 1.0
            border_radius: 7.0
        }
    }

    let ModelFilesActions = View {
        width: Fill
        height: Fit

        align: Align {y: 0.5}
        spacing: 10

        margin: Inset {bottom: 12}

        Label {
            draw_text: {
                text_style: theme.font_bold {font_size: 9}
                color: #667085
            }
            text: "SHOW"
        }

        tab_buttons := RoundedView {
            width: Fit
            height: Fit

            draw_bg: {
                color: #D0D5DD
                border_radius: 7.0
            }

            show_all_button := ActionToggleButton {
                animator: {selected: {default: @on}}
            }
            only_recommended_button := ActionToggleButton {}
        }
    }

    let ModelFilesHeader = ModelFilesRow {
        show_bg: true
        draw_bg: {
            color: #F2F4F7
            border_radius: vec2(3.0 0.5)
        }

        cell1: {
            height: 40
            Label {
                draw_text: {
                    text_style: theme.font_bold {font_size: 9}
                    color: #667085
                }
                text: "File name"
            }
        }

        cell2: {
            height: 40
            Label {
                draw_text: {
                    text_style: theme.font_bold {font_size: 9}
                    color: #667085
                }
                text: "Full Size"
            }
        }

        cell3: {
            height: 40
            Label {
                draw_text: {
                    text_style: theme.font_bold {font_size: 9}
                    color: #667085
                }
                text: "Quantization"
            }
        }
        cell4: {
            height: 40
        }
    }

    let FooterLink = View {
        cursor: MouseCursor.Hand
        align: Align {x: 0.0 y: 0.5}
        spacing: 10
        icon := Icon {
            draw_icon: {
                svg_file: (ICON_ADD)
                get_color: fn() -> vec4 {
                    return #667085
                }
            }
            icon_walk: {width: 14 height: 14}
        }
        link := Label {
            width: Fit
            draw_text: {
                text_style: theme.font_bold {font_size: 9}
                color: #667085
            }
        }
    }

    mod.widgets.ModelFiles = #(ModelFiles::register_widget(vm)) RoundedView {
        width: Fill
        height: Fit
        flow: Down

        model_files_actions := ModelFilesActions {}
        ModelFilesHeader {}
        ModelFilesList { show_featured: true }
        remaining_files_wrapper := View {
            width: Fill
            height: Fit
            remaining_files := ModelFilesList { show_featured: false }
        }

        show_all_animation_progress: 0.0
        animator: {
            show_all = {
                default: @hide
                show: AnimatorState {
                    redraw: true
                    from: {all: Forward {duration: 0.3}}
                    ease: ExpDecay {d1: 0.80 d2: 0.97}
                    apply: {show_all_animation_progress: 1.0}
                }
                hide: AnimatorState {
                    redraw: true
                    from: {all: Forward {duration: 0.3}}
                    ease: ExpDecay {d1: 0.80 d2: 0.97}
                    apply: {show_all_animation_progress: 0.0}
                }
            }
        }
    }
}

#[derive(Animator, Script, ScriptHook, Widget)]
pub struct ModelFiles {
    #[deref]
    view: View,

    #[live]
    show_all_animation_progress: f64,

    #[apply_default]
    animator: Animator,

    #[rust]
    actual_height: Option<f64>,
}

impl Widget for ModelFiles {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);

        if self.animator_handle_event(cx, event).must_redraw() {
            if let Some(total_height) = self.actual_height {
                let height = self.show_all_animation_progress * total_height;
                let wrapper = self.view(ids!(remaining_files_wrapper));
                script_apply_eval!(cx, wrapper, {height: #(height)});
                self.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let model = &scope.data.get::<ModelWithDownloadInfo>().unwrap();
        let files_count = model.files.len();
        let featured_count = model.files.iter().filter(|f| f.file.featured).count();

        let show_all_button = self.radio_button(ids!(tab_buttons.show_all_button));
        show_all_button.set_text(cx, &format!("All Files ({})", files_count));

        let show_all_button = self.radio_button(ids!(tab_buttons.only_recommended_button));
        show_all_button.set_text(cx, &format!("Only Recommended Files ({})", featured_count));

        let _ = self.view.draw_walk(cx, scope, walk);

        // Let's remember the actual rendered height of the remaining_files element.
        if self.actual_height.is_none() {
            self.actual_height = Some(self.model_files_list(ids!(remaining_files)).get_height(cx))
        }

        DrawStep::done()
    }
}

impl WidgetMatchEvent for ModelFiles {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let actions_tab_buttons =
            self.widget(ids!(model_files_actions))
                .radio_button_set(ids_array!(
                    tab_buttons.show_all_button,
                    tab_buttons.only_recommended_button,
                ));

        if let Some(index) = actions_tab_buttons.selected(cx, actions) {
            match index {
                0 => {
                    self.animator_play(cx, ids!(show_all.show));
                    self.redraw(cx);
                }
                1 => {
                    self.animator_play(cx, ids!(show_all.hide));
                    self.redraw(cx);
                }
                _ => {}
            }
        }

        for action in actions.iter() {
            match action.cast() {
                StoreAction::Search(_) | StoreAction::ResetSearch | StoreAction::Sort(_) => {
                    self.expand_without_animation(cx);
                    self.actual_height = None;
                    self.radio_button(ids!(show_all_button)).select(cx, scope);
                    self.redraw(cx);
                }
                _ => {}
            }
        }
    }
}

impl ModelFiles {
    fn expand_without_animation(&mut self, cx: &mut Cx) {
        let wrapper = self.view(ids!(remaining_files_wrapper));
        script_apply_eval!(cx, wrapper, {height: Fit});
        self.show_all_animation_progress = 0.0;
        self.redraw(cx);
    }
}
