# Input map

## Confirmed application behavior

| Input | Behavior | Evidence |
|---|---|---|
| Android back callback / key fallback | First back temporarily hides the portrait for the current run; a second back exits. Opening the launcher again restores the last portrait. Back from minimal/calibration first returns to portrait. | Verified on device with injected `KEYCODE_BACK`; first press produced a black screenshot without persisting `visible=false`, and both warm reopen and cold restart restored the same portrait. |
| ADB debug Intent | Whitelisted anchors, sizes, opacity values, booleans, assets, profiles, and display modes update/persist configuration. Unknown values are ignored. | Verified on device across six anchors, three sizes, four opacity values, minimal/calibration/portrait modes, and process restart. |

No Rokid touchpad, shutter button, confirm key, swipe, double-tap, or long-press code is bound yet. The device is online, but a complete human-performed gesture sequence was not captured during this run, so gesture-to-key binding would still be speculation.

## Live input-device inventory

- Touch controller: `/dev/input/event1`, `ROKID,PSOC-TP-R`, I2C, Android keyboard source `0x101`.
- Advertised Linux key capabilities: Enter, Up, Down, Left, Right, Back, Prog1/2/3, F13/F14, and Dashboard.
- Active key layout: `/system/usr/keylayout/Generic.kl`.
- Confirmed layout entries include Linux 28 → Android Enter, 103/105/106/108 → DPAD directions, 158 → Back, 148 → Prog Blue, and 184 → vendor `SPRITE_SWIPE_BACK`.
- A second `qpnp_pon` input exposes Volume Down and Menu; whether the physical photo key produces these events is still pending a raw capture.
- The debug probe successfully logged injected MotionEvent down/move/up and scroll gestures from source `0x1002`; these prove probe operation but are not evidence of the physical touchpad mapping.

## Debug-only input probe

Install the debug APK, then run:

```sh
adb shell am start -n dev.local.lockethud.poc/.InputProbeActivity
adb logcat -c
adb logcat -v time -s LocketHUD.Input:I '*:S'
```

While seated and stationary, perform each action once, then again to confirm repeatability:

1. touchpad single tap;
2. touchpad double tap;
3. touchpad long press;
4. swipe up, down, left, and right;
5. shutter/photo key short press;
6. the device's normal back action.

The probe reports `KeyEvent` action/keyCode/scanCode/repeat/source, `MotionEvent` action/coordinates/source/button state, and interpreted single/double/long/scroll gestures. It neither uploads data nor writes a file.

## Pending result table

| Physical action | Android event | Consistent? | System interception? | Binding decision |
|---|---|---:|---:|---|
| Single tap | Pending physical capture | Pending | Pending | None |
| Double tap | Pending | Pending | Pending | None |
| Long press | Pending | Pending | Pending | None |
| Swipe four directions | Pending | Pending | Pending | None |
| Photo key | Pending | Pending | Pending | None |
| Back | Injected Android Back verified; physical gesture pending | App fallback consistent | Pending | Generic fallback only |
