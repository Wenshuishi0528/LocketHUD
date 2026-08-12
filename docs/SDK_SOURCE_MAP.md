# SDK source map

Checked 2026-08-11. Only official vendor/platform sources are accepted for proprietary API or dependency claims.

| Source | What was verified | Limitation / decision |
|---|---|---|
| [Rokid Open Platform](https://open.rokid.com/?lang=cn) | Official site was reachable. | Dynamic public page did not expose a CXR-S artifact version, dependency coordinate, sample build matrix, or license text to the non-authenticated inspection. |
| [Rokid CXR-S / Sprite](https://open.rokid.com/sprite?lang=zh) | Official CXR-S/Sprite entry was reachable; its served web frontend identified itself as open-platform-web 3.3.0. | 3.3.0 is a website bundle version, not evidence of a CXR-S SDK version. No SDK version is claimed. |
| [Rokid CXR-L](https://open.rokid.com/sdk?lang=en) | Official mobile SDK entry was reachable. | CXR-L/iPhone work is out of POC-0 scope and no CXR-L component is linked. |
| [乐奇 AI 眼镜技术规格](https://glasses.rokid.com/profile) | Public baseline lists AR1, 2 GB RAM, 32 GB ROM, Wi-Fi 6, Bluetooth 5.3, dual-eye green Micro-LED, 30° FOV, 1500 nits, and 480×640 resolution. | Product-page specifications do not replace live device/runtime evidence. |
| [Android: keep the screen on](https://developer.android.com/develop/background-work/background-tasks/awake/screen-on) | `FLAG_KEEP_SCREEN_ON` is an Activity/window mechanism and background apps may allow normal screen-off. | Implemented only while the portrait Activity is foreground; no WakeLock or service. |

## Local SDK audit

No official CXR-S SDK archive, AAR, dependency package, or sample project was found in the inspected local Downloads/Documents paths. An iOS CXR-L sample was found but not used.

## Implementation route

This POC uses only public Android SDK APIs and no invented Rokid API. This is the specification's allowed fallback: first test whether the exact firmware can install and run a standard Android APK. If it cannot, the next step is an authenticated download of the official CXR-S sample/template and its license/build requirements; do not substitute forum attachments or third-party mirrors.

Current CXR-S version, AGP/Kotlin/JDK matrix, min/target SDK requirements, license, and store/review implications remain `UNCONFIRMED`.
