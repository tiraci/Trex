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
| Orca feature parity reconciled vs `../orca` | done | — |

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

## Feature parity vs Orca (reconciled 2026-09-22 against `../orca`)

The original 10-feature table above covers the first port wave. Trex also carries many Orca features that predate it — mapped here so the ledger is the single parity source. Evidence = crate/dir names in this repo.

### Ported (present in Trex beyond the first wave)

| Orca feature | Trex location |
|---|---|
| Terminal (PTY, scrollback, splits, OSC7, grid snapshots) | `crates/pty` + `shell/terminal` |
| Session history & workspace restore | `session_restore/` + `shell/session_history` |
| Editor / markdown / syntax / UI primitives | `crates/editor`, `crates/markdown`, `crates/syntax`, `crates/ui` |
| Source control + PR create (gh/glab forge) | `shell/{git_panel,stash_panel,pr_dialog,commit_dialog,source_control}` + `crates/git` |
| Computer use | `crates/computer-use` |
| Dictation | `crates/dictation` |
| Auto-update (channels, staging, sig verify) | `crates/auto-update` |
| Notifications | `notifier/{mac,null}` |
| Onboarding, usage/stats, ports panel | `shell/{onboarding,usage,ports_panel}` + `crates/proc-ports` |
| Remote host/session runtime | `crates/remote-{host,session,local,iroh,proto}` + `remote_control/` |
| In-app relay daemon (agent hooks, pty, terminals) | `crates/relay*` (relay, client, proto, supervisor, terminals) |
| Settings surface (theme/typography/keybindings/dictation/computer-use/autosave/…) | `crates/settings` + `app_settings/` + `shell/settings_modal` |
| Worktree ops | `crates/worktree-ops` |
| Mobile companion | `apps/mobile` + `crates/mobile-core` (+ native module `apps/mobile/modules/trex-core`) |
| Native chat / agent sessions | `shell/agent_chat` + `crates/agent-core`, `crates/agents` |
| External-CLI integration health (gh/glab/rg) | `shell/integrations` |
| Landing page | `apps/landing` |

### Partial

| Orca feature | Trex status |
|---|---|
| Browser pane | `shell/browser_view` exists; no Design Mode or agent-browser automation |
| PR / issue surfaces | PR create dialog + CLI-gated Tasks page; no dedicated provider panels |
| Notifications | macOS + null backends only; no badges/attention model |
| Diff comments | Per-line via `trex-diff-annotate`; no multi-line range comments |

### Missing (Orca features with no Trex counterpart)

- Cloud relay stack (director / cell / fence-broker / ops console, terraform) — `cloud/` absent; Trex relay is in-process only
- Push notifications gateway (APNs/FCM) + two-way audio
- OTA phased web-shell migration (desktop-served mobile web bundle, host generation store, page routes)
- Plugin system (marketplace, install trust, worker supervision, language packs, audit log)
- i18n / translation catalog
- Device emulator control (scrcpy / iOS simctl)
- Agent hibernation (pause idle PTYs, auto-resume)
- Telemetry / crash reporting / hang watchdog / diagnostics
- Ephemeral per-workspace cloud VMs
- Provider panels beyond gh/glab (GitHub PRs/checks, Linear, Jira, GitLab, Bitbucket, Azure DevOps, Gitea as first-class panels)
- Rich markdown editor extras (Mermaid, wiki links, front-matter tables, slash menu)
- Floating terminal / Floating Workspace
- Workspace cleanup, multi-window cross-client sync, source-control AI recipes

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
