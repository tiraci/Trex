# Rename Ledger: OxiMux → Trex

Started: 2026-09-21
Completed: 2026-09-21

## Scope
- 1,012 files, ~6,953 occurrences
- 30 file types

## Rename Map
| Old | New |
|-----|-----|
| OxiMux | Trex |
| oximux | trex |
| OXIMUX | TREX |
| oximux- | trex- |
| dev.nhtera.oximux | dev.tiraci.trex |
| nhtera/OxiMux | tiraci/Trex |
| nhtera (org) | tiraci |

## Progress

| Phase | Files | Status |
|-------|-------|--------|
| docs/markdown | 23 | done |
| scripts (.sh/.ps1) | 16 | done |
| CI workflows (.yml) | 3 | done |
| mobile app (.ts/.tsx/.json/.gradle/.podspec) | ~40 | done |
| landing page (.astro/.css/.jsonc) | ~10 | done |
| config (.toml/.iss/.plist/.sql/.tmTheme/.cpp/.swift/.py) | ~50 | done |
| Rust source (.rs) | 836 | done |
| root Cargo.toml + Cargo.lock | 2 | done |
| directory rename (oximux-core → trex-core) | 1 | done |
| file renames (icons, podspec, entitlements, scripts) | 7 | done |
| env var casing fix (TREX_ preserved for env vars) | ~740 | done |
| crate import casing fix (trex_ for Rust crate names) | 697 | done |
| org rename (nhtera → tiraci) | 53 | done |

## Orca Feature Ports

| Feature | Crate | Status | Notes |
|---------|-------|--------|-------|
| Parallel worktrees | `trex-orchestration` | done | Coordinator, DAG, task dispatch, worker preamble |
| Agent lifecycle | `trex-orchestration` | done | Status states, dispatch lifecycle |
| SSH worktrees | `trex-ssh` | done | Connection, reconnect ladder, relay session |
| Agent hooks | `trex-agent-hooks` (existing) | existing | Already had hooks; orchestration extends it |
| Workspace persistence | `trex-storage` (existing) | existing | SQLite-backed; orchestration adds run/task tables |
| Skills system | `trex-skills` | done | Manifest, discovery, install, tar+gzip |
| Automations/cron | `trex-automations` | done | Cron triggers, tick loop, run retention |
| Account switcher | `trex-accounts` | done | Multi-provider, rate limits, usage tracking |
| Diff annotation | `trex-diff-annotate` | done | Comments per line, format for agent |
| CLI commands | `trex-cli` (existing) | done | Orchestration, accounts, diff commands wired |

## New Crates Added to Workspace

```
crates/orchestration/   — Task DAG + coordinator loop
crates/ssh/             — SSH connection + relay session
crates/skills/          — Skill manifest + discovery + install
crates/automations/     — Cron schedules + automation service
crates/accounts/        — Multi-provider account management
crates/diff-annotate/   — Diff line comments
```

## Build Results

| Artifact | Command | Status |
|----------|---------|--------|
| trex.ico | `cargo run -p xtask -- icon` | done (108975 bytes, 6 frames) |
| trex-relay.exe | `cargo build -p trex-relay --release` | done (2.5 MB) |
| trex-cli.exe | `cargo build -p trex-cli --release` | done (29.8 MB) |
| trex-app.exe | `cargo build -p trex-app --release` | done (128.4 MB) |
| workspace check | `cargo check --workspace` | done (warnings only) |

## To try locally
```powershell
# Run CLI
./target/release/trex-cli.exe --help

# Run relay
./target/release/trex-relay.exe

# Run desktop app
./target/release/TREX.exe

# Test new commands
./target/release/trex-cli.exe orchestration ls
./target/release/trex-cli.exe accounts ls
./target/release/trex-cli.exe diff ls
```

## Remaining
- Push to `github.com/tiraci/Trex`
- Wire new crates into trex-app UI
- Add integration tests
