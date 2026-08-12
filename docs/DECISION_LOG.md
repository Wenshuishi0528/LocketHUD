# Decision log

## 2026-08-11 — Independent project boundary

Created `LocketHUD-POC` as a separate local Git repository with package `dev.local.lockethud.poc`. Existing level/HUD, AIUI cycling, and translation projects were not modified. No Git remote was added.

## 2026-08-11 — Standard Android fallback

The official CXR-S entry was reachable, but a current artifact version, sample template, dependency matrix, and license were not available to the non-authenticated inspection, and no local official CXR-S package was found. The specification explicitly permits testing a standard Android APK first. The implementation therefore uses only Android SDK APIs and makes no Rokid proprietary API claim.

If the exact firmware rejects the APK, stop and obtain the authenticated official CXR-S sample. Do not invent dependencies or use third-party SDK mirrors.

## 2026-08-11 — Minimal native rendering

Selected one Kotlin Activity plus programmatic Canvas/Views. No Compose, Unity, OpenGL scene, database, dependency injection, background service, WakeLock, network stack, or analytics library was added. Static content invalidates only on configuration/asset/size change or the optional minute clock tick.

## 2026-08-11 — Safety and input

Positions are calculated from runtime View bounds/insets and constrained to side bands outside a central 50% protection zone. Large portraits may be clamped slightly below their nominal 0.22 width ratio. Calibration can show the protected area for measurement.

No unverified Rokid key code is bound. Generic back hides then exits; a debug-only input probe is ready for actual touchpad/key mapping.

## 2026-08-11 — Privacy

No INTERNET, camera, microphone, location, storage, overlay, WakeLock, or service permission/component is present. Private PNGs stay in ignored local paths and app-specific device storage. Image processing is local and strips metadata by reconstructing/saving PNG output.

## 2026-08-11 — Initial product gate

The initial decision was `POC0_BLOCKED` because ADB had no device. V1 remained gated pending live installation, optical comfort, rapid hide, persistence, and power/thermal evidence.

## 2026-08-11 — Rokid Android 12 full-screen compatibility

The first device launch crashed because this firmware's `PhoneWindow.getInsetsController()` dereferenced a not-yet-attached DecorView. Full-screen setup was moved after `setContentView` and uses the standard API-32 immersive system-UI flags. The rebuilt APK then cold-started, rendered, and remained stable. No Rokid proprietary API was introduced.

## 2026-08-11 — Device-sized preset correction

Device screenshots showed Medium and Large collapsing to the same width because the side-band margin was subtracted twice. The layout now subtracts the outer margin once and still clamps at the central protection boundary. A regression test was added. Device-visible bounds became approximately 49×98, 62×126, and 73×147 pixels for Small, Medium, and Large.

## 2026-08-11 — Revised product gate

G0 and the standard Android rendering path pass. A 30-minute USB-powered foreground run completed on the same process with no crash, screen-off, or thermal warning. The decision is `POC0_PASS_WITH_LIMITATIONS`: binocular optical judgment, a physical quick-hide mapping, unplugged battery drain, and the optional 60-minute run remain pending. Do not begin V1 Mac editor work until the user accepts at least one optical combination and the physical input path is confirmed.
