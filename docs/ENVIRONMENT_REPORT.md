# Environment report

Audit date: 2026-08-11 (America/Los_Angeles)

## Mac and tools

| Item | Observed value |
|---|---|
| macOS | 15.7.5, build 24G624 |
| CPU | Apple Silicon, arm64 |
| Xcode | 26.1.1, build 17B100 |
| Android Studio | 2026.1 |
| System `java` | Amazon Corretto 11.0.26 |
| Project Gradle runtime | Android Studio bundled JBR 21.0.10 |
| Gradle Wrapper | 9.6.1 |
| Android Gradle Plugin | 9.2.0 |
| Android SDK | `/Users/apple/Library/Android/sdk` |
| Installed platforms used/available | android-32, android-36, android-36.1 |
| Build Tools observed | 32.0.0, 36.0.0, 36.1.0, 37.0.0 |
| adb | 1.0.41 / 37.0.0-14910828, arm64 |
| Python | 3.12.7 |
| Pillow | 10.4.0 |

The host's default Java 11 cannot run Gradle 9.6.1. This repository sets `org.gradle.java.home` to Android Studio's bundled JBR. No system Java installation was upgraded or replaced.

## Android build baseline

- Namespace/application ID: `dev.local.lockethud.poc`
- Version: 0.1.0 (1)
- minSdk: 32
- compileSdk/targetSdk: 36
- Java bytecode target: 17
- Kotlin compilation: Android Gradle Plugin built-in Kotlin support
- No local CXR-S AAR, SDK archive, or official sample was found in the inspected Downloads/Documents paths.
- A local CXR-L iOS sample exists in Downloads, but CXR-L and iPhone work are explicitly out of scope and were not used.

## Environment status

`HOST_BUILD_READY`: the Mac can build, lint, test, sign, and inspect the debug APK. Device readiness is tracked separately in `HARDWARE_REPORT.md`.
