# 照片浮窗

## Identity

- **Name**: 照片浮窗
- **Version**: 0.1.0
- **Description**: 配合 Mac 端接收本地处理后的照片或 GIF 动图，并在 Rokid 眼镜中按指定位置、大小和透明度显示。
- **Author**: wenshuishi0528

## Capabilities

- **Permissions**: None
- **Skills**: None

## 功能边界

- Mac 端负责选图、绿色显示处理、参数调整和 USB 发送。
- AIUI 端只负责显示；Mac 通过 USB 生成并更新包含当前画面的本地 AIX。
- 不上传用户照片到云端，也不申请网络、相机或麦克风权限。
