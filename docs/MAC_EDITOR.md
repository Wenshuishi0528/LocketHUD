# LocketHUD Mac editor

## Product boundary

The Mac editor owns photo selection, local green-screen processing, layout preview, display parameters, and transfer. The glasses application remains a small final display endpoint. V1 deliberately does not expose calibration, code, or parameter editing on the glasses.

## Delivered application

- Product: `LocketHUD Editor`
- Version: 0.1.1
- Architecture: Apple Silicon arm64
- Stack: Tauri 2, TypeScript/Vite, Rust
- Application bundle: `mac-editor/src-tauri/target/release/bundle/macos/LocketHUD Editor.app`
- Install image: `artifacts/LocketHUD-Editor-0.1.1-arm64.dmg`
- DMG size: 3,556,268 bytes
- DMG SHA-256: `df8c94021942cc033af298b478958833a533b7bb028f59e7ca52a4c5fa239d94`
- Signing: complete ad-hoc bundle signature; strict `codesign` verification passes
- Notarization: not notarized; local development use only

## Current workflow

1. Install and launch the LocketHUD Android display APK once.
2. Connect the glasses over an authorized USB data connection.
3. Open `LocketHUD Editor` on the Mac.
4. Select a PNG, JPEG, HEIC, WebP, or TIFF image.
5. Choose natural green, 8-level, 16-level, or dithered processing; optionally adjust gamma, contrast, and sharpening.
6. Set one of six positions, three sizes, four opacity levels, visibility, and foreground keep-screen-on.
7. Press “发送到眼镜”. The editor pushes a metadata-free processed PNG to app-specific storage and launches the display Activity with whitelisted parameters.

All image processing is local. The application does not add a cloud service, account, analytics, or network sync protocol. It stores UI parameter preferences locally but does not copy the selected source photo into the repository.

## Validation completed

```sh
cd mac-editor
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
codesign --verify --deep --strict "src-tauri/target/release/bundle/macos/LocketHUD Editor.app"
hdiutil verify "src-tauri/target/release/bundle/dmg/LocketHUD Editor_0.1.1_aarch64.dmg"
```

- TypeScript/Vite production build: pass.
- Rust unit tests: 3/3 pass.
- Real release window launch: pass.
- Real-window selection of a synthetic PNG and local 16-level processing: pass.
- Position, size, opacity, switches, persistent settings, and disconnected-device UI: pass.
- USB-present/ADB-unavailable detection: pass; the editor now asks for USB debugging authorization instead of incorrectly saying the glasses are unplugged.
- App bundle strict signature verification: pass.
- DMG filesystem/checksum verification: pass.
- Final ADB send: pending only because the glasses were disconnected after development began.

## V1 limitation

ADB is intentionally the first transport because it reuses the verified display app without adding a server or unauthenticated network port. A consumer wireless transport requires a separate paired/encrypted design and is not implied by this release.
