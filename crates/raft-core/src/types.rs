//! Basic identifiers used throughout the protocol.
//!
//! Each one wraps a plain `u64`, but as a distinct type, so the compiler
//! rejects mixing them up: passing a `Term` where a `LogIndex` is expected,
//! or comparing a node id with a term, is a compile error instead of a bug.

use std::fmt;

/// Unique identifier of a server in the cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(u64);

impl NodeId {
    /// Creates a node id from its raw value.
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the raw value.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "n{}", self.0)
    }
}

/// A term: Raft's logical clock (§5.1).
///
/// Time is divided into terms of arbitrary length, numbered with consecutive
/// integers. Each term begins with an election, and at most one leader can be
/// elected in a given term. Servers start in term 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Term(u64);

impl Term {
    /// The term every server starts in, before any election has happened.
    pub const ZERO: Self = Self(0);

    /// Creates a term from its raw value.
    pub const fn new(term: u64) -> Self {
        Self(term)
    }

    /// Returns the raw value.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the term that follows this one.
    ///
    /// A candidate increments its current term when it starts an election.
    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "t{}", self.0)
    }
}

/// Position of an entry in the replicated log.
///
/// Log indices start at 1, as in the paper. Index 0 means "before the first
/// entry": it is the `prev_log_index` sent along with the very first entries,
/// and the last log index of an empty log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LogIndex(u64);

impl LogIndex {
    /// The position just before the first entry of the log.
    pub const ZERO: Self = Self(0);

    /// Creates a log index from its raw value.
    pub const fn new(index: u64) -> Self {
        Self(index)
    }

    /// Returns the raw value.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the index that follows this one.
    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

impl fmt::Display for LogIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terms_start_at_zero_and_increase() {
        assert_eq!(Term::default(), Term::ZERO);
        assert_eq!(Term::ZERO.next(), Term::new(1));
        assert!(Term::new(2) > Term::new(1));
    }

    #[test]
    fn log_indices_start_at_zero_and_increase() {
        assert_eq!(LogIndex::default(), LogIndex::ZERO);
        assert_eq!(LogIndex::ZERO.next(), LogIndex::new(1));
        assert!(LogIndex::new(5) > LogIndex::new(4));
    }

    #[test]
    fn display_is_short_and_tagged() {
        assert_eq!(NodeId::new(3).to_string(), "n3");
        assert_eq!(Term::new(7).to_string(), "t7");
        assert_eq!(LogIndex::new(42).to_string(), "i42");
    }
}
