# 乐奇相片hud

“乐奇相片hud”（代码名 LocketHUD）是在 Mac 上处理相片、并把最终画面显示到 Rokid 眼镜视野边缘的本地软件。仓库包含轻量 Android 显示端和 V1 Mac 编辑器，不修改父目录中的水平仪/HUD 或翻译应用。

Current conclusion: `POC0_PASS_WITH_LIMITATIONS`; `V1_MAC_EDITOR_IMPLEMENTED`. The standard Android display path passed on the Rokid RG-glasses and the wearer reported no ghosting for the default layout. Product control has intentionally moved to the Mac: select/process a photo, preview it, adjust position/size/opacity, and send it over the existing local ADB connection. Physical touchpad mapping is no longer on the active development path.

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
- Tauri 2 Mac editor with local image selection, green conversion, 8/16-level quantization, dithering, gamma, contrast, sharpening, layout preview, device detection, and one-click ADB delivery.

The temporary package name is `dev.local.lockethud.poc`; it must be replaced before any distribution.

Both installed applications display the name `乐奇相片hud`. Their launcher icons use the same green smiling figure; the Mac icon has a transparent exterior outside its rounded square.

## Download 0.1.2

The public release contains both applications and their checksums: [乐奇相片hud 0.1.2](https://github.com/Wenshuishi0528/LocketHUD/releases/tag/v0.1.2).

- `LocketHUD-0.1.2-arm64.dmg`: Apple Silicon Mac application; the installed name is “乐奇相片hud”.
- `LocketHUD-Glasses-0.1.2-debug.apk`: Rokid glasses Android application; the launcher name is “乐奇相片hud”.

No personal photo is stored in this repository or its releases. Only source code, documentation, generated test assets, application packages, and checksums are published.

## Build

```sh
./gradlew clean assembleDebug lintDebug testDebugUnitTest
python3 -m unittest discover -s tools/tests -v
```

The build uses Android Studio's bundled JBR because the Mac's default Java 11 is too old for Gradle 9.6/AGP 9.2.

Build the Mac editor:

```sh
cd mac-editor
npm install
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

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
- [Mac editor](docs/MAC_EDITOR.md)
- [Changelog](CHANGELOG.md)
- [Development handoff](HANDOFF.md)

Open the Mac editor, connect the glasses by USB, select a photo, adjust it in the 480×640 preview, and press “发送到眼镜”. ADB is the V1 development transport; a consumer transport is deliberately deferred. Unplugged endurance is still required before making any battery-life claim.
