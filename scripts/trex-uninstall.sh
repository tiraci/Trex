#!/usr/bin/env bash
#
# Remove the TREX relay daemon plus all of its on-disk artefacts.
# Idempotent: re-running on a clean machine is a no-op.
#
# Usage:
#   ./scripts/trex-uninstall.sh
#
# Removes:
#   - launchd agent (if installed via trex-launchd-install.sh)
#   - running trex-relay daemon (SIGTERM)
#   - ~/Library/Application Support/dev.tiraci.trex/relay-v*.{sock,token,pid}
#   - ~/Library/Logs/dev.tiraci.trex/
#
# Does NOT remove:
#   - the trex.app bundle itself (user moves that to Trash)
#   - the TREX database (settings/projects/recent workspaces)
#     — that survives an uninstaller run so reinstalling restores state.
#     Delete manually if you really want a wipe:
#       rm -rf "$HOME/Library/Application Support/dev.tiraci.trex"

set -euo pipefail

APP_DATA_DIR="$HOME/Library/Application Support/dev.tiraci.trex"
LOG_DIR="$HOME/Library/Logs/dev.tiraci.trex"
LAUNCHD_PLIST="$HOME/Library/LaunchAgents/dev.tiraci.trex.relay.plist"

log() { printf "[uninstall] %s\n" "$*"; }

if [[ -f "$LAUNCHD_PLIST" ]]; then
    log "unloading launchd agent"
    launchctl unload "$LAUNCHD_PLIST" 2>/dev/null || true
    rm -f "$LAUNCHD_PLIST"
fi

if pgrep -x trex-relay >/dev/null 2>&1; then
    log "stopping running trex-relay processes"
    pkill -TERM -x trex-relay 2>/dev/null || true
    for _ in 1 2 3 4 5; do
        sleep 0.5
        if ! pgrep -x trex-relay >/dev/null 2>&1; then
            break
        fi
    done
    if pgrep -x trex-relay >/dev/null 2>&1; then
        log "SIGTERM did not stop relay; sending SIGKILL"
        pkill -KILL -x trex-relay 2>/dev/null || true
    fi
fi

if [[ -d "$APP_DATA_DIR" ]]; then
    # Match relay-v1.sock relay-v1.token relay-v1.pid plus any future
    # version-bumped siblings; leaves the database file untouched.
    shopt -s nullglob
    for f in "$APP_DATA_DIR"/relay-v*.sock \
             "$APP_DATA_DIR"/relay-v*.token \
             "$APP_DATA_DIR"/relay-v*.pid; do
        log "rm $f"
        rm -f "$f"
    done
    shopt -u nullglob
fi

if [[ -d "$LOG_DIR" ]]; then
    log "rm -rf $LOG_DIR"
    rm -rf "$LOG_DIR"
fi

log "done"
