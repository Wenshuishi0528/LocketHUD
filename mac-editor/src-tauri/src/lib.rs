use base64::Engine;
use image::codecs::gif::{GifDecoder, GifEncoder, Repeat};
use image::{
    imageops, AnimationDecoder, DynamicImage, Frame, GrayImage, ImageFormat, Rgba, RgbaImage,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Cursor};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager;

const LEGACY_PACKAGE: &str = "dev.local.lockethud.poc";
const LEGACY_COMPONENT: &str = "dev.local.lockethud.poc/.MainActivity";
const AIUI_HOST_PACKAGE: &str = "com.rokid.os.sprite.assistserver";
const AIUI_SERVICE: &str = "com.rokid.os.sprite.assistserver/com.rokid.os.sprite.jsai.JsaiService";
const AIUI_AGENT_ID: &str = "fea33d142f1443b282eb9c3a62d54183";
const LEGACY_AIUI_AGENT_ID: &str = "lockethud-photo";
const AIUI_REMOTE_DIRECTORY: &str = "/sdcard/jsai/package";
const AIUI_FRAME_WIDTH: u32 = 448;
const AIUI_FRAME_HEIGHT: u32 = 352;
const AIUI_FRAME_MARGIN: u32 = 18;
const AIUI_AGENTS: &str = include_str!("../../../aiui-app/AGENTS.md");
const AIUI_APP_JS: &str = include_str!("../../../aiui-app/app.js");
const AIUI_APP_JSON: &str = include_str!("../../../aiui-app/app.json");
const AIUI_PAGE_TEMPLATE: &str = include_str!("../../../aiui-app/pages/index/index.ink");
const AIUI_ICON: &[u8] = include_bytes!("../../../aiui-app/assets/icon.png");
const AIUI_DEFAULT_FRAME: &[u8] = include_bytes!("../../../aiui-app/assets/display_default.png");
const REMOTE_DIRECTORY: &str = "/sdcard/Android/data/dev.local.lockethud.poc/files/portraits";
const REMOTE_PNG_FILE: &str =
    "/sdcard/Android/data/dev.local.lockethud.poc/files/portraits/current.png";
const REMOTE_GIF_FILE: &str =
    "/sdcard/Android/data/dev.local.lockethud.poc/files/portraits/current.gif";

#[derive(Serialize)]
struct PreparedPortrait {
    data_url: String,
    output_path: String,
    width: u32,
    height: u32,
    sha256: String,
    animated: bool,
}

#[derive(Serialize)]
struct DeviceStatus {
    connected: bool,
    usb_connected: bool,
    model: Option<String>,
    package_installed: bool,
    message: String,
}

#[derive(Serialize)]
struct SendResult {
    message: String,
}

#[derive(Deserialize)]
struct GlassesSettings {
    anchor: String,
    size: String,
    opacity: f32,
    keep_screen_on: bool,
    visible: bool,
    render_profile: String,
}

impl GlassesSettings {
    fn validate(&self) -> Result<(), String> {
        const ANCHORS: &[&str] = &[
            "left_top",
            "left_middle",
            "left_bottom",
            "right_top",
            "right_middle",
            "right_bottom",
        ];
        const SIZES: &[&str] = &["small", "medium", "large"];
        const PROFILES: &[&str] = &["natural_green", "quantized_8", "quantized_16", "dithered"];
        const OPACITIES: &[f32] = &[0.4, 0.6, 0.8, 1.0];

        if !ANCHORS.contains(&self.anchor.as_str()) {
            return Err("不支持的位置参数".into());
        }
        if !SIZES.contains(&self.size.as_str()) {
            return Err("不支持的大小参数".into());
        }
        if !PROFILES.contains(&self.render_profile.as_str()) {
            return Err("不支持的显示处理方式".into());
        }
        if !OPACITIES
            .iter()
            .any(|allowed| (self.opacity - allowed).abs() < 0.001)
        {
            return Err("不支持的透明度参数".into());
        }
        Ok(())
    }
}

#[tauri::command]
fn prepare_portrait(
    app: tauri::AppHandle,
    source_path: String,
    profile: String,
    gamma: f32,
    contrast: f32,
    sharpen: f32,
    max_width: u32,
) -> Result<PreparedPortrait, String> {
    if !(0.6..=1.8).contains(&gamma) {
        return Err("Gamma 必须在 0.6–1.8 之间".into());
    }
    if !(0.6..=1.8).contains(&contrast) {
        return Err("对比度必须在 0.6–1.8 之间".into());
    }
    if !(0.0..=1.5).contains(&sharpen) {
        return Err("锐化必须在 0–1.5 之间".into());
    }
    if !(32..=1024).contains(&max_width) {
        return Err("处理宽度超出安全范围".into());
    }
    if !matches!(
        profile.as_str(),
        "natural-green" | "quantized-8" | "quantized-16" | "dithered"
    ) {
        return Err("不支持的处理方式".into());
    }

    let source = fs::canonicalize(&source_path).map_err(|_| "所选图片不存在".to_string())?;
    if !source.is_file() {
        return Err("所选路径不是图片文件".into());
    }

    let cache = app
        .path()
        .app_cache_dir()
        .map_err(|error| format!("无法取得应用缓存目录：{error}"))?;
    fs::create_dir_all(&cache).map_err(|error| format!("无法创建图片缓存：{error}"))?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();

    if source
        .extension()
        .and_then(|value| value.to_str())
        .is_some_and(|value| value.eq_ignore_ascii_case("gif"))
    {
        return prepare_animated_gif(
            &source, &cache, nonce, &profile, gamma, contrast, sharpen, max_width,
        );
    }

    let normalized_path = cache.join(format!("source-{nonce}.png"));

    let normalized = Command::new("/usr/bin/sips")
        .args(["-s", "format", "png"])
        .arg(&source)
        .arg("--out")
        .arg(&normalized_path)
        .output()
        .map_err(|error| format!("无法启动 macOS 图片转换工具：{error}"))?;
    if !normalized.status.success() {
        return Err("无法读取该图片；请改用 PNG、JPEG、HEIC、WebP 或 GIF".into());
    }

    let decoded =
        image::open(&normalized_path).map_err(|error| format!("图片解码失败：{error}"))?;
    let output = process_frame(
        &decoded.to_rgba8(),
        &profile,
        gamma,
        contrast,
        sharpen,
        max_width,
    );
    let (width, height) = output.dimensions();

    let mut cursor = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(output)
        .write_to(&mut cursor, ImageFormat::Png)
        .map_err(|error| format!("无法编码处理结果：{error}"))?;
    let bytes = cursor.into_inner();
    let output_path = cache.join(format!("portrait-{nonce}.png"));
    fs::write(&output_path, &bytes).map_err(|error| format!("无法保存处理结果：{error}"))?;
    let _ = fs::remove_file(normalized_path);

    let sha256 = format!("{:x}", Sha256::digest(&bytes));
    let data_url = format!(
        "data:image/png;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(&bytes)
    );
    Ok(PreparedPortrait {
        data_url,
        output_path: output_path.to_string_lossy().into_owned(),
        width,
        height,
        sha256,
        animated: false,
    })
}

fn prepare_animated_gif(
    source: &Path,
    cache: &Path,
    nonce: u128,
    profile: &str,
    gamma: f32,
    contrast: f32,
    sharpen: f32,
    max_width: u32,
) -> Result<PreparedPortrait, String> {
    if fs::metadata(source)
        .map_err(|error| error.to_string())?
        .len()
        > 32 * 1024 * 1024
    {
        return Err("GIF 文件不能超过 32 MB".into());
    }
    let file = File::open(source).map_err(|error| format!("无法读取 GIF：{error}"))?;
    let decoder =
        GifDecoder::new(BufReader::new(file)).map_err(|error| format!("GIF 解码失败：{error}"))?;
    let frames = decoder
        .into_frames()
        .collect_frames()
        .map_err(|error| format!("GIF 帧读取失败：{error}"))?;
    if frames.is_empty() {
        return Err("GIF 中没有可显示的画面".into());
    }
    if frames.len() > 300 {
        return Err("GIF 帧数不能超过 300 帧".into());
    }

    let mut bytes = Vec::new();
    let mut output_size = None;
    {
        let mut encoder = GifEncoder::new(&mut bytes);
        encoder
            .set_repeat(Repeat::Infinite)
            .map_err(|error| format!("无法设置 GIF 循环：{error}"))?;
        for frame in frames {
            let delay = frame.delay();
            let output = process_frame(
                &frame.into_buffer(),
                profile,
                gamma,
                contrast,
                sharpen,
                max_width,
            );
            output_size.get_or_insert(output.dimensions());
            encoder
                .encode_frame(Frame::from_parts(output, 0, 0, delay))
                .map_err(|error| format!("GIF 编码失败：{error}"))?;
        }
    }

    if bytes.len() > 32 * 1024 * 1024 {
        return Err("处理后的 GIF 超过 32 MB；请缩短动画或减少帧数".into());
    }
    let (width, height) = output_size.ok_or_else(|| "GIF 中没有可显示的画面".to_string())?;
    let output_path = cache.join(format!("portrait-{nonce}.gif"));
    fs::write(&output_path, &bytes).map_err(|error| format!("无法保存 GIF：{error}"))?;
    let sha256 = format!("{:x}", Sha256::digest(&bytes));
    let data_url = format!(
        "data:image/gif;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(&bytes)
    );
    Ok(PreparedPortrait {
        data_url,
        output_path: output_path.to_string_lossy().into_owned(),
        width,
        height,
        sha256,
        animated: true,
    })
}

fn process_frame(
    source_rgba: &RgbaImage,
    profile: &str,
    gamma: f32,
    contrast: f32,
    sharpen: f32,
    max_width: u32,
) -> RgbaImage {
    let max_height = AIUI_FRAME_HEIGHT - AIUI_FRAME_MARGIN * 2;
    let scale = (max_width as f32 / source_rgba.width() as f32)
        .min(max_height as f32 / source_rgba.height() as f32)
        .min(1.0);
    let width = ((source_rgba.width() as f32 * scale).round() as u32).max(1);
    let height = ((source_rgba.height() as f32 * scale).round() as u32).max(1);
    let resized = imageops::resize(source_rgba, width, height, imageops::FilterType::Lanczos3);

    let mut gray = GrayImage::new(width, height);
    for (x, y, pixel) in resized.enumerate_pixels() {
        let luminance = 0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32;
        let contrasted = (((luminance / 255.0 - 0.5) * contrast) + 0.5).clamp(0.0, 1.0);
        let corrected = contrasted.powf(1.0 / gamma) * 255.0;
        gray.put_pixel(x, y, image::Luma([corrected.round() as u8]));
    }

    if sharpen > 0.0 {
        let blurred = imageops::blur(&gray, 1.0);
        for (original, soft) in gray.pixels_mut().zip(blurred.pixels()) {
            let value = original[0] as f32 + sharpen * (original[0] as f32 - soft[0] as f32);
            original[0] = value.clamp(0.0, 255.0).round() as u8;
        }
    }

    let mut values: Vec<f32> = gray.pixels().map(|pixel| pixel[0] as f32).collect();
    match profile {
        "quantized-8" => quantize_all(&mut values, 8),
        "quantized-16" => quantize_all(&mut values, 16),
        "dithered" => dither(&mut values, width as usize, height as usize, 8),
        _ => {}
    }

    let mut output = RgbaImage::new(width, height);
    for (index, pixel) in output.pixels_mut().enumerate() {
        let green = values[index].clamp(0.0, 255.0).round() as u8;
        let alpha = resized.as_raw()[index * 4 + 3];
        *pixel = Rgba([0, green, (green as f32 * 0.30).round() as u8, alpha]);
    }
    output
}

#[tauri::command]
fn get_device_status() -> DeviceStatus {
    inspect_device()
}

struct AiuiBuild {
    workspace: PathBuf,
    aix_path: PathBuf,
    md5: String,
}

fn render_aiui_page(image_src: &str) -> String {
    AIUI_PAGE_TEMPLATE.replace(
        "imageSrc: '/assets/display_default.png'",
        &format!("imageSrc: '{image_src}'"),
    )
}

fn frame_origin(width: u32, height: u32, anchor: &str) -> (u32, u32) {
    let x = if anchor.starts_with("left_") {
        AIUI_FRAME_MARGIN
    } else {
        AIUI_FRAME_WIDTH.saturating_sub(AIUI_FRAME_MARGIN + width)
    };
    let y = if anchor.ends_with("_top") {
        AIUI_FRAME_MARGIN
    } else if anchor.ends_with("_bottom") {
        AIUI_FRAME_HEIGHT.saturating_sub(AIUI_FRAME_MARGIN + height)
    } else {
        AIUI_FRAME_HEIGHT.saturating_sub(height) / 2
    };
    (x, y)
}

fn compose_aiui_frame(portrait: &RgbaImage, settings: &GlassesSettings) -> RgbaImage {
    let mut frame =
        RgbaImage::from_pixel(AIUI_FRAME_WIDTH, AIUI_FRAME_HEIGHT, Rgba([0, 0, 0, 255]));
    if !settings.visible {
        return frame;
    }
    let (origin_x, origin_y) = frame_origin(portrait.width(), portrait.height(), &settings.anchor);
    for (x, y, pixel) in portrait.enumerate_pixels() {
        let target_x = origin_x + x;
        let target_y = origin_y + y;
        if target_x >= AIUI_FRAME_WIDTH || target_y >= AIUI_FRAME_HEIGHT {
            continue;
        }
        let alpha = (pixel[3] as f32 / 255.0) * settings.opacity;
        frame.put_pixel(
            target_x,
            target_y,
            Rgba([
                (pixel[0] as f32 * alpha).round() as u8,
                (pixel[1] as f32 * alpha).round() as u8,
                (pixel[2] as f32 * alpha).round() as u8,
                255,
            ]),
        );
    }
    frame
}

fn write_aiui_static_frame(
    source: &Path,
    target: &Path,
    settings: &GlassesSettings,
) -> Result<(), String> {
    let portrait = image::open(source)
        .map_err(|error| format!("无法读取处理后的 PNG：{error}"))?
        .to_rgba8();
    compose_aiui_frame(&portrait, settings)
        .save_with_format(target, ImageFormat::Png)
        .map_err(|error| format!("无法生成 AIUI 最终画面：{error}"))
}

fn write_aiui_gif_frame(
    source: &Path,
    target: &Path,
    settings: &GlassesSettings,
) -> Result<(), String> {
    let input = File::open(source).map_err(|error| format!("无法读取处理后的 GIF：{error}"))?;
    let frames = GifDecoder::new(BufReader::new(input))
        .map_err(|error| format!("GIF 解码失败：{error}"))?
        .into_frames()
        .collect_frames()
        .map_err(|error| format!("GIF 帧读取失败：{error}"))?;
    let output = File::create(target).map_err(|error| format!("无法创建 AIUI GIF：{error}"))?;
    let mut encoder = GifEncoder::new(output);
    encoder
        .set_repeat(Repeat::Infinite)
        .map_err(|error| format!("无法设置 GIF 循环：{error}"))?;
    for frame in frames {
        let delay = frame.delay();
        encoder
            .encode_frame(Frame::from_parts(
                compose_aiui_frame(&frame.into_buffer(), settings),
                0,
                0,
                delay,
            ))
            .map_err(|error| format!("无法生成 AIUI GIF：{error}"))?;
    }
    if fs::metadata(target)
        .map_err(|error| error.to_string())?
        .len()
        > 32 * 1024 * 1024
    {
        return Err("最终 AIUI GIF 超过 32 MB；请缩短动画或减少帧数".into());
    }
    Ok(())
}

fn build_aiui_aix(
    cache: &Path,
    processed_path: Option<&str>,
    settings: &GlassesSettings,
) -> Result<AiuiBuild, String> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let workspace = cache.join(format!("aiui-send-{nonce}"));
    let package_dir = workspace.join("package");
    let assets_dir = package_dir.join("assets");
    let page_dir = package_dir.join("pages/index");
    fs::create_dir_all(&assets_dir).map_err(|error| format!("无法创建 AIUI 图片包：{error}"))?;
    fs::create_dir_all(&page_dir).map_err(|error| format!("无法创建 AIUI 页面包：{error}"))?;

    let image_src = if let Some(path_text) = processed_path {
        let source = PathBuf::from(path_text);
        let extension = source
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        let (target_name, animated) = if extension.eq_ignore_ascii_case("gif") {
            ("display_frame.gif", true)
        } else if extension.eq_ignore_ascii_case("png") {
            ("display_frame.png", false)
        } else {
            return Err("处理后的 PNG/GIF 不存在，请重新选择照片".into());
        };
        let target = assets_dir.join(target_name);
        if animated {
            write_aiui_gif_frame(&source, &target, settings)?;
        } else {
            write_aiui_static_frame(&source, &target, settings)?;
        }
        format!("/assets/{target_name}")
    } else {
        fs::write(assets_dir.join("display_default.png"), AIUI_DEFAULT_FRAME)
            .map_err(|error| format!("无法写入 AIUI 测试画面：{error}"))?;
        "/assets/display_default.png".into()
    };

    fs::write(package_dir.join("AGENTS.md"), AIUI_AGENTS)
        .map_err(|error| format!("无法写入 AIUI 清单：{error}"))?;
    fs::write(package_dir.join("app.js"), AIUI_APP_JS)
        .map_err(|error| format!("无法写入 AIUI 应用：{error}"))?;
    fs::write(package_dir.join("app.json"), AIUI_APP_JSON)
        .map_err(|error| format!("无法写入 AIUI 配置：{error}"))?;
    fs::write(page_dir.join("index.ink"), render_aiui_page(&image_src))
        .map_err(|error| format!("无法写入 AIUI 页面：{error}"))?;
    fs::write(assets_dir.join("icon.png"), AIUI_ICON)
        .map_err(|error| format!("无法写入 AIUI 图标：{error}"))?;
    let version_hex = format!(
        "{:x}",
        Sha256::digest(format!("{nonce}:{image_src}:{}", settings.anchor).as_bytes())
    );
    let version = format!(
        "{}-{}-{}-{}-{}",
        &version_hex[0..8],
        &version_hex[8..12],
        &version_hex[12..16],
        &version_hex[16..20],
        &version_hex[20..32]
    );
    fs::write(package_dir.join("VERSION"), version)
        .map_err(|error| format!("无法写入 AIUI 版本：{error}"))?;

    let aix_path = workspace.join("lockethud-photo.aix");
    let zip = Command::new("/usr/bin/zip")
        .args(["-X", "-q", "-r"])
        .arg(&aix_path)
        .arg(".")
        .current_dir(&package_dir)
        .output()
        .map_err(|error| format!("无法打包 AIUI 文件：{error}"))?;
    if !zip.status.success() {
        return Err("AIUI 文件打包失败".into());
    }
    let aix = fs::read(&aix_path).map_err(|error| format!("无法读取 AIUI 文件：{error}"))?;
    Ok(AiuiBuild {
        workspace,
        aix_path,
        md5: format!("{:x}", md5::compute(aix)),
    })
}

fn install_aiui_portrait(
    app: &tauri::AppHandle,
    adb: &Path,
    processed_path: Option<&str>,
    settings: &GlassesSettings,
) -> Result<(), String> {
    let cache = app
        .path()
        .app_cache_dir()
        .map_err(|error| format!("无法取得应用缓存目录：{error}"))?;
    fs::create_dir_all(&cache).map_err(|error| format!("无法创建应用缓存：{error}"))?;
    let build = build_aiui_aix(&cache, processed_path, settings)?;
    let result = (|| {
        let remote_file = format!(
            "{AIUI_REMOTE_DIRECTORY}/{AIUI_AGENT_ID}_1.0.1_{}.aix",
            &build.md5[..8]
        );
        let remote_index = format!("{AIUI_REMOTE_DIRECTORY}/agents_index.json");
        let local_index = build.workspace.join("agents_index.json");
        let pulled_index = build.workspace.join("agents_index.original.json");
        let _ = Command::new(adb)
            .args(["pull", &remote_index])
            .arg(&pulled_index)
            .output();

        let mut agents: Vec<serde_json::Value> = fs::read_to_string(&pulled_index)
            .ok()
            .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
            .and_then(|value| {
                value
                    .get("agents")
                    .and_then(|items| items.as_array())
                    .cloned()
            })
            .unwrap_or_default();
        let previous_files: Vec<String> = agents
            .iter()
            .filter(|agent| {
                matches!(
                    agent.get("agentId").and_then(|value| value.as_str()),
                    Some(AIUI_AGENT_ID) | Some(LEGACY_AIUI_AGENT_ID)
                )
            })
            .filter_map(|agent| agent.get("filePath").and_then(|value| value.as_str()))
            .filter(|path| {
                path.starts_with(&format!("{AIUI_REMOTE_DIRECTORY}/{AIUI_AGENT_ID}_"))
                    || path.starts_with(&format!("{AIUI_REMOTE_DIRECTORY}/{LEGACY_AIUI_AGENT_ID}_"))
            })
            .map(str::to_string)
            .collect();
        agents.retain(|agent| {
            !matches!(
                agent.get("agentId").and_then(|value| value.as_str()),
                Some(AIUI_AGENT_ID) | Some(LEGACY_AIUI_AGENT_ID)
            )
        });
        let updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| error.to_string())?
            .as_millis() as u64;
        agents.push(serde_json::json!({
            "agentId": AIUI_AGENT_ID,
            "agentName": "照片浮窗",
            "agentDesc": "Display a locally transferred photo or animated GIF",
            "agentLogo": "",
            "url": "",
            "permissions": [],
            "nativeVersion": "1.0.1",
            "fileMd5": build.md5,
            "filePath": remote_file,
            "updatedAt": updated_at
        }));
        fs::write(
            &local_index,
            serde_json::to_vec(&serde_json::json!({ "agents": agents }))
                .map_err(|error| error.to_string())?,
        )
        .map_err(|error| format!("无法更新 AIUI 索引：{error}"))?;

        run_adb(adb, &["shell", "am", "force-stop", AIUI_HOST_PACKAGE])?;
        for previous in previous_files {
            if previous != remote_file {
                let _ = run_adb(adb, &["shell", "rm", "-f", &previous]);
            }
        }
        run_adb(adb, &["shell", "mkdir", "-p", AIUI_REMOTE_DIRECTORY])?;
        run_adb(
            adb,
            &["push", &build.aix_path.to_string_lossy(), &remote_file],
        )?;
        run_adb(
            adb,
            &["push", &local_index.to_string_lossy(), &remote_index],
        )?;
        run_adb(
            adb,
            &[
                "push",
                &local_index.to_string_lossy(),
                &format!("{remote_index}.bak"),
            ],
        )?;
        run_adb(adb, &["shell", "input", "keyevent", "224"])?;
        let open_params = format!("'{{\"agentId\":\"{AIUI_AGENT_ID}\"}}'");
        run_adb(
            adb,
            &[
                "shell",
                "am",
                "startservice",
                "-n",
                AIUI_SERVICE,
                "-a",
                "com.rokid.os.sprite.jsai.OPEN_PAGE",
                "--es",
                "open_params",
                &open_params,
            ],
        )?;
        Ok(())
    })();
    let _ = fs::remove_dir_all(&build.workspace);
    result
}

#[tauri::command]
fn send_to_glasses(
    app: tauri::AppHandle,
    processed_path: Option<String>,
    settings: GlassesSettings,
) -> Result<SendResult, String> {
    settings.validate()?;
    let status = inspect_device();
    if !status.connected {
        return Err("没有检测到已授权的眼镜，请连接 USB 后重试".into());
    }
    if !status.package_installed {
        return Err("眼镜已连接，但尚未安装 LocketHUD 显示应用".into());
    }
    let adb = find_adb().ok_or_else(|| "Mac 上没有找到 adb".to_string())?;

    let aiui_available = run_adb(&adb, &["shell", "pm", "path", AIUI_HOST_PACKAGE])
        .map(|output| output.contains("package:"))
        .unwrap_or(false);
    if aiui_available {
        install_aiui_portrait(&app, &adb, processed_path.as_deref(), &settings)?;
        return Ok(SendResult {
            message: if settings.visible {
                "已发送到 AIUI 眼镜端并开始显示".into()
            } else {
                "已同步设置并隐藏 AIUI 画面".into()
            },
        });
    }

    let asset = if let Some(path_text) = processed_path {
        let path = PathBuf::from(path_text);
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        let (remote_file, asset) = if extension.eq_ignore_ascii_case("gif") {
            (REMOTE_GIF_FILE, "private_gif")
        } else if extension.eq_ignore_ascii_case("png") {
            (REMOTE_PNG_FILE, "private")
        } else {
            return Err("处理后的 PNG/GIF 不存在，请重新选择照片".into());
        };
        if !path.is_file() {
            return Err("处理后的图片不存在，请重新选择照片".into());
        }
        run_adb(&adb, &["shell", "mkdir", "-p", REMOTE_DIRECTORY])?;
        let local_path = path.to_string_lossy().into_owned();
        run_adb(&adb, &["push", &local_path, remote_file])?;
        asset
    } else {
        "default"
    };

    run_adb(&adb, &["shell", "am", "force-stop", LEGACY_PACKAGE])?;
    let opacity = format!("{:.1}", settings.opacity);
    let keep_screen_on = settings.keep_screen_on.to_string();
    let visible = settings.visible.to_string();
    run_adb(
        &adb,
        &[
            "shell",
            "am",
            "start",
            "-n",
            LEGACY_COMPONENT,
            "--es",
            "mode",
            "portrait",
            "--es",
            "asset",
            asset,
            "--es",
            "anchor",
            &settings.anchor,
            "--es",
            "size",
            &settings.size,
            "--es",
            "opacity",
            &opacity,
            "--es",
            "keep_screen_on",
            &keep_screen_on,
            "--es",
            "visible",
            &visible,
            "--es",
            "clock_enabled",
            "false",
            "--es",
            "render_profile",
            &settings.render_profile,
        ],
    )?;

    Ok(SendResult {
        message: if settings.visible {
            "已发送到眼镜并开始显示".into()
        } else {
            "已同步设置并隐藏眼镜画面".into()
        },
    })
}

fn quantize_all(values: &mut [f32], levels: u8) {
    let scale = levels as f32 - 1.0;
    for value in values {
        *value = (*value * scale / 255.0).round() * 255.0 / scale;
    }
}

fn dither(values: &mut [f32], width: usize, height: usize, levels: u8) {
    let scale = levels as f32 - 1.0;
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let old = values[index];
            let new = (old * scale / 255.0).round() * 255.0 / scale;
            values[index] = new;
            let error = old - new;
            add_error(
                values,
                width,
                height,
                x as isize + 1,
                y as isize,
                error * 7.0 / 16.0,
            );
            add_error(
                values,
                width,
                height,
                x as isize - 1,
                y as isize + 1,
                error * 3.0 / 16.0,
            );
            add_error(
                values,
                width,
                height,
                x as isize,
                y as isize + 1,
                error * 5.0 / 16.0,
            );
            add_error(
                values,
                width,
                height,
                x as isize + 1,
                y as isize + 1,
                error / 16.0,
            );
        }
    }
}

fn add_error(values: &mut [f32], width: usize, height: usize, x: isize, y: isize, error: f32) {
    if x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height {
        let index = y as usize * width + x as usize;
        values[index] = (values[index] + error).clamp(0.0, 255.0);
    }
}

fn inspect_device() -> DeviceStatus {
    let Some(adb) = find_adb() else {
        return DeviceStatus {
            connected: false,
            usb_connected: false,
            model: None,
            package_installed: false,
            message: "未找到 Android Platform Tools".into(),
        };
    };
    let devices = match run_adb(&adb, &["devices", "-l"]) {
        Ok(output) => output,
        Err(message) => {
            return DeviceStatus {
                connected: false,
                usb_connected: false,
                model: None,
                package_installed: false,
                message,
            }
        }
    };
    let online: Vec<&str> = devices
        .lines()
        .filter(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            fields.get(1) == Some(&"device")
        })
        .collect();
    if online.len() != 1 {
        let unauthorized = devices
            .lines()
            .any(|line| line.split_whitespace().nth(1) == Some("unauthorized"));
        let usb_connected = has_usb_glasses();
        return DeviceStatus {
            connected: false,
            usb_connected,
            model: None,
            package_installed: false,
            message: if unauthorized {
                "眼镜已连接，等待确认 USB 调试授权".into()
            } else if usb_connected {
                "USB 已连接；请在眼镜中重新开启 USB 调试并确认授权".into()
            } else if online.len() > 1 {
                "连接了多个 Android 设备，请只保留眼镜".into()
            } else {
                "未连接眼镜".into()
            },
        };
    }

    let model = online[0]
        .split_whitespace()
        .find_map(|field| field.strip_prefix("model:"))
        .map(|value| value.replace('_', " "));
    let legacy_installed = run_adb(&adb, &["shell", "pm", "path", LEGACY_PACKAGE])
        .map(|output| output.contains("package:"))
        .unwrap_or(false);
    let aiui_installed = run_adb(&adb, &["shell", "pm", "path", AIUI_HOST_PACKAGE])
        .map(|output| output.contains("package:"))
        .unwrap_or(false);
    let package_installed = aiui_installed || legacy_installed;
    DeviceStatus {
        connected: true,
        usb_connected: true,
        model,
        package_installed,
        message: if package_installed {
            if aiui_installed {
                "眼镜已连接，AIUI 显示端可用".into()
            } else {
                "眼镜已连接，显示应用可用".into()
            }
        } else {
            "眼镜已连接，尚未安装显示应用".into()
        },
    }
}

fn has_usb_glasses() -> bool {
    Command::new("/usr/sbin/ioreg")
        .args(["-p", "IOUSB", "-l", "-w", "0"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| {
            let text = String::from_utf8_lossy(&output.stdout);
            text.contains("RG-glasses") || text.contains("Rokid")
        })
        .unwrap_or(false)
}

fn find_adb() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(root) = env::var_os("ANDROID_HOME").or_else(|| env::var_os("ANDROID_SDK_ROOT")) {
        candidates.push(PathBuf::from(root).join("platform-tools/adb"));
    }
    if let Some(home) = env::var_os("HOME") {
        candidates.push(PathBuf::from(home).join("Library/Android/sdk/platform-tools/adb"));
    }
    candidates.push(PathBuf::from("/opt/homebrew/bin/adb"));
    candidates.push(PathBuf::from("/usr/local/bin/adb"));
    if let Some(path) = candidates.into_iter().find(|path| path.is_file()) {
        return Some(path);
    }
    Command::new("adb")
        .arg("version")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|_| PathBuf::from("adb"))
}

fn run_adb(adb: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new(adb)
        .args(args)
        .output()
        .map_err(|error| format!("无法执行 adb：{error}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            "adb 操作失败".into()
        } else {
            stderr
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            prepare_portrait,
            get_device_status,
            send_to_glasses
        ])
        .run(tauri::generate_context!())
        .expect("failed to run LocketHUD editor");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_supported_glasses_settings() {
        let settings = GlassesSettings {
            anchor: "right_middle".into(),
            size: "small".into(),
            opacity: 0.6,
            keep_screen_on: true,
            visible: true,
            render_profile: "quantized_16".into(),
        };
        assert!(settings.validate().is_ok());
    }

    #[test]
    fn rejects_unlisted_opacity() {
        let settings = GlassesSettings {
            anchor: "right_middle".into(),
            size: "small".into(),
            opacity: 0.7,
            keep_screen_on: true,
            visible: true,
            render_profile: "quantized_16".into(),
        };
        assert_eq!(settings.validate().unwrap_err(), "不支持的透明度参数");
    }

    #[test]
    fn quantization_stays_within_range() {
        let mut values = vec![0.0, 18.0, 127.0, 254.0, 255.0];
        quantize_all(&mut values, 8);
        assert!(values.iter().all(|value| (0.0..=255.0).contains(value)));
        assert_eq!(values[0], 0.0);
        assert_eq!(values[4], 255.0);
    }

    #[test]
    fn aiui_page_only_displays_the_precomposed_frame() {
        let page = render_aiui_page("/assets/display_frame.gif");
        assert!(page.contains("imageSrc: '/assets/display_frame.gif'"));
        assert!(page.contains("mode=\"scaleToFill\""));
        assert!(!page.contains("widthFix"));
        assert!(!page.contains("anchorClass"));
        assert!(!page.contains("opacityClass"));
    }

    #[test]
    fn aiui_frame_bakes_position_and_opacity() {
        let portrait = RgbaImage::from_pixel(20, 30, Rgba([0, 200, 60, 255]));
        let settings = GlassesSettings {
            anchor: "right_bottom".into(),
            size: "medium".into(),
            opacity: 0.4,
            keep_screen_on: true,
            visible: true,
            render_profile: "natural_green".into(),
        };
        let frame = compose_aiui_frame(&portrait, &settings);
        assert_eq!(frame.dimensions(), (448, 352));
        assert_eq!(frame.get_pixel(0, 0), &Rgba([0, 0, 0, 255]));
        assert_eq!(frame.get_pixel(410, 304), &Rgba([0, 80, 24, 255]));
    }

    #[test]
    fn animated_gif_keeps_multiple_frames() {
        let test_dir = env::temp_dir().join(format!(
            "lockethud-gif-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        fs::create_dir_all(&test_dir).unwrap();
        let source = test_dir.join("source.gif");
        let mut source_bytes = Vec::new();
        {
            let mut encoder = GifEncoder::new(&mut source_bytes);
            encoder.set_repeat(Repeat::Infinite).unwrap();
            for green in [80, 220] {
                let frame = RgbaImage::from_pixel(8, 12, Rgba([0, green, 0, 255]));
                encoder
                    .encode_frame(Frame::from_parts(
                        frame,
                        0,
                        0,
                        image::Delay::from_numer_denom_ms(120, 1),
                    ))
                    .unwrap();
            }
        }
        fs::write(&source, source_bytes).unwrap();

        let prepared =
            prepare_animated_gif(&source, &test_dir, 1, "natural-green", 1.0, 1.0, 0.0, 240)
                .unwrap();
        let output = File::open(&prepared.output_path).unwrap();
        let frames = GifDecoder::new(BufReader::new(output))
            .unwrap()
            .into_frames()
            .collect_frames()
            .unwrap();

        assert!(prepared.animated);
        assert_eq!(frames.len(), 2);
        assert_eq!((prepared.width, prepared.height), (8, 12));

        let settings = GlassesSettings {
            anchor: "left_top".into(),
            size: "small".into(),
            opacity: 1.0,
            keep_screen_on: true,
            visible: true,
            render_profile: "natural_green".into(),
        };
        let display_gif = test_dir.join("display.gif");
        write_aiui_gif_frame(Path::new(&prepared.output_path), &display_gif, &settings).unwrap();
        let display_frames = GifDecoder::new(BufReader::new(File::open(display_gif).unwrap()))
            .unwrap()
            .into_frames()
            .collect_frames()
            .unwrap();
        assert_eq!(display_frames.len(), 2);
        assert!(display_frames
            .iter()
            .all(|frame| frame.buffer().dimensions() == (448, 352)));
        fs::remove_dir_all(test_dir).unwrap();
    }
}
