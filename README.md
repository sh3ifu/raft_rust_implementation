# raft_rust_implementation

[![CI](https://github.com/sh3ifu/raft_rust_implementation/actions/workflows/ci.yml/badge.svg)](https://github.com/sh3ifu/raft_rust_implementation/actions/workflows/ci.yml)

An implementation of the [Raft consensus algorithm](https://raft.github.io/) in Rust,
built from scratch as a learning project and grown step by step towards production quality.

> **Status:** early development. The workspace is set up; protocol logic is next.

## Goals

- Follow the Raft paper closely, so the code can be read side by side with it.
- Keep the protocol core **pure**: no I/O, no clocks, no threads inside. Inputs go in,
  effects come out as data.
- Test it with a **deterministic simulator** that injects message loss, reordering, partitions
  and crashes, and checks Raft's safety properties after every step. Any failure must replay
  exactly from its seed.
- On top of the core, run a replicated, linearizable **key-value store** over a real network.

## Planned milestones

| Version | Milestone |
|---------|-----------|
| v0.1 | Leader election |
| v0.2 | Log replication |
| v0.3 | Fault-injecting simulator and safety checkers |
| v0.4 | Persistence and crash recovery |
| v0.5 | Real network runtime (tokio) |
| v0.6 | Replicated key-value store |
| v0.7 | Log compaction and snapshots |
| v0.8 | Cluster membership changes |
| v0.9 | Pre-vote, CheckQuorum, pipelining, metrics |
| v1.0 | Linearizability checking, Jepsen-style tests, benchmarks |

## Project layout

```
crates/
  raft-core    pure protocol logic
```

More crates (simulator, storage, network runtime, key-value store) will be added as the
project grows.

## Building

Requires a stable Rust toolchain (installed via [rustup](https://rustup.rs/)).

```bash
cargo build
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

## References

- Diego Ongaro, John Ousterhout. *In Search of an Understandable Consensus Algorithm
  (Extended Version)*. 2014.
- Diego Ongaro. *Consensus: Bridging Theory and Practice*. PhD thesis, Stanford University, 2014.

## License

[MIT](LICENSE)
