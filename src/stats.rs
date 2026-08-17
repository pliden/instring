use std::fmt::Debug;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static NUM_INTERNED_STRINGS: AtomicUsize = AtomicUsize::new(0);
static NUM_INTERNED_BYTES: AtomicUsize = AtomicUsize::new(0);
static NUM_DEDUPED_STRINGS: AtomicUsize = AtomicUsize::new(0);
static NUM_DEDUPED_BYTES: AtomicUsize = AtomicUsize::new(0);

/// Interned string statistics.
/// Mostly useful for introspection and debugging.
#[derive(Copy, Clone, Debug)]
pub struct InStringStats {
    /// Number of interned strings.
    pub interned_strings: usize,
    /// Sum of the length of all currently interned strings.
    pub interned_bytes: usize,
    /// Number of deduplicated strings. I.e. current number of
    /// [`InString`](super::InString)s that shares its backing
    /// storage with one or more other [`InString`](super::InString)s.
    pub deduped_strings: usize,
    /// Sum of the length of all deduplicated strings. I.e. number
    /// of bytes saved on the heap because of string interning.
    pub deduped_bytes: usize,
}

pub fn collect() -> InStringStats {
    InStringStats {
        interned_strings: NUM_INTERNED_STRINGS.load(Ordering::Relaxed),
        interned_bytes: NUM_INTERNED_BYTES.load(Ordering::Relaxed),
        deduped_strings: NUM_DEDUPED_STRINGS.load(Ordering::Relaxed),
        deduped_bytes: NUM_DEDUPED_BYTES.load(Ordering::Relaxed),
    }
}

pub fn interned_add(bytes: usize) {
    NUM_INTERNED_STRINGS.fetch_add(1, Ordering::Relaxed);
    NUM_INTERNED_BYTES.fetch_add(bytes, Ordering::Relaxed);
}

pub fn interned_sub(bytes: usize) {
    NUM_INTERNED_STRINGS.fetch_sub(1, Ordering::Relaxed);
    NUM_INTERNED_BYTES.fetch_sub(bytes, Ordering::Relaxed);
}

pub fn deduped_add(bytes: usize) {
    NUM_DEDUPED_STRINGS.fetch_add(1, Ordering::Relaxed);
    NUM_DEDUPED_BYTES.fetch_add(bytes, Ordering::Relaxed);
}

pub fn deduped_sub(bytes: usize) {
    NUM_DEDUPED_STRINGS.fetch_sub(1, Ordering::Relaxed);
    NUM_DEDUPED_BYTES.fetch_sub(bytes, Ordering::Relaxed);
}
