# 照片浮窗（乐奇相片hud AIUI 版）

眼镜端 AIUI 应用只负责最终显示。Mac 端继续负责照片/GIF 选择、绿色显示处理、位置、大小、透明度和 USB 发送。

Mac 会直接把位置、大小、透明度和绿色处理合成为最终 448×352 PNG 或 GIF，再生成 AIX 并通过 USB 更新眼镜中的本地 AIUI 包。AIUI 只按原尺寸显示完整画面，不再对单独照片做二次缩放。该包会保留在眼镜中，因此退出再打开仍显示最后一次画面；用户照片不会上传到云端或写入源码仓库。

- AIUI 智能体名称：`照片浮窗`
- 正式 Agent ID：`fea33d142f1443b282eb9c3a62d54183`
- GitHub AIUI 版：`1.0.1`
- AIUI Studio：已提交的 `1.0.0` 当前仍为“审核中”
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
