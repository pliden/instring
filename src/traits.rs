use crate::InString;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::convert::Infallible;
use std::ffi::OsStr;
use std::ops::Deref;
use std::ops::Index;
use std::path::Path;
use std::path::PathBuf;
use std::slice::SliceIndex;
use std::str::FromStr;

// To seamlessly use InString as if it was a String, we implement Deref
// as well as (almost) all traits that (an immutable) String implements.

impl Deref for InString {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0.0
    }
}

impl AsRef<OsStr> for InString {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.deref().as_ref()
    }
}

impl AsRef<Path> for InString {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.deref().as_ref()
    }
}

impl AsRef<[u8]> for InString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.deref().as_ref()
    }
}

impl AsRef<str> for InString {
    #[inline]
    fn as_ref(&self) -> &str {
        self.deref().as_ref()
    }
}

impl Borrow<str> for InString {
    #[inline]
    fn borrow(&self) -> &str {
        self.deref().borrow()
    }
}

impl FromStr for InString {
    type Err = Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<InString, Self::Err> {
        Ok(InString::from(s))
    }
}

impl<I: SliceIndex<str>> Index<I> for InString {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.deref().index(index)
    }
}

impl PartialEq<Cow<'_, str>> for InString {
    #[inline]
    fn eq(&self, other: &Cow<'_, str>) -> bool {
        self.0.0.as_ref() == other.as_ref()
    }
}

impl PartialEq<&InString> for InString {
    #[inline]
    fn eq(&self, other: &&InString) -> bool {
        self.0.0.as_str() == other.as_str()
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

impl PartialEq<String> for InString {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.0.0.as_str() == other
    }
}

impl PartialEq<&String> for InString {
    #[inline]
    fn eq(&self, other: &&String) -> bool {
        self.0.0.as_str() == other.as_str()
    }
}

impl PartialEq<Path> for InString {
    #[inline]
    fn eq(&self, other: &Path) -> bool {
        self.0.0.as_str() == other
    }
}

impl PartialEq<PathBuf> for InString {
    #[inline]
    fn eq(&self, other: &PathBuf) -> bool {
        self.0.0.as_str() == other
    }
}
