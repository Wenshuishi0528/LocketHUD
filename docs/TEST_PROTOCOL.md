# POC-0 test protocol

## Safety boundary

Perform all setup and comparisons while seated, stationary, and clear of obstacles. Do not debug while driving, cycling, crossing a street, using stairs, or moving. Stop immediately for eye strain, dizziness, headache, heat, or unsafe distraction.

## Phase 1: G0 and minimal APK

1. Complete the read-only audit in `HARDWARE_REPORT.md`.
2. Build and install the debug APK.
3. Start `--es mode minimal` and record Activity start result, first visible time, actual runtime View size from `LocketHUD.Main`, orientation, clipping, and scaling.
4. Exit, relaunch, and confirm the black background and green `乐奇相片hud` text.

## Phase 2: input probe

Use `INPUT_MAP.md`. Do not bind a physical input until at least two repetitions produce the same Android event and the action does not steal a safety-critical system function.

## Phase 3: calibration

Start calibration mode and record:

- whether black appears optically transparent/dark;
- whether both eyes see the full border and six edge boxes;
- top/bottom/left/right clipping and any single-eye loss;
- 1/2/3/4 px line visibility;
- distinguishable levels in the 8- and 16-step green strips;
- alpha test appearance and ghosting;
- Small/Medium/Large frame comfort.

An Android screenshot may verify drawing geometry but cannot pass optical testing.

## Phase 4: portrait matrix

Use ADB extras to test all six anchors, three sizes, and four opacity levels. Compare generated rectangular/transparent assets and the `natural-green`, `quantized-8`, `quantized-16`, and `dithered` outputs.

For each useful combination, record:

- face recognizability, including eyes, mouth, and hair;
- central-view obstruction;
- refocusing effort;
- ghosting or left/right eye mismatch;
- edge clipping;
- indoor light, bright window, and outdoor shade visibility.

Only after synthetic assets are safe and stable should a private processed PNG be imported.

## Phase 5: hide/recovery and persistence

1. Confirm the chosen verified input hides and restores the portrait with acceptable delay.
2. Repeat 20 times without crash, accidental exit, or lost system navigation.
3. Change anchor/size/opacity/keep-screen-on, exit, relaunch, and verify persistence.
4. Sleep/wake the display and confirm no crash.

## Phase 6: foreground keep-screen-on endurance

1. Record start time, battery percentage, power state, and thermal status.
2. Keep portrait Activity foreground for 30 minutes while stationary.
3. Record screen-off/dimming, crash/process exit, end battery, and thermal status.
4. If acceptable, repeat to 60 minutes.
5. Clear/disable keep-screen-on and verify normal sleep resumes.

Do not infer official battery life from this test. `FLAG_KEEP_SCREEN_ON` cannot override thermal, low-battery, or vendor power policy.
