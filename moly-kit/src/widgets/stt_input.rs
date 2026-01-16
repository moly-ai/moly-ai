use crate::aitk::protocol::{Attachment, BotClient, BotId, EntityId, Message, MessageContent};
use crate::aitk::utils::asynchronous::{AbortOnDropHandle, spawn_abort_on_drop};
use crate::utils::makepad::events::EventExt;
use makepad_widgets::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SttUtility {
    pub client: Box<dyn BotClient>,
    pub bot_id: BotId,
}

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::shared::widgets::*;

    HorizontalFiller = <View>{width: Fill, height: 0}

    IconButton = <Button> {
        width: Fit, height: Fit
        padding: {top: 6, bottom: 6, left: 8, right: 8}
        draw_text: {
            text_style: <THEME_FONT_ICONS> {
                font_size: 13.
            }
        }
        draw_bg: {
            border_radius: 8.
            border_size: 0.
        }
    }

    pub SttInput = {{SttInput}} <RoundedView> {
        flow: Right,
        height: 50,
        align: {y: 0.5},
        spacing: 10,
        padding: 10,
        draw_bg: {
            border_radius: 12,
            border_color: #8888,
            border_size: 1.0,
        }

        cancel = <IconButton> {
            text: "", // fa-xmark, unicode f00d
            draw_text: {
                color: #000,
                color_hover: #000,
                color_down: #000,
                color_focus: #000,
            }
            draw_bg: {
                color: #fff
                color_hover: #fff
                color_down: #fff
                color_focus: #fff
            }
        }
        <HorizontalFiller> {}
        status = <Label> { text: "Recording...", draw_text: { color: #000, text_style: {font_size: 11}  } }
        <HorizontalFiller> {}
        confirm = <IconButton> {
            text: "", // fa-check, unicode f00c
            draw_text: {
                color: #fff,
                color_hover: #fff,
                color_down: #fff,
                color_focus: #fff,
            }
            draw_bg: {
                color: #000
                color_hover: #000
                color_down: #000
                color_focus: #000
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct AudioData {
    pub data: Vec<f32>,
    pub sample_rate: Option<f64>,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum SttInputAction {
    Transcribed(String),
    Cancelled,
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

const TIMER_PRECISION: f64 = 0.1;

#[derive(Live, Widget, LiveHook)]
pub struct SttInput {
    #[deref]
    deref: View,

    #[rust]
    state: SttInputState,

    #[rust]
    pub stt_utility: Option<SttUtility>,

    #[rust]
    audio_buffer: Option<Arc<Mutex<AudioData>>>,

    #[rust]
    abort_handle: Option<AbortOnDropHandle>,

    #[rust]
    timer: Timer,
}

impl Widget for SttInput {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.ui_runner().handle(cx, event, scope, self);
        self.deref.handle_event(cx, event, scope);

        if self.timer.is_event(event).is_some() {
            if let SttInputState::Recording(recording_state) = &self.state {
                let elapsed = Cx::time_now() - recording_state.start_time;
                self.label(ids!(status))
                    .set_text(cx, &time_to_minutes_seconds(elapsed));
                self.timer = cx.start_timeout(TIMER_PRECISION);
            }
        }

        if self.button(ids!(confirm)).clicked(event.actions()) {
            self.finish_recording(cx, scope);
        }

        if self.button(ids!(cancel)).clicked(event.actions()) {
            self.cancel_recording(cx, scope);
        }
    }
}

impl SttInput {
    pub fn set_stt_utility(&mut self, utility: Option<SttUtility>) {
        self.stt_utility = utility;
    }

    pub fn start_recording(&mut self, cx: &mut Cx) {
        self.state = SttInputState::Recording(RecordingState {
            start_time: Cx::time_now(),
        });
        self.label(ids!(status))
            .set_text(cx, &time_to_minutes_seconds(0.));
        self.timer = cx.start_timeout(TIMER_PRECISION);

        // Initialize or reset buffer
        if self.audio_buffer.is_none() {
            self.audio_buffer = Some(Arc::new(Mutex::new(AudioData::default())));
        }

        if let Some(arc) = &self.audio_buffer {
            if let Ok(mut buffer) = arc.lock() {
                buffer.data.clear();
                buffer.sample_rate = None;
            }

            let buffer_clone = arc.clone();
            cx.audio_input(0, move |info, input_buffer| {
                let channel = input_buffer.channel(0); // Mono

                if let Ok(mut recorded) = buffer_clone.try_lock() {
                    if recorded.sample_rate.is_none() {
                        recorded.sample_rate = Some(info.sample_rate);
                    }
                    recorded.data.extend_from_slice(channel);
                }
            });
        }
    }

    fn stop_recording(&mut self, cx: &mut Cx) {
        cx.audio_input(0, |_, _| {});
    }

    // TODO: We should stop recording on widget drop.

    pub fn finish_recording(&mut self, cx: &mut Cx, scope: &mut Scope) {
        self.stop_recording(cx);
        self.state = SttInputState::Sending;
        self.label(ids!(status)).set_text(cx, "Transcribing...");

        if let Some(buffer_arc) = self.audio_buffer.clone() {
            self.process_stt_audio(cx, buffer_arc, scope);
        }
    }

    pub fn cancel_recording(&mut self, cx: &mut Cx, scope: &mut Scope) {
        self.stop_recording(cx);
        self.state = SttInputState::Idle;
        self.abort_handle = None;

        let uid = self.widget_uid();
        cx.widget_action(uid, &scope.path, SttInputAction::Cancelled);
    }

    fn process_stt_audio(
        &mut self,
        cx: &mut Cx,
        buffer_arc: Arc<Mutex<AudioData>>,
        scope: &mut Scope,
    ) {
        if let Some(utility) = &self.stt_utility {
            let mut client = utility.client.clone();
            let bot_id = utility.bot_id.clone();
            let ui = self.ui_runner();

            let (samples, sample_rate) = {
                let guard = buffer_arc.lock().unwrap();
                (guard.data.clone(), guard.sample_rate)
            };

            if samples.is_empty() {
                self.cancel_recording(cx, scope);
                return;
            }

            let sample_rate = sample_rate.unwrap_or(48000.0) as u32;
            let wav_bytes = crate::utils::audio::build_wav(&samples, sample_rate, 1);

            let attachment = Attachment::from_bytes(
                "recording.wav".to_string(),
                Some("audio/wav".to_string()),
                &wav_bytes,
            );

            let message = Message {
                from: EntityId::User,
                content: MessageContent {
                    attachments: vec![attachment],
                    ..Default::default()
                },
                ..Default::default()
            };

            let future = async move {
                use futures::{StreamExt, pin_mut};
                let stream = client.send(&bot_id, &[message], &[]);

                let filtered = stream
                    .filter_map(|r| async move { r.value().map(|c| c.text.clone()) })
                    .filter(|text| futures::future::ready(!text.is_empty()));
                pin_mut!(filtered);
                let text = filtered.next().await;

                if let Some(text) = text {
                    ui.defer_with_redraw(move |me, cx, scope| {
                        me.handle_transcription(cx, text, scope);
                    });
                } else {
                    // Handle no text? Treat as cancel or empty?
                    // Chat impl didn't handle else.
                    ui.defer_with_redraw(move |me, cx, scope| {
                        me.cancel_recording(cx, scope);
                    });
                }
            };

            self.abort_handle = Some(spawn_abort_on_drop(future));
        }
    }

    fn handle_transcription(&mut self, cx: &mut Cx, text: String, scope: &mut Scope) {
        self.state = SttInputState::Idle;
        self.abort_handle = None;
        let uid = self.widget_uid();
        cx.widget_action(uid, &scope.path, SttInputAction::Transcribed(text));
    }

    pub fn recorded<'a>(&self, actions: &'a Actions) -> Option<&'a str> {
        actions
            .find_widget_action(self.widget_uid())
            .and_then(|widget_action| widget_action.downcast_ref::<SttInputAction>())
            .and_then(|action| match action {
                SttInputAction::Transcribed(text) => Some(text.as_str()),
                _ => None,
            })
    }
}

impl SttInputRef {
    /// Immutable access to the underlying [[SttInput]].
    ///
    /// Panics if the widget reference is empty or if it's already borrowed.
    pub fn read(&self) -> std::cell::Ref<'_, SttInput> {
        self.borrow().unwrap()
    }

    /// Mutable access to the underlying [[SttInput]].
    ///
    /// Panics if the widget reference is empty or if it's already borrowed.
    pub fn write(&mut self) -> std::cell::RefMut<'_, SttInput> {
        self.borrow_mut().unwrap()
    }
}

fn time_to_minutes_seconds(time_secs: f64) -> String {
    let total_seconds = time_secs.floor() as u64;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{}:{:02}", minutes, seconds)
}
