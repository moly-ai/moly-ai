use crate::aitk::utils::asynchronous::{AbortOnDropHandle, abort_on_drop};
use crate::utils::makepad::events::EventExt;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::shared::widgets::*;

    pub SttInput = {{SttInput}} {
        flow: Right,
        cancel = <Button> { text: "Cancel" }
        <HorizontalFiller> {}
        status = <Label> {}
        <HorizontalFiller> {}
        confirm = <Button> { text: "Confirm" }
    }
}

#[derive(Clone, Debug, Default)]
struct AudioData {
    pub data: Vec<f32>,
    pub sample_rate: Option<f64>,
}

#[derive(Clone, Debug, DefaultNone)]
enum SttInputAction {
    Transcribed(String),
    None,
}

#[derive(PartialEq, Clone, Debug, Default)]
enum SttInputState {
    #[default]
    Idle,
    Recording(RecordingState),
    Sending,
}

#[derive(PartialEq, Clone, Debug)]
struct RecordingState {
    start_time: f64,
}

#[derive(Live, Widget, LiveHook)]
pub struct SttInput {
    #[deref]
    deref: View,

    #[rust]
    state: SttInputState,
}

impl Widget for SttInput {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope);

        if self.button(ids!(confirm)).clicked(event.actions()) {
            self.finish_recording(cx);
        }

        if self.button(ids!(cancel)).clicked(event.actions()) {
            self.cancel_recording(cx);
        }
    }
}

impl SttInput {
    pub fn start_recording(&mut self, cx: &mut Cx) {
        // Start recording logic here
    }

    pub fn finish_recording(&mut self, cx: &mut Cx) {
        // Stop recording logic here
    }

    pub fn cancel_recording(&mut self, cx: &mut Cx) {
        // Cancel recording logic here
    }

    pub fn recorded<'a>(&self, actions: &'a Actions) -> Option<&'a str> {
        actions
            .find_widget_action(self.widget_uid())
            .and_then(|widget_action| widget_action.downcast_ref::<SttInputAction>())
            .and_then(|action| match action {
                SttInputAction::Transcribed(text) => Some(text.as_str()),
                SttInputAction::None => None,
            })
    }
}
