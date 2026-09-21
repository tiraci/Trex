//! The desktop's worktree service — now just a re-export.
//!
//! The implementation moved to `trex-worktree-ops` so `TREX serve` hosts
//! the same one: a headless host that could not create worktrees answered
//! `Unsupported` to `TREX worktree create`, `run --worktree`, and
//! `team run --worktree-each`, which is most of the reason to run one.
//!
//! Kept as a module rather than deleted because this path is what
//! `remote_control` wires, and the indirection costs nothing.

pub use trex_worktree_ops::RepoWorktrees;
