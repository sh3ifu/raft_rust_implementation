//! Messages exchanged between Raft servers (Figure 2).
//!
//! The paper describes two RPCs, each with arguments and results. We model
//! every half as a separate message instead of a blocking call: the core sends
//! a request and, at some later point, may or may not receive a response.
//! This matches what a real network does (messages get lost, delayed and
//! reordered) and keeps the core free of I/O.
//!
//! The sender of a message is not part of the message itself; whoever delivers
//! it to a node tells the node where it came from.

use crate::entry::LogEntry;
use crate::types::{LogIndex, NodeId, Term};

/// Invoked by candidates to gather votes (§5.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestVoteRequest {
    /// Candidate's term.
    pub term: Term,
    /// Candidate requesting the vote.
    pub candidate_id: NodeId,
    /// Index of the candidate's last log entry (§5.4).
    pub last_log_index: LogIndex,
    /// Term of the candidate's last log entry (§5.4).
    pub last_log_term: Term,
}

/// Reply to a [`RequestVoteRequest`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestVoteResponse {
    /// Voter's current term, for the candidate to update itself.
    pub term: Term,
    /// `true` means the candidate received the vote.
    pub vote_granted: bool,
}

/// Invoked by the leader to replicate log entries (§5.3); with no entries,
/// it serves as a heartbeat (§5.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppendEntriesRequest {
    /// Leader's term.
    pub term: Term,
    /// Leader's id, so followers can redirect clients.
    pub leader_id: NodeId,
    /// Index of the log entry immediately preceding the new ones.
    pub prev_log_index: LogIndex,
    /// Term of the entry at `prev_log_index`.
    pub prev_log_term: Term,
    /// Entries to store; empty for a heartbeat.
    pub entries: Vec<LogEntry>,
    /// Leader's commit index.
    pub leader_commit: LogIndex,
}

/// Reply to an [`AppendEntriesRequest`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppendEntriesResponse {
    /// Follower's current term, for the leader to update itself.
    pub term: Term,
    /// `true` if the follower had an entry matching `prev_log_index` and
    /// `prev_log_term`, and therefore accepted the entries.
    pub success: bool,
}

/// Any message one Raft server can send to another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// See [`RequestVoteRequest`].
    RequestVoteRequest(RequestVoteRequest),
    /// See [`RequestVoteResponse`].
    RequestVoteResponse(RequestVoteResponse),
    /// See [`AppendEntriesRequest`].
    AppendEntriesRequest(AppendEntriesRequest),
    /// See [`AppendEntriesResponse`].
    AppendEntriesResponse(AppendEntriesResponse),
}

impl Message {
    /// The sender's term at the time the message was sent.
    ///
    /// Every message carries a term, and every server checks it first: a
    /// higher term makes the receiver step down to follower, a lower one marks
    /// the message as stale (§5.1).
    pub fn term(&self) -> Term {
        match self {
            Message::RequestVoteRequest(m) => m.term,
            Message::RequestVoteResponse(m) => m.term,
            Message::AppendEntriesRequest(m) => m.term,
            Message::AppendEntriesResponse(m) => m.term,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_message_exposes_its_term() {
        let term = Term::new(4);
        let messages = [
            Message::RequestVoteRequest(RequestVoteRequest {
                term,
                candidate_id: NodeId::new(1),
                last_log_index: LogIndex::ZERO,
                last_log_term: Term::ZERO,
            }),
            Message::RequestVoteResponse(RequestVoteResponse {
                term,
                vote_granted: true,
            }),
            Message::AppendEntriesRequest(AppendEntriesRequest {
                term,
                leader_id: NodeId::new(1),
                prev_log_index: LogIndex::ZERO,
                prev_log_term: Term::ZERO,
                entries: Vec::new(),
                leader_commit: LogIndex::ZERO,
            }),
            Message::AppendEntriesResponse(AppendEntriesResponse {
                term,
                success: false,
            }),
        ];

        for message in &messages {
            assert_eq!(message.term(), term);
        }
    }
}
