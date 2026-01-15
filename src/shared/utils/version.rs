//! Versioned data for fast change detection.
//!
//! Start reading at [`Versioned<T>`].
//!
//! The combination of [`Version`] and [`Versioned<T>`] is useful when dealing with
//! data inside Moly's Store, as this allows fast and lightweight change detection
//! against a single source of truth, and controlled data binding, where the store
//! holds the [`Versioned<T>`] data, and widgets/components can hold an [`Option<Version>`],
//! pulling new data when the source version is different, and pushing new data on input.

use makepad_widgets::SignalToUI;
use serde::{Deserialize, Serialize};
use std::num::NonZero;
use std::num::NonZeroU64;

/// Identifies the "version" of some data.
///
/// This is used in combination with [`Versioned<T>`] to track changes to data.
///
/// Optimization note: [`Option<Version>`] should be the same size as [`Version`]
/// (tested), because of the internal implementation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Version(NonZero<u64>);

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl Version {
    /// Creates a initial version.
    ///
    /// This is the starting version of a [`Versioned<T>`].
    ///
    /// If you manually create this and use it with a fresh [`Versioned<T>`] to
    /// call [`Self::pull`], it will return `None` since the versions will match.
    pub const fn new() -> Self {
        Version(NonZeroU64::new(1).unwrap())
    }

    /// Increments the version by one, wrapping around on overflow.
    ///
    /// This is called automatically by [`Versioned<T>`] when its data is changed.
    /// This method may not be relevant if you are already using [`Versioned<T>`].
    pub fn bump(&mut self) {
        self.0 = NonZeroU64::new(self.0.get().wrapping_add(1)).unwrap();
    }
}

pub trait Pull {
    /// A method that imposes a "workflow" to react to version changes.
    ///
    /// When the version held by `self` is different than the one in `versioned`,
    /// it updates `self` to match and returns `Some(&T)`.
    ///
    /// This method is implemented by [`Version`], but also [`Option<Version>`] for
    /// ergonomics.
    fn pull<'a, 'b, T>(&'a mut self, versioned: &'b Versioned<T>) -> Option<&'b T>;
}

impl Pull for Version {
    fn pull<'a, 'b, T>(&'a mut self, versioned: &'b Versioned<T>) -> Option<&'b T> {
        if *self != versioned.version() {
            *self = versioned.version();
            Some(versioned.data())
        } else {
            None
        }
    }
}

impl Pull for Option<Version> {
    fn pull<'a, 'b, T>(&'a mut self, versioned: &'b Versioned<T>) -> Option<&'b T> {
        match self {
            Some(v) => v.pull(versioned),
            None => {
                *self = Some(versioned.version());
                Some(versioned.data())
            }
        }
    }
}

/// A wrapper around some data paired with a [`Version`] to track changes to it.
///
/// This is useful to track changes to data without needing to duplicate/hash and compare
/// the data itself.
///
/// Version is bumped automatically when data is changed via [`Self::set`] or [`Self::update`].
///
/// # Clone
///
/// Cloning a `Versioned<T>` works as expected, cloning both, the data and the version,
/// so [`PartialEq`] and other traits work as expected. However, the clone should be
/// treated as a different entity.
///
/// # Makepad
///
/// Use [`Self::set_and_notify`] and [`Self::update_and_notify`] to let `handle_event` run in
/// an app.
pub struct Versioned<T> {
    data: T,
    version: Version,
}

impl<T: Default> Default for Versioned<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Clone> Clone for Versioned<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            version: self.version(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Versioned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Versioned")
            .field("data", &self.data)
            .field("version", &self.version)
            .finish()
    }
}

impl<T: PartialEq> PartialEq for Versioned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.version == other.version
    }
}

impl<T: Eq> Eq for Versioned<T> {}

impl<T: Copy> Copy for Versioned<T> {}

impl<T: std::hash::Hash> std::hash::Hash for Versioned<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
        self.version.hash(state);
    }
}

impl<T: Serialize> Serialize for Versioned<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Versioned<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = T::deserialize(deserializer)?;
        Ok(Self::new(data))
    }
}

impl<T> Versioned<T> {
    /// Creates a new `Versioned<T>` wrapping the provided data with the initial version.
    pub fn new(data: T) -> Self {
        Self {
            data,
            version: Version::new(),
        }
    }

    /// Sets the data to the provided value, bumping the version.
    pub fn set(&mut self, data: T) {
        self.data = data;
        self.version.bump();
    }

    /// Mutates the data using the provided closure, bumping the version at the end.
    pub fn update<F>(&mut self, update_fn: F)
    where
        F: FnOnce(&mut T),
    {
        update_fn(&mut self.data);
        self.version.bump();
    }

    /// Get the current version.
    pub fn version(&self) -> Version {
        self.version
    }

    /// Get an immutable reference to the current data.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Same as [`Self::set`] but also notifies Makepad's UI about the change.
    pub fn set_and_notify(&mut self, data: T) {
        self.set(data);
        SignalToUI::set_ui_signal();
    }

    /// Same as [`Self::update`] but also notifies Makepad's UI about the change.
    pub fn update_and_notify<F>(&mut self, update_fn: F)
    where
        F: FnOnce(&mut T),
    {
        self.update(update_fn);
        SignalToUI::set_ui_signal();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_size() {
        assert_eq!(std::mem::size_of::<Version>(), std::mem::size_of::<u64>());
        assert_eq!(
            std::mem::size_of::<Option<Version>>(),
            std::mem::size_of::<Version>()
        );
    }

    #[test]
    fn test_version_bump() {
        let mut v = Version::new();
        let initial = v;
        v.bump();
        assert_ne!(v, initial);
    }

    #[test]
    fn test_pull_version() {
        let mut versioned = Versioned::new(10);
        let mut tracker = Version::new();

        assert_eq!(tracker.pull(&versioned), None);

        versioned.set(20);
        assert_eq!(tracker.pull(&versioned), Some(&20));
        assert_eq!(tracker.pull(&versioned), None);
    }

    #[test]
    fn test_pull_option_version() {
        let mut versioned = Versioned::new(10);
        let mut tracker: Option<Version> = None;

        assert_eq!(tracker.pull(&versioned), Some(&10));
        assert_eq!(tracker, Some(versioned.version()));

        assert_eq!(tracker.pull(&versioned), None);

        versioned.set(20);
        assert_eq!(tracker.pull(&versioned), Some(&20));
    }

    #[test]
    fn test_serialization_ignores_version() {
        let mut versioned = Versioned::new(vec![1, 2, 3]);
        versioned.set(vec![4, 5, 6]);
        let json = serde_json::to_string(&versioned).expect("Failed to serialize");
        assert_eq!(json, "[4,5,6]");

        let deserialized: Versioned<Vec<i32>> =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.data, vec![4, 5, 6]);
        assert_eq!(deserialized.version(), Version::new());
        assert_ne!(deserialized.version(), versioned.version());
    }

    #[test]
    fn test_clone_keeps_version() {
        let mut v1 = Versioned::new(10);
        v1.set(20);

        assert_ne!(v1.version(), Version::new());

        let v2 = v1.clone();
        assert_eq!(v2.data(), &20);
        assert_eq!(v2.version(), v1.version());
    }
}
