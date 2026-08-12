import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./styles.css";

type Anchor =
  | "left_top"
  | "left_middle"
  | "left_bottom"
  | "right_top"
  | "right_middle"
  | "right_bottom";
type PortraitSize = "small" | "medium" | "large";
type Profile = "natural-green" | "quantized-8" | "quantized-16" | "dithered";

interface EditorSettings {
  anchor: Anchor;
  size: PortraitSize;
  opacity: 0.4 | 0.6 | 0.8 | 1;
  profile: Profile;
  gamma: number;
  contrast: number;
  sharpen: number;
  keepScreenOn: boolean;
  visible: boolean;
}

interface PreparedPortrait {
  data_url: string;
  output_path: string;
  width: number;
  height: number;
  sha256: string;
  animated: boolean;
}

interface DeviceStatus {
  connected: boolean;
  usb_connected: boolean;
  model: string | null;
  package_installed: boolean;
  message: string;
}

interface SendResult {
  message: string;
}

const defaults: EditorSettings = {
  anchor: "right_middle",
  size: "small",
  opacity: 0.6,
  profile: "quantized-16",
  gamma: 1,
  contrast: 1,
  sharpen: 0.35,
  keepScreenOn: true,
  visible: true,
};

const saved = localStorage.getItem("lockethud-editor-settings");
let settings: EditorSettings = defaults;
if (saved) {
  try {
    settings = { ...defaults, ...(JSON.parse(saved) as Partial<EditorSettings>) };
  } catch {
    localStorage.removeItem("lockethud-editor-settings");
  }
}

let sourcePath: string | null = null;
let prepared: PreparedPortrait | null = null;
let deviceReady = false;
let processingTimer: number | undefined;
let processingSequence = 0;

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <main class="app-shell">
    <header class="topbar">
      <div class="brand">
        <span class="brand-mark" aria-hidden="true">
          <svg viewBox="0 0 32 32"><path d="M8 16c0-4.8 3.2-8 8-8s8 3.2 8 8-3.2 8-8 8-8-3.2-8-8Z"/><path d="M5 16h3m16 0h3M16 5v3m0 16v3"/></svg>
        </span>
        <div>
          <h1>乐奇相片hud</h1>
          <p>Mac 相片编辑器 <span class="brand-meta">版本 0.1.4 · 作者 wenshuishi0528</span></p>
        </div>
      </div>
      <button class="device-pill" id="refresh-device" type="button" aria-label="刷新眼镜连接状态">
        <span class="status-dot"></span>
        <span id="device-label">正在检测眼镜…</span>
        <svg viewBox="0 0 20 20" aria-hidden="true"><path d="M16 10a6 6 0 1 1-1.76-4.24M16 3v4h-4"/></svg>
      </button>
    </header>

    <section class="workspace">
      <div class="preview-column">
        <div class="section-heading">
          <div>
            <span class="eyebrow">眼镜画面</span>
            <h2>实时构图预览</h2>
          </div>
          <span class="resolution">AIUI · 448 × 352</span>
        </div>

        <div class="preview-frame">
          <div class="glasses-preview" id="glasses-preview">
            <div class="safe-zone"><span>中央保护区</span></div>
            <div class="edge-label left">左侧</div>
            <div class="edge-label right">右侧</div>
            <img id="preview-image" src="/portrait_default.png" alt="处理后的人像预览" />
          </div>
        </div>
        <p class="preview-note">预览用于构图和明暗比较；最终亮度以眼镜实际显示为准。</p>

        <div class="photo-card">
          <div class="photo-icon" aria-hidden="true">
            <svg viewBox="0 0 24 24"><rect x="3" y="4" width="18" height="16" rx="3"/><circle cx="9" cy="10" r="2"/><path d="m5 18 5-5 3 3 2-2 4 4"/></svg>
          </div>
          <div class="photo-info">
            <strong id="photo-name">内置测试人像</strong>
            <span id="photo-meta">选择 PNG、JPEG、HEIC、WebP 或 GIF 动图</span>
          </div>
          <button class="secondary-button" id="choose-photo" type="button">选择照片</button>
        </div>
      </div>

      <aside class="controls-panel">
        <div class="control-scroll">
          <section class="control-section">
            <div class="control-title">
              <span>显示位置</span>
              <small>避开中央视野</small>
            </div>
            <div class="anchor-grid" role="group" aria-label="显示位置">
              <button type="button" data-anchor="left_top"><span class="anchor-dot top left"></span>左上</button>
              <button type="button" data-anchor="right_top"><span class="anchor-dot top right"></span>右上</button>
              <button type="button" data-anchor="left_middle"><span class="anchor-dot middle left"></span>左中</button>
              <button type="button" data-anchor="right_middle"><span class="anchor-dot middle right"></span>右中</button>
              <button type="button" data-anchor="left_bottom"><span class="anchor-dot bottom left"></span>左下</button>
              <button type="button" data-anchor="right_bottom"><span class="anchor-dot bottom right"></span>右下</button>
            </div>
          </section>

          <section class="control-section split-controls">
            <div>
              <div class="control-title"><span>人像大小</span></div>
              <div class="segmented" id="size-control" role="group" aria-label="人像大小">
                <button type="button" data-size="small">小</button>
                <button type="button" data-size="medium">中</button>
                <button type="button" data-size="large">大</button>
              </div>
            </div>
            <div>
              <div class="control-title"><span>透明度</span></div>
              <div class="segmented" id="opacity-control" role="group" aria-label="透明度">
                <button type="button" data-opacity="0.4">40</button>
                <button type="button" data-opacity="0.6">60</button>
                <button type="button" data-opacity="0.8">80</button>
                <button type="button" data-opacity="1">100</button>
              </div>
            </div>
          </section>

          <section class="control-section">
            <div class="control-title">
              <span>绿色显示处理</span>
              <small id="processing-state">等待选择照片</small>
            </div>
            <select id="profile-control" aria-label="绿色显示处理方式">
              <option value="natural-green">自然绿色</option>
              <option value="quantized-8">8 级量化</option>
              <option value="quantized-16">16 级量化</option>
              <option value="dithered">8 级抖动</option>
            </select>

            <label class="range-control">
              <span>Gamma <output id="gamma-value">1.00</output></span>
              <input id="gamma-control" type="range" min="0.6" max="1.8" step="0.05" />
            </label>
            <label class="range-control">
              <span>对比度 <output id="contrast-value">1.00</output></span>
              <input id="contrast-control" type="range" min="0.6" max="1.8" step="0.05" />
            </label>
            <label class="range-control">
              <span>锐化 <output id="sharpen-value">0.35</output></span>
              <input id="sharpen-control" type="range" min="0" max="1.5" step="0.05" />
            </label>
          </section>

          <section class="control-section switch-section">
            <label class="switch-row">
              <span><strong>眼镜持续显示</strong><small>仅在显示应用位于前台时常亮</small></span>
              <input id="keep-screen-control" type="checkbox" />
              <i aria-hidden="true"></i>
            </label>
            <label class="switch-row">
              <span><strong>显示人像</strong><small>关闭后同步为黑屏隐藏状态</small></span>
              <input id="visible-control" type="checkbox" />
              <i aria-hidden="true"></i>
            </label>
          </section>
        </div>

        <footer class="send-area">
          <div class="send-status" id="send-status">连接眼镜后即可发送</div>
          <button class="primary-button" id="send-button" type="button" disabled>
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="m4 12 16-8-6 16-3-7-7-1Z"/><path d="m11 13 9-9"/></svg>
            <span>发送到眼镜</span>
          </button>
        </footer>
      </aside>
    </section>
  </main>
`;

function element<T extends HTMLElement>(selector: string): T {
  const found = document.querySelector<T>(selector);
  if (!found) throw new Error(`Missing element: ${selector}`);
  return found;
}

const previewImage = element<HTMLImageElement>("#preview-image");
const devicePill = element<HTMLButtonElement>("#refresh-device");
const deviceLabel = element<HTMLSpanElement>("#device-label");
const sendButton = element<HTMLButtonElement>("#send-button");
const sendStatus = element<HTMLDivElement>("#send-status");
const processingState = element<HTMLElement>("#processing-state");
const profileControl = element<HTMLSelectElement>("#profile-control");
const gammaControl = element<HTMLInputElement>("#gamma-control");
const contrastControl = element<HTMLInputElement>("#contrast-control");
const sharpenControl = element<HTMLInputElement>("#sharpen-control");
const keepScreenControl = element<HTMLInputElement>("#keep-screen-control");
const visibleControl = element<HTMLInputElement>("#visible-control");

function saveSettings(): void {
  localStorage.setItem("lockethud-editor-settings", JSON.stringify(settings));
}

function syncControls(): void {
  document.querySelectorAll<HTMLButtonElement>("[data-anchor]").forEach((button) => {
    button.classList.toggle("active", button.dataset.anchor === settings.anchor);
  });
  document.querySelectorAll<HTMLButtonElement>("[data-size]").forEach((button) => {
    button.classList.toggle("active", button.dataset.size === settings.size);
  });
  document.querySelectorAll<HTMLButtonElement>("[data-opacity]").forEach((button) => {
    button.classList.toggle("active", Number(button.dataset.opacity) === settings.opacity);
  });
  profileControl.value = settings.profile;
  gammaControl.value = String(settings.gamma);
  contrastControl.value = String(settings.contrast);
  sharpenControl.value = String(settings.sharpen);
  element<HTMLOutputElement>("#gamma-value").value = settings.gamma.toFixed(2);
  element<HTMLOutputElement>("#contrast-value").value = settings.contrast.toFixed(2);
  element<HTMLOutputElement>("#sharpen-value").value = settings.sharpen.toFixed(2);
  keepScreenControl.checked = settings.keepScreenOn;
  visibleControl.checked = settings.visible;
  sendButton.querySelector("span")!.textContent = settings.visible ? "发送到眼镜" : "同步并隐藏";
  updatePreview();
}

function updatePreview(): void {
  const stageWidth = 448;
  const stageHeight = 352;
  const margin = 18;
  const targetWidth = { small: 96, medium: 140, large: 190 }[settings.size];
  const aspect = previewImage.naturalWidth && previewImage.naturalHeight
    ? previewImage.naturalWidth / previewImage.naturalHeight
    : 0.5;
  let width = targetWidth;
  let height = width / Math.max(0.05, aspect);
  const maxHeight = stageHeight - margin * 2;
  if (height > maxHeight) {
    height = maxHeight;
    width = height * aspect;
  }
  const isLeft = settings.anchor.startsWith("left");
  const vertical = settings.anchor.split("_")[1];
  const left = isLeft ? margin : stageWidth - margin - width;
  const top = vertical === "top"
    ? margin
    : vertical === "bottom"
      ? stageHeight - margin - height
      : (stageHeight - height) / 2;

  previewImage.style.left = `${(left / stageWidth) * 100}%`;
  previewImage.style.top = `${(top / stageHeight) * 100}%`;
  previewImage.style.width = `${(width / stageWidth) * 100}%`;
  previewImage.style.height = `${(height / stageHeight) * 100}%`;
  previewImage.style.opacity = settings.visible ? String(settings.opacity) : "0";
}

function setMessage(message: string, tone: "neutral" | "success" | "error" = "neutral"): void {
  sendStatus.textContent = message;
  sendStatus.dataset.tone = tone;
}

async function choosePhoto(): Promise<void> {
  const selected = await open({
    multiple: false,
    directory: false,
    title: "选择要显示的人像",
    filters: [
      { name: "图片与 GIF 动图", extensions: ["png", "jpg", "jpeg", "heic", "heif", "webp", "tif", "tiff", "gif"] },
    ],
  });
  if (typeof selected !== "string") return;
  sourcePath = selected;
  prepared = null;
  element<HTMLElement>("#photo-name").textContent = selected.split("/").pop() || "已选择照片";
  element<HTMLElement>("#photo-meta").textContent = "正在进行本地绿色显示处理…";
  await processPhoto();
}

async function processPhoto(): Promise<void> {
  if (!sourcePath) return;
  const sequence = ++processingSequence;
  processingState.textContent = "处理中…";
  processingState.className = "busy";
  try {
    const result = await invoke<PreparedPortrait>("prepare_portrait", {
      sourcePath,
      profile: settings.profile,
      gamma: settings.gamma,
      contrast: settings.contrast,
      sharpen: settings.sharpen,
      maxWidth: 240,
    });
    if (sequence !== processingSequence) return;
    prepared = result;
    previewImage.src = result.data_url;
    element<HTMLElement>("#photo-meta").textContent = `${result.width} × ${result.height} · ${result.animated ? "GIF 动图" : "静态图片"} · 本地处理完成`;
    processingState.textContent = "已更新";
    processingState.className = "ready";
    setMessage(deviceReady ? "参数就绪，可以发送" : "照片已就绪，连接眼镜后发送");
  } catch (error) {
    if (sequence !== processingSequence) return;
    prepared = null;
    processingState.textContent = "处理失败";
    processingState.className = "error";
    setMessage(String(error), "error");
  }
  updateSendState();
}

function scheduleProcessing(): void {
  window.clearTimeout(processingTimer);
  if (sourcePath) processingTimer = window.setTimeout(() => void processPhoto(), 180);
}

async function refreshDevice(): Promise<void> {
  devicePill.classList.add("checking");
  try {
    const status = await invoke<DeviceStatus>("get_device_status");
    deviceReady = status.connected && status.package_installed;
    devicePill.classList.toggle("connected", deviceReady);
    devicePill.classList.toggle("warning", status.usb_connected && !deviceReady);
    deviceLabel.textContent = status.model && deviceReady ? `${status.model} 已连接` : status.message;
    if (deviceReady && sendStatus.dataset.tone !== "success") {
      setMessage(sourcePath && !prepared ? "等待照片处理完成" : "参数就绪，可以发送");
    } else if (!deviceReady && sendStatus.dataset.tone !== "error") {
      setMessage(status.message);
    }
  } catch (error) {
    deviceReady = false;
    deviceLabel.textContent = "设备检测失败";
    setMessage(String(error), "error");
  } finally {
    devicePill.classList.remove("checking");
    updateSendState();
  }
}

function updateSendState(): void {
  sendButton.disabled = !deviceReady || (sourcePath !== null && prepared === null);
}

async function sendToGlasses(): Promise<void> {
  sendButton.disabled = true;
  sendButton.classList.add("sending");
  setMessage("正在传输并刷新眼镜…");
  try {
    const result = await invoke<SendResult>("send_to_glasses", {
      processedPath: prepared?.output_path ?? null,
      settings: {
        anchor: settings.anchor,
        size: settings.size,
        opacity: settings.opacity,
        keep_screen_on: settings.keepScreenOn,
        visible: settings.visible,
        render_profile: settings.profile.replace("-", "_"),
      },
    });
    setMessage(result.message, "success");
  } catch (error) {
    setMessage(String(error), "error");
    await refreshDevice();
  } finally {
    sendButton.classList.remove("sending");
    updateSendState();
  }
}

document.querySelectorAll<HTMLButtonElement>("[data-anchor]").forEach((button) => {
  button.addEventListener("click", () => {
    settings.anchor = button.dataset.anchor as Anchor;
    saveSettings();
    syncControls();
  });
});

document.querySelectorAll<HTMLButtonElement>("[data-size]").forEach((button) => {
  button.addEventListener("click", () => {
    settings.size = button.dataset.size as PortraitSize;
    saveSettings();
    syncControls();
  });
});

document.querySelectorAll<HTMLButtonElement>("[data-opacity]").forEach((button) => {
  button.addEventListener("click", () => {
    settings.opacity = Number(button.dataset.opacity) as EditorSettings["opacity"];
    saveSettings();
    syncControls();
  });
});

profileControl.addEventListener("change", () => {
  settings.profile = profileControl.value as Profile;
  saveSettings();
  scheduleProcessing();
});

for (const [control, key, output] of [
  [gammaControl, "gamma", "#gamma-value"],
  [contrastControl, "contrast", "#contrast-value"],
  [sharpenControl, "sharpen", "#sharpen-value"],
] as const) {
  control.addEventListener("input", () => {
    settings[key] = Number(control.value);
    element<HTMLOutputElement>(output).value = settings[key].toFixed(2);
    saveSettings();
    scheduleProcessing();
  });
}

keepScreenControl.addEventListener("change", () => {
  settings.keepScreenOn = keepScreenControl.checked;
  saveSettings();
});

visibleControl.addEventListener("change", () => {
  settings.visible = visibleControl.checked;
  saveSettings();
  syncControls();
});

element<HTMLButtonElement>("#choose-photo").addEventListener("click", () => void choosePhoto());
devicePill.addEventListener("click", () => void refreshDevice());
sendButton.addEventListener("click", () => void sendToGlasses());
previewImage.addEventListener("load", updatePreview);

syncControls();
void refreshDevice();
window.setInterval(() => void refreshDevice(), 4000);
