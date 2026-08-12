# 乐奇相片hud / 照片浮窗开发交接

更新时间：2026-08-12（America/Los_Angeles）

## 1. 当前结论

- Mac 软件：`乐奇相片hud` 0.1.4，作者 `wenshuishi0528`。
- AIUI 智能体：`照片浮窗` 1.0.0，Agent ID `fea33d142f1443b282eb9c3a62d54183`。
- AIUI Studio 账号：`wenshuishi26`；icon、预览图和 AIX 已上传并正式提交，当前状态为“审核中”。审核通过前不能描述为已公开上架。
- 产品边界保持不变：Mac 负责选图、GIF 解码、图像处理、位置、大小、透明度和 USB 发送；眼镜端只负责最终显示。
- 原生 Android `glasses-app/` 仍保留为兼容/回退方案，当前主线改为 AIUI。

## 2. 仓库与隐私边界

- 本地仓库：`/Users/apple/Documents/乐奇AI眼镜开发/LocketHUD-POC`
- GitHub：`https://github.com/Wenshuishi0528/LocketHUD`
- 用户私人照片不得进入 Git、发布素材、测试资源或远程仓库。
- AIUI 商店包、icon 和预览使用的都是绿色小人等程序生成素材，不含私人照片。
- 用户选择的照片/GIF 只在 Mac 本地处理，并通过 USB 写入用户自己的眼镜；AIUI 应用不申请网络、相机、语音或麦克风权限。
- 项目采用 CC BY 4.0，署名为 `wenshuishi0528`；第三方依赖保持各自许可证。

## 3. 当前架构

- `mac-editor/`：Tauri 2 + TypeScript/Vite + Rust。导入 PNG、JPEG、HEIC、WebP 或 GIF，完成绿色显示处理并预览 448×352 AIUI 画面。
- `aiui-app/`：AIUI 显示代码、测试和基础 AIX。固定 448×352，支持六个位置、96/140/190 三档大小、40/60/80/100 四档透明度。
- `store-assets/`：AIUI icon 源文件和 512×512 上传成品。
- `test-assets/synthetic-portraits/`：程序生成的绿色小人测试素材。
- `glasses-app/`：旧的原生 Android 显示端，保留但不再作为主发布路径。
- `artifacts/`：本地成品和验证截图，默认不提交 Git。

每次点击“发送到眼镜”时，Mac 会：

1. 在本地处理静态照片或 GIF 的全部帧，并应用当前显示参数。
2. 动态生成一个包含画面、参数和 AIUI 代码的 AIX。
3. 通过 USB/ADB 写入眼镜 AIUI 本地目录，更新 Agent 索引并打开正式 Agent ID。
4. 删除旧开发 ID `lockethud-photo` 的索引和文件，防止出现重复入口。

动态 AIX 会保留在眼镜本地，所以退出后再次打开仍能看到最后发送的画面。没有云端图片上传路径。

## 4. AIUI 发布资料

- 智能体名称：`照片浮窗`
- 版本：`1.0.0`
- 分类：`生活`
- Agent ID：`fea33d142f1443b282eb9c3a62d54183`
- 当前状态：`审核中`
- 软件下载地址：`https://github.com/Wenshuishi0528/LocketHUD`
- 功能介绍：

  > 照片浮窗配合「乐奇相片hud」Mac 软件使用，可通过 USB 把本地照片或 GIF 动图发送到乐奇眼镜，并设置显示位置、大小和透明度；退出后再次打开仍会保留最后发送的画面。软件下载地址：https://github.com/Wenshuishi0528/LocketHUD

- 开场提示：

  > 照片浮窗已启动。请先在 Mac 安装并打开「乐奇相片hud」，连接眼镜后选择照片或 GIF，调整位置、大小和透明度，再点击「发送到眼镜」。软件下载：https://github.com/Wenshuishi0528/LocketHUD

若审核通过，不需要重复提交。若被退回，只针对明确反馈修改并重新提交。

## 5. 最终成品

- Mac DMG：`artifacts/LocketHUD-0.1.4-arm64.dmg`
  - 大小：2,405,155 bytes
  - SHA-256：`c9a8dee76e436f98bd2a2af181876c8522f32ad0cab620b4bd7a7e780bfc60f4`
- AIUI AIX：`artifacts/PhotoFloatingWindow-AIUI-1.0.0.aix`
  - 大小：20,587 bytes
  - SHA-256：`3c2e7e8f49b7f724cb53bf71c11e855f3a56e0125d98bfe6d13adb2514e381d1`
  - 内部 VERSION：`6926bd26-a440-4c59-ac8c-dcd04052ec39`
- AIUI icon：`artifacts/PhotoFloatingWindow-icon-512.png`
  - 512×512 PNG，大小 15,747 bytes
  - SHA-256：`41382878348562ca461b043cafd9eae3eb31355b2ee12e851a37aaae2a979cec`
- 本机安装：`/Applications/乐奇相片hud.app`

Mac App 为完整 ad-hoc 签名但没有 Apple notarization。它可用于当前开发 Mac 和测试分发，但不要描述为 Apple 公证版本。

## 6. 构建与验证

AIUI：

```sh
cd "/Users/apple/Documents/乐奇AI眼镜开发/LocketHUD-POC/aiui-app"
npm install
npm run check
npm run pack:aix
npm run verify:aix
```

Mac：

```sh
cd "/Users/apple/Documents/乐奇AI眼镜开发/LocketHUD-POC/mac-editor"
npm install
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

成品检查：

```sh
codesign --verify --deep --strict "/Applications/乐奇相片hud.app"
hdiutil verify "/Users/apple/Documents/乐奇AI眼镜开发/LocketHUD-POC/artifacts/LocketHUD-0.1.4-arm64.dmg"
```

已完成的必要验证：

- AIUI 检查 2/2 通过，AIX 官方格式验证通过，共 10 个条目。
- Rust 测试 5/5、Mac 前端生产构建、Tauri release 打包通过。
- Mac App 严格签名检查和 DMG 完整性检查通过。
- RG-glasses 实机发送成功，正式 Agent ID 能启动和显示。
- 设备迁移清理后，本地 AIUI 索引只保留正式 Agent ID，没有旧开发入口。
- AIUI Studio 指令测试“打开照片浮窗”通过，并显示“提交成功”。

## 7. 连接与已知限制

- V1 仍使用 USB/ADB 作为开发传输方式；插上线不代表 ADB 已授权。
- 若 Mac 能看到 USB 设备但 ADB 为空，请在眼镜中重新开启 USB 调试并确认这台 Mac 的 RSA 授权。
- 用户动态画面包保留在眼镜本地；眼镜固件更新、清理调试数据或恢复出厂可能清除它，重新连接 Mac 发送即可恢复。
- AIUI 商店当前仍在审核；只有审核通过后，其他用户才能从官方入口公开安装。
- Mac App 未做 Apple Developer ID 签名和 notarization。
- 编辑器会保留参数，但重启后不会自动重新载入上次选择的源照片。

## 8. 下一步

1. 等待 AIUI 审核结果。
2. 若审核通过，确认公开页面名称、icon、介绍和下载链接显示正确。
3. 若审核退回，只修复审核明确指出的问题，不重复无关审计。

## 9. 明确不要做

- 不把参数调整界面放到眼镜端。
- 不上传用户私人照片、日志或实机截图到公开仓库/AIUI 发布素材。
- 不引入云端图片存储、账号、分析 SDK、无认证局域网端口、Root、刷机或系统分区修改。
- 不重复与当前修改无关的 Android、SDK 或硬件审计。
