use std::sync::{Arc, Mutex};

/// Upgrade types for enhanced communication modes
#[derive(Debug, Clone, PartialEq)]
pub enum Upgrade {
    /// Realtime audio/voice communication
    Realtime(RealtimeChannel),
}

/// Channel for realtime communication events
#[derive(Debug, Clone)]
pub struct RealtimeChannel {
    /// Sender for realtime events to the UI
    pub event_sender: futures::channel::mpsc::UnboundedSender<RealtimeEvent>,
    /// Receiver for realtime events from the client
    pub event_receiver:
        Arc<Mutex<Option<futures::channel::mpsc::UnboundedReceiver<RealtimeEvent>>>>,
    /// Sender for commands to the realtime client
    pub command_sender: futures::channel::mpsc::UnboundedSender<RealtimeCommand>,
}

impl PartialEq for RealtimeChannel {
    fn eq(&self, _other: &Self) -> bool {
        // For now, we'll consider all channels equal since we can't compare the actual channels
        true
    }
}

/// Events sent from the realtime client to the UI
#[derive(Debug, Clone)]
pub enum RealtimeEvent {
    /// Session is ready for communication
    SessionReady,
    /// Audio data received (PCM16 format)
    AudioData(Vec<u8>),
    /// Text transcript of received audio (delta)
    AudioTranscript(String),
    /// Complete AI audio transcript
    AudioTranscriptCompleted(String, String), // (transcript, item_id)
    /// Complete user audio transcript
    UserTranscriptCompleted(String, String), // (transcript, item_id)
    /// User started speaking
    SpeechStarted,
    /// User stopped speaking
    SpeechStopped,
    /// AI response completed
    ResponseCompleted,
    /// Function call requested by AI
    FunctionCallRequest {
        name: String,
        call_id: String,
        arguments: String,
    },
    /// Error occurred
    Error(String),
}

/// Commands sent from the UI to the realtime client
#[derive(Debug, Clone)]
pub enum RealtimeCommand {
    /// Stop the realtime session
    StopSession,
    /// Send audio data (PCM16 format)
    SendAudio(Vec<u8>),
    /// Send text message
    SendText(String),
    /// Interrupt current AI response
    Interrupt,
    /// Update session configuration
    UpdateSessionConfig {
        voice: String,
        transcription_model: String,
    },
    /// Create a greeting response from AI
    CreateGreetingResponse,
    /// Send function call result back to AI
    SendFunctionCallResult { call_id: String, output: String },
}
