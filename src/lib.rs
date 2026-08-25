//! InString - Simple and efficient string interning
//!
//! ## Examples
//!
//! Using the [`Intern`] trait.
//!
//! ```rust
//! use instring::Intern;
//!
//! let a = "hello world".intern();
//! let b = String::from("hello world").intern();
//!
//! assert!(a == b);
//! assert!(a.as_str() == b.as_str());
//! assert!(std::ptr::eq(a.as_str(), b.as_str()));
//! ```
//!
//! Using [`InString::from()`].
//!
//! ```rust
//! use instring::InString;
//!
//! let a = InString::from("hello world");
//! let b = InString::from(String::from("hello world"));
//!
//! assert!(a == b);
//! assert!(a.as_str() == b.as_str());
//! assert!(std::ptr::eq(a.as_str(), b.as_str()));
//! ```

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use dashmap::DashMap;
use itertools::Itertools;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::LazyLock;
use std::vec::IntoIter;

mod traits;

#[cfg(feature = "stats")]
mod stats;

#[cfg(feature = "stats")]
pub use stats::InStringStats;

static INTERNED: LazyLock<DashMap<InStringInner, ()>> = LazyLock::new(DashMap::default);

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

/// Type representing an interned string.
///
/// This type acts as a [`String`], but cloning an instance will not duplicate the
/// string's backing storage on heap, but instead create a new reference to the same
/// backing storage. The cost of cloning is the same as cloning an [`Arc`].
///
/// An interned string will automatically be uninterned, and the string's backing
/// storage on heap will be freed, when the last [`InString`] referencing the string
/// is dropped.
#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InString(InStringInner);

impl InString {
    /// Returns an iterator over all currently interned string.
    /// Mostly useful for introspection and debugging.
    pub fn all() -> IntoIter<InString> {
        INTERNED
            .iter()
            .map(|entry| {
                let inner = entry.key();
                #[cfg(feature = "stats")]
                InStringStats::deduped_add(inner.0.len());
                InString(inner.clone())
            })
            .sorted()
    }

    /// Returns the number of active references to the backing string.
    /// Mostly useful for introspection and debugging.
    #[inline]
    pub fn ref_count(&self) -> usize {
        self.0.ref_count()
    }

    /// Returns statistics for all currently interned string.
    /// Mostly useful for introspection and debugging.
    #[cfg(feature = "stats")]
    pub fn stats() -> InStringStats {
        InStringStats::collect()
    }
}

impl From<Cow<'_, str>> for InString {
    fn from(string: Cow<'_, str>) -> Self {
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
            #[cfg(feature = "stats")]
            InStringStats::interned_add(inner.0.len());
        } else {
            #[cfg(feature = "stats")]
            InStringStats::deduped_add(inner.0.len());
        }

        InString(inner)
    }
}

impl From<&InString> for InString {
    #[inline]
    fn from(string: &InString) -> Self {
        string.clone()
    }
}

impl From<char> for InString {
    #[inline]
    fn from(string: char) -> Self {
        Self::from(Cow::Owned(string.to_string()))
    }
}

impl From<Box<str>> for InString {
    #[inline]
    fn from(string: Box<str>) -> Self {
        Self::from(string.into_string())
    }
}

impl From<&mut str> for InString {
    #[inline]
    fn from(string: &mut str) -> Self {
        Self::from(Cow::Borrowed(string))
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

impl From<&String> for InString {
    #[inline]
    fn from(string: &String) -> Self {
        Self::from(Cow::Borrowed(string.as_str()))
    }
}

impl Clone for InString {
    fn clone(&self) -> Self {
        #[cfg(feature = "stats")]
        InStringStats::deduped_add(self.len());
        Self(self.0.clone())
    }
}

impl Drop for InString {
    fn drop(&mut self) {
        let inner = &self.0;

        // Unintern the string if its reference count is 1. This could
        // be racing with other threads, so the reference count needs
        // to be re-checked while holding a lock on the INTERNED entry.
        let uninterned = inner.ref_count() == 1
            && INTERNED
                .remove_if(inner, |inner, _| inner.ref_count() == 1)
                .is_some();

        if uninterned {
            debug_assert!(inner.ref_count() == 0);
            #[cfg(feature = "stats")]
            InStringStats::interned_sub(self.len());
        } else {
            #[cfg(feature = "stats")]
            InStringStats::deduped_sub(self.len());
        }
    }
}

impl Display for InString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct InStringInner(Arc<String>);

impl InStringInner {
    #[inline]
    fn from(string: Cow<'_, str>) -> Self {
        Self(Arc::new(string.into_owned()))
    }

    #[inline]
    fn ref_count(&self) -> usize {
        // Minus one to discount the reference held by INTERNED
        Arc::strong_count(&self.0) - 1
    }
}

impl Borrow<str> for InStringInner {
    #[inline]
    fn borrow(&self) -> &str {
        &self.0
    }
}
