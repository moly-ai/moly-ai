use super::deep_inquire_stages::StagesWidgetExt;
use crate::data::deep_inquire_client::{Data, StageType};
use makepad_widgets::*;
use moly_kit::prelude::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.DeepInquireContent = #(DeepInquireContent::register_widget(vm)) {
        flow: Down spacing: 10
        height: Fit
        Label {
            text: "Steps"
            draw_text +: {
                color: #x0
                text_style: theme.font_bold {font_size: 12}
            }
        }

        stages := Stages {}

        completed_block := View {
            width: Fill height: Fit
            padding: Inset {right: 18 top: 18 bottom: 14}
            completed_markdown := MessageMarkdown {}
        }
    }
}

#[derive(Widget, Script, ScriptHook)]
pub struct DeepInquireContent {
    #[deref]
    view: View,
}

impl Widget for DeepInquireContent {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl DeepInquireContent {
    pub(crate) fn set_content(&mut self, cx: &mut Cx, content: &MessageContent) {
        let data = content
            .data
            .as_deref()
            .expect("message without custom data should not reach here");

        let data = serde_json::from_str::<Data>(data)
            .expect("custom data without valid format should not reach here");

        let stages = data.stages.as_slice();

        let mut stages_ui = self.view.stages(cx, ids!(stages));
        stages_ui.update_stages(cx, stages);

        // Check if there is a completion block in any of the stages
        let completion_stage = stages
            .iter()
            .find(|stage| stage.stage_type == StageType::Completion);
        if let Some(stage) = completion_stage {
            let final_text = stage
                .substages
                .iter()
                .map(|s| s.text.clone())
                .collect::<String>();
            self.markdown(cx, ids!(completed_block.completed_markdown))
                .set_text(cx, &final_text);
        }
    }
}
