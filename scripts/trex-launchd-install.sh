#!/usr/bin/env bash
#
# Install dev.tiraci.trex.relay as a per-user launchd agent so the
# relay daemon starts at login and restarts on unexpected exit.
# Opt-in only — the app's ad-hoc spawn-detached path already satisfies
# the "survives Cmd-Q" requirement; this is power-user polish for
# folks who want the relay reachable even before launching TREX.
#
# Usage:
#   ./scripts/trex-launchd-install.sh /Applications/trex.app
#     (auto-detects trex-relay inside the bundle)
#
#   ./scripts/trex-launchd-install.sh --binary /path/to/trex-relay
#
# To remove: ./scripts/trex-uninstall.sh (handles launchctl unload).

set -euo pipefail

LABEL="dev.tiraci.trex.relay"
APP_DATA_DIR="$HOME/Library/Application Support/dev.tiraci.trex"
LOG_DIR="$HOME/Library/Logs/dev.tiraci.trex"
LAUNCHAGENTS_DIR="$HOME/Library/LaunchAgents"
PLIST_PATH="$LAUNCHAGENTS_DIR/$LABEL.plist"

SOCKET_PATH="$APP_DATA_DIR/relay-v1.sock"
TOKEN_PATH="$APP_DATA_DIR/relay-v1.token"
PID_PATH="$APP_DATA_DIR/relay-v1.pid"

BINARY=""

usage() {
    cat <<EOF >&2
usage: $0 <app-bundle-path> | --binary <path-to-trex-relay>

  /Applications/trex.app   (or wherever the bundle lives)
EOF
    exit 2
}

if [[ $# -eq 0 ]]; then
    usage
fi

case "$1" in
    --binary)
        [[ $# -ge 2 ]] || usage
        BINARY="$2"
        ;;
    -h|--help)
        usage
        ;;
    *)
        APP_BUNDLE="$1"
        BINARY="$APP_BUNDLE/Contents/MacOS/trex-relay"
        ;;
esac

if [[ ! -x "$BINARY" ]]; then
    echo "error: relay binary not found or not executable: $BINARY" >&2
    exit 1
fi

mkdir -p "$APP_DATA_DIR" "$LOG_DIR" "$LAUNCHAGENTS_DIR"

# The daemon refuses to start without a token file (it reads the token
# at boot to authenticate clients). TREX generates that token on
# first launch via the RelaySupervisor fresh-spawn path. If we install
# launchd before TREX has ever run, the agent will crash-loop on
# missing token. Warn loudly + bail so the user knows to launch TREX
# once before enabling the agent.
if [[ ! -s "$TOKEN_PATH" ]]; then
    cat >&2 <<EOF
error: token file is missing or empty: $TOKEN_PATH

The relay daemon will refuse to start without it. Launch TREX at
least once to let the app generate the token, then re-run this
installer.
EOF
    exit 1
fi

cat > "$PLIST_PATH" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>$LABEL</string>
    <key>ProgramArguments</key>
    <array>
        <string>$BINARY</string>
        <string>--socket</string>
        <string>$SOCKET_PATH</string>
        <string>--token</string>
        <string>$TOKEN_PATH</string>
        <string>--pid-file</string>
        <string>$PID_PATH</string>
        <string>--log-dir</string>
        <string>$LOG_DIR</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <dict>
        <key>SuccessfulExit</key>
        <false/>
    </dict>
    <key>ProcessType</key>
    <string>Background</string>
    <key>StandardErrorPath</key>
    <string>$LOG_DIR/relay-launchd.err</string>
    <key>StandardOutPath</key>
    <string>$LOG_DIR/relay-launchd.out</string>
</dict>
</plist>
PLIST

plutil -lint "$PLIST_PATH" >/dev/null

# Reload: unload first so an already-installed older plist gets the
# new ProgramArguments. `|| true` because the first install has
# nothing to unload.
launchctl unload "$PLIST_PATH" 2>/dev/null || true
launchctl load "$PLIST_PATH"

echo "installed: $PLIST_PATH"
echo "loaded:    launchctl list | grep $LABEL"
echo "uninstall: ./scripts/trex-uninstall.sh"
