# Rename Ledger: OxiMux → Trex

Started: 2026-09-21
Completed: 2026-09-22

## Status (2026-09-22)

All items closed. `main` pushed to `github.com/tiraci/Trex`:

| Area | Status | Commit |
|------|--------|--------|
| Rebrand OxiMux → Trex (1,012 files) | done | `0f12c407` |
| 10 Orca feature ports + 6 new crates | done | `0f12c407` |
| Workspace release builds (app/cli/relay) | done | — |
| Desktop UI wiring (nav rail panels) | done | `10105090` |
| Integration tests for new crates (38) | done | `84271427` |
| Ledger closed out | done | `6cfaffa4` |

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

- ~~Push to `github.com/tiraci/Trex`~~ — done (`main` == `origin/main`)
- ~~Wire new crates into trex-app UI~~ — done: Orchestration / Accounts / DiffAnnotation reachable from the left nav rail; CLI wired (_see below_)
- ~~Add integration tests~~ — done: 38 tests across the six new crates (_see below_)

## Post-ledger work (2026-09-22)

| Item | Status | Notes |
|------|--------|-------|
| Desktop UI wiring | done (`10105090`) | Orchestration / Accounts / DiffAnnotation singletons in the nav rail, mirroring Tasks/Automations. Verified: `cargo check -p trex-app` clean, nav_section tests 12 pass, ci-check green. |
| Integration tests | done (`84271427`) | diff-annotate (6), accounts (6), automations (9), orchestration (7), ssh (5), skills (5) — run lifecycle, events, rate limits, DAG convergence, discovery/install, SSH state + reconnect ladder. ci-check green. |

> Note: ledger's "mobile app" rename phase refers to `apps/mobile` (Expo RN + native module `apps/mobile/modules/trex-core`), which is in this repo alongside `apps/desktop`, `apps/cli`, `apps/landing`.
