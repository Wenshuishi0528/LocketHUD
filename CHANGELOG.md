# Changelog

本文件记录 LocketHUD 用户可见的版本变化。日期采用 `YYYY-MM-DD`。

## [0.1.1] - 2026-08-11

### Added

- 为 Mac 应用重绘独立图标：以眼镜 POC 中的绿色微笑小人为主体，适配 macOS 的方形图标和不同显示尺寸。
- 增加 USB 物理连接检测；可以区分“没有插眼镜”和“USB 已连接但 ADB/USB 调试未授权”。
- 增加根目录 `CHANGELOG.md` 和 `HANDOFF.md`。

### Changed

- Mac 编辑器连接提示改为可执行的故障说明，不再把所有 ADB 离线状态统称为“未连接眼镜”。
- 更新本地 `.app` 与 DMG 使用的新小人图标。
- 修正 Tauri bundle 配置，确保 `icon.icns` 真正进入 App Bundle，而不是只生成在源码目录。

### Verified

- 用户已使用自己选择的样本照片从 Mac 编辑器成功发送到眼镜。
- Mac 能识别 RG-glasses 的 USB 和 ADB 连接；眼镜端显示 APK 已安装。

## [0.1.0] - 2026-08-11

### Added

- 首个 LocketHUD Mac 编辑器：本地选图、预览和发送。
- 支持 PNG、JPEG、HEIC、WebP 和 TIFF 输入。
- 支持自然绿色、8 级量化、16 级量化和抖动处理。
- 支持 Gamma、对比度、锐化、六个位置、三档大小、四档透明度、显示/隐藏和前台常亮控制。
- 通过 ADB 将处理后的 PNG 写入眼镜应用专属目录，并启动眼镜显示 Activity。
- 提供 Apple Silicon `.app` 和 DMG 本地构建物。

### Verified

- TypeScript/Vite production build 通过。
- Rust 单元测试 3/3 通过。
- Release 窗口、选图、本地处理和布局控件完成真实 GUI 验证。
- App Bundle 严格 ad-hoc 签名校验和 DMG 校验通过。

## [Glasses POC 0.1.0] - 2026-08-11

### Added

- Kotlin/Android 原生眼镜显示端。
- 静态 PNG、六个位置、三档大小、四档透明度、持久化、显示/隐藏和前台常亮。
- 校准模式、本地图片处理 CLI 和 debug 私人图片导入工具。

### Verified

- Rokid RG-glasses，Android 12 / API 32，480×640 实机安装和显示通过。
- 用户确认默认绿色小人没有重影。
- 30 分 13 秒 USB 供电稳定性测试无崩溃、无自动熄屏、热状态为 0。
