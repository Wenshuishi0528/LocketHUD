# Changelog

本文件记录 LocketHUD 用户可见的版本变化。日期采用 `YYYY-MM-DD`。

## [Mac 0.1.4 / AIUI 1.0.0] - 2026-08-12

### Added

- 增加 448×352 AIUI 眼镜显示端，支持静态照片和 GIF 动图、六个位置、三档大小、四档透明度。
- Mac 每次发送时会在本地生成包含当前处理画面和参数的 AIX，并通过 USB 安装到眼镜；最后发送的画面会保留在眼镜本地。
- AIUI 官方智能体使用名称“照片浮窗”，Agent ID 为 `fea33d142f1443b282eb9c3a62d54183`。
- AIUI 介绍和启动提示中加入软件下载地址：`https://github.com/Wenshuishi0528/LocketHUD`。

### Changed

- Mac 更新至 0.1.4，并改为向正式 AIUI Agent ID 发送；发送时会清理旧开发 ID `lockethud-photo`，避免眼镜出现两个同类入口。
- AIUI 智能体使用简洁的深色底、亮绿色描边微笑小人 icon；上传的预览素材和默认画面全部为程序生成的测试素材，不含私人照片。
- AIUI 应用不申请网络、相机、语音或麦克风权限。

### Published

- 已向 AIUI Studio 上传 icon、4 张测试预览图和 AIX，并提交 1.0.0 审核；当前状态为“审核中”。

### Verified

- AIUI 检查 2/2、AIX 官方格式验证、Mac Rust 测试 5/5、前端生产构建和 Tauri release 打包通过。
- DMG 完整性和 Mac App 严格签名检查通过。
- 已在 RG-glasses 实机发送并打开正式 Agent ID；迁移后设备索引只保留一个正式“乐奇相片hud”本地包。

## [Glasses 0.1.4] - 2026-08-12

### Fixed

- 针对审核“没有 icon”的反馈，将眼镜 APK 从 `@drawable` 矢量图标改为审核系统更容易识别的标准 `mipmap` PNG 图标。
- 增加 `android:roundIcon`，并提供可单独上传审核的 512×512 不透明 PNG 图标。

### Changed

- 图标简化为深色方形底和亮绿色微笑小人，不含文字、渐变、透明边缘或额外装饰。
- 原生 Android 眼镜端更新为 0.1.4（versionCode 5），作为兼容/回退方案；当前主线 Mac 端为 0.1.4，眼镜显示主线改用 AIUI 1.0.0。

## [0.1.3] - 2026-08-11

### Added

- Mac 导入支持 GIF 动图，并逐帧应用现有绿色、Gamma、对比度、锐化、量化和抖动处理。
- Mac 预览保留 GIF 动画；眼镜端使用 Android 原生动画解码循环播放。
- 眼镜端会记住最后发送的 GIF，后台重开或冷启动后继续显示和播放。
- 仓库加入 `CC BY 4.0` 许可证和作者署名要求。

### Changed

- Mac 和眼镜端版本统一更新为 0.1.3；Mac 界面版本号同步更新。

### Verified

- Rust GIF 双帧处理测试、现有 Rust 测试、Mac 前端构建和 Android 单元测试/构建通过。
- RG-glasses 实机连续截图证明 GIF 正在播放；强制停止后从启动器重开仍继续播放。验证前后原静态照片 SHA-256 完全一致。

## [0.1.2] - 2026-08-11

### Changed

- Mac 应用和眼镜端启动器的用户可见名称统一改为“乐奇相片hud”；内部包名和 bundle identifier 保持不变。
- Mac 与眼镜端统一使用绿色微笑小人图标；头部与身体改为相切连接，不再互相重叠。
- 恢复头部和身体的亮绿色实线描边。
- Mac 图标圆角方块外的白色背景改为真正透明像素。
- 眼镜端更新为 0.1.2（versionCode 3），Mac 端更新为 0.1.2。
- 修复眼镜端退出或退到后台后重新打开不显示上一张照片的问题；返回键隐藏现在只对当前运行有效。
- Mac 界面增加“版本 0.1.2”和作者“wenshuishi0528”，并同步写入应用作者元数据。

### Verified

- Mac 成品图标四角 alpha 为 0；头部、身体轮廓像素为 `#00FF70`。
- Mac 前端构建、Rust 单元测试 3/3、Tauri release 打包、严格签名检查和 DMG 校验通过。
- Android debug APK 构建通过；APK 标签为“乐奇相片hud”，已覆盖安装并在 RG-glasses 启动。
- RG-glasses 实机验证后台重开和完全停止后冷启动均恢复同一张照片，照片文件 SHA-256 保持不变。
- 公开 GitHub 源码仅包含源码、文档和程序生成的测试素材；Release 只附 DMG、APK 和校验文件，不包含用户照片、截图、日志或构建缓存。

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
