//! Minimal updater helper using CrabNebula's `cargo-packager-updater`.
//! Replace `UPDATER_PUBKEY` with the minisign public key that matches the
//! private key configured in the release pipeline.

use anyhow::Result;
use cargo_packager_updater::{check_update, Config};
use makepad_widgets::*;

/// Minisign public key that verifies update signatures.
/// TODO: replace with the real public key (e.g. `RWQ...`).
// const UPDATER_PUBKEY: &str = "REPLACE_WITH_MINISIGN_PUBLIC_KEY";
const UPDATER_PUBKEY: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEFGOTlFM0UzMTZCNUQzMTUKUldRVjA3VVc0K09acnhYRFU3My9wK1BuV2ZOeElNT1M4RWIvb1ZDSktrTHU5bkJmMUthZkdKUWoK";

/// Action emitted back into Makepad once an update check finishes.
#[derive(Clone, Debug, DefaultNone)]
pub enum UpdaterAction {
    UpdateAvailable(String),
    NoUpdate,
    Failed(String),
    None,
}

/// Checks GitHub Releases for an available update without blocking the UI thread.
///
/// This queries the `latest` release endpoint using target/arch-specific
/// manifests uploaded by the release workflow (e.g. `latest-windows-x86_64.json`).
/// When finished, it posts a [`UpdaterAction`] into the Makepad event queue so the UI
/// can react without the startup path blocking on network I/O.
pub async fn check_moly_update() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION").parse()?;
    let config = Config {
        endpoints: vec![ "https://github.com/tyreseluo/moly/releases/latest/download/latest-{{target}}-{{arch}}.json".parse()? ],
        pubkey: UPDATER_PUBKEY.into(),
        ..Default::default()
    };

    let update_result = tokio::task::spawn_blocking(move || check_update(current_version, config))
        .await?;

    match update_result {
        Ok(Some(update)) => {
            Cx::post_action(UpdaterAction::UpdateAvailable(update.version.to_string()));
        }
        Ok(None) => {
            Cx::post_action(UpdaterAction::NoUpdate);
        }
        Err(err) => {
            Cx::post_action(UpdaterAction::Failed(err.to_string()));
            return Err(err.into());
        }
    }

    Ok(())
}
