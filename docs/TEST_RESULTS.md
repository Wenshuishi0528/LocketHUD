# Test results

Status date: 2026-08-11

## Current decision

`POC0_PASS_WITH_LIMITATIONS`

G0 and the standard Android APK path passed on the connected Rokid RG-glasses. Installation, cold start, the full 480×640 viewport, six anchors, three distinct sizes, four opacity levels, hide/persistence, foreground keep-screen-on, synthetic private-PNG import, and a 30-minute stability run were verified on device. The result is limited because binocular optical quality and comfort require wearer observation, physical touchpad/key events were not captured, the endurance run was USB-powered, and the optional 60-minute run was not performed.

## Acceptance evidence

| Acceptance item | Status | Evidence |
|---|---|---|
| Independent repository/package/data boundary | Pass | Separate local Git repository; package `dev.local.lockethud.poc`; no remote. |
| Kotlin native Activity/Views | Pass | Debug APK compiles with AGP built-in Kotlin. |
| Six anchors | Device pass | Screenshots confirmed left/right top/middle/bottom placements outside the central 50% protection zone. |
| Three sizes | Device pass | After correcting a double-subtracted margin, screenshots measured approximately 49×98, 62×126, and 73×147 px; a regression test now protects distinctness. |
| Four opacity levels | Device pass | Screenshot maximum green values were 102, 153, 204, and 255 for 0.4/0.6/0.8/1.0. Optical preference remains pending. |
| Alpha/rectangular assets | Device pass, optical pending | Generated PNG resources, Pillow alpha tests, and non-black device composition pass; perceived transparency/ghosting requires wearer judgment. |
| Show/hide and persistence | Device pass | First Android Back hid, second Back exited, and force-stop/restart reproduced the saved portrait configuration. Twenty alternating ADB debug visibility updates kept the same process and produced no crash. Physical glasses gesture pending. |
| Foreground-only keep-screen-on | Device pass | Foreground window exposed `KEEP_SCREEN_ON`; disabling it allowed `mWakefulness=Asleep`, and re-enabling restored the intended foreground behavior. No WakeLock/service is used. |
| No sensitive permissions | Pass | Final merged Manifest has no `uses-permission` entries. |
| Build/lint/unit tests | Pass | Gradle build/lint/JUnit and Python tests pass. |
| Private photos excluded | Pass | `local_assets/*` ignored; repository resources are program-generated. |
| Device identification | Pass | Rokid RG-glasses, Android 12/API 32, arm64-v8a, user/release-key firmware; serial deliberately omitted. |
| Install/start/restart | Pass | Streamed ADB install succeeded; minimal cold start was 842 ms and Activity reached `RESUMED`. The final artifact passed uninstall/reinstall and a 514 ms cold start. |
| Actual View/insets/orientation | Pass | App View/surface measured 480×640, portrait rotation 0, with zero cutout inset. |
| Input map | Partial | Generic Android Back and debug Intents pass; live input inventory captured, but human physical gesture/key events remain unconfirmed and unbound. |
| Optical visibility/ghosting | User test pending | Screenshots prove render output, not binocular recognition, ghosting, comfort, or distraction. |
| 30/60 minute power and thermal | 30 min pass with limitation | 30:13 foreground run stayed Awake on the same PID with no crash; battery 85→100% while USB powered, temperature 25.0→22.5°C, thermal status 0. Unplugged drain and 60-minute evidence are pending. |

## User-observed results

Not yet collected. Complete the matrix in `TEST_PROTOCOL.md`; screenshots and code tests cannot substitute for the user's binocular comfort and distraction judgment. At minimum, record one acceptable or unacceptable combination across profile, anchor, size, opacity, and lighting condition.

## Endurance evidence

- Window: 18:11:00–18:41:13 local time (30 minutes 13 seconds).
- Configuration: default synthetic portrait, right-middle, Small, opacity 0.6, visible, clock off, foreground keep-screen-on enabled.
- Start: PID 4144, Awake, USB powered, battery 85%, 25.0°C, thermal status 0.
- End: same PID 4144, Awake, USB powered, battery 100%, 22.5°C, thermal status 0.
- Crash buffer contained no matching fatal/crash entry; the final screenshot hash matched the starting portrait screenshot.
- The battery increase is charging evidence, not a battery-life result.

## Conditions for a new decision

- Promote to `POC0_PASS` only after the user accepts at least one binocular optical combination, a physical quick-hide action is confirmed, and unplugged power behavior is acceptable.
- Keep `POC0_PASS_WITH_LIMITATIONS` while Android rendering is stable but optical/input/power constraints remain explicit.
- `POC0_REDESIGN_REQUIRED`: APK runs but the static edge portrait is not comfortable or legible after profile/layout adjustment.
- Return to `POC0_BLOCKED` only if a future firmware/device path prevents installation or operation and requires an unavailable authenticated CXR-S artifact.
