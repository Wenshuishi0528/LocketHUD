# 乐奇相片hud 开发交接

更新时间：2026-08-11（America/Los_Angeles）

## 1. 当前结论

- 眼镜端 POC：`POC0_PASS_WITH_LIMITATIONS`。
- Mac 端：`V1_MAC_EDITOR_IMPLEMENTED`，用户已用自己选择的样本照片成功发送到眼镜。
- 用户确认默认绿色小人在眼镜中没有重影。
- 当前产品边界：Mac 负责选图、处理、预览、参数和发送；眼镜应用只负责最终显示。
- 不再继续眼镜端 Input Probe 或物理触控映射。用户明确认为这些调整应在 Mac 完成。

## 2. 仓库与隐私边界

- 仓库：`/Users/apple/Documents/乐奇AI眼镜开发/LocketHUD-POC`
- 分支：`main`
- 私有 GitHub 备份：`https://github.com/Wenshuishi0528/LocketHUD-Backup`
- GitHub 只备份源码、文档和程序生成的测试素材；不上传用户照片、APK、DMG、截图、日志或构建缓存。
- Android 包名：`dev.local.lockethud.poc`
- Mac bundle identifier：`dev.local.lockethud.mac`
- 用户私人照片不得放入 Git、Android resources、测试资源或任何远程仓库。
- `local_assets/`、APK、DMG、截图和构建目录均被 Git 忽略。

## 3. 目录与架构

- `glasses-app/`：Kotlin/Android 原生显示端，普通全屏 Activity + Canvas/View。
- `mac-editor/`：Tauri 2、TypeScript/Vite 前端、Rust 后端。
- `tools/prepare_portrait.py`：Pillow 离线图片处理 CLI。
- `artifacts/`：本地 APK、DMG 和实机截图，不提交 Git。
- `docs/`：硬件、构建、SDK、测试和设计决策记录。
- `CHANGELOG.md`：版本变化。

Mac 编辑器通过 Rust 命令执行三件事：

1. 使用 macOS `sips` 规范化用户选择的图片，再在本地进行绿色单色、Gamma、对比度、锐化、量化或抖动处理。
2. 检测 RG-glasses 的 USB 枚举、ADB 状态和眼镜显示 APK。
3. 用 ADB 将处理后的 `current.png` 推送到应用专属目录，再用白名单 Intent extras 启动显示 Activity。

没有云端、账号、分析 SDK、网络同步服务或无认证监听端口。

## 4. 当前版本和构建物

- Mac 编辑器：0.1.2，用户可见名称“乐奇相片hud”，Apple Silicon arm64。
- `/Applications/乐奇相片hud.app` 已安装并启动；旧英文版已移入废纸篓，可恢复。
- Android 显示端：0.1.1（versionCode 2），用户可见名称“乐奇相片hud”。
- Mac 图标源：`mac-editor/src-tauri/icons/lockethud-source.svg`。
- Mac 和 Android 启动图标均使用绿色微笑小人；头身相切连接并保留亮绿色轮廓。
- Mac 图标圆角外区域为真实透明像素，不是白底。
- App Bundle 已包含 `Contents/Resources/icon.icns`，其 SHA-256 与源码生成的 `icon.icns` 一致。
- Mac DMG：`artifacts/乐奇相片hud-0.1.2-arm64.dmg`。
- Mac DMG 大小：2,330,403 bytes。
- Mac DMG SHA-256：`d209575dbcd8e80c482f12ac5346558bbe124a65975d0f4b7e77216dc4653adc`。
- Android APK：`artifacts/乐奇相片hud-Glasses-0.1.1-debug.apk`。
- Android APK 大小：2,541,833 bytes。
- Android APK SHA-256：`68696ac9b1d813472d2cce72ded9ef0745e4cf143d16817164ed24e0c443689c`。

DMG 是完整 ad-hoc 签名但未经过 Apple notarization，只用于当前 Mac 本地安装。不要将其描述为可公开分发或已公证版本。

## 5. 构建与验证

Mac 编辑器：

```sh
cd "/Users/apple/Documents/乐奇AI眼镜开发/LocketHUD-POC/mac-editor"
npm install
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

发布检查：

```sh
codesign --verify --deep --strict \
  "src-tauri/target/release/bundle/macos/乐奇相片hud.app"
hdiutil verify \
  "src-tauri/target/release/bundle/dmg/乐奇相片hud_0.1.2_aarch64.dmg"
```

Android 显示端仅在相关代码改变时重跑：

```sh
cd "/Users/apple/Documents/乐奇AI眼镜开发/LocketHUD-POC"
./gradlew clean assembleDebug lintDebug testDebugUnitTest
python3 -m unittest discover -s tools/tests -v
```

不要为了 Mac UI、文档或图标修改而重复 Android 全套审计。

## 6. 已验证行为

- 标准 Android APK 可在 Rokid RG-glasses 安装、启动、退出、卸载和重装。
- 眼镜 View 为 480×640；六位置、三大小和四透明度均实机通过。
- Mac release 窗口实际打开过；选图、16 级量化处理、位置、大小和透明度控件通过 GUI 验证。
- Mac 编辑器能区分 USB 未连接、USB 已连接但 ADB 不可用、ADB 在线但 APK 未安装、完全就绪四类状态。
- 用户已确认从 Mac 编辑器发送样本照片到眼镜成功。

## 7. 连接注意事项

- “插着线”不等于 ADB 在线。Mac 可能能看到 `RG-glasses-IDP`，但 `adb devices -l` 为空。
- 若 USB 存在但 ADB 为空：在眼镜中关闭再开启 USB 调试，确认这台 Mac 的 RSA 授权，必要时拔插一次数据线。
- 尽量直连 Mac，避免不稳定扩展坞。
- 编辑器每约 4 秒自动刷新设备状态。
- USB/ADB 状态会随眼镜休眠或重新插线改变，交接文件中的在线状态不是永久事实。

## 8. 已知限制

- V1 使用 ADB，仅是开发传输方式，不是消费者无线同步方案。
- Mac App 为 ad-hoc 签名、未公证。
- 编辑器重启后会保留参数，但当前不会自动重新载入上次选择的源照片；用户需重新选择。
- 30 分钟稳定性测试时 USB 正在充电，不能据此给出脱线续航结论。
- 60 分钟和不插 USB 的耗电测试尚未完成。

## 9. 下一步优先级

1. 请用户目视确认带亮绿色描边的小人图标在访达、Dock、DMG 和眼镜启动器中符合预期；0.1.2 已安装并打开。
2. 用用户选择的样本照片再做一次 0.1.2 发送冒烟测试。
3. 若用户需要，增加“恢复上次照片”或最近照片列表；仍须保持源照片仅在本地。
4. 只有用户明确要求后，才设计配对和加密的消费者传输；不要直接开放无认证局域网端口。
5. 在不插 USB 的情况下测量用户可接受的实际续航，再决定是否移除 `POC0_PASS_WITH_LIMITATIONS`。

## 10. 明确不要做

- 不再做眼镜 Input Probe、触摸板/拍照键映射或眼镜内参数界面，除非用户重新改变产品决定。
- 不重复与当前修改无关的环境、SDK、Android 或硬件审计；优先直接研发和目标验证。
- 不引入 iPhone、云、账号、地图、AI、动画、后台服务、系统 overlay 或开机自启动。
- 不 Root、不刷机、不修改系统分区，不使用来源不明的 Rokid SDK。
- 不公开发布代码，不上传 APK、DMG、日志或私人照片；仅按用户授权维护私有源码备份。
