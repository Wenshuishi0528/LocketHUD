# LocketHUD POC

LocketHUD POC is an independent, local-only Android prototype for displaying one static portrait at the edge of a Rokid glasses viewport. This repository implements only G0 support and POC-0 from the 2026-08-11 specification. It does not modify the existing level/HUD or translation apps in the parent workspace.

Current conclusion: `POC0_PASS_WITH_LIMITATIONS`. G0 and the standard Android APK path passed on the connected Rokid RG-glasses: install, launch, rendering, configuration persistence, foreground keep-screen-on behavior, synthetic private-image import, and a 30-minute USB-powered stability run were verified. Binocular optical comfort, physical touchpad/key mapping, unplugged battery drain, and the optional 60-minute run remain open.

## Scope implemented

- Native Kotlin `Activity` and Canvas/Views; no Compose, Unity, database, service, WakeLock, or overlay.
- Black full-screen portrait mode with a generated transparent PNG.
- Six edge anchors, Small/Medium/Large sizes, and 40/60/80/100% opacity.
- Runtime-size and inset-aware layout with a central 50% protection zone.
- Persistent local configuration with schema version 1.
- Foreground-only `FLAG_KEEP_SCREEN_ON` behavior.
- First-back hide, second-back exit fallback; no unverified Rokid key code is bound.
- Debug-only minimal, calibration, input-probe, and validated Intent controls.
- Private PNG import from app-specific storage with size, format, and dimension validation.
- Offline Pillow CLI for green conversion, gamma, contrast, sharpening, 8/16-level quantization, and Floyd-Steinberg dithering.
- Program-generated calibration and synthetic portrait assets only.

The temporary package name is `dev.local.lockethud.poc`; it must be replaced before any distribution.

## Build

```sh
./gradlew clean assembleDebug lintDebug testDebugUnitTest
python3 -m unittest discover -s tools/tests -v
```

The build uses Android Studio's bundled JBR because the Mac's default Java 11 is too old for Gradle 9.6/AGP 9.2.

## Install and launch

```sh
adb devices -l
adb install -r glasses-app/build/outputs/apk/debug/glasses-app-debug.apk
adb shell am start -n dev.local.lockethud.poc/.MainActivity
```

Debug modes and configuration examples:

```sh
adb shell am start -n dev.local.lockethud.poc/.MainActivity --es mode minimal
adb shell am start -n dev.local.lockethud.poc/.MainActivity --es mode calibration
adb shell am start -n dev.local.lockethud.poc/.MainActivity \
  --es mode portrait --es anchor right_middle --es size small \
  --es opacity 0.6 --es keep_screen_on true --es visible true
adb shell am start -n dev.local.lockethud.poc/.InputProbeActivity
adb logcat -s LocketHUD.Input:I LocketHUD.Main:I LocketHUD.Asset:I '*:S'
```

Allowed anchors are `left_top`, `left_middle`, `left_bottom`, `right_top`, `right_middle`, and `right_bottom`. Allowed sizes are `small`, `medium`, and `large`; allowed opacity values are `0.4`, `0.6`, `0.8`, and `1.0`. Unknown values are ignored.

## Prepare and install a private portrait

Private files belong under ignored `local_assets/`; never add a personal image to `res/` or Git.

```sh
python3 tools/prepare_portrait.py \
  --input local_assets/portrait.png \
  --output-dir local_assets/processed \
  --max-width 120 \
  --profiles natural-green,quantized-8,quantized-16,dithered

cp local_assets/processed/portrait_quantized-16.png local_assets/processed/current.png
tools/install_private_portrait.sh local_assets/processed/current.png
```

The processor does not modify the source, does not connect to a network, preserves alpha, and writes PNGs without copying EXIF metadata. The import script writes only to this debug application's app-specific directory.

## Evidence and next action

- [Environment report](docs/ENVIRONMENT_REPORT.md)
- [Hardware report](docs/HARDWARE_REPORT.md)
- [SDK source map](docs/SDK_SOURCE_MAP.md)
- [Input map](docs/INPUT_MAP.md)
- [Build and install](docs/BUILD_AND_INSTALL.md)
- [Test protocol](docs/TEST_PROTOCOL.md)
- [Test results](docs/TEST_RESULTS.md)
- [Decision log](docs/DECISION_LOG.md)

The next gate is a seated, wearer-observed optical matrix using the connected build: confirm recognition, ghosting, comfort, distraction, and one acceptable profile/anchor/size combination. Capture the physical quick-hide gesture at the same time. Do not start the Mac editor, iPhone app, networking, animation, or AI work until those two product-experience checks are accepted; rerun endurance unplugged before making any battery-life claim.
