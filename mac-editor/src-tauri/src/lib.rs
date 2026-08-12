use base64::Engine;
use image::{imageops, DynamicImage, GrayImage, ImageFormat, Rgba, RgbaImage};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager;

const PACKAGE: &str = "dev.local.lockethud.poc";
const COMPONENT: &str = "dev.local.lockethud.poc/.MainActivity";
const REMOTE_DIRECTORY: &str = "/sdcard/Android/data/dev.local.lockethud.poc/files/portraits";
const REMOTE_FILE: &str =
    "/sdcard/Android/data/dev.local.lockethud.poc/files/portraits/current.png";

#[derive(Serialize)]
struct PreparedPortrait {
    data_url: String,
    output_path: String,
    width: u32,
    height: u32,
    sha256: String,
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
    let normalized_path = cache.join(format!("source-{nonce}.png"));

    let normalized = Command::new("/usr/bin/sips")
        .args(["-s", "format", "png"])
        .arg(&source)
        .arg("--out")
        .arg(&normalized_path)
        .output()
        .map_err(|error| format!("无法启动 macOS 图片转换工具：{error}"))?;
    if !normalized.status.success() {
        return Err("无法读取该图片；请改用 PNG、JPEG、HEIC 或 WebP".into());
    }

    let decoded =
        image::open(&normalized_path).map_err(|error| format!("图片解码失败：{error}"))?;
    let source_rgba = decoded.to_rgba8();
    let scale = (max_width as f32 / source_rgba.width() as f32)
        .min(1024.0 / source_rgba.height() as f32)
        .min(1.0);
    let width = ((source_rgba.width() as f32 * scale).round() as u32).max(1);
    let height = ((source_rgba.height() as f32 * scale).round() as u32).max(1);
    let resized = imageops::resize(&source_rgba, width, height, imageops::FilterType::Lanczos3);

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
    match profile.as_str() {
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
    })
}

#[tauri::command]
fn get_device_status() -> DeviceStatus {
    inspect_device()
}

#[tauri::command]
fn send_to_glasses(
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

    let asset = if let Some(path_text) = processed_path {
        let path = PathBuf::from(path_text);
        if !path.is_file() || path.extension().and_then(|value| value.to_str()) != Some("png") {
            return Err("处理后的 PNG 不存在，请重新选择照片".into());
        }
        run_adb(&adb, &["shell", "mkdir", "-p", REMOTE_DIRECTORY])?;
        let local_path = path.to_string_lossy().into_owned();
        run_adb(&adb, &["push", &local_path, REMOTE_FILE])?;
        "private"
    } else {
        "default"
    };

    run_adb(&adb, &["shell", "am", "force-stop", PACKAGE])?;
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
            COMPONENT,
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
    let package_installed = run_adb(&adb, &["shell", "pm", "path", PACKAGE])
        .map(|output| output.contains("package:"))
        .unwrap_or(false);
    DeviceStatus {
        connected: true,
        usb_connected: true,
        model,
        package_installed,
        message: if package_installed {
            "眼镜已连接，显示应用可用".into()
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
}
