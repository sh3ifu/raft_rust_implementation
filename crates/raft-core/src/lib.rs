//! Core of the Raft consensus protocol.
//!
//! This crate contains only the protocol logic. It performs no I/O, owns no
//! clocks or threads, and has no randomness of its own: every input (a timer
//! tick, an incoming message, a client proposal) is handed to it from the
//! outside, and every effect (sending a message, persisting state, applying a
//! committed entry) is returned to the caller as data.
//!
//! Keeping the core pure lets the exact same code run inside a deterministic
//! simulator during testing and inside a real networked runtime in production.

mod entry;
mod message;
mod types;

pub use entry::LogEntry;
pub use message::{
    AppendEntriesRequest, AppendEntriesResponse, Message, RequestVoteRequest, RequestVoteResponse,
};
pub use types::{LogIndex, NodeId, Term};
