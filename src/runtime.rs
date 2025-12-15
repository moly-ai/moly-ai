//! Runtime management for Moly across different platforms.
//!
//! This module provides utilities to ensure async runtimes are properly
//! initialized regardless of the platform context (binary vs library).

use std::sync::OnceLock;

/// Global Tokio runtime for Moly when running as a library (e.g., Android)
static MOLY_RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

/// Initialize the Moly runtime if needed.
///
/// This function is safe to call multiple times and will only initialize once.
/// Needed for Android where the app runs as a library.
pub fn ensure_runtime() {
    MOLY_RUNTIME.get_or_init(|| {
        log::info!("Initializing Moly Tokio runtime");
        tokio::runtime::Builder::new_multi_thread()
            .enable_io()
            .enable_time()
            .thread_name("moly-runtime")
            .build()
            .expect("Failed to create Moly runtime")
    });
}

/// Get a handle to the Moly runtime, creating it if necessary.
pub fn runtime_handle() -> tokio::runtime::Handle {
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => {
            log::trace!("Using existing Tokio runtime");
            handle
        }
        Err(_) => {
            log::trace!("No runtime found, using Moly runtime");
            ensure_runtime();
            MOLY_RUNTIME.get().unwrap().handle().clone()
        }
    }
}

/// Spawn a future on the appropriate runtime.
///
/// This is similar to moly_kit::spawn but provides Moly-specific runtime management.
pub fn spawn<F>(future: F) -> tokio::task::JoinHandle<()>
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    runtime_handle().spawn(future)
}

/// Initialize runtime early in the application lifecycle.
///
/// Call this as early as possible in your app, ideally before any async operations.
/// This is especially important on Android where the app runs as a library.
///
/// # Example
/// ```rust
/// // Call this early in your app initialization
/// moly::runtime::init_early();
///
/// // Now async operations will work regardless of context
/// moly_kit::aitk::utils::asynchronous::spawn(async {
///     // Your async code here
/// });
/// ```
pub fn init_early() {
    log::debug!("Early runtime initialization requested");
    ensure_runtime();
}
