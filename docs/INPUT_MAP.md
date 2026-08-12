# Input map

## Confirmed application behavior

| Input | Behavior | Evidence |
|---|---|---|
| Android back callback / key fallback | First back hides the portrait and persists `visible=false`; a second back exits. Back from minimal/calibration first returns to portrait. | Implemented and unit/build checked; device event delivery not yet verified. |
| ADB debug Intent | Whitelisted anchors, sizes, opacity values, booleans, assets, profiles, and display modes update/persist configuration. Unknown values are ignored. | Code and configuration unit tests pass; device delivery not yet verified. |

No Rokid touchpad, shutter button, confirm key, swipe, double-tap, or long-press code is bound. The device was offline, so assigning a proprietary key code would be speculation.

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
| Single tap | Pending | Pending | Pending | None |
| Double tap | Pending | Pending | Pending | None |
| Long press | Pending | Pending | Pending | None |
| Swipe four directions | Pending | Pending | Pending | None |
| Photo key | Pending | Pending | Pending | None |
| Back | Pending | Pending | Pending | Generic fallback only |
