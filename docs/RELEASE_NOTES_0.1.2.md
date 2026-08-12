# 乐奇相片hud 0.1.2

首个公开发布版本，包含 Mac 相片编辑器和 Rokid 眼镜显示端。

## 主要功能

- 在 Mac 本地选择、处理和预览相片，调整位置、大小、透明度与绿色显示效果。
- 通过已授权的 USB/ADB 连接发送到 Rokid 眼镜。
- 眼镜端只负责最终显示；退出或退到后台后重新打开，会恢复上一张照片。
- Mac 与眼镜端统一使用带亮绿色描边的微笑小人图标。
- Mac 图标圆角外为透明背景。
- Mac 软件内显示版本 `0.1.2` 和作者 `wenshuishi0528`。

## 下载

- `LocketHUD-0.1.2-arm64.dmg`：Apple Silicon Mac 安装包，安装后名称为“乐奇相片hud”。
- `LocketHUD-Glasses-0.1.2-debug.apk`：Rokid 眼镜 Android 安装包，启动器名称为“乐奇相片hud”。
- `SHA256SUMS.txt`：两个安装包的 SHA-256 校验值。

## 注意

- Mac 应用使用 ad-hoc 签名，尚未经过 Apple notarization。
- 眼镜端是 debug APK，当前传输方式依赖 ADB，适合开发测试。
- 仓库与 Release 不包含用户私人照片；只包含源码、文档、程序生成测试素材和安装包。
