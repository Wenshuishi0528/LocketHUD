# Build and install

## Reproducible host checks

From the repository root:

```sh
./gradlew clean assembleDebug lintDebug testDebugUnitTest
python3 -m unittest discover -s tools/tests -v
sh -n tools/install_private_portrait.sh
python3 -m py_compile tools/prepare_portrait.py tools/generate_calibration_assets.py
```

Observed result on 2026-08-11:

- Gradle build: pass
- Android lint: pass (0 errors; informational compatibility/performance warnings reviewed)
- Kotlin/JUnit tests: 6 tests pass
- Python unittest: 3 tests pass
- shell syntax and Python byte-compilation checks: pass

## APK

- Package: `dev.local.lockethud.poc`
- Version: 0.1.0 (1)
- Build path: `glasses-app/build/outputs/apk/debug/glasses-app-debug.apk`
- Delivered local path: `artifacts/LocketHUD-POC-0.1.0-debug.apk`
- Size: 2,541,381 bytes
- SHA-256: `ef56faecea8f6dcbb86b4bc8a3905493dc6d73a16961e556513dbbe6cd171f31`
- Signing: Android debug key, APK Signature Scheme v2; not a release/store signature
- Manifest permission inspection: no `uses-permission` entries

## Install and start

```sh
adb devices -l
adb install -r glasses-app/build/outputs/apk/debug/glasses-app-debug.apk
adb shell am start -W -n dev.local.lockethud.poc/.MainActivity --es mode minimal
adb shell am start -W -n dev.local.lockethud.poc/.MainActivity --es mode calibration
adb shell am start -W -n dev.local.lockethud.poc/.MainActivity --es mode portrait
```

Observed device result: streamed install succeeded; minimal mode cold-started in 842 ms during the first validation; `MainActivity` reached `RESUMED`; runtime View and surface were 480×640. Calibration and portrait modes produced distinct, non-black device screenshots with no fatal exception after the full-screen compatibility fix. The final artifact then passed `adb uninstall`, streamed reinstall, and a 514 ms cold start.

A 30-minute 13-second foreground run then completed on the same PID with no crash or screen-off. Thermal status remained 0 and temperature changed from 25.0°C to 22.5°C. The glasses remained USB powered and charged from 85% to 100%, so this run does not establish unplugged battery drain.

## Debug configuration

All controls are string extras in debug builds only:

```sh
adb shell am start -n dev.local.lockethud.poc/.MainActivity \
  --es mode portrait \
  --es anchor right_middle \
  --es size small \
  --es opacity 0.6 \
  --es keep_screen_on true \
  --es visible true \
  --es clock_enabled false \
  --es render_profile quantized_16
```

Whitelist:

- mode: `portrait`, `calibration`, `minimal`
- anchor: `left_top`, `left_middle`, `left_bottom`, `right_top`, `right_middle`, `right_bottom`
- size: `small`, `medium`, `large`
- opacity: `0.4`, `0.6`, `0.8`, `1.0`
- booleans: `true`, `false`
- asset: `default`, `private`
- render profile: `natural_green`, `quantized_8`, `quantized_16`, `dithered`

## Verify package and permissions

```sh
$ANDROID_HOME/build-tools/36.0.0/aapt2 dump permissions \
  glasses-app/build/outputs/apk/debug/glasses-app-debug.apk
$ANDROID_HOME/build-tools/36.0.0/apksigner verify --verbose --print-certs \
  glasses-app/build/outputs/apk/debug/glasses-app-debug.apk
```

The first command should print only the package line and no permission names.

## Private portrait import

```sh
tools/install_private_portrait.sh local_assets/processed/current.png
```

The script requires an authorized ADB device and an installed debuggable build. It first tries app-specific external files, then falls back to a validated `run-as` copy into app-private files. It prints only the package, source SHA-256, and method—not image content.
