use std::collections::HashMap;

use makepad_widgets::*;

use crate::data::deep_inquire_client::{Stage, StageType, SubStage};
use moly_kit::prelude::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    // A workaround for RoundedShadowView having border_size defined as
    // a uniform, which breaks whenever updated through script_apply_eval.
    // This custom version uses instance fields instead.
    let CustomRoundedShadowView = ViewBase {
        clip_x: false
        clip_y: false

        show_bg: true
        draw_bg +: {
            color: #8
            border_radius: instance(2.5)
            border_size: instance(0.0)
            border_color: instance(#0000)
            shadow_color: instance(#0007)
            shadow_radius: instance(20.0)
            shadow_offset: instance(vec2(0 0))

            rect_size2: varying(vec2(0))
            rect_size3: varying(vec2(0))
            rect_pos2: varying(vec2(0))
            rect_shift: varying(vec2(0))
            sdf_rect_pos: varying(vec2(0))
            sdf_rect_size: varying(vec2(0))

            get_color: fn() -> vec4 {
                return self.color
            }

            vertex: fn() {
                let min_offset = min(self.shadow_offset vec2(0))
                self.rect_size2 = self.rect_size + 2.0*vec2(self.shadow_radius)
                self.rect_size3 = self.rect_size2 + abs(self.shadow_offset)
                self.rect_pos2 = self.rect_pos - vec2(self.shadow_radius) + min_offset
                self.sdf_rect_size = self.rect_size2 - vec2(self.shadow_radius * 2.0 + self.border_size * 2.0)
                self.sdf_rect_pos = -min_offset + vec2(self.border_size + self.shadow_radius)
                self.rect_shift = -min_offset

                return self.clip_and_transform_vertex(self.rect_pos2 self.rect_size3)
            }

            get_border_color: fn() -> vec4 {
                return self.border_color
            }

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size3)
                sdf.box(
                    self.sdf_rect_pos.x
                    self.sdf_rect_pos.y
                    self.sdf_rect_size.x
                    self.sdf_rect_size.y
                    self.border_radius
                )
                if sdf.shape > -1.0 {
                    let m = self.shadow_radius
                    let o = self.shadow_offset + self.rect_shift
                    let v = GaussShadow.rounded_box_shadow(vec2(m) + o self.rect_size2+o self.pos * (self.rect_size3+vec2(m)) self.shadow_radius*0.5 self.border_radius*2.0)
                    sdf.clear(self.shadow_color*v)
                }

                sdf.fill_keep(self.get_color())
                if self.border_size > 0.0 {
                    sdf.stroke(self.get_border_color() self.border_size)
                }
                return sdf.result
            }
        }
    }

    let StageBlockBase = View {
        padding: Inset {left: 30}
        margin: Inset {left: 30}
        width: Fill height: 20
        show_bg: true
        draw_bg +: {
            color: #xf9f9f9
            left_border_color: instance(#xeaeaea)
            left_border_width: instance(3.0)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                sdf.rect(0. 0. self.rect_size.x self.rect_size.y)
                sdf.fill(self.color)

                sdf.rect(0. 0. self.left_border_width self.rect_size.y)
                sdf.fill(self.left_border_color)

                return sdf.result
            }
        }
    }

    let SubStage = StageBlockBase {
        width: Fill height: Fit
        padding: Inset {left: 30}
        margin: Inset {left: 30}

        flow: Down
        spacing: 10
        content_heading_label := Label {
            width: Fill
            draw_text +: {
                wrap: Word
                text_style: theme.font_bold {font_size: 11}
                color: #x003E62
            }
        }
        content_block_markdown := MessageMarkdown {}
    }

    mod.widgets.SubStages = #(SubStages::register_widget(vm)) {
        flow: Down
        width: Fill height: Fit
        spacing: 20

        substage_template := SubStage {}
    }

    mod.widgets.StageView = #(StageView::register_widget(vm)) {
        visible: false
        width: Fill height: Fit
        wrapper := View {
            width: Fill height: Fit
            cursor: MouseCursor.Hand
            flow: Down
            align: Align {x: 0 y: 0.5}
            header := View {
                width: Fill height: Fit
                spacing: 10
                align: Align {x: 0 y: 0.5}
                padding: 10

                stage_toggle := CustomRoundedShadowView {
                    width: 45 height: 45
                    padding: 4
                    draw_bg +: {
                        color: #xf9f9f9
                        border_radius: instance(11.0)
                        shadow_color: uniform(#x0001)
                        shadow_radius: instance(8.0)
                        shadow_offset: instance(vec2(0.0 f32(-2.0)))
                        border_size: instance(0.0)
                        border_color: instance(#x1A2533)
                    }
                    align: Align {x: 0.5 y: 0.5}

                    stage_bubble_text := Label {
                        text: "1"
                        draw_text +: {
                            text_style: theme.font_bold {font_size: 14}
                            color: #000
                        }
                    }
                }
                stage_title := Label {
                    draw_text +: {
                        text_style: theme.font_bold {font_size: 11}
                        color: #000
                    }
                }
            }
            stage_content_preview := StageBlockBase {
                padding: Inset {left: 30}
                margin: Inset {left: 30}
                width: Fill height: 20

                stage_preview_label := Label {
                    width: Fill
                    draw_text +: {
                        wrap: Word
                        text_style +: {font_size: 11}
                        color: #x0
                    }
                }
            }

            expanded_stage_content := View {
                visible: false
                flow: Down
                spacing: 25
                height: Fit
                citations_view := StageBlockBase {
                    visible: false
                    height: Fit
                    flow: Down spacing: 10
                    Label {
                        draw_text +: {
                            color: #x003E62
                            text_style: theme.font_bold {font_size: 11}
                        }
                        text: "Sources"
                    }
                    citations_list := CitationList {}
                }
                substages := SubStages {}
            }
        }

        animator: Animator {
            streaming: {
                default: @off
                off: AnimatorState {
                    from: {all: Snap}
                    apply: {
                        wrapper: {
                            header: {
                                stage_toggle: { draw_bg +: {
                                    shadow_offset: vec2(0.0 f32(-2.0))
                                    shadow_color: #x0001
                                } }
                            }
                        }
                    }
                }
                move_up: AnimatorState {
                    redraw: true
                    from: {all: Forward {duration: 0.4}}
                    apply: {
                        wrapper: {
                            header: {
                                stage_toggle: { draw_bg +: {
                                    shadow_offset: vec2(0.0 f32(-4.0))
                                    shadow_color: #x0002
                                } }
                            }
                        }
                    }
                }
                move_right: AnimatorState {
                    redraw: true
                    from: {all: Forward {duration: 0.4}}
                    apply: {
                        wrapper: {
                            header: {
                                stage_toggle: { draw_bg +: {
                                    shadow_offset: vec2(3.0 f32(-2.0))
                                    shadow_color: #x0002
                                } }
                            }
                        }
                    }
                }
                move_down: AnimatorState {
                    redraw: true
                    from: {all: Forward {duration: 0.4}}
                    apply: {
                        wrapper: {
                            header: {
                                stage_toggle: { draw_bg +: {
                                    shadow_offset: vec2(0.0 1.0)
                                    shadow_color: #x0002
                                } }
                            }
                        }
                    }
                }
                move_left: AnimatorState {
                    redraw: true
                    from: {all: Forward {duration: 0.4}}
                    apply: {
                        wrapper: {
                            header: {
                                stage_toggle: { draw_bg +: {
                                    shadow_offset: vec2(f32(-3.0) f32(-2.0))
                                    shadow_color: #x0002
                                } }
                            }
                        }
                    }
                }
            }
        }
    }

    mod.widgets.Stages = #(Stages::register_widget(vm)) {
        flow: Down
        visible: false
        width: Fill height: Fit

        thinking_stage := StageView {
            stage_type: Thinking
            wrapper: {
                header: {
                    stage_title: { text: "Thinking" }
                    stage_toggle: {
                        stage_bubble_text: { text: "🧠" }
                    }
                }
            }
        }

        content_stage := StageView {
            stage_type: Content
            wrapper: {
                header: {
                    stage_title: { text: "Detailed Anaylsis" }
                    stage_toggle: {
                        stage_bubble_text: { text: "🔬" }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct Stages {
    #[deref]
    view: View,

    #[rust]
    stage_ids: Vec<String>,
}

impl Widget for Stages {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for Stages {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        for action in actions {
            if let StageViewAction::StageViewClicked(clicked_stage) = action.cast() {
                match clicked_stage {
                    StageType::Thinking => {
                        self.stage_view(cx, ids!(content_stage))
                            .set_active(cx, false);
                    }
                    StageType::Content => {
                        self.stage_view(cx, ids!(thinking_stage))
                            .set_active(cx, false);
                    }
                    _ => {}
                }
                self.redraw(cx);
            }
        }
    }
}

impl Stages {
    fn update_stages(&mut self, cx: &mut Cx, stages: &[Stage]) {
        self.visible = true;
        self.stage_ids = stages.iter().map(|stage| stage.id.clone()).collect();

        let has_content_stage = stages.iter().any(|s| s.stage_type == StageType::Content);
        let has_completion_stage = stages.iter().any(|s| s.stage_type == StageType::Completion);

        for stage in stages.iter() {
            match stage.stage_type {
                StageType::Thinking => {
                    let mut thinking_stage = self.stage_view(cx, ids!(thinking_stage));
                    thinking_stage.set_stage(cx, stage);
                    thinking_stage.set_streaming_state(cx, !has_content_stage);
                }
                StageType::Content => {
                    let mut content_stage = self.stage_view(cx, ids!(content_stage));
                    content_stage.set_stage(cx, stage);
                    content_stage.set_streaming_state(cx, !has_completion_stage);
                }
                _ => {}
            }
        }

        self.redraw(cx);
    }
}

impl StagesRef {
    pub fn update_stages(&mut self, cx: &mut Cx, stages: &[Stage]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.update_stages(cx, stages);
        }
    }
}

#[derive(Script, Widget, Animator)]
pub struct StageView {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,

    #[apply_default]
    animator: Animator,

    #[rust]
    timer: Timer,

    #[rust]
    id: String,

    #[rust]
    stage_type: StageType,

    #[rust]
    is_active: bool,

    #[rust]
    is_streaming: bool,
}

impl ScriptHook for StageView {}

impl Widget for StageView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.timer.is_event(event).is_some() {
            if self.is_streaming {
                if self.animator_in_state(cx, ids!(streaming.move_up)) {
                    self.animator_play(cx, ids!(streaming.move_right));
                } else if self.animator_in_state(cx, ids!(streaming.move_right)) {
                    self.animator_play(cx, ids!(streaming.move_down));
                } else if self.animator_in_state(cx, ids!(streaming.move_down)) {
                    self.animator_play(cx, ids!(streaming.move_left));
                } else {
                    self.animator_play(cx, ids!(streaming.move_up));
                }
                self.timer = cx.start_timeout(0.4);
            } else {
                self.animator_cut(cx, ids!(streaming.off));
                if !self.timer.is_empty() {
                    cx.stop_timer(self.timer);
                    self.timer = Timer::empty();
                }
            }
        }

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.is_active {
            let mut toggle = self.view(cx, ids!(stage_toggle));
            script_apply_eval!(cx, toggle, {
                draw_bg: { border_size: 1 }
            });
        } else {
            let mut toggle = self.view(cx, ids!(stage_toggle));
            script_apply_eval!(cx, toggle, {
                draw_bg: { border_size: 0 }
            });
        }

        self.view(cx, ids!(expanded_stage_content))
            .set_visible(cx, self.is_active);
        self.view(cx, ids!(stage_content_preview))
            .set_visible(cx, !self.is_active);

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for StageView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.view(cx, ids!(wrapper)).finger_down(actions).is_some() {
            self.is_active = !self.is_active;
            cx.action(StageViewAction::StageViewClicked(self.stage_type.clone()));
            self.redraw(cx);
        }
    }
}

impl StageView {
    fn set_stage(&mut self, cx: &mut Cx, stage: &Stage) {
        self.id = stage.id.clone();
        self.stage_type = stage.stage_type.clone();
        self.visible = true;

        self.sub_stages(cx, ids!(substages))
            .set_substages(cx, &stage.substages);

        if !stage.citations.is_empty() {
            self.view(cx, ids!(citations_view)).set_visible(cx, true);
            let citations = self.citation_list(cx, ids!(citations_list));
            let mut citations = citations.borrow_mut().unwrap();
            citations.urls = stage.citations.iter().map(|a| a.url.clone()).collect();
        } else {
            self.view(cx, ids!(citations_view)).set_visible(cx, false);
        }

        let stage_preview_text: Option<String> = stage.substages.first().and_then(|substage| {
            let cleaned_text = substage
                .text
                .replace('*', "")
                .replace('_', "")
                .replace('#', "")
                .replace('`', "")
                .replace('[', "")
                .replace(']', "")
                .replace('(', "")
                .replace(')', "")
                .replace('>', "");

            let words: Vec<&str> = cleaned_text.split_whitespace().collect();
            if words.len() > 10 {
                Some(words[0..10].join(" "))
            } else {
                Some(cleaned_text)
            }
        });

        if let Some(stage_preview_text) = stage_preview_text {
            self.label(cx, ids!(stage_preview_label))
                .set_text(cx, &format!("{}...", stage_preview_text));
        } else {
            self.label(cx, ids!(stage_preview_label))
                .set_text(cx, "Loading...");
        }

        self.redraw(cx);
    }

    fn set_streaming_state(&mut self, cx: &mut Cx, is_streaming: bool) {
        if is_streaming == self.is_streaming {
            return;
        }
        self.is_streaming = is_streaming;

        if self.is_streaming {
            if self.timer.is_empty() {
                self.animator_play(cx, ids!(streaming.move_up));
                self.timer = cx.start_timeout(0.01);
            }
        } else {
            self.animator_cut(cx, ids!(streaming.off));
            if !self.timer.is_empty() {
                cx.stop_timer(self.timer);
                self.timer = Timer::empty();
            }
        }
        self.redraw(cx);
    }
}

impl StageViewRef {
    pub fn set_stage(&mut self, cx: &mut Cx, stage: &Stage) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_stage(cx, stage);
        }
    }

    pub fn set_active(&mut self, cx: &mut Cx, is_active: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.is_active = is_active;
            inner.redraw(cx);
        }
    }

    pub fn set_streaming_state(&mut self, cx: &mut Cx, is_streaming: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_streaming_state(cx, is_streaming);
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum StageViewAction {
    #[default]
    None,
    StageViewClicked(StageType),
}

#[derive(Widget, Script)]
pub struct SubStages {
    #[deref]
    view: View,

    #[live]
    substage_template: Option<ScriptObjectRef>,

    #[rust]
    substage_ids: Vec<String>,

    #[rust]
    substage_views: HashMap<String, View>,
}

impl ScriptHook for SubStages {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        value: ScriptValue,
    ) {
        if let Some(obj) = value.as_object() {
            vm.vec_with(obj, |vm, vec| {
                for kv in vec {
                    if let Some(id) = kv.key.as_id() {
                        if id == id!(substage_template) {
                            if let Some(template_obj) = kv.value.as_object() {
                                self.substage_template =
                                    Some(vm.bx.heap.new_object_ref(template_obj));
                            }
                        }
                    }
                }
            });
        }
    }
}

impl Widget for SubStages {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        cx.begin_turtle(walk, self.layout);

        for stage_id in self.substage_ids.iter() {
            if let Some(substage_view) = self.substage_views.get_mut(stage_id) {
                let _ = substage_view.draw(cx, scope);
            }
        }

        cx.end_turtle();
        DrawStep::done()
    }
}

impl SubStages {
    pub fn update_substages(&mut self, cx: &mut Cx, substages: &[SubStage]) {
        self.substage_ids = substages
            .iter()
            .map(|substage| substage.id.clone())
            .collect();
        self.visible = true;
        for substage in substages.iter() {
            let substage_view =
                if let Some(substage_view) = self.substage_views.get_mut(&substage.id) {
                    substage_view
                } else {
                    let substage_view = {
                        let template = self.substage_template.clone();
                        cx.with_vm(|vm| {
                            let obj = template.as_object().expect("substage_template not set");
                            View::script_from_value(vm, obj.into())
                        })
                    };
                    self.substage_views
                        .insert(substage.id.clone(), substage_view);
                    self.substage_views.get_mut(&substage.id).unwrap()
                };

            substage_view
                .label(cx, ids!(content_heading_label))
                .set_text(cx, &get_human_readable_stage_name(&substage.name));
            substage_view
                .view(cx, ids!(citations_view))
                .set_visible(cx, false);
            substage_view
                .markdown(cx, ids!(content_block_markdown))
                .set_text(cx, &substage.text);
        }
    }
}

impl SubStagesRef {
    pub fn set_substages(&mut self, cx: &mut Cx, substages: &[SubStage]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.update_substages(cx, substages);
        }
    }
}

/// Replaces underscores with spaces and capitalizes the first letter
/// of each word.
pub fn get_human_readable_stage_name(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
