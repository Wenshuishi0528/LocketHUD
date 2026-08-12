# 照片浮窗（乐奇相片hud AIUI 版）

眼镜端 AIUI 应用只负责最终显示。Mac 端继续负责照片/GIF 选择、绿色显示处理、位置、大小、透明度和 USB 发送。

Mac 应用发送时会在本地生成一个只包含当前画面和显示参数的 AIX，再通过 USB 更新眼镜中的本地 AIUI 包。该包会保留在眼镜中，因此退出再打开仍显示最后一次画面；用户照片不会上传到云端或写入源码仓库。

- AIUI 智能体名称：`照片浮窗`
- 正式 Agent ID：`fea33d142f1443b282eb9c3a62d54183`
- 版本：`1.0.0`
- 状态：已提交 AIUI Studio 审核，当前“审核中”
- Mac 软件下载：<https://github.com/Wenshuishi0528/LocketHUD>
- 权限：不申请网络、相机、语音或麦克风权限

```sh
npm install
npm run check
npm run pack:aix
npm run verify:aix
npm run device:install
```

发布包：`dist/lockethud-photo.aix`。
