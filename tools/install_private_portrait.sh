#!/bin/sh
set -eu

PACKAGE="dev.local.lockethud.poc"
COMPONENT="$PACKAGE/.MainActivity"
SOURCE_PATH="${1:-local_assets/processed/current.png}"
REMOTE_NAME="current.png"
TEMP_PATH="/data/local/tmp/lockethud-current.png"
EXTERNAL_DIR="/sdcard/Android/data/$PACKAGE/files/portraits"

if [ ! -f "$SOURCE_PATH" ]; then
    echo "error: source PNG not found" >&2
    exit 2
fi

case "$SOURCE_PATH" in
    *.png|*.PNG) ;;
    *) echo "error: source must be a PNG" >&2; exit 2 ;;
esac

if [ "$(adb get-state 2>/dev/null || true)" != "device" ]; then
    echo "error: no authorized ADB device" >&2
    exit 3
fi

HASH=$(shasum -a 256 "$SOURCE_PATH" | awk '{print $1}')
echo "Package: $PACKAGE"
echo "Source SHA-256: $HASH"

if adb shell mkdir -p "$EXTERNAL_DIR" >/dev/null 2>&1 && \
    adb push "$SOURCE_PATH" "$EXTERNAL_DIR/$REMOTE_NAME" >/dev/null 2>&1; then
    METHOD="app-specific external files"
else
    adb push "$SOURCE_PATH" "$TEMP_PATH" >/dev/null
    adb shell chmod 644 "$TEMP_PATH" >/dev/null
    adb shell run-as "$PACKAGE" mkdir -p files/portraits >/dev/null
    adb shell run-as "$PACKAGE" cp "$TEMP_PATH" "files/portraits/$REMOTE_NAME" >/dev/null
    adb shell rm -f "$TEMP_PATH" >/dev/null
    METHOD="debug run-as private files"
fi

adb shell am force-stop "$PACKAGE"
adb shell am start -n "$COMPONENT" --es asset private --es visible true >/dev/null
echo "Installed via $METHOD and restarted LocketHUD POC"
