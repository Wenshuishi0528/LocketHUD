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

## 2026-08-11 — Product gate

The current decision is `POC0_BLOCKED` because ADB had no device. Do not enter V1 Mac editor work. Revisit V1 only after live installation, optical comfort, rapid hide, persistence, and 30/60-minute power/thermal evidence produce `POC0_PASS` or `POC0_PASS_WITH_LIMITATIONS`.
