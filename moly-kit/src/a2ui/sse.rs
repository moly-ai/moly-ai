//! SSE (Server-Sent Events) Transport Layer
//!
//! Implements SSE parsing for A2UI streaming protocol.
//! SSE format:
//! - Lines starting with "data:" contain JSON payload
//! - Lines starting with ":" are comments (keep-alive pings)
//! - Empty lines mark message boundaries

use std::io::{BufRead, BufReader, Read};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

/// SSE event parsed from stream
#[derive(Debug, Clone)]
pub enum SseEvent {
    /// Data event with JSON payload
    Data(String),
    /// Comment (keep-alive)
    Comment(String),
    /// Connection error
    Error(String),
    /// Stream ended
    Done,
}

/// SSE parser state
pub struct SseParser {
    data_buffer: Vec<String>,
}

impl SseParser {
    pub fn new() -> Self {
        SseParser {
            data_buffer: Vec::new(),
        }
    }

    /// Parse a single line from SSE stream
    /// Returns Some(event) when a complete event is ready
    pub fn parse_line(&mut self, line: &str) -> Option<SseEvent> {
        if line.starts_with("data:") {
            // Extract data after "data:" prefix
            let data = line[5..].trim();
            self.data_buffer.push(data.to_string());
            None
        } else if line.starts_with(':') {
            // Comment line (keep-alive)
            Some(SseEvent::Comment(line[1..].trim().to_string()))
        } else if line.is_empty() {
            // Empty line = message boundary
            if !self.data_buffer.is_empty() {
                let data = self.data_buffer.join("\n");
                self.data_buffer.clear();
                Some(SseEvent::Data(data))
            } else {
                None
            }
        } else {
            // Unknown line format, ignore
            None
        }
    }

    /// Flush any remaining data
    pub fn flush(&mut self) -> Option<SseEvent> {
        if !self.data_buffer.is_empty() {
            let data = self.data_buffer.join("\n");
            self.data_buffer.clear();
            Some(SseEvent::Data(data))
        } else {
            None
        }
    }
}

impl Default for SseParser {
    fn default() -> Self {
        Self::new()
    }
}

/// SSE HTTP client for streaming responses
pub struct SseClient {
    url: String,
    headers: Vec<(String, String)>,
}

impl SseClient {
    pub fn new(url: impl Into<String>) -> Self {
        SseClient {
            url: url.into(),
            headers: Vec::new(),
        }
    }

    /// Add a header to the request
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    /// Add authorization header
    pub fn auth(self, token: impl Into<String>) -> Self {
        self.header("Authorization", format!("Bearer {}", token.into()))
    }

    /// Send POST request and return SSE event receiver
    pub fn post(self, body: &str) -> Result<Receiver<SseEvent>, String> {
        let (tx, rx) = mpsc::channel();
        let url = self.url.clone();
        let headers = self.headers.clone();
        let body = body.to_string();

        // Spawn thread to handle streaming response
        thread::spawn(move || {
            if let Err(e) = Self::stream_request(&url, &headers, &body, &tx) {
                let _ = tx.send(SseEvent::Error(e));
            }
            let _ = tx.send(SseEvent::Done);
        });

        Ok(rx)
    }

    fn stream_request(
        url: &str,
        headers: &[(String, String)],
        body: &str,
        tx: &Sender<SseEvent>,
    ) -> Result<(), String> {
        // Build request
        let mut request = ureq::post(url)
            .set("Content-Type", "application/json")
            .set("Accept", "text/event-stream");

        for (key, value) in headers {
            request = request.set(key, value);
        }

        // Send request
        let response = request
            .send_string(body)
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        // Check status
        if response.status() != 200 {
            return Err(format!("HTTP error: {}", response.status()));
        }

        // Parse SSE stream
        let reader = response.into_reader();
        let buf_reader = BufReader::new(reader);
        let mut parser = SseParser::new();

        for line_result in buf_reader.lines() {
            match line_result {
                Ok(line) => {
                    if let Some(event) = parser.parse_line(&line) {
                        if tx.send(event).is_err() {
                            // Receiver dropped, stop streaming
                            break;
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(SseEvent::Error(format!("Read error: {}", e)));
                    break;
                }
            }
        }

        // Flush remaining data
        if let Some(event) = parser.flush() {
            let _ = tx.send(event);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sse_parser_data() {
        let mut parser = SseParser::new();

        assert!(parser.parse_line("data: {\"hello\": \"world\"}").is_none());
        let event = parser.parse_line("").unwrap();

        match event {
            SseEvent::Data(data) => {
                assert_eq!(data, "{\"hello\": \"world\"}");
            }
            _ => panic!("Expected Data event"),
        }
    }

    #[test]
    fn test_sse_parser_multiline() {
        let mut parser = SseParser::new();

        parser.parse_line("data: line1");
        parser.parse_line("data: line2");
        let event = parser.parse_line("").unwrap();

        match event {
            SseEvent::Data(data) => {
                assert_eq!(data, "line1\nline2");
            }
            _ => panic!("Expected Data event"),
        }
    }

    #[test]
    fn test_sse_parser_comment() {
        let mut parser = SseParser::new();
        let event = parser.parse_line(": keep-alive").unwrap();

        match event {
            SseEvent::Comment(comment) => {
                assert_eq!(comment, "keep-alive");
            }
            _ => panic!("Expected Comment event"),
        }
    }
}
