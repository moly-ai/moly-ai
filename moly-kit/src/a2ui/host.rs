//! A2UI Host
//!
//! Manages the connection between an A2A agent and the A2uiSurface widget.
//! Handles streaming, message processing, and user action forwarding.

use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;

use makepad_widgets::*;
use serde_json::Value;

use super::a2a_client::{A2aClient, A2aStreamEvent, A2aEventStream};
use super::message::{A2uiMessage, UserAction};
use super::processor::ProcessorEvent;
use super::surface::{A2uiSurface, A2uiSurfaceAction};

/// A2UI Host configuration
#[derive(Clone, Debug)]
pub struct A2uiHostConfig {
    /// A2A server URL
    pub url: String,
    /// Optional authentication token
    pub auth_token: Option<String>,
}

/// Events from A2UI host
#[derive(Clone, Debug)]
pub enum A2uiHostEvent {
    /// Connected to server
    Connected,
    /// Received A2UI message
    Message(A2uiMessage),
    /// Task status update
    TaskStatus { task_id: String, state: String },
    /// Error occurred
    Error(String),
    /// Disconnected from server
    Disconnected,
}

/// A2UI Host manages streaming connection to an A2A server
pub struct A2uiHost {
    config: A2uiHostConfig,
    client: Option<A2aClient>,
    event_receiver: Option<Receiver<A2uiHostEvent>>,
    event_sender: Option<Sender<A2uiHostEvent>>,
    is_connected: bool,
    pending_messages: Vec<A2uiMessage>,
}

impl A2uiHost {
    /// Create a new A2UI host with the given configuration
    pub fn new(config: A2uiHostConfig) -> Self {
        let (tx, rx) = mpsc::channel();
        A2uiHost {
            config,
            client: None,
            event_receiver: Some(rx),
            event_sender: Some(tx),
            is_connected: false,
            pending_messages: Vec::new(),
        }
    }

    /// Connect to the A2A server and send initial message
    pub fn connect(&mut self, initial_message: &str) -> Result<(), String> {
        let mut client = A2aClient::new(&self.config.url);
        if let Some(token) = &self.config.auth_token {
            client = client.with_auth(token);
        }

        // Start streaming
        let stream = client.message_stream(initial_message)?;

        // Take sender for background thread
        let tx = self.event_sender.take().ok_or("Already connected")?;

        // Spawn thread to process stream
        thread::spawn(move || {
            Self::process_stream(stream, tx);
        });

        self.client = Some(client);
        self.is_connected = true;

        Ok(())
    }

    fn process_stream(mut stream: A2aEventStream, tx: Sender<A2uiHostEvent>) {
        // Send connected event
        let _ = tx.send(A2uiHostEvent::Connected);

        // Process events
        while let Some(event) = stream.next() {
            let host_event = match event {
                A2aStreamEvent::A2uiMessage(msg) => A2uiHostEvent::Message(msg),
                A2aStreamEvent::TaskStatus { task_id, state } => {
                    A2uiHostEvent::TaskStatus { task_id, state }
                }
                A2aStreamEvent::Error(e) => A2uiHostEvent::Error(e),
            };

            if tx.send(host_event).is_err() {
                // Receiver dropped
                break;
            }
        }

        // Send disconnected event
        let _ = tx.send(A2uiHostEvent::Disconnected);
    }

    /// Poll for pending events (non-blocking)
    pub fn poll(&mut self) -> Option<A2uiHostEvent> {
        if let Some(rx) = &self.event_receiver {
            match rx.try_recv() {
                Ok(event) => {
                    // If it's a message, also store it
                    if let A2uiHostEvent::Message(ref msg) = event {
                        self.pending_messages.push(msg.clone());
                    }
                    if let A2uiHostEvent::Disconnected = event {
                        self.is_connected = false;
                        // Clear receiver to prevent returning Disconnected repeatedly
                        self.event_receiver = None;
                    }
                    Some(event)
                }
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => {
                    self.is_connected = false;
                    // Clear receiver to prevent returning Disconnected repeatedly
                    self.event_receiver = None;
                    Some(A2uiHostEvent::Disconnected)
                }
            }
        } else {
            None
        }
    }

    /// Poll all pending events
    pub fn poll_all(&mut self) -> Vec<A2uiHostEvent> {
        let mut events = Vec::new();
        while let Some(event) = self.poll() {
            events.push(event);
        }
        events
    }

    /// Send a user action to the server
    pub fn send_action(&mut self, action: &UserAction) -> Result<(), String> {
        if let Some(client) = &mut self.client {
            let component_id = action.component_id.as_deref().unwrap_or("");
            client.send_action(
                &action.action.name,
                component_id,
                action.action.context.clone(),
            )
        } else {
            Err("Not connected".to_string())
        }
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    /// Get pending messages that haven't been processed yet
    pub fn take_pending_messages(&mut self) -> Vec<A2uiMessage> {
        std::mem::take(&mut self.pending_messages)
    }
}

/// Helper function to process A2UI host events and update the surface
pub fn process_host_events(
    host: &mut A2uiHost,
    surface: &mut A2uiSurface,
) -> Vec<ProcessorEvent> {
    let mut all_events = Vec::new();

    // Poll all pending host events
    let host_events = host.poll_all();

    for event in host_events {
        match event {
            A2uiHostEvent::Message(msg) => {
                // Process the A2UI message
                let processor_events = surface.process_message(msg);
                all_events.extend(processor_events);
            }
            A2uiHostEvent::Error(e) => {
                log!("A2UI Host Error: {}", e);
            }
            A2uiHostEvent::Connected => {
                log!("A2UI Host Connected");
            }
            A2uiHostEvent::Disconnected => {
                log!("A2UI Host Disconnected");
            }
            A2uiHostEvent::TaskStatus { task_id, state } => {
                log!("A2UI Task {}: {}", task_id, state);
            }
        }
    }

    all_events
}
