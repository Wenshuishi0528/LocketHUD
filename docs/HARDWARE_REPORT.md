# Hardware report

Audit date: 2026-08-11 (America/Los_Angeles)

## Current live result

`adb devices -l` returned an empty device list. No serial number or unique identifier was captured. Because the glasses were not connected and authorized during this run, the following current values are unknown:

- exact manufacturer, model, region, device name, and firmware fingerprint;
- Android release/API and CPU ABI;
- physical/logical display size, density, orientation, cutouts, and insets;
- input devices and key/touch event mappings;
- current power, battery, thermal, and display policies;
- whether this exact firmware accepts and launches the standard Android debug APK.

Older files in the parent workspace describe prior testing on an `RG-glasses` device with Android 12/API 32, arm64-v8a, and a 480×640 logical display. Those values are historical context only and are not counted as current G0 evidence.

## Required one-time physical action

Connect the glasses with a USB data cable, enable/confirm USB debugging on the glasses, and keep the glasses awake. Do not enable root, unlock the bootloader, or change system security settings.

## Read-only audit commands

Run after `adb devices -l` shows exactly one authorized device:

```sh
adb devices -l
adb shell getprop ro.product.manufacturer
adb shell getprop ro.product.model
adb shell getprop ro.product.device
adb shell getprop ro.build.version.release
adb shell getprop ro.build.version.sdk
adb shell getprop ro.build.fingerprint
adb shell getprop ro.product.cpu.abilist
adb shell wm size
adb shell wm density
adb shell dumpsys display
adb shell dumpsys window
adb shell dumpsys input
adb shell dumpsys power
adb shell dumpsys thermalservice
```

Before saving output, redact serial numbers, MAC addresses, advertising identifiers, and other unique device IDs. If a command is unavailable or permission-denied, record that result and stop there; do not use `adb root`, `adb remount`, or any system modification.

## G0 hardware status

`BLOCKED_NO_ADB_DEVICE`. Host build readiness is not a substitute for device install, view-size, input, or optical evidence.
