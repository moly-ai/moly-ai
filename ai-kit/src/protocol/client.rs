use super::*;
use std::sync::Arc;
use std::{error::Error, fmt};

/// The standard error kinds a client implementatiin should facilitate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientErrorKind {
    /// The network connection could not be established properly or was lost.
    Network,

    /// The connection could be established, but the remote server/peer gave us
    /// an error.
    ///
    /// Example: On a centralized HTTP server, this would happen when it returns
    /// an HTTP error code.
    Response,

    /// The remote server/peer returned a successful response, but we can't parse
    /// its content.
    ///
    /// Example: When working with JSON APIs, this can happen when the schema of
    /// the JSON response is not what we expected or is not JSON at all.
    Format,

    /// A kind of error that is not contemplated by MolyKit at the client layer.
    Unknown,
}

impl ClientErrorKind {
    pub fn to_human_readable(&self) -> &str {
        match self {
            ClientErrorKind::Network => "Network error",
            ClientErrorKind::Response => "Remote error",
            ClientErrorKind::Format => "Format error",
            ClientErrorKind::Unknown => "Unknown error",
        }
    }
}

/// Standard error returned from client operations.
#[derive(Debug, Clone)]
pub struct ClientError {
    kind: ClientErrorKind,
    message: String,
    source: Option<Arc<dyn Error + Send + Sync + 'static>>,
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind.to_human_readable(), self.message)
    }
}

impl Error for ClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|s| &**s as _)
    }
}

impl From<ClientError> for Vec<ClientError> {
    fn from(error: ClientError) -> Self {
        vec![error]
    }
}

impl<T> From<ClientError> for ClientResult<T> {
    fn from(error: ClientError) -> Self {
        ClientResult::new_err(vec![error])
    }
}

impl ClientError {
    /// Construct a simple client error without source.
    ///
    /// If you have an underlying error you want to include as the source, use
    /// [ClientError::new_with_source] instead.
    pub fn new(kind: ClientErrorKind, message: String) -> Self {
        ClientError {
            kind,
            message,
            source: None,
        }
    }

    /// Construct a client error using an underlying error as the source.
    pub fn new_with_source<S>(kind: ClientErrorKind, message: String, source: Option<S>) -> Self
    where
        S: Error + Send + Sync + 'static,
    {
        ClientError {
            kind,
            message,
            source: source.map(|s| Arc::new(s) as _),
        }
    }

    /// Error kind accessor.
    pub fn kind(&self) -> ClientErrorKind {
        self.kind
    }

    /// Error message accessor.
    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

/// The outcome of a client operation.
///
/// Different from the standard Result, this one may contain more than one error.
/// And at the same time, even if an error ocurrs, there may be a value to rescue.
///
/// It would be mistake if this contains no value and no errors at the same time.
/// This is taken care on creation time, and it can't be modified afterwards.
#[derive(Debug, Clone)]
#[must_use]
pub struct ClientResult<T> {
    errors: Vec<ClientError>,
    value: Option<T>,
}

impl<T> ClientResult<T> {
    /// Creates a result containing a successful value and no errors.
    pub fn new_ok(value: T) -> Self {
        ClientResult {
            errors: Vec::new(),
            value: Some(value),
        }
    }

    /// Creates a result containing errors and no value to rescue.
    ///
    /// The errors list should be non empty. If it's empty a default error will
    /// be added to avoid the invariant of having no value and no errors at the
    /// same time.
    pub fn new_err(errors: Vec<ClientError>) -> Self {
        let errors = if errors.is_empty() {
            vec![ClientError::new(
                ClientErrorKind::Unknown,
                "An error ocurred, but no details were provided.".into(),
            )]
        } else {
            errors
        };

        ClientResult {
            errors,
            value: None,
        }
    }

    /// Creates a result containing errors and a value to rescue.
    ///
    /// This method should only be used when there are both errors and a value.
    /// - If there are no errors, use [ClientResult::new_ok] instead.
    /// - Similar to [ClientResult::new_err], if the errors list is empty, a default
    /// error will be added.
    pub fn new_ok_and_err(value: T, errors: Vec<ClientError>) -> Self {
        let errors = if errors.is_empty() {
            vec![ClientError::new(
                ClientErrorKind::Unknown,
                "An error ocurred, but no details were provided.".into(),
            )]
        } else {
            errors
        };

        ClientResult {
            errors,
            value: Some(value),
        }
    }

    /// Returns the successful value if there is one.
    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Returns the errors list.
    pub fn errors(&self) -> &[ClientError] {
        &self.errors
    }

    /// Returns the successful value and the errors list.
    pub fn value_and_errors(&self) -> (Option<&T>, &[ClientError]) {
        (self.value.as_ref(), &self.errors)
    }

    /// Creates a result with the given value and errors without checking the
    /// invariant of having at least one of them.
    ///
    /// Warning: The main purpose of this method is to construct [`ClientResult`]s
    /// from already existing [`ClientResult`]s that are known to be valid. Using
    /// this without caution may lead to invalid states and panic at runtime.
    pub fn new_unchecked(value: Option<T>, errors: Vec<ClientError>) -> Self {
        ClientResult { value, errors }
    }

    /// Maps the successful value to another value, cloning the errors list.
    pub fn map_value<'t, U: 't>(&'t self, f: impl FnOnce(&'t T) -> U) -> ClientResult<U> {
        let value = self.value.as_ref().map(f);
        ClientResult::new_unchecked(value, self.errors.clone())
    }

    /// Returns true if there is a successful value.
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Returns true if there are errors.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Consume the result and return the successful value if there is one.
    pub fn into_value(self) -> Option<T> {
        self.value
    }

    /// Consume the result and return the errors list.
    pub fn into_errors(self) -> Vec<ClientError> {
        self.errors
    }

    /// Consume the result and return the successful value and the errors list.
    pub fn into_value_and_errors(self) -> (Option<T>, Vec<ClientError>) {
        (self.value, self.errors)
    }

    /// Consume the result to convert it into a standard Result.
    pub fn into_result(self) -> Result<T, Vec<ClientError>> {
        if self.errors.is_empty() {
            Ok(self.value.expect("ClientResult has no value nor errors"))
        } else {
            Err(self.errors)
        }
    }
}

/// A standard interface to fetch bots information and send messages to them.
///
/// Warning: Expect this to be cloned to avoid borrow checking issues with
/// makepad's widgets. Also, it may be cloned inside async contexts. So keep this
/// cheap to clone and synced.
///
/// Note: Generics do not play well with makepad's widgets, so this trait relies
/// on dynamic dispatch (with its limitations).
pub trait BotClient: Send {
    /// Send a message to a bot with support for streamed response.
    ///
    /// Each message yielded by the stream should be a snapshot of the full
    /// message as it is being built.
    ///
    /// You are free to add, modify or remove content on-the-go.
    fn send(
        &mut self,
        bot_id: &BotId,
        messages: &[Message],
        tools: &[Tool],
    ) -> BoxPlatformSendStream<'static, ClientResult<MessageContent>>;

    /// Interrupt the bot's current operation.
    // TODO: There may be many chats with the same bot/model/agent so maybe this
    // should be implemented by using cancellation tokens.
    // fn stop(&mut self, bot: BotId);

    /// Bots available under this client.
    // NOTE: Could be a stream, but may add complexity rarely needed.
    // TODO: Support partial results with errors for an union multi client/service
    // later.
    fn bots(&self) -> BoxPlatformSendFuture<'static, ClientResult<Vec<Bot>>>;

    /// Make a boxed dynamic clone of this client to pass around.
    fn clone_box(&self) -> Box<dyn BotClient>;
}

impl Clone for Box<dyn BotClient> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
