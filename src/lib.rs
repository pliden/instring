//! InString - Quick and easy string interning
//!
//! ## Examples
//!
//! Using the [`Intern`] trait.
//!
//! ```rust
//! use instring::Intern;
//!
//! let interned0 = "hello world".intern();
//! let interned1 = String::from("hello world").intern();
//!
//! assert!(interned0 == interned1);
//! assert!(interned0.as_str() == interned1.as_str());
//! assert!(std::ptr::eq(interned0.as_str(), interned1.as_str()));
//! ```
//!
//! Using [`InString::from()`].
//!
//! ```rust
//! use instring::InString;
//!
//! let interned0 = InString::from("hello world");
//! let interned1 = InString::from(String::from("hello world"));
//!
//! assert!(interned0 == interned1);
//! assert!(interned0.as_str() == interned1.as_str());
//! assert!(std::ptr::eq(interned0.as_str(), interned1.as_str()));
//! ```

#![warn(missing_docs)]
#![forbid(unsafe_code)]

use dashmap::DashMap;
use itertools::Itertools;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::vec::IntoIter;

static INTERNED: LazyLock<DashMap<InStringInner, ()>> = LazyLock::new(DashMap::default);
static NUM_INTERNED_STRINGS: AtomicUsize = AtomicUsize::new(0);
static NUM_INTERNED_BYTES: AtomicUsize = AtomicUsize::new(0);
static NUM_DEDUPED_STRINGS: AtomicUsize = AtomicUsize::new(0);
static NUM_DEDUPED_BYTES: AtomicUsize = AtomicUsize::new(0);

/// Trait for interning a string.
pub trait Intern {
    /// Interns the string and returns an [InString] instance.
    fn intern(self) -> InString;
}

impl Intern for &str {
    #[inline]
    fn intern(self) -> InString {
        InString::from(self)
    }
}

impl Intern for String {
    #[inline]
    fn intern(self) -> InString {
        InString::from(self)
    }
}

/// Type representing an interned string. This type acts as a [`String`],
/// but cloning an instance has the same cost as cloning an [`Arc`] and
/// will thus not duplicate the string's backing storage on heap.
#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InString(InStringInner);

impl InString {
    /// Returns an iterator over currently interned string.
    /// Mostly useful for introspection and debugging.
    pub fn all() -> IntoIter<InString> {
        INTERNED
            .iter()
            .map(|entry| {
                let inner = entry.key();
                let bytes = inner.0.len();
                InStringStats::deduped_add(bytes);
                InString(inner.clone())
            })
            .sorted()
    }

    /// Returns statistics for currently interned string.
    /// Mostly useful for introspection and debugging.
    pub fn stats() -> InStringStats {
        InStringStats::collect()
    }
}

impl Borrow<str> for InString {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl Deref for InString {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0.0
    }
}

impl From<Cow<'_, str>> for InString {
    fn from(string: Cow<'_, str>) -> Self {
        let bytes = string.len();
        let mut interned = false;

        let inner = match INTERNED.get(string.as_ref()) {
            Some(entry) => entry.key().clone(),
            None => INTERNED
                .entry(InStringInner::from(string))
                .or_insert_with(|| interned = true)
                .key()
                .clone(),
        };

        if interned {
            InStringStats::interned_add(bytes);
        } else {
            InStringStats::deduped_add(bytes);
        }

        InString(inner)
    }
}

impl From<&str> for InString {
    #[inline]
    fn from(string: &str) -> Self {
        Self::from(Cow::Borrowed(string))
    }
}

impl From<String> for InString {
    #[inline]
    fn from(string: String) -> Self {
        Self::from(Cow::Owned(string))
    }
}

impl AsRef<str> for InString {
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

impl PartialEq<str> for InString {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.0.0.as_str() == other
    }
}

impl PartialEq<&str> for InString {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.0.0.as_str() == *other
    }
}

impl Clone for InString {
    fn clone(&self) -> Self {
        let bytes = self.len();
        InStringStats::deduped_add(bytes);
        Self(self.0.clone())
    }
}

impl Drop for InString {
    fn drop(&mut self) {
        let bytes = self.len();
        let inner = &self.0;

        // Unintern the string if the reference count is 2. I.e. one
        // reference is owned by the caller of this function (and this
        // reference is about to be dropped), and the only remaining
        // reference will then be the one owned by INTERNED_STRINGS,
        // in which case we want to remove it from INTERNED_STRINGS.
        // This could be racing with other threads, so the ref_count
        // needs to be re-checked while holding a lock on the entry.
        let uninterned = inner.ref_count() == 2
            && INTERNED
                .remove_if(self.as_str(), |inner, _| inner.ref_count() == 2)
                .is_some();

        if uninterned {
            debug_assert!(self.0.ref_count() == 1);
            InStringStats::interned_sub(bytes);
        } else {
            InStringStats::deduped_sub(bytes);
        }
    }
}

impl Display for InString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.0)
    }
}

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct InStringInner(Arc<String>);

impl InStringInner {
    fn from(string: Cow<'_, str>) -> Self {
        Self(Arc::new(string.into_owned()))
    }

    fn ref_count(&self) -> usize {
        Arc::strong_count(&self.0)
    }
}

impl Borrow<str> for InStringInner {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/// Interned string statistics.
/// Mostly useful for introspection and debugging.
#[derive(Copy, Clone, Debug)]
pub struct InStringStats {
    /// Number of interned strings.
    pub interned_strings: usize,
    /// Sum of the length of all currently interned strings.
    pub interned_bytes: usize,
    /// Number of deduplicated strings. I.e. current number of
    /// [InString]s that shares its backing storage with another
    /// [InString].
    pub deduped_strings: usize,
    /// Sum of the length of all deduplicated strings. I.e. number
    /// of bytes saved on the heap because of string interning.
    pub deduped_bytes: usize,
}

impl InStringStats {
    fn collect() -> Self {
        Self {
            interned_strings: NUM_INTERNED_STRINGS.load(Ordering::Relaxed),
            interned_bytes: NUM_INTERNED_BYTES.load(Ordering::Relaxed),
            deduped_strings: NUM_DEDUPED_STRINGS.load(Ordering::Relaxed),
            deduped_bytes: NUM_DEDUPED_BYTES.load(Ordering::Relaxed),
        }
    }

    fn interned_add(bytes: usize) {
        NUM_INTERNED_STRINGS.fetch_add(1, Ordering::Relaxed);
        NUM_INTERNED_BYTES.fetch_add(bytes, Ordering::Relaxed);
    }

    fn interned_sub(bytes: usize) {
        NUM_INTERNED_STRINGS.fetch_sub(1, Ordering::Relaxed);
        NUM_INTERNED_BYTES.fetch_sub(bytes, Ordering::Relaxed);
    }

    fn deduped_add(bytes: usize) {
        NUM_DEDUPED_STRINGS.fetch_add(1, Ordering::Relaxed);
        NUM_DEDUPED_BYTES.fetch_add(bytes, Ordering::Relaxed);
    }

    fn deduped_sub(bytes: usize) {
        NUM_DEDUPED_STRINGS.fetch_sub(1, Ordering::Relaxed);
        NUM_DEDUPED_BYTES.fetch_sub(bytes, Ordering::Relaxed);
    }
}
