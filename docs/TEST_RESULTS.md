# Test results

Status date: 2026-08-11

## Current decision

`POC0_BLOCKED`

The implementation and host-side verification are ready, but no authorized ADB device was visible. The blocker is the missing live device connection—not a known rendering failure. Therefore G0, device install, optical quality, physical input, foreground endurance, and the product-experience decision remain open.

## Acceptance evidence

| Acceptance item | Status | Evidence |
|---|---|---|
| Independent repository/package/data boundary | Pass | Separate local Git repository; package `dev.local.lockethud.poc`; no remote. |
| Kotlin native Activity/Views | Pass | Debug APK compiles with AGP built-in Kotlin. |
| Six anchors | Host pass | Pure layout tests cover all six and central 50% protection. Device view pending. |
| Three sizes | Host pass | 0.14/0.18/0.22 width ratios with boundary clamp. Device view pending. |
| Four opacity levels | Host pass | Whitelisted 0.4/0.6/0.8/1.0. Optical result pending. |
| Alpha/rectangular assets | Host pass | Generated PNG resources and Pillow alpha test pass. Optical composition pending. |
| Show/hide and persistence | Host pass | Back fallback and SharedPreferences implemented; device interaction pending. |
| Foreground-only keep-screen-on | Host pass | Activity flag set on resume and cleared on pause; endurance pending. |
| No sensitive permissions | Pass | Final merged Manifest has no `uses-permission` entries. |
| Build/lint/unit tests | Pass | Gradle build/lint/JUnit and Python tests pass. |
| Private photos excluded | Pass | `local_assets/*` ignored; repository resources are program-generated. |
| Device identification | Blocked | `adb devices -l` was empty. |
| Install/start/restart | Blocked | No authorized device. |
| Actual View/insets/orientation | Blocked | Requires device launch/log. |
| Input map | Blocked | Debug probe ready; no live events. |
| Optical visibility/ghosting | Blocked | Requires user wearing test. |
| 30/60 minute power and thermal | Blocked | Requires foreground device run. |

## User-observed results

Not yet collected. Complete the matrix in `TEST_PROTOCOL.md`; code compilation cannot substitute for the user's binocular comfort and distraction judgment.

## Conditions for a new decision

- `POC0_PASS`: G0 succeeds and at least one portrait profile/anchor/size is recognizable, quickly hideable, stable, and acceptable in power/comfort testing.
- `POC0_PASS_WITH_LIMITATIONS`: the above works with explicit optical/input/power constraints.
- `POC0_REDESIGN_REQUIRED`: APK runs but the static edge portrait is not comfortable or legible after profile/layout adjustment.
- Remain `POC0_BLOCKED`: the current firmware cannot be connected/installed without an authenticated official CXR-S route.
