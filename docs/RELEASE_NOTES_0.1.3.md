# 乐奇相片hud 0.1.3

本版本增加 GIF 动图导入和眼镜端动画播放，并采用 CC BY 4.0 许可证。

## 新功能

- Mac 文件选择器可导入 GIF 动图。
- GIF 每一帧都会应用当前的绿色显示、Gamma、对比度、锐化、量化或抖动设置。
- Mac 预览保持动画，发送后眼镜循环播放。
- 位置、大小、透明度、显示/隐藏和前台常亮设置同样适用于 GIF。
- 眼镜端会记住最后发送的 GIF，后台重开和冷启动后继续显示。

## 已验证

- Mac 逐帧处理测试确认输出 GIF 保留多帧。
- RG-glasses 实机连续截图确认动画正在播放，强制停止后从启动器重开仍继续播放。
- GIF 测试前后原静态照片文件校验值保持一致。

## 下载

- `LocketHUD-0.1.3-arm64.dmg`：Apple Silicon Mac 安装包。
- `LocketHUD-Glasses-0.1.3-debug.apk`：Rokid 眼镜 Android 安装包。
- `SHA256SUMS-0.1.3.txt`：安装包 SHA-256 校验值。

## 许可证

© 2026 `wenshuishi0528`。项目采用 [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)（CC BY 4.0）许可证。转载、修改或再发布时须保留署名、许可证链接，并说明是否修改。

## 注意

- Mac 应用使用 ad-hoc 签名，尚未经过 Apple notarization。
- 眼镜端仍为 debug APK，传输依赖已授权的 USB/ADB 连接。
- Release 不包含用户导入的照片或 GIF。
