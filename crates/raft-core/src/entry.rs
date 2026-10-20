//! Entries of the replicated log.

use crate::types::{LogIndex, Term};

/// A single entry in the replicated log (§5.3).
///
/// Raft itself never looks inside `command`: it only agrees on the order of
/// commands. Interpreting them is up to the state machine built on top
/// (for example, a key-value store).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    /// Term in which the leader received this entry from a client.
    pub term: Term,
    /// Position of this entry in the log.
    pub index: LogIndex,
    /// Opaque command for the state machine.
    pub command: Vec<u8>,
}
