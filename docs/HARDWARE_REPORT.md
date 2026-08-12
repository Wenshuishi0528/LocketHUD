# Hardware report

Audit date: 2026-08-11 (America/Los_Angeles)

## Current live result

The glasses connected as one authorized USB ADB device. The serial number was deliberately omitted from this report.

| Item | Live value |
|---|---|
| Manufacturer / model | Rokid / RG-glasses |
| Product / device | glasses / glasses |
| Android | 12, API 32 |
| Firmware fingerprint | `Rokid/glasses/glasses:12/SKQ1.240613.001/1.21.009-20260623-150201:user/release-keys` |
| Security patch | 2024-07-05 |
| Build type | user, `ro.debuggable=0` |
| ABI | arm64-v8a, armeabi-v7a, armeabi |
| Physical/runtime display | 480×640, portrait rotation 0, 60 Hz |
| Density | 240 dpi (1.5 logical density) |
| App viewport | 480×640 with zero display cutout insets |
| Power at initial audit | USB powered, battery 64%, 25.5°C |
| Thermal status at initial audit | 0 (no throttling reported) |

The standard Android debug APK installed successfully on this user/release-key firmware. `MainActivity` cold-started in 842 ms during the first minimal-mode validation, reached `RESUMED`, drew a full 480×640 surface, and produced non-black device screenshots once the display was awake. The final artifact also passed uninstall/reinstall and then cold-started in 514 ms.

The display follows a glasses sleep/wear policy: while asleep, Activity switching can succeed but `screencap` is black and the app Surface is hidden. `KEYCODE_WAKEUP`/wearing the glasses restored the display. Black screenshots captured during `mWakefulness=Asleep` are not rendering failures.

## Physical connection action

Completed: the glasses were connected with a USB data cable and USB debugging was authorized. Keep the glasses awake or worn for visual screenshots. No root, bootloader change, or system security change was used.

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

`G0_PASS_STANDARD_ANDROID_PATH`. Device identity, standard APK install/start/exit/uninstall/reinstall, runtime View size, display geometry, and input-device inventory are confirmed. CXR-S remains unintegrated because the standard Android route is sufficient for this POC.
