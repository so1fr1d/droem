// src-tauri/src/main.rs
// drøm (droem) — Phase 37 Clean Release Native Windows Tauri v2 Backend
// - Settings Tab Renamed: '테마' -> '일반' (General) across all 21 locales
// - Redefined Lunar Icon System:
//   * Group 1 (Single Main App Icon: `icon.ico` / `1_main_ref_clean.ico` / `icon.svg`):
//     Approved Lunar Crescent + 2-Wave geometry on a subtle semi-transparent high-radius rounded square,
//     thin soft border, resolution-adaptive legibility (16/24/32px crisp, 48/64px subtle highlight/glow)
//   * Group 2 (Unified Common Master Family: `4_ui_return_dark.ico`, `5_ui_return_light.ico`, `audio_*.ico`):
//     100% identical master geometry & border; varies ONLY by palette (foreground, background, border)
// - Gapless Playback Support: Extracts codec `enc_delay`, `enc_padding`, and `total_samples`
//   (MP3 Xing/LAME, M4A/AAC iTunSMPB, Opus pre_skip, and 0-trim Lossless FLAC/WAV/AIF/AIFF)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use lofty::config::ParseOptions;
use lofty::prelude::*;
use lofty::probe::Probe;
use lofty::tag::ItemKey;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::UNIX_EPOCH;
use tauri::{
    Emitter, LogicalSize, Manager, PhysicalPosition, Position, Size, State, WebviewWindow,
    WindowEvent,
};
use walkdir::WalkDir;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
use windows::{
    core::HSTRING,
    Foundation::TypedEventHandler,
    Media::{
        MediaPlaybackStatus, MediaPlaybackType, SystemMediaTransportControls,
        SystemMediaTransportControlsButton, SystemMediaTransportControlsButtonPressedEventArgs,
    },
    Storage::StorageFile,
    Storage::Streams::RandomAccessStreamReference,
    Win32::System::WinRT::{ISystemMediaTransportControlsInterop, RoGetActivationFactory},
    Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID,
    Win32::UI::WindowsAndMessaging::{FindWindowW, GetForegroundWindow},
};

static SMTC_BUTTON_HANDLER_REGISTERED: AtomicBool = AtomicBool::new(false);
static IS_SWITCHING_MODE: AtomicBool = AtomicBool::new(false);
static NATIVE_AUDIO_CACHE: OnceLock<Mutex<Option<NativeAudioOutputs>>> = OnceLock::new();

/// Phase 26: Group 1 — Drøm Single Official Main App Icon (`icon.svg`)
/// Approved Lunar Crescent + 2-Wave master geometry on a subtle semi-transparent high-radius rounded rectangle (`rx="124"`),
/// very thin soft outer border (`stroke-width="3"`), and restrained subtle highlight.
const DROEM_BRAND_ICON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
  <rect x="0" y="0" width="512" height="512" rx="134.6" fill="#141d2e" fill-opacity="1"/>
  <rect x="3" y="3" width="506" height="506" rx="131.6" fill="none" stroke="#405270" stroke-width="6"/>
  <g transform="translate(256 256) scale(1.36) translate(-234.25 -254.75)">
    <path d="M 257.04 112.55 A 104.22 104.22 0 1 0 371.03 256.31 A 92.18 92.18 0 0 1 257.04 112.55 Z" fill="#f8fafc"/>
    <circle cx="346.3" cy="173.9" r="5.5" fill="#f8fafc"/>
    <path d="M 97.45 309.12 C 97.19 308.57, 108.66 298.74, 114.56 295.30 C 120.46 291.87, 126.65 289.77, 132.84 288.51 C 139.03 287.26, 145.40 287.18, 151.70 287.77 C 158.00 288.37, 164.36 290.01, 170.64 292.09 C 176.92 294.18, 183.18 297.10, 189.38 300.29 C 195.58 303.47, 201.73 307.29, 207.84 311.20 C 213.95 315.11, 220.00 319.46, 226.02 323.74 C 232.03 328.02, 238.00 332.56, 243.93 336.86 C 249.85 341.17, 255.73 345.56, 261.56 349.55 C 267.40 353.55, 273.18 357.46, 278.92 360.82 C 284.66 364.17, 290.35 367.28, 296.01 369.69 C 301.67 372.11, 307.25 374.12, 312.89 375.31 C 318.52 376.49, 324.09 377.16, 329.82 376.79 C 335.56 376.43, 341.28 375.39, 347.30 373.14 C 353.32 370.90, 365.70 362.76, 365.94 363.31 C 366.19 363.86, 354.69 373.22, 348.78 376.44 C 342.87 379.65, 336.66 381.55, 330.46 382.61 C 324.26 383.68, 317.89 383.58, 311.60 382.82 C 305.30 382.07, 298.95 380.29, 292.68 378.09 C 286.41 375.88, 280.16 372.86, 273.97 369.59 C 267.77 366.33, 261.63 362.45, 255.53 358.49 C 249.43 354.54, 243.37 350.15, 237.36 345.86 C 231.35 341.57, 225.39 337.03, 219.47 332.75 C 213.55 328.47, 207.68 324.11, 201.85 320.17 C 196.03 316.23, 190.25 312.39, 184.52 309.12 C 178.78 305.85, 173.11 302.84, 167.45 300.54 C 161.80 298.24, 156.23 296.36, 150.59 295.32 C 144.96 294.28, 139.40 293.77, 133.66 294.31 C 127.91 294.85, 122.17 296.08, 116.14 298.55 C 110.10 301.02, 97.71 309.66, 97.45 309.12 Z" fill="#f8fafc"/>
    <path d="M 116.51 341.50 C 116.22 340.92, 125.97 331.94, 131.04 328.80 C 136.10 325.66, 141.51 323.77, 146.91 322.67 C 152.30 321.58, 157.91 321.59, 163.41 322.22 C 168.91 322.84, 174.47 324.44, 179.92 326.41 C 185.38 328.37, 190.79 331.11, 196.14 334.01 C 201.48 336.92, 206.76 340.38, 211.99 343.85 C 217.22 347.32, 222.39 351.15, 227.50 354.83 C 232.62 358.51, 237.68 362.38, 242.69 365.93 C 247.69 369.48, 252.64 373.04, 257.55 376.14 C 262.46 379.24, 267.30 382.18, 272.14 384.52 C 276.97 386.86, 281.71 388.88, 286.55 390.18 C 291.38 391.48, 296.11 392.31, 301.14 392.31 C 306.16 392.31, 316.65 389.54, 316.71 390.20 C 316.77 390.85, 306.72 395.18, 301.48 396.21 C 296.24 397.24, 290.71 397.07, 285.27 396.37 C 279.84 395.67, 274.32 394.01, 268.88 392.00 C 263.45 389.99, 258.03 387.24, 252.67 384.33 C 247.31 381.42, 242.00 377.97, 236.74 374.53 C 231.48 371.09, 226.27 367.30, 221.12 363.69 C 215.97 360.07, 210.87 356.30, 205.83 352.86 C 200.79 349.42, 195.81 345.99, 190.89 343.05 C 185.97 340.11, 181.11 337.36, 176.30 335.23 C 171.49 333.10, 166.78 331.31, 162.01 330.26 C 157.25 329.21, 152.59 328.60, 147.72 328.94 C 142.85 329.28, 137.99 330.21, 132.79 332.31 C 127.59 334.40, 116.80 342.09, 116.51 341.50 Z" fill="#e2e8f0"/>
  </g>
</svg>"##;

const DROEM_BRAND_ICON_LIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
  <rect x="0" y="0" width="512" height="512" rx="134.6" fill="#e0eaf8" fill-opacity="0.7686"/>
  <rect x="3" y="3" width="506" height="506" rx="131.6" fill="none" stroke="#64748b" stroke-width="6" stroke-opacity="0.88"/>
  <g transform="translate(256 256) scale(1.36) translate(-234.25 -254.75)">
    <path d="M 257.04 112.55 A 104.22 104.22 0 1 0 371.03 256.31 A 92.18 92.18 0 0 1 257.04 112.55 Z" fill="#0a142c" fill-opacity="1"/>
    <circle cx="346.3" cy="173.9" r="5.5" fill="#0a142c" fill-opacity="1"/>
    <path d="M 97.45 309.12 C 97.19 308.57, 108.66 298.74, 114.56 295.30 C 120.46 291.87, 126.65 289.77, 132.84 288.51 C 139.03 287.26, 145.40 287.18, 151.70 287.77 C 158.00 288.37, 164.36 290.01, 170.64 292.09 C 176.92 294.18, 183.18 297.10, 189.38 300.29 C 195.58 303.47, 201.73 307.29, 207.84 311.20 C 213.95 315.11, 220.00 319.46, 226.02 323.74 C 232.03 328.02, 238.00 332.56, 243.93 336.86 C 249.85 341.17, 255.73 345.56, 261.56 349.55 C 267.40 353.55, 273.18 357.46, 278.92 360.82 C 284.66 364.17, 290.35 367.28, 296.01 369.69 C 301.67 372.11, 307.25 374.12, 312.89 375.31 C 318.52 376.49, 324.09 377.16, 329.82 376.79 C 335.56 376.43, 341.28 375.39, 347.30 373.14 C 353.32 370.90, 365.70 362.76, 365.94 363.31 C 366.19 363.86, 354.69 373.22, 348.78 376.44 C 342.87 379.65, 336.66 381.55, 330.46 382.61 C 324.26 383.68, 317.89 383.58, 311.60 382.82 C 305.30 382.07, 298.95 380.29, 292.68 378.09 C 286.41 375.88, 280.16 372.86, 273.97 369.59 C 267.77 366.33, 261.63 362.45, 255.53 358.49 C 249.43 354.54, 243.37 350.15, 237.36 345.86 C 231.35 341.57, 225.39 337.03, 219.47 332.75 C 213.55 328.47, 207.68 324.11, 201.85 320.17 C 196.03 316.23, 190.25 312.39, 184.52 309.12 C 178.78 305.85, 173.11 302.84, 167.45 300.54 C 161.80 298.24, 156.23 296.36, 150.59 295.32 C 144.96 294.28, 139.40 293.77, 133.66 294.31 C 127.91 294.85, 122.17 296.08, 116.14 298.55 C 110.10 301.02, 97.71 309.66, 97.45 309.12 Z" fill="#0a142c" fill-opacity="1"/>
    <path d="M 116.51 341.50 C 116.22 340.92, 125.97 331.94, 131.04 328.80 C 136.10 325.66, 141.51 323.77, 146.91 322.67 C 152.30 321.58, 157.91 321.59, 163.41 322.22 C 168.91 322.84, 174.47 324.44, 179.92 326.41 C 185.38 328.37, 190.79 331.11, 196.14 334.01 C 201.48 336.92, 206.76 340.38, 211.99 343.85 C 217.22 347.32, 222.39 351.15, 227.50 354.83 C 232.62 358.51, 237.68 362.38, 242.69 365.93 C 247.69 369.48, 252.64 373.04, 257.55 376.14 C 262.46 379.24, 267.30 382.18, 272.14 384.52 C 276.97 386.86, 281.71 388.88, 286.55 390.18 C 291.38 391.48, 296.11 392.31, 301.14 392.31 C 306.16 392.31, 316.65 389.54, 316.71 390.20 C 316.77 390.85, 306.72 395.18, 301.48 396.21 C 296.24 397.24, 290.71 397.07, 285.27 396.37 C 279.84 395.67, 274.32 394.01, 268.88 392.00 C 263.45 389.99, 258.03 387.24, 252.67 384.33 C 247.31 381.42, 242.00 377.97, 236.74 374.53 C 231.48 371.09, 226.27 367.30, 221.12 363.69 C 215.97 360.07, 210.87 356.30, 205.83 352.86 C 200.79 349.42, 195.81 345.99, 190.89 343.05 C 185.97 340.11, 181.11 337.36, 176.30 335.23 C 171.49 333.10, 166.78 331.31, 162.01 330.26 C 157.25 329.21, 152.59 328.60, 147.72 328.94 C 142.85 329.28, 137.99 330.21, 132.79 332.31 C 127.59 334.40, 116.80 342.09, 116.51 341.50 Z" fill="#18284c" fill-opacity="1"/>
  </g>
</svg>"##;

/// Embedded 1x1 subtle translucent PNG icon for native OS file drag preview (`drag::Image::Raw`)
const DRAG_PREVIEW_PNG: &[u8] = &[
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4,
    0x89, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x60, 0x60, 0x60, 0xF8,
    0x0F, 0x00, 0x01, 0x04, 0x01, 0x00, 0x5F, 0xE4, 0x9B, 0xA3, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45,
    0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
];

fn write_file_if_changed(path: &Path, new_bytes: &[u8]) -> bool {
    if let Ok(existing) = fs::read(path) {
        if existing == new_bytes {
            return false;
        }
    }
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            let _ = fs::create_dir_all(parent);
        }
    }
    fs::write(path, new_bytes).is_ok()
}

pub struct WindowPositionMemory {
    pub current_mode: String,
    pub shared_main_pos: Option<PhysicalPosition<i32>>,
    pub mini_pos: Option<PhysicalPosition<i32>>,
    pub backdrop_effect: String,
}

impl Default for WindowPositionMemory {
    fn default() -> Self {
        Self {
            current_mode: "full".to_string(),
            shared_main_pos: None,
            mini_pos: None,
            backdrop_effect: "glass".to_string(),
        }
    }
}

pub struct PendingStartupFiles(pub Mutex<Vec<String>>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricLine {
    pub time: f64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScannedTrack {
    pub id: String,
    pub file_path: String,
    pub folder_path: String,
    pub file_name: String,
    pub track_number: u32,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub year: u32,
    pub duration: u64,
    pub duration_ms: u64,
    pub format: String,
    pub bitrate: u32,
    pub sample_rate: String,
    pub modified_ms: u64,
    pub file_size: u64,
    pub enc_delay: u32,
    pub enc_padding: u32,
    pub total_samples: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackDetail {
    pub file_path: String,
    pub cover_data_url: Option<String>,
    pub lyrics: Vec<LyricLine>,
    pub has_synced_timestamps: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderScanResponse {
    pub folder_path: String,
    pub tracks: Vec<ScannedTrack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DroppedScanResult {
    pub has_folders: bool,
    pub folder_paths: Vec<String>,
    pub tracks: Vec<ScannedTrack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitoredFolderSpec {
    pub path: String,
    pub include_subfolders: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnownFileStat {
    pub file_path: String,
    pub modified_ms: u64,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartSyncResult {
    pub added_tracks: Vec<ScannedTrack>,
    pub updated_tracks: Vec<ScannedTrack>,
    pub removed_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveEntry {
    pub path: String,
    pub name: String,
    pub has_children: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubdirEntry {
    pub path: String,
    pub name: String,
    pub has_children: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeAudioOutputs {
    pub render_devices: Vec<String>,
    pub asio_drivers: Vec<String>,
}

fn get_audio_cache_mutex() -> &'static Mutex<Option<NativeAudioOutputs>> {
    NATIVE_AUDIO_CACHE.get_or_init(|| Mutex::new(None))
}

fn query_native_audio_outputs_sync() -> NativeAudioOutputs {
    let mut render_devices: Vec<String> = Vec::new();
    let mut asio_drivers: Vec<String> = Vec::new();

    #[cfg(target_os = "windows")]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        for reg_path in [r"HKLM\SOFTWARE\ASIO", r"HKLM\SOFTWARE\WOW6432Node\ASIO"] {
            if let Ok(output) = Command::new("reg")
                .args(["query", reg_path, "/s"])
                .creation_flags(CREATE_NO_WINDOW)
                .output()
            {
                let text = String::from_utf8_lossy(&output.stdout);
                let mut current_subkey_name = String::new();
                for line in text.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("HKEY_") {
                        if let Some(last) = trimmed.rsplit('\\').next() {
                            if !last.eq_ignore_ascii_case("ASIO") {
                                current_subkey_name = last.trim().to_string();
                            }
                        }
                    } else if trimmed.starts_with("Description") && trimmed.contains("REG_SZ") {
                        if let Some(idx) = trimmed.find("REG_SZ") {
                            let desc = trimmed[idx + 6..].trim().to_string();
                            if !desc.is_empty() && !asio_drivers.contains(&desc) {
                                asio_drivers.push(desc);
                            }
                        }
                    }
                }
                if asio_drivers.is_empty() && !current_subkey_name.is_empty() {
                    if !asio_drivers.contains(&current_subkey_name) {
                        asio_drivers.push(current_subkey_name);
                    }
                }
            }
        }

        let mm_render_key = r"HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\MMDevices\Audio\Render";
        if let Ok(output) = Command::new("reg")
            .args(["query", mm_render_key, "/s"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
        {
            let dump = String::from_utf8_lossy(&output.stdout);
            let mut active_endpoint = false;
            let mut endpoint_type_name = String::new();
            let mut device_interface_name = String::new();
            let mut friendly_full = String::new();

            let flush_endpoint = |active: bool,
                                  ep_type: &mut String,
                                  dev_if: &mut String,
                                  full: &mut String,
                                  out_vec: &mut Vec<String>| {
                if active {
                    let combined = if !full.is_empty() {
                        full.clone()
                    } else if !ep_type.is_empty() && !dev_if.is_empty() {
                        if ep_type.contains(dev_if.as_str()) {
                            ep_type.clone()
                        } else {
                            format!("{} ({})", ep_type, dev_if)
                        }
                    } else if !ep_type.is_empty() {
                        ep_type.clone()
                    } else {
                        dev_if.clone()
                    };
                    if !combined.is_empty() && !out_vec.contains(&combined) {
                        out_vec.push(combined);
                    }
                }
                ep_type.clear();
                dev_if.clear();
                full.clear();
            };

            for line in dump.lines() {
                let t = line.trim();
                if t.starts_with("HKEY_") {
                    if !t.ends_with("\\Properties") && !t.ends_with("\\FxProperties") {
                        flush_endpoint(
                            active_endpoint,
                            &mut endpoint_type_name,
                            &mut device_interface_name,
                            &mut friendly_full,
                            &mut render_devices,
                        );
                        active_endpoint = false;
                    }
                    continue;
                }

                if t.starts_with("DeviceState") && t.contains("REG_DWORD") {
                    active_endpoint = t.ends_with("0x1");
                    continue;
                }

                if active_endpoint && t.contains("REG_SZ") {
                    if let Some(idx) = t.find("REG_SZ") {
                        let val = t[idx + 6..].trim();
                        if val.is_empty() || val.starts_with('{') || val.starts_with('@') {
                            continue;
                        }
                        if t.contains("{b3f8fa53-0004-438e-9003-51a46e139bfc},6") {
                            device_interface_name = val.to_string();
                        } else if t.contains("{a45c254e-df1c-4efd-8020-67d146a850e0},2") {
                            endpoint_type_name = val.to_string();
                        } else if val.contains('(') && val.contains(')') && friendly_full.is_empty() {
                            friendly_full = val.to_string();
                        }
                    }
                }
            }

            flush_endpoint(
                active_endpoint,
                &mut endpoint_type_name,
                &mut device_interface_name,
                &mut friendly_full,
                &mut render_devices,
            );
        }
    }

    NativeAudioOutputs {
        render_devices,
        asio_drivers,
    }
}

#[tauri::command]
async fn list_native_audio_outputs(force_refresh: Option<bool>) -> Result<NativeAudioOutputs, String> {
    let should_force = force_refresh.unwrap_or(false);
    if !should_force {
        if let Ok(guard) = get_audio_cache_mutex().lock() {
            if let Some(ref cached) = *guard {
                return Ok(cached.clone());
            }
        }
    }

    let fresh = tauri::async_runtime::spawn_blocking(query_native_audio_outputs_sync)
        .await
        .map_err(|e| e.to_string())?;

    if let Ok(mut guard) = get_audio_cache_mutex().lock() {
        *guard = Some(fresh.clone());
    }
    Ok(fresh)
}

/// Phase 38-F: Official v0.1.0 Supported 10 Audio Formats (`MP3, FLAC, WAV, M4A, AAC, OGG, OPUS, WMA, AIF, AIFF`)
/// (`WV, APE, MPC` are excluded from v0.1.0 official support; `is_recognized_external_audio` still accepts them
/// on direct file drop / CLI launch so the app never crashes, shows an unsupported format notice, and skips cleanly.)
fn is_supported_audio(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_ascii_lowercase().as_str(),
            "mp3" | "flac" | "wav" | "m4a" | "aac" | "ogg" | "opus" | "wma" | "aif" | "aiff"
        ),
        None => false,
    }
}

fn is_recognized_external_audio(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_ascii_lowercase().as_str(),
            "mp3" | "flac" | "wav" | "m4a" | "aac" | "ogg" | "opus" | "wma" | "aif" | "aiff" | "wv" | "ape" | "mpc"
        ),
        None => false,
    }
}

fn get_file_stat_pair(path: &Path) -> (u64, u64) {
    if let Ok(meta) = fs::metadata(path) {
        let size = meta.len();
        let mtime = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        return (mtime, size);
    }
    (0, 0)
}

static COVER_CACHE_WRITE_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_COVER_CACHE_FILES: usize = 400;
const MAX_COVER_CACHE_BYTES: u64 = 150 * 1024 * 1024; // 150 MB cap
const MAX_SINGLE_COVER_RAW_BYTES: usize = 4 * 1024 * 1024; // Skip caching abnormally huge >4MB raw covers

/// Phase 35 Part 3C: Persistent User Cover Cache Directory with %TEMP% Fallback
/// - Uses `%LOCALAPPDATA%\droem\cover_cache` on Windows so OS temp-folder cleanup does not wipe cached covers,
///   falling back cleanly to `%TEMP%\droem_cover_cache` if `LOCALAPPDATA` is unavailable.
/// - Always ensures the directory exists so runtime deletion of the cache folder self-heals automatically.
fn get_cover_cache_dir() -> PathBuf {
    let base_dir = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .filter(|p| p.is_dir())
        .map(|p| p.join("droem").join("cover_cache"))
        .unwrap_or_else(|| std::env::temp_dir().join("droem_cover_cache"));
    if let Err(_) = fs::create_dir_all(&base_dir) {
        let fallback = std::env::temp_dir().join("droem_cover_cache");
        let _ = fs::create_dir_all(&fallback);
        return fallback;
    }
    base_dir
}

/// Phase 35 Part 3C: Bounded Disk Cache Eviction Policy
/// - Never touches user music files or library folders; only prunes `.b64` cache entries inside `get_cover_cache_dir()`.
fn prune_cover_cache_if_needed() {
    let dir = get_cover_cache_dir();
    let Ok(read_dir) = fs::read_dir(&dir) else {
        return;
    };
    let mut entries: Vec<(PathBuf, u64, u64)> = Vec::new();
    let mut total_bytes: u64 = 0;

    for entry in read_dir.filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.extension().and_then(|e| e.to_str()) != Some("b64") {
            continue;
        }
        if let Ok(meta) = entry.metadata() {
            if meta.is_file() {
                let len = meta.len();
                let mtime = meta
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0);
                total_bytes += len;
                entries.push((p, len, mtime));
            }
        }
    }

    if entries.len() <= MAX_COVER_CACHE_FILES && total_bytes <= MAX_COVER_CACHE_BYTES {
        return;
    }

    // Sort oldest first by modified timestamp
    entries.sort_by_key(|item| item.2);

    let mut remaining_files = entries.len();
    let mut remaining_bytes = total_bytes;
    let target_files = (MAX_COVER_CACHE_FILES * 3) / 4;
    let target_bytes = (MAX_COVER_CACHE_BYTES * 3) / 4;

    for (path, size, _) in entries {
        if remaining_files <= target_files && remaining_bytes <= target_bytes {
            break;
        }
        if fs::remove_file(&path).is_ok() {
            remaining_files = remaining_files.saturating_sub(1);
            remaining_bytes = remaining_bytes.saturating_sub(size);
        }
    }
}

fn compute_cover_cache_file_path(path: &Path) -> PathBuf {
    let (mtime, size) = get_file_stat_pair(path);
    let norm = path.to_string_lossy().replace('\\', "/").to_lowercase();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    norm.hash(&mut hasher);
    mtime.hash(&mut hasher);
    size.hash(&mut hasher);
    let digest = hasher.finish();
    get_cover_cache_dir().join(format!("{:016x}.b64", digest))
}

fn extract_leading_track_num(stem: &str) -> Option<u32> {
    let trimmed = stem.trim();
    let digits: String = trimmed.chars().take_while(|c| c.is_ascii_digit()).collect();
    if !digits.is_empty() && digits.len() <= 3 {
        if let Ok(num) = digits.parse::<u32>() {
            if num > 0 {
                return Some(num);
            }
        }
    }
    None
}

fn parse_lyrics_text(content: &str) -> (Vec<LyricLine>, bool) {
    let mut timed_lines = Vec::new();
    let mut plain_lines = Vec::new();

    for raw_line in content.lines() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('[') {
            if let Some(close_idx) = trimmed.find(']') {
                let time_part = &trimmed[1..close_idx];
                let text_part = trimmed[close_idx + 1..].trim();
                let mut parts = time_part.split(':');
                if let (Some(m_str), Some(s_str)) = (parts.next(), parts.next()) {
                    let norm_s = s_str.trim().replace(',', ".");
                    if let (Ok(mins), Ok(secs)) = (m_str.trim().parse::<u64>(), norm_s.parse::<f64>()) {
                        if !text_part.is_empty() {
                            let exact_sec = (mins as f64) * 60.0 + secs;
                            timed_lines.push(LyricLine {
                                time: (exact_sec * 1000.0).round() / 1000.0,
                                text: text_part.to_string(),
                            });
                        }
                        continue;
                    }
                }
                if time_part.contains(':') {
                    continue;
                }
            }
        }
        plain_lines.push(trimmed.to_string());
    }

    if !timed_lines.is_empty() {
        timed_lines.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(std::cmp::Ordering::Equal));
        return (timed_lines, true);
    }

    let static_lines: Vec<LyricLine> = plain_lines
        .into_iter()
        .map(|text| LyricLine { time: 0.0, text })
        .collect();
    (static_lines, false)
}

fn parse_companion_lrc(audio_path: &Path) -> (Vec<LyricLine>, bool) {
    let lrc_path = audio_path.with_extension("lrc");
    if let Ok(content) = fs::read_to_string(&lrc_path) {
        let (parsed, has_sync) = parse_lyrics_text(&content);
        if !parsed.is_empty() {
            return (parsed, has_sync);
        }
    }
    let txt_path = audio_path.with_extension("txt");
    if let Ok(content) = fs::read_to_string(&txt_path) {
        let (parsed, has_sync) = parse_lyrics_text(&content);
        if !parsed.is_empty() {
            return (parsed, has_sync);
        }
    }
    (Vec::new(), false)
}

/// Phase 26 Part 5: Extracts encoder delay (`enc_delay`), padding (`enc_padding`), and `total_samples`
/// for accurate Gapless Playback across MP3 (Xing/Info LAME tag), M4A/AAC (`iTunSMPB`), and Opus (`OpusHead`),
/// while keeping 0 trim for lossless formats (`FLAC`, `WAV`, `AIF`, `AIFF`, `WV`, `APE`).
fn extract_codec_gapless_info(
    path: &Path,
    format_upper: &str,
    tagged_file: Option<&lofty::file::TaggedFile>,
) -> (u32, u32, u64) {
    match format_upper {
        "FLAC" | "WAV" | "AIF" | "AIFF" | "WV" | "APE" => (0, 0, 0),
        "M4A" | "AAC" => {
            if let Some(tf) = tagged_file {
                for tag in tf.tags() {
                    for item in tag.items() {
                        if let Some(txt) = item.value().text() {
                            let trimmed = txt.trim();
                            // iTunSMPB format: "00000000 00000840 000001C0 00000000004A8000 ..."
                            let parts: Vec<&str> = trimmed.split_whitespace().collect();
                            if parts.len() >= 4 && parts[0].len() == 8 && parts[1].len() == 8 && parts[2].len() == 8 {
                                if let (Ok(d), Ok(p), Ok(tot)) = (
                                    u32::from_str_radix(parts[1], 16),
                                    u32::from_str_radix(parts[2], 16),
                                    u64::from_str_radix(parts[3], 16),
                                ) {
                                    if d < 16384 && p < 32768 {
                                        return (d, p, tot);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            (0, 0, 0)
        }
        "MP3" => {
            use std::io::{Read, Seek, SeekFrom};
            let Ok(mut f) = fs::File::open(path) else {
                return (0, 0, 0);
            };
            let mut id3_hdr = [0u8; 10];
            let mut audio_offset: u64 = 0;
            if f.read_exact(&mut id3_hdr).is_ok() && &id3_hdr[0..3] == b"ID3" {
                let sz = ((id3_hdr[6] as u64 & 0x7F) << 21)
                    | ((id3_hdr[7] as u64 & 0x7F) << 14)
                    | ((id3_hdr[8] as u64 & 0x7F) << 7)
                    | (id3_hdr[9] as u64 & 0x7F);
                audio_offset = 10 + sz;
            }
            let _ = f.seek(SeekFrom::Start(audio_offset));
            let mut buf = vec![0u8; 1024];
            let Ok(n) = f.read(&mut buf) else {
                return (0, 0, 0);
            };
            let slice = &buf[..n];
            for idx in 0..slice.len().saturating_sub(36) {
                if &slice[idx..idx + 4] == b"Xing" || &slice[idx..idx + 4] == b"Info" {
                    // Search for "LAME" marker within next 160 bytes
                    let end_search = (idx + 160).min(slice.len().saturating_sub(24));
                    for l_idx in idx..end_search {
                        if &slice[l_idx..l_idx + 4] == b"LAME" && l_idx + 24 <= slice.len() {
                            let b0 = slice[l_idx + 21];
                            let b1 = slice[l_idx + 22];
                            let b2 = slice[l_idx + 23];
                            let delay = ((b0 as u32) << 4) | ((b1 as u32) >> 4);
                            let padding = (((b1 & 0x0F) as u32) << 8) | (b2 as u32);
                            if delay < 8192 && padding < 16384 {
                                return (delay, padding, 0);
                            }
                        }
                    }
                }
            }
            (0, 0, 0)
        }
        "OPUS" => {
            use std::io::Read;
            let Ok(mut f) = fs::File::open(path) else {
                return (0, 0, 0);
            };
            let mut buf = [0u8; 256];
            if let Ok(n) = f.read(&mut buf) {
                let slice = &buf[..n];
                for i in 0..slice.len().saturating_sub(12) {
                    if &slice[i..i + 8] == b"OpusHead" {
                        let pre_skip = u16::from_le_bytes([slice[i + 10], slice[i + 11]]) as u32;
                        return (pre_skip, 0, 0);
                    }
                }
            }
            (0, 0, 0)
        }
        _ => (0, 0, 0),
    }
}


#[derive(Debug, Clone, Default)]
struct AsfWmaMetadata {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    genre: Option<String>,
    year: Option<u32>,
    track_number: Option<u32>,
    lyrics: Option<String>,
    cover_data_url: Option<String>,
    duration_ms: u64,
    bitrate_kbps: u32,
    sample_rate_hz: u32,
}

fn read_utf16le_clean(bytes: &[u8]) -> String {
    let mut u16s = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks_exact(2) {
        let v = u16::from_le_bytes([chunk[0], chunk[1]]);
        if v == 0 {
            break;
        }
        u16s.push(v);
    }
    String::from_utf16_lossy(&u16s).trim().to_string()
}

fn parse_wm_picture_payload(val_bytes: &[u8]) -> Option<String> {
    // WM/Picture layout:
    // byte 0: picture type
    // bytes 1..5: image data length (u32 LE)
    // followed by null-terminated UTF-16LE MIME string
    // followed by null-terminated UTF-16LE Description string
    // followed by raw image bytes
    if val_bytes.len() < 12 {
        return None;
    }
    let data_len = u32::from_le_bytes([val_bytes[1], val_bytes[2], val_bytes[3], val_bytes[4]]) as usize;
    if data_len == 0 || data_len > MAX_SINGLE_COVER_RAW_BYTES || data_len + 5 > val_bytes.len() {
        return None;
    }
    let mut pos = 5usize;
    let mut mime_u16 = Vec::new();
    while pos + 2 <= val_bytes.len() {
        let ch = u16::from_le_bytes([val_bytes[pos], val_bytes[pos + 1]]);
        pos += 2;
        if ch == 0 {
            break;
        }
        mime_u16.push(ch);
    }
    while pos + 2 <= val_bytes.len() {
        let ch = u16::from_le_bytes([val_bytes[pos], val_bytes[pos + 1]]);
        pos += 2;
        if ch == 0 {
            break;
        }
    }
    if pos + data_len > val_bytes.len() {
        return None;
    }
    let img_bytes = &val_bytes[pos..pos + data_len];
    let mut mime = String::from_utf16_lossy(&mime_u16).trim().to_string();
    if mime.is_empty() || !mime.contains('/') {
        if img_bytes.starts_with(&[0x89, b'P', b'N', b'G']) {
            mime = "image/png".to_string();
        } else {
            mime = "image/jpeg".to_string();
        }
    }
    let b64 = BASE64.encode(img_bytes);
    Some(format!("data:{};base64,{}", mime, b64))
}

/// Pure-Rust ASF / WMA Binary Header, Tag & Cover Art Parser (`parse_asf_wma_metadata_and_cover`)
/// Reason: `lofty 0.21` has no ASF/WMA support, which previously left `.wma` files with `duration = 0`
/// and missing tags/covers. This parser reads standard ASF GUID objects directly from the file header.
fn parse_asf_wma_metadata_and_cover(path: &Path, need_cover: bool) -> Option<AsfWmaMetadata> {
    use std::io::Read;
    let mut f = fs::File::open(path).ok()?;
    let mut head30 = [0u8; 30];
    if f.read_exact(&mut head30).is_err() {
        return None;
    }
    const GUID_ASF_HEADER: [u8; 16] = [
        0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C,
    ];
    if head30[0..16] != GUID_ASF_HEADER {
        return None;
    }
    let header_size = u64::from_le_bytes([
        head30[16], head30[17], head30[18], head30[19], head30[20], head30[21], head30[22], head30[23],
    ]) as usize;
    let max_read = header_size.clamp(30, 8 * 1024 * 1024);
    let mut buf = Vec::with_capacity(max_read);
    buf.extend_from_slice(&head30);
    let rem = max_read.saturating_sub(30);
    if rem > 0 {
        let mut tail = vec![0u8; rem];
        let n = f.read(&mut tail).ok()?;
        buf.extend_from_slice(&tail[..n]);
    }

    const GUID_FILE_PROPS: [u8; 16] = [
        0xA1, 0xDC, 0xAB, 0x8C, 0x47, 0xA9, 0xCF, 0x11, 0x8E, 0xE4, 0x00, 0xC0, 0x0C, 0x20, 0x53, 0x65,
    ];
    const GUID_STREAM_PROPS: [u8; 16] = [
        0x91, 0x07, 0xDC, 0xB7, 0xB7, 0xA9, 0xCF, 0x11, 0x8E, 0xE6, 0x00, 0xC0, 0x0C, 0x20, 0x53, 0x65,
    ];
    const GUID_AUDIO_MEDIA: [u8; 16] = [
        0x40, 0x9E, 0x69, 0xF8, 0x4D, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80, 0x5F, 0x5C, 0x44, 0x2B,
    ];
    const GUID_CONTENT_DESC: [u8; 16] = [
        0x33, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C,
    ];
    const GUID_EXT_CONTENT_DESC: [u8; 16] = [
        0x40, 0xA4, 0xD0, 0xD2, 0x07, 0xE3, 0xD2, 0x11, 0x97, 0xF0, 0x00, 0xA0, 0xC9, 0x5E, 0xA8, 0x50,
    ];

    let mut meta = AsfWmaMetadata::default();
    let mut offset = 30usize;
    while offset + 24 <= buf.len() {
        let guid = &buf[offset..offset + 16];
        let obj_size = u64::from_le_bytes([
            buf[offset + 16],
            buf[offset + 17],
            buf[offset + 18],
            buf[offset + 19],
            buf[offset + 20],
            buf[offset + 21],
            buf[offset + 22],
            buf[offset + 23],
        ]) as usize;
        if obj_size < 24 || offset + obj_size > buf.len() {
            break;
        }
        let body = &buf[offset + 24..offset + obj_size];

        if guid == GUID_FILE_PROPS && body.len() >= 80 {
            let play_dur_100ns = u64::from_le_bytes([
                body[40], body[41], body[42], body[43], body[44], body[45], body[46], body[47],
            ]);
            let preroll_ms = u64::from_le_bytes([
                body[56], body[57], body[58], body[59], body[60], body[61], body[62], body[63],
            ]);
            let max_bitrate = u32::from_le_bytes([body[76], body[77], body[78], body[79]]);
            let play_ms = play_dur_100ns / 10_000;
            let eff_ms = play_ms.saturating_sub(preroll_ms);
            if eff_ms > 0 {
                meta.duration_ms = eff_ms;
            } else if play_ms > 0 {
                meta.duration_ms = play_ms;
            }
            if max_bitrate > 0 && meta.bitrate_kbps == 0 {
                meta.bitrate_kbps = (max_bitrate / 1000).max(1);
            }
        } else if guid == GUID_STREAM_PROPS && body.len() >= 54 {
            if body[0..16] == GUID_AUDIO_MEDIA {
                let type_spec_len = u32::from_le_bytes([body[40], body[41], body[42], body[43]]) as usize;
                if body.len() >= 54 + 14 && type_spec_len >= 14 {
                    let wf = &body[54..54 + type_spec_len.min(body.len() - 54)];
                    let sr = u32::from_le_bytes([wf[4], wf[5], wf[6], wf[7]]);
                    let avg_bps = u32::from_le_bytes([wf[8], wf[9], wf[10], wf[11]]);
                    if sr > 0 {
                        meta.sample_rate_hz = sr;
                    }
                    if avg_bps > 0 {
                        meta.bitrate_kbps = ((avg_bps * 8) / 1000).max(1);
                    }
                }
            }
        } else if guid == GUID_CONTENT_DESC && body.len() >= 10 {
            let title_len = u16::from_le_bytes([body[0], body[1]]) as usize;
            let author_len = u16::from_le_bytes([body[2], body[3]]) as usize;
            let mut p = 10usize;
            if p + title_len <= body.len() {
                let s = read_utf16le_clean(&body[p..p + title_len]);
                if !s.is_empty() {
                    meta.title = Some(s);
                }
                p += title_len;
            }
            if p + author_len <= body.len() {
                let s = read_utf16le_clean(&body[p..p + author_len]);
                if !s.is_empty() {
                    meta.artist = Some(s);
                }
            }
        } else if guid == GUID_EXT_CONTENT_DESC && body.len() >= 2 {
            let desc_count = u16::from_le_bytes([body[0], body[1]]) as usize;
            let mut p = 2usize;
            for _ in 0..desc_count {
                if p + 2 > body.len() {
                    break;
                }
                let name_len = u16::from_le_bytes([body[p], body[p + 1]]) as usize;
                p += 2;
                if p + name_len + 4 > body.len() {
                    break;
                }
                let name = read_utf16le_clean(&body[p..p + name_len]);
                p += name_len;
                let dtype = u16::from_le_bytes([body[p], body[p + 1]]);
                let vlen = u16::from_le_bytes([body[p + 2], body[p + 3]]) as usize;
                p += 4;
                if p + vlen > body.len() {
                    break;
                }
                let val_slice = &body[p..p + vlen];
                p += vlen;

                match dtype {
                    0 => {
                        let val_str = read_utf16le_clean(val_slice);
                        if val_str.is_empty() {
                            continue;
                        }
                        if name.eq_ignore_ascii_case("WM/AlbumTitle") {
                            meta.album = Some(val_str);
                        } else if name.eq_ignore_ascii_case("WM/Author")
                            || (name.eq_ignore_ascii_case("WM/AlbumArtist") && meta.artist.is_none())
                        {
                            meta.artist = Some(val_str);
                        } else if name.eq_ignore_ascii_case("WM/Genre") {
                            meta.genre = Some(val_str);
                        } else if name.eq_ignore_ascii_case("WM/Year") {
                            if let Ok(y) = val_str.chars().take(4).collect::<String>().parse::<u32>() {
                                if (1900..=2100).contains(&y) {
                                    meta.year = Some(y);
                                }
                            }
                        } else if name.eq_ignore_ascii_case("WM/TrackNumber") {
                            if let Ok(tn) = val_str.split('/').next().unwrap_or("").trim().parse::<u32>() {
                                if tn > 0 {
                                    meta.track_number = Some(tn);
                                }
                            }
                        } else if name.eq_ignore_ascii_case("WM/Track") && meta.track_number.is_none() {
                            if let Ok(tn) = val_str.trim().parse::<u32>() {
                                meta.track_number = Some(tn + 1);
                            }
                        } else if name.eq_ignore_ascii_case("WM/Lyrics") {
                            meta.lyrics = Some(val_str);
                        }
                    }
                    1 => {
                        if need_cover && name.eq_ignore_ascii_case("WM/Picture") && meta.cover_data_url.is_none() {
                            meta.cover_data_url = parse_wm_picture_payload(val_slice);
                        }
                    }
                    3 => {
                        if vlen >= 4 {
                            let num = u32::from_le_bytes([val_slice[0], val_slice[1], val_slice[2], val_slice[3]]);
                            if name.eq_ignore_ascii_case("WM/TrackNumber") && num > 0 {
                                meta.track_number = Some(num);
                            } else if name.eq_ignore_ascii_case("WM/Track") && meta.track_number.is_none() {
                                meta.track_number = Some(num + 1);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        offset += obj_size;
    }

    Some(meta)
}

fn skip_leading_id3v2_Slice(data: &[u8]) -> &[u8] {
    if data.len() >= 10 && &data[0..3] == b"ID3" {
        let sz = (((data[6] & 0x7F) as usize) << 21)
            | (((data[7] & 0x7F) as usize) << 14)
            | (((data[8] & 0x7F) as usize) << 7)
            | ((data[9] & 0x7F) as usize);
        let skip = (10 + sz).min(data.len());
        &data[skip..]
    } else {
        data
    }
}

/// Pure-Rust WavPack (`wvpk`) Header Summary Probe (`(duration_ms, bitrate_kbps, sample_rate_str)`)
fn probe_wv_header_summary(path: &Path) -> Option<(u64, u32, String)> {
    use std::io::Read;
    let file_size = fs::metadata(path).ok()?.len();
    let mut f = fs::File::open(path).ok()?;
    let mut buf = vec![0u8; 8192];
    let n = f.read(&mut buf).ok()?;
    let slice = skip_leading_id3v2_Slice(&buf[..n]);
    for i in 0..slice.len().saturating_sub(32) {
        if &slice[i..i + 4] == b"wvpk" {
            let total_samples = u32::from_le_bytes([
                slice[i + 12],
                slice[i + 13],
                slice[i + 14],
                slice[i + 15],
            ]);
            let flags = u32::from_le_bytes([
                slice[i + 24],
                slice[i + 25],
                slice[i + 26],
                slice[i + 27],
            ]);
            if total_samples == 0 || total_samples == u32::MAX {
                return None;
            }
            const WV_SR_TABLE: [u32; 15] = [
                6000, 8000, 9600, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000, 64000,
                88200, 96000, 192000,
            ];
            let sr_idx = ((flags >> 23) & 0xF) as usize;
            let sr = if sr_idx < WV_SR_TABLE.len() {
                WV_SR_TABLE[sr_idx]
            } else {
                44100
            };
            let dur_ms = (((total_samples as f64) / (sr as f64)) * 1000.0).round().max(1.0) as u64;
            let kbps = if dur_ms > 0 {
                (((file_size as f64) * 8.0) / (dur_ms as f64)).round().max(64.0) as u32
            } else {
                900
            };
            let sr_str = format!("{:.1}kHz", (sr as f64) / 1000.0).replace(".0kHz", "kHz");
            return Some((dur_ms, kbps, sr_str));
        }
    }
    None
}

/// Pure-Rust Monkey's Audio (`MAC `) Header Summary Probe (`(duration_ms, bitrate_kbps, sample_rate_str)`)
fn probe_ape_header_summary(path: &Path) -> Option<(u64, u32, String)> {
    use std::io::Read;
    let file_size = fs::metadata(path).ok()?.len();
    let mut f = fs::File::open(path).ok()?;
    let mut buf = vec![0u8; 8192];
    let n = f.read(&mut buf).ok()?;
    let slice = skip_leading_id3v2_Slice(&buf[..n]);
    for i in 0..slice.len().saturating_sub(76) {
        if &slice[i..i + 4] == b"MAC " {
            let version = u16::from_le_bytes([slice[i + 4], slice[i + 5]]);
            let (total_samples, sr) = if version >= 3980 {
                let desc_len = u32::from_le_bytes([
                    slice[i + 8],
                    slice[i + 9],
                    slice[i + 10],
                    slice[i + 11],
                ]) as usize;
                let hdr_off = i + desc_len;
                if hdr_off + 24 > slice.len() {
                    return None;
                }
                let blocks_per_frame = u32::from_le_bytes([
                    slice[hdr_off + 4],
                    slice[hdr_off + 5],
                    slice[hdr_off + 6],
                    slice[hdr_off + 7],
                ]) as u64;
                let final_blocks = u32::from_le_bytes([
                    slice[hdr_off + 8],
                    slice[hdr_off + 9],
                    slice[hdr_off + 10],
                    slice[hdr_off + 11],
                ]) as u64;
                let total_frames = u32::from_le_bytes([
                    slice[hdr_off + 12],
                    slice[hdr_off + 13],
                    slice[hdr_off + 14],
                    slice[hdr_off + 15],
                ]) as u64;
                let srate = u32::from_le_bytes([
                    slice[hdr_off + 20],
                    slice[hdr_off + 21],
                    slice[hdr_off + 22],
                    slice[hdr_off + 23],
                ]);
                let samples = if total_frames > 0 {
                    (total_frames - 1) * blocks_per_frame + final_blocks
                } else {
                    0
                };
                (samples, srate)
            } else {
                let comp_level = u16::from_le_bytes([slice[i + 6], slice[i + 7]]);
                let srate = u32::from_le_bytes([
                    slice[i + 12],
                    slice[i + 13],
                    slice[i + 14],
                    slice[i + 15],
                ]);
                let total_frames = u32::from_le_bytes([
                    slice[i + 20],
                    slice[i + 21],
                    slice[i + 22],
                    slice[i + 23],
                ]) as u64;
                let final_blocks = u32::from_le_bytes([
                    slice[i + 24],
                    slice[i + 25],
                    slice[i + 26],
                    slice[i + 27],
                ]) as u64;
                let bpf = if version >= 3950 {
                    73728 * 4
                } else if version >= 3900 || (version >= 3800 && comp_level == 4000) {
                    73728
                } else {
                    9216
                };
                let samples = if total_frames > 0 {
                    (total_frames - 1) * bpf + final_blocks
                } else {
                    0
                };
                (samples, srate)
            };

            if total_samples == 0 || sr == 0 {
                return None;
            }
            let dur_ms = (((total_samples as f64) / (sr as f64)) * 1000.0).round().max(1.0) as u64;
            let kbps = if dur_ms > 0 {
                (((file_size as f64) * 8.0) / (dur_ms as f64)).round().max(64.0) as u32
            } else {
                900
            };
            let sr_str = format!("{:.1}kHz", (sr as f64) / 1000.0).replace(".0kHz", "kHz");
            return Some((dur_ms, kbps, sr_str));
        }
    }
    None
}

/// Pure-Rust Musepack (`MP+` SV7 / `MPCK` SV8) Header Summary Probe (`(duration_ms, bitrate_kbps, sample_rate_str)`)
fn probe_mpc_header_summary(path: &Path) -> Option<(u64, u32, String)> {
    use std::io::Read;
    let file_size = fs::metadata(path).ok()?.len();
    let mut f = fs::File::open(path).ok()?;
    let mut buf = vec![0u8; 4096];
    let n = f.read(&mut buf).ok()?;
    let slice = skip_leading_id3v2_Slice(&buf[..n]);
    if slice.len() >= 16 && &slice[0..3] == b"MP+" {
        let frame_count = u32::from_le_bytes([slice[4], slice[5], slice[6], slice[7]]) as u64;
        let sr_idx = ((slice[10] >> 4) & 0x03) as usize;
        const MPC_SV7_SR: [u32; 4] = [44100, 48000, 37800, 32000];
        let sr = MPC_SV7_SR[sr_idx];
        let total_samples = frame_count * 1152;
        if total_samples > 0 {
            let dur_ms = (((total_samples as f64) / (sr as f64)) * 1000.0).round().max(1.0) as u64;
            let kbps = (((file_size as f64) * 8.0) / (dur_ms as f64)).round().max(64.0) as u32;
            let sr_str = format!("{:.1}kHz", (sr as f64) / 1000.0).replace(".0kHz", "kHz");
            return Some((dur_ms, kbps, sr_str));
        }
    }
    None
}

/// Pure-Rust RIFF WAVE Summary Probe (`(duration_ms, bitrate_kbps, sample_rate_str)`)
fn probe_wav_header_summary(path: &Path) -> Option<(u64, u32, String)> {
    use std::io::Read;
    let mut f = fs::File::open(path).ok()?;
    let mut buf = [0u8; 512];
    let n = f.read(&mut buf).ok()?;
    let data = &buf[..n];
    if data.len() < 44 || &data[0..4] != b"RIFF" || &data[8..12] != b"WAVE" {
        return None;
    }
    let mut idx = 12usize;
    let mut sr = 44100u32;
    let mut byte_rate = 176400u32;
    let mut data_bytes = 0u64;
    while idx + 8 <= data.len() {
        let cid = &data[idx..idx + 4];
        let csz = u32::from_le_bytes([data[idx + 4], data[idx + 5], data[idx + 6], data[idx + 7]]) as usize;
        let body = idx + 8;
        if cid == b"fmt " && body + 16 <= data.len() {
            sr = u32::from_le_bytes([data[body + 4], data[body + 5], data[body + 6], data[body + 7]]).max(1);
            byte_rate = u32::from_le_bytes([data[body + 8], data[body + 9], data[body + 10], data[body + 11]]).max(1);
        } else if cid == b"data" {
            data_bytes = csz as u64;
            break;
        }
        idx = body.saturating_add(csz).saturating_add(csz & 1);
    }
    if data_bytes == 0 {
        data_bytes = fs::metadata(path).map(|m| m.len().saturating_sub(44)).unwrap_or(0);
    }
    let dur_ms = (((data_bytes as f64) / (byte_rate as f64)) * 1000.0).round().max(1.0) as u64;
    let kbps = ((byte_rate * 8) / 1000).max(64);
    let sr_str = format!("{:.1}kHz", (sr as f64) / 1000.0).replace(".0kHz", "kHz");
    Some((dur_ms, kbps, sr_str))
}

fn parse_audio_file_fast(path: &Path) -> Option<ScannedTrack> {
    if !path.is_file() || !is_recognized_external_audio(path) {
        return None;
    }

    let file_path = path.to_string_lossy().replace('\\', "/");
    let folder_path = path
        .parent()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|| "Local Music".to_string());

    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown.flac")
        .to_string();

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown Track");

    let fallback_title = stem.to_string();
    let mut track_number = extract_leading_track_num(stem).unwrap_or(0);

    let format = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("FLAC")
        .to_ascii_uppercase();

    let (modified_ms, file_size) = get_file_stat_pair(path);

    let mut title = fallback_title;
    let mut artist = "Unknown Artist".to_string();
    let mut album = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown Album")
        .to_string();
    let mut genre = "Music".to_string();
    let mut year = 2025u32;
    let mut duration = 0u64;
    let mut duration_ms = 0u64;
    let mut bitrate = if format == "FLAC" || format == "WAV" { 1411 } else { 320 };
    let mut sample_rate = "44.1kHz".to_string();

    let parse_opts = ParseOptions::new().read_cover_art(false);
    if let Ok(probe) = Probe::open(path) {
        if let Ok(tagged_file) = probe.options(parse_opts).read() {
            let props = tagged_file.properties();
            let dur_ms = props.duration().as_millis() as u64;
            if dur_ms > 0 {
                duration_ms = dur_ms;
                duration = ((dur_ms as f64) / 1000.0).round().max(1.0) as u64;
            }
            if let Some(br) = props.audio_bitrate() {
                if br > 0 {
                    bitrate = br;
                }
            }
            if let Some(sr) = props.sample_rate() {
                if sr > 0 {
                    sample_rate = format!("{:.1}kHz", (sr as f64) / 1000.0).replace(".0kHz", "kHz");
                }
            }

            if let Some(tag) = tagged_file.primary_tag().or_else(|| tagged_file.first_tag()) {
                if let Some(t) = tag.title() {
                    if !t.trim().is_empty() {
                        title = t.into_owned();
                    }
                }
                if let Some(a) = tag.artist() {
                    if !a.trim().is_empty() {
                        artist = a.into_owned();
                    }
                }
                if let Some(al) = tag.album() {
                    if !al.trim().is_empty() {
                        album = al.into_owned();
                    }
                }
                if let Some(g) = tag.genre() {
                    if !g.trim().is_empty() {
                        genre = g.into_owned();
                    }
                }
                if let Some(y) = tag.year() {
                    if y > 1900 && y < 2100 {
                        year = y;
                    }
                }
                if let Some(tr) = tag.track() {
                    if tr > 0 {
                        track_number = tr;
                    }
                }
            }
            if (format == "AIF" || format == "AIFF") && duration == 0 {
                if let Some((aiff_dur_ms, aiff_br, aiff_sr)) = probe_aiff_header_summary(path) {
                    duration_ms = aiff_dur_ms.max(1);
                    duration = ((aiff_dur_ms as f64) / 1000.0).round().max(1.0) as u64;
                    if aiff_br > 0 {
                        bitrate = aiff_br;
                    }
                    if !aiff_sr.is_empty() {
                        sample_rate = aiff_sr;
                    }
                }
            }
            let (enc_delay, enc_padding, total_samples) =
                extract_codec_gapless_info(path, &format, Some(&tagged_file));
            return Some(ScannedTrack {
                id: format!("track-{}", file_path),
                file_path,
                folder_path,
                file_name,
                track_number,
                title,
                artist,
                album,
                genre,
                year,
                duration,
                duration_ms,
                format,
                bitrate,
                sample_rate,
                modified_ms,
                file_size,
                enc_delay,
                enc_padding,
                total_samples,
            });
        }
    }

    if format == "WMA" {
        if let Some(wma) = parse_asf_wma_metadata_and_cover(path, false) {
            if let Some(t) = wma.title { if !t.is_empty() { title = t; } }
            if let Some(a) = wma.artist { if !a.is_empty() { artist = a; } }
            if let Some(al) = wma.album { if !al.is_empty() { album = al; } }
            if let Some(g) = wma.genre { if !g.is_empty() { genre = g; } }
            if let Some(y) = wma.year { year = y; }
            if let Some(tn) = wma.track_number { track_number = tn; }
            if wma.duration_ms > 0 {
                duration_ms = wma.duration_ms;
                duration = ((wma.duration_ms as f64) / 1000.0).round().max(1.0) as u64;
            }
            if wma.bitrate_kbps > 0 {
                bitrate = wma.bitrate_kbps;
            }
            if wma.sample_rate_hz > 0 {
                sample_rate = format!("{:.1}kHz", (wma.sample_rate_hz as f64) / 1000.0).replace(".0kHz", "kHz");
            }
        }
    } else if (format == "AIF" || format == "AIFF") && duration == 0 {
        if let Some((aiff_dur_ms, aiff_br, aiff_sr)) = probe_aiff_header_summary(path) {
            duration_ms = aiff_dur_ms.max(1);
            duration = ((aiff_dur_ms as f64) / 1000.0).round().max(1.0) as u64;
            if aiff_br > 0 {
                bitrate = aiff_br;
            }
            if !aiff_sr.is_empty() {
                sample_rate = aiff_sr;
            }
        }
    } else if format == "WV" && duration == 0 {
        if let Some((wv_dur_ms, wv_br, wv_sr)) = probe_wv_header_summary(path) {
            duration_ms = wv_dur_ms.max(1);
            duration = ((wv_dur_ms as f64) / 1000.0).round().max(1.0) as u64;
            if wv_br > 0 { bitrate = wv_br; }
            if !wv_sr.is_empty() { sample_rate = wv_sr; }
        }
    } else if format == "APE" && duration == 0 {
        if let Some((ape_dur_ms, ape_br, ape_sr)) = probe_ape_header_summary(path) {
            duration_ms = ape_dur_ms.max(1);
            duration = ((ape_dur_ms as f64) / 1000.0).round().max(1.0) as u64;
            if ape_br > 0 { bitrate = ape_br; }
            if !ape_sr.is_empty() { sample_rate = ape_sr; }
        }
    } else if format == "MPC" && duration == 0 {
        if let Some((mpc_dur_ms, mpc_br, mpc_sr)) = probe_mpc_header_summary(path) {
            duration_ms = mpc_dur_ms.max(1);
            duration = ((mpc_dur_ms as f64) / 1000.0).round().max(1.0) as u64;
            if mpc_br > 0 { bitrate = mpc_br; }
            if !mpc_sr.is_empty() { sample_rate = mpc_sr; }
        }
    }

    let (enc_delay, enc_padding, total_samples) = extract_codec_gapless_info(path, &format, None);

    Some(ScannedTrack {
        id: format!("track-{}", file_path),
        file_path,
        folder_path,
        file_name,
        track_number,
        title,
        artist,
        album,
        genre,
        year,
        duration,
        duration_ms,
        format,
        bitrate,
        sample_rate,
        modified_ms,
        file_size,
        enc_delay,
        enc_padding,
        total_samples,
    })
}

/// Phase 36: Natural Sort comparator for multi-file imports (`1.mp3` -> `2.mp3` -> `10.mp3`, `01` -> `02`)
/// Handles ASCII digits as numeric values, case-insensitive comparison first, then exact case/path tie-breaking.
fn natural_cmp_str(a: &str, b: &str) -> std::cmp::Ordering {
    let mut chars_a = a.chars().peekable();
    let mut chars_b = b.chars().peekable();

    loop {
        match (chars_a.peek(), chars_b.peek()) {
            (None, None) => return a.cmp(b),
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (Some(&ca), Some(&cb)) => {
                if ca.is_ascii_digit() && cb.is_ascii_digit() {
                    let mut num_a = String::new();
                    while let Some(&c) = chars_a.peek() {
                        if c.is_ascii_digit() {
                            num_a.push(c);
                            chars_a.next();
                        } else {
                            break;
                        }
                    }
                    let mut num_b = String::new();
                    while let Some(&c) = chars_b.peek() {
                        if c.is_ascii_digit() {
                            num_b.push(c);
                            chars_b.next();
                        } else {
                            break;
                        }
                    }
                    let trim_a = num_a.trim_start_matches('0');
                    let trim_b = num_b.trim_start_matches('0');
                    let cmp_len = trim_a.len().cmp(&trim_b.len());
                    if cmp_len != std::cmp::Ordering::Equal {
                        return cmp_len;
                    }
                    let cmp_val = trim_a.cmp(trim_b);
                    if cmp_val != std::cmp::Ordering::Equal {
                        return cmp_val;
                    }
                    let cmp_zeros = num_a.len().cmp(&num_b.len());
                    if cmp_zeros != std::cmp::Ordering::Equal {
                        return cmp_zeros;
                    }
                } else {
                    let la = ca.to_lowercase().to_string();
                    let lb = cb.to_lowercase().to_string();
                    let cmp_ch = la.cmp(&lb);
                    if cmp_ch != std::cmp::Ordering::Equal {
                        return cmp_ch;
                    }
                    chars_a.next();
                    chars_b.next();
                }
            }
        }
    }
}

/// Phase 36: Sort multiple external audio files by `file_name` Natural Sort with deterministic tie-breakers
/// Tie-breaker order:
/// 1. `file_name` natural sort (case-insensitive numeric)
/// 2. `file_name` exact case
/// 3. `folder_path` natural sort
/// 4. `file_path` natural sort
fn sort_tracks_by_filename_natural(tracks: &mut [ScannedTrack]) {
    if tracks.len() <= 1 {
        return;
    }
    tracks.sort_by(|a, b| {
        natural_cmp_str(&a.file_name, &b.file_name)
            .then_with(|| a.file_name.cmp(&b.file_name))
            .then_with(|| natural_cmp_str(&a.folder_path, &b.folder_path))
            .then_with(|| natural_cmp_str(&a.file_path, &b.file_path))
    });
}

/// Phase 36: Sort folder-scanned tracks preserving folder hierarchy, track number, and natural filename order
fn sort_tracks_for_album_folder(tracks: &mut [ScannedTrack]) {
    if tracks.len() <= 1 {
        return;
    }
    tracks.sort_by(|a, b| {
        natural_cmp_str(&a.folder_path, &b.folder_path)
            .then_with(|| {
                let ta = if a.track_number > 0 { a.track_number } else { u32::MAX };
                let tb = if b.track_number > 0 { b.track_number } else { u32::MAX };
                ta.cmp(&tb)
            })
            .then_with(|| natural_cmp_str(&a.file_name, &b.file_name))
            .then_with(|| natural_cmp_str(&a.file_path, &b.file_path))
    });
}

fn collect_audio_files_from_specs(specs: &[MonitoredFolderSpec]) -> Vec<PathBuf> {
    let mut seen = HashSet::new();
    let mut audio_paths = Vec::new();

    for spec in specs {
        let norm_input = spec.path.replace('/', "\\");
        let root = if Path::new(&norm_input).exists() {
            PathBuf::from(&norm_input)
        } else {
            PathBuf::from(&spec.path)
        };
        if !root.exists() {
            continue;
        }

        let walker = if spec.include_subfolders {
            WalkDir::new(&root).follow_links(true)
        } else {
            WalkDir::new(&root).max_depth(1).follow_links(true)
        };

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.is_file() && is_supported_audio(p) {
                let key = p.to_string_lossy().replace('\\', "/");
                if seen.insert(key) {
                    audio_paths.push(p.to_path_buf());
                }
            }
        }
    }

    audio_paths
}

fn dir_has_subdirectories(dir: &Path) -> bool {
    let Ok(entries) = fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.filter_map(|e| e.ok()) {
        if let Ok(ft) = entry.file_type() {
            if ft.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                if !name.starts_with('$')
                    && !name.starts_with('.')
                    && !name.eq_ignore_ascii_case("System Volume Information")
                {
                    return true;
                }
            }
        }
    }
    false
}

#[tauri::command]
fn list_system_drives() -> Result<Vec<DriveEntry>, String> {
    let mut drives = Vec::new();

    #[cfg(target_os = "windows")]
    {
        for letter in b'A'..=b'Z' {
            let ch = letter as char;
            let win_root = format!("{}:\\", ch);
            let p = Path::new(&win_root);
            if p.exists() {
                let norm_path = format!("{}:/", ch);
                let display = format!("Local Disk ({}:)", ch);
                drives.push(DriveEntry {
                    path: norm_path,
                    name: display,
                    has_children: true,
                });
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        drives.push(DriveEntry {
            path: "/".to_string(),
            name: "Root (/)".to_string(),
            has_children: true,
        });
    }

    Ok(drives)
}

#[tauri::command]
fn list_subdirectories(folder_path: String) -> Result<Vec<SubdirEntry>, String> {
    let target_str = if cfg!(target_os = "windows") {
        let s = folder_path.replace('/', "\\");
        if s.len() == 2 && s.ends_with(':') {
            format!("{}\\", s)
        } else {
            s
        }
    } else {
        folder_path.clone()
    };

    let root = Path::new(&target_str);
    let Ok(read_dir) = fs::read_dir(root) else {
        return Ok(Vec::new());
    };

    let mut subdirs = Vec::new();
    for entry in read_dir.filter_map(|e| e.ok()) {
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if !ft.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.')
            || name.starts_with('$')
            || name.eq_ignore_ascii_case("System Volume Information")
            || name.eq_ignore_ascii_case("Recovery")
        {
            continue;
        }
        let full_path = entry.path();
        let norm_path = full_path.to_string_lossy().replace('\\', "/");
        let has_children = dir_has_subdirectories(&full_path);
        subdirs.push(SubdirEntry {
            path: norm_path,
            name,
            has_children,
        });
    }

    subdirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(subdirs)
}

#[tauri::command]
fn scan_monitored_folders(folders: Vec<MonitoredFolderSpec>) -> Result<Vec<ScannedTrack>, String> {
    let audio_paths = collect_audio_files_from_specs(&folders);
    let mut tracks: Vec<ScannedTrack> = audio_paths
        .par_iter()
        .filter_map(|p| parse_audio_file_fast(p))
        .collect();
    sort_tracks_for_album_folder(&mut tracks);
    Ok(tracks)
}

#[tauri::command]
fn smart_sync_library(
    folders: Vec<MonitoredFolderSpec>,
    known_files: Vec<KnownFileStat>,
) -> Result<SmartSyncResult, String> {
    let audio_paths = collect_audio_files_from_specs(&folders);

    let mut known_map: HashMap<String, (u64, u64)> = HashMap::with_capacity(known_files.len());
    for kf in known_files {
        let norm = kf.file_path.replace('\\', "/");
        known_map.insert(norm, (kf.modified_ms, kf.file_size));
    }

    let mut current_disk_set: HashSet<String> = HashSet::with_capacity(audio_paths.len());
    let mut to_add_paths: Vec<PathBuf> = Vec::new();
    let mut to_update_paths: Vec<PathBuf> = Vec::new();

    for p in audio_paths {
        let norm = p.to_string_lossy().replace('\\', "/");
        current_disk_set.insert(norm.clone());

        if let Some(&(old_mtime, old_size)) = known_map.get(&norm) {
            let (cur_mtime, cur_size) = get_file_stat_pair(&p);
            if (old_mtime > 0 && cur_mtime != old_mtime) || (old_size > 0 && cur_size != old_size) {
                to_update_paths.push(p);
            }
        } else {
            to_add_paths.push(p);
        }
    }

    let added_tracks: Vec<ScannedTrack> = to_add_paths
        .par_iter()
        .filter_map(|p| parse_audio_file_fast(p))
        .collect();

    let updated_tracks: Vec<ScannedTrack> = to_update_paths
        .par_iter()
        .filter_map(|p| parse_audio_file_fast(p))
        .collect();

    let removed_paths: Vec<String> = known_map
        .keys()
        .filter(|k| !current_disk_set.contains(*k))
        .cloned()
        .collect();

    Ok(SmartSyncResult {
        added_tracks,
        updated_tracks,
        removed_paths,
    })
}

#[tauri::command]
fn get_track_detail(file_path: String) -> Result<TrackDetail, String> {
    let norm = if cfg!(target_os = "windows") {
        file_path.replace('/', "\\")
    } else {
        file_path.clone()
    };
    let path = if Path::new(&norm).is_file() {
        Path::new(&norm)
    } else {
        Path::new(&file_path)
    };
    let mut cover_data_url: Option<String> = None;
    let mut embedded_lyrics_raw: Option<String> = None;

    if path.is_file() && is_recognized_external_audio(path) {
        let cache_file = compute_cover_cache_file_path(path);
        let mut cache_hit = false;
        if cache_file.is_file() {
            if let Ok(cached_str) = fs::read_to_string(&cache_file) {
                if cached_str.starts_with("data:") {
                    cover_data_url = Some(cached_str);
                    cache_hit = true;
                } else if cached_str == "NONE" {
                    cover_data_url = None;
                    cache_hit = true;
                }
            }
        }

        let (mut lrc_lyrics, mut lrc_synced) = parse_companion_lrc(path);

        if !cache_hit {
            if let Ok(tagged_file) = Probe::open(path).and_then(|p| p.read()) {
                for tag in tagged_file.tags() {
                    if cover_data_url.is_none() {
                        if let Some(pic) = tag.pictures().first() {
                            let raw_bytes = pic.data();
                            let mime = pic
                                .mime_type()
                                .map(|m| m.as_str())
                                .unwrap_or("image/jpeg");
                            let b64 = BASE64.encode(raw_bytes);
                            let data_url = format!("data:{};base64,{}", mime, b64);
                            if raw_bytes.len() <= MAX_SINGLE_COVER_RAW_BYTES {
                                let _ = fs::write(&cache_file, &data_url);
                                let writes = COVER_CACHE_WRITE_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
                                if writes % 64 == 0 {
                                    prune_cover_cache_if_needed();
                                }
                            }
                            cover_data_url = Some(data_url);
                        }
                    }

                    if lrc_lyrics.is_empty() && embedded_lyrics_raw.is_none() {
                        if let Some(s) = tag.get_string(&ItemKey::Lyrics) {
                            if !s.trim().is_empty() {
                                embedded_lyrics_raw = Some(s.to_string());
                            }
                        }
                    }

                    if lrc_lyrics.is_empty() && embedded_lyrics_raw.is_none() {
                        for item in tag.items() {
                            let key_dbg = format!("{:?}", item.key()).to_ascii_lowercase();
                            if key_dbg.contains("lyric") || key_dbg.contains("uslt") || key_dbg.contains("sylt") {
                                if let Some(val_str) = item.value().text() {
                                    if !val_str.trim().is_empty() {
                                        embedded_lyrics_raw = Some(val_str.to_string());
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                if cover_data_url.is_none() {
                    let _ = fs::write(&cache_file, "NONE");
                }
            } else if path
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| s.eq_ignore_ascii_case("wma"))
                .unwrap_or(false)
            {
                if let Some(wma_meta) = parse_asf_wma_metadata_and_cover(path, true) {
                    if let Some(data_url) = wma_meta.cover_data_url {
                        let _ = fs::write(&cache_file, &data_url);
                        cover_data_url = Some(data_url);
                    } else {
                        let _ = fs::write(&cache_file, "NONE");
                    }
                    if lrc_lyrics.is_empty() && embedded_lyrics_raw.is_none() {
                        embedded_lyrics_raw = wma_meta.lyrics;
                    }
                }
            }
        } else if lrc_lyrics.is_empty() {
            let parse_opts = ParseOptions::new().read_cover_art(false);
            if let Ok(probe) = Probe::open(path) {
                if let Ok(tagged_file) = probe.options(parse_opts).read() {
                    for tag in tagged_file.tags() {
                        if let Some(s) = tag.get_string(&ItemKey::Lyrics) {
                            if !s.trim().is_empty() {
                                embedded_lyrics_raw = Some(s.to_string());
                                break;
                            }
                        }
                    }
                }
            }
        }

        if lrc_lyrics.is_empty() {
            if let Some(raw_lyr) = embedded_lyrics_raw {
                let (parsed, synced) = parse_lyrics_text(&raw_lyr);
                lrc_lyrics = parsed;
                lrc_synced = synced;
            }
        }

        return Ok(TrackDetail {
            file_path,
            cover_data_url,
            lyrics: lrc_lyrics,
            has_synced_timestamps: lrc_synced,
        });
    }

    Err("FILE_NOT_FOUND".to_string())
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayableAudioResolution {
    pub playable: bool,
    pub resolved_path: String,
    pub format: String,
    pub method: String,
    pub reason: String,
    pub duration_sec: Option<f64>,
    pub sample_rate: Option<String>,
    pub bitrate_kbps: Option<u32>,
}

fn get_audio_transcode_cache_dir() -> PathBuf {
    let base_dir = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .filter(|p| p.is_dir())
        .map(|p| p.join("droem").join("audio_cache"))
        .unwrap_or_else(|| std::env::temp_dir().join("droem_audio_cache"));
    if fs::create_dir_all(&base_dir).is_err() {
        let fallback = std::env::temp_dir().join("droem_audio_cache");
        let _ = fs::create_dir_all(&fallback);
        return fallback;
    }
    base_dir
}

fn compute_audio_cache_path(path: &Path, ext: &str) -> PathBuf {
    let (mtime, size) = get_file_stat_pair(path);
    let norm = path.to_string_lossy().replace('\\', "/").to_lowercase();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    norm.hash(&mut hasher);
    mtime.hash(&mut hasher);
    size.hash(&mut hasher);
    let digest = hasher.finish();
    get_audio_transcode_cache_dir().join(format!("{:016x}.{}", digest, ext))
}

fn decode_ieee_extended_80(bytes: &[u8]) -> u32 {
    if bytes.len() < 10 {
        return 44100;
    }
    let exp = (((bytes[0] & 0x7F) as i32) << 8) | (bytes[1] as i32);
    let mant = u64::from_be_bytes([
        bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9],
    ]);
    if exp == 0 && mant == 0 {
        return 44100;
    }
    let val = (mant as f64) * 2.0f64.powi(exp - 16383 - 63);
    if val.is_finite() && val >= 4000.0 && val <= 384000.0 {
        val.round() as u32
    } else {
        44100
    }
}

fn probe_aiff_header_summary(path: &Path) -> Option<(u64, u32, String)> {
    let data = fs::read(path).ok()?;
    if data.len() < 28 || &data[0..4] != b"FORM" {
        return None;
    }
    let form_type = &data[8..12];
    if form_type != b"AIFF" && form_type != b"AIFC" {
        return None;
    }
    let mut idx = 12usize;
    while idx + 8 <= data.len() {
        let cid = &data[idx..idx + 4];
        let csz = u32::from_be_bytes([data[idx + 4], data[idx + 5], data[idx + 6], data[idx + 7]]) as usize;
        let body_start = idx + 8;
        if cid == b"COMM" && body_start + 18 <= data.len() {
            let channels = u16::from_be_bytes([data[body_start], data[body_start + 1]]).max(1) as u32;
            let frames = u32::from_be_bytes([
                data[body_start + 2],
                data[body_start + 3],
                data[body_start + 4],
                data[body_start + 5],
            ]) as u64;
            let bits = u16::from_be_bytes([data[body_start + 6], data[body_start + 7]]).max(8) as u32;
            let sr = decode_ieee_extended_80(&data[body_start + 8..body_start + 18]).max(1);
            let dur_ms = (((frames as f64) / (sr as f64)) * 1000.0).round().max(1.0) as u64;
            let kbps = (sr * channels * bits) / 1000;
            let sr_str = format!("{:.1}kHz", (sr as f64) / 1000.0).replace(".0kHz", "kHz");
            return Some((dur_ms, kbps, sr_str));
        }
        idx = body_start.saturating_add(csz).saturating_add(csz & 1);
    }
    None
}

static TRANSCODE_TMP_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
static TRANSCODE_PATH_LOCKS: OnceLock<Mutex<HashMap<String, std::sync::Arc<Mutex<()>>>>> = OnceLock::new();

/// Obtains a per-cache-path Mutex so concurrent playback & gapless-preload requests for the same
/// source file never race or overwrite each other's temporary/target files.
fn acquire_transcode_path_lock(target_path: &Path) -> std::sync::Arc<Mutex<()>> {
    let key = target_path.to_string_lossy().replace('\\', "/").to_lowercase();
    let map_mutex = TRANSCODE_PATH_LOCKS.get_or_init(|| Mutex::new(HashMap::new()));
    if let Ok(mut map) = map_mutex.lock() {
        return map
            .entry(key)
            .or_insert_with(|| std::sync::Arc::new(Mutex::new(())))
            .clone();
    }
    std::sync::Arc::new(Mutex::new(()))
}

fn make_unique_temp_cache_path(out_path: &Path, tag: &str, ext: &str) -> PathBuf {
    let seq = TRANSCODE_TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    out_path.with_extension(format!("{}.{}.{}.{}", tag, pid, seq, ext))
}

/// Non-destructive, read-only validation that a cached `.wav` file exists, has a valid RIFF/WAVE header,
/// a valid `fmt ` chunk, and a non-empty `data` payload whose declared size is present on disk.
/// Never deletes `path` during read-only inspection so concurrent readers are never disrupted.
fn is_valid_cached_wav(path: &Path) -> bool {
    use std::io::Read;
    let Ok(meta) = fs::metadata(path) else {
        return false;
    };
    let file_len = meta.len();
    if !meta.is_file() || file_len <= 44 {
        return false;
    }
    let Ok(mut f) = fs::File::open(path) else {
        return false;
    };
    let mut hdr = [0u8; 256];
    let Ok(n) = f.read(&mut hdr) else {
        return false;
    };
    let slice = &hdr[..n];
    if slice.len() < 44 || &slice[0..4] != b"RIFF" || &slice[8..12] != b"WAVE" {
        return false;
    }
    let mut idx = 12usize;
    let mut found_fmt = false;
    let mut data_chunk_len = 0u64;
    let mut data_body_offset = 0u64;
    while idx + 8 <= slice.len() {
        let cid = &slice[idx..idx + 4];
        let csz = u32::from_le_bytes([slice[idx + 4], slice[idx + 5], slice[idx + 6], slice[idx + 7]]) as u64;
        let body = idx + 8;
        if cid == b"fmt " && csz >= 16 && body + 16 <= slice.len() {
            let channels = u16::from_le_bytes([slice[body + 2], slice[body + 3]]);
            let sr = u32::from_le_bytes([slice[body + 4], slice[body + 5], slice[body + 6], slice[body + 7]]);
            if channels > 0 && sr >= 4000 {
                found_fmt = true;
            }
        } else if cid == b"data" {
            data_chunk_len = csz;
            data_body_offset = body as u64;
            break;
        }
        idx = body.saturating_add(csz as usize).saturating_add((csz as usize) & 1);
    }
    found_fmt
        && data_chunk_len > 0
        && file_len >= data_body_offset.saturating_add(data_chunk_len)
}

/// Commits a validated temporary `.wav` file (`tmp_wav`) to `out_wav` atomically in the same directory:
/// - Never pre-deletes an existing valid `out_wav` before replacement.
/// - Never falls back to non-atomic `fs::copy` (which could expose partial bytes to readers).
/// - If `out_wav` is already valid and locked by an active reader, cleans up `tmp_wav` and returns `out_wav`.
fn commit_temp_wav_atomically(tmp_wav: &Path, out_wav: &Path) -> Result<PathBuf, String> {
    if !is_valid_cached_wav(tmp_wav) {
        let _ = fs::remove_file(tmp_wav);
        return Err("INVALID_GENERATED_WAV".to_string());
    }
    match fs::rename(tmp_wav, out_wav) {
        Ok(()) => Ok(out_wav.to_path_buf()),
        Err(e) => {
            let _ = fs::remove_file(tmp_wav);
            if is_valid_cached_wav(out_wav) {
                return Ok(out_wav.to_path_buf());
            }
            Err(format!("ATOMIC_RENAME_WAV_ERROR: {}", e))
        }
    }
}

/// Atomically writes `wav_bytes` to a unique temporary file in the same cache directory, flushes/syncs
/// the file handle, verifies its RIFF WAVE structure, and atomically renames it to `out_wav`.
fn write_wav_cache_atomically(out_wav: &Path, wav_bytes: &[u8]) -> Result<PathBuf, String> {
    use std::io::Write;
    if wav_bytes.len() <= 44 {
        return Err("WAV_PAYLOAD_EMPTY".to_string());
    }
    if is_valid_cached_wav(out_wav) {
        return Ok(out_wav.to_path_buf());
    }
    let tmp_wav = make_unique_temp_cache_path(out_wav, "tmp", "wav");
    let write_res = (|| -> std::io::Result<()> {
        let mut f = fs::File::create(&tmp_wav)?;
        f.write_all(wav_bytes)?;
        f.sync_all()?;
        Ok(())
    })();
    if let Err(e) = write_res {
        let _ = fs::remove_file(&tmp_wav);
        return Err(format!("WRITE_TMP_WAV_ERROR: {}", e));
    }
    commit_temp_wav_atomically(&tmp_wav, out_wav)
}

/// Pure-Rust Lossless AIFF / AIFC to RIFF WAV Transcoder
fn transcode_aiff_to_wav_cached(path: &Path) -> Result<PathBuf, String> {
    let out_wav = compute_audio_cache_path(path, "wav");
    if is_valid_cached_wav(&out_wav) {
        return Ok(out_wav);
    }
    let path_lock = acquire_transcode_path_lock(&out_wav);
    let _guard = path_lock.lock().unwrap_or_else(|e| e.into_inner());
    if is_valid_cached_wav(&out_wav) {
        return Ok(out_wav);
    }

    let data = fs::read(path).map_err(|e| format!("READ_ERROR: {}", e))?;
    if data.len() < 36 || &data[0..4] != b"FORM" {
        return Err("NOT_AIFF_FORM".to_string());
    }
    let form_type = &data[8..12];
    if form_type != b"AIFF" && form_type != b"AIFC" {
        return Err("UNSUPPORTED_FORM_TYPE".to_string());
    }

    let mut channels: u16 = 0;
    let mut sample_frames: u32 = 0;
    let mut sample_bits: u16 = 0;
    let mut sample_rate: u32 = 44100;
    let mut comp_type = *b"NONE";
    let mut ssnd_pcm_slice: Option<&[u8]> = None;

    let mut idx = 12usize;
    while idx + 8 <= data.len() {
        let cid = &data[idx..idx + 4];
        let csz = u32::from_be_bytes([data[idx + 4], data[idx + 5], data[idx + 6], data[idx + 7]]) as usize;
        let body_start = idx + 8;
        let body_end = (body_start + csz).min(data.len());

        if cid == b"COMM" && body_end >= body_start + 18 {
            channels = u16::from_be_bytes([data[body_start], data[body_start + 1]]);
            sample_frames = u32::from_be_bytes([
                data[body_start + 2],
                data[body_start + 3],
                data[body_start + 4],
                data[body_start + 5],
            ]);
            sample_bits = u16::from_be_bytes([data[body_start + 6], data[body_start + 7]]);
            sample_rate = decode_ieee_extended_80(&data[body_start + 8..body_start + 18]);
            if form_type == b"AIFC" && body_end >= body_start + 22 {
                comp_type.copy_from_slice(&data[body_start + 18..body_start + 22]);
            }
        } else if cid == b"SSND" && body_end >= body_start + 8 {
            let offset = u32::from_be_bytes([
                data[body_start],
                data[body_start + 1],
                data[body_start + 2],
                data[body_start + 3],
            ]) as usize;
            let pcm_start = (body_start + 8 + offset).min(body_end);
            ssnd_pcm_slice = Some(&data[pcm_start..body_end]);
        }

        idx = body_start.saturating_add(csz).saturating_add(csz & 1);
    }

    let pcm_raw = ssnd_pcm_slice.ok_or_else(|| "MISSING_SSND_CHUNK".to_string())?;
    if channels == 0 || sample_frames == 0 {
        return Err("INVALID_COMM_PARAMS".to_string());
    }

    let is_float = &comp_type == b"fl32" || &comp_type == b"FL32";
    let is_le_pcm = &comp_type == b"sowt" || &comp_type == b"SOWT";
    let is_be_pcm = &comp_type == b"NONE" || &comp_type == b"twos" || &comp_type == b"TWOS" || &comp_type == b"raw ";

    if !is_float && !is_le_pcm && !is_be_pcm {
        return Err(format!(
            "UNSUPPORTED_AIFC_COMPRESSION_{}",
            String::from_utf8_lossy(&comp_type)
        ));
    }

    let bytes_per_sample = ((sample_bits + 7) / 8) as usize;
    if !matches!(bytes_per_sample, 1 | 2 | 3 | 4) {
        return Err(format!("UNSUPPORTED_SAMPLE_BITS_{}", sample_bits));
    }

    let total_samples = (sample_frames as usize).saturating_mul(channels as usize);
    let expected_bytes = total_samples.saturating_mul(bytes_per_sample);
    if pcm_raw.len() < expected_bytes || expected_bytes == 0 {
        return Err("TRUNCATED_AIFF_SSND_DATA".to_string());
    }
    let src_bytes = &pcm_raw[..expected_bytes];

    let mut wav_pcm = Vec::with_capacity(expected_bytes);
    if bytes_per_sample == 1 {
        if &comp_type == b"raw " {
            wav_pcm.extend_from_slice(src_bytes);
        } else {
            for &b in src_bytes {
                wav_pcm.push((b as i8 as i16 + 128) as u8);
            }
        }
    } else if is_le_pcm {
        wav_pcm.extend_from_slice(src_bytes);
    } else if bytes_per_sample == 2 {
        for chunk in src_bytes.chunks_exact(2) {
            wav_pcm.push(chunk[1]);
            wav_pcm.push(chunk[0]);
        }
    } else if bytes_per_sample == 3 {
        for chunk in src_bytes.chunks_exact(3) {
            wav_pcm.push(chunk[2]);
            wav_pcm.push(chunk[1]);
            wav_pcm.push(chunk[0]);
        }
    } else if bytes_per_sample == 4 {
        for chunk in src_bytes.chunks_exact(4) {
            wav_pcm.push(chunk[3]);
            wav_pcm.push(chunk[2]);
            wav_pcm.push(chunk[1]);
            wav_pcm.push(chunk[0]);
        }
    }

    let wav_bits_per_sample = (bytes_per_sample * 8) as u16;
    let block_align = channels * (bytes_per_sample as u16);
    let byte_rate = sample_rate * (block_align as u32);
    let data_len = wav_pcm.len() as u32;
    let riff_len = 36u32.saturating_add(data_len);
    let format_tag: u16 = if is_float { 3 } else { 1 };

    let mut out_buf = Vec::with_capacity(44 + wav_pcm.len());
    out_buf.extend_from_slice(b"RIFF");
    out_buf.extend_from_slice(&riff_len.to_le_bytes());
    out_buf.extend_from_slice(b"WAVE");
    out_buf.extend_from_slice(b"fmt ");
    out_buf.extend_from_slice(&16u32.to_le_bytes());
    out_buf.extend_from_slice(&format_tag.to_le_bytes());
    out_buf.extend_from_slice(&channels.to_le_bytes());
    out_buf.extend_from_slice(&sample_rate.to_le_bytes());
    out_buf.extend_from_slice(&byte_rate.to_le_bytes());
    out_buf.extend_from_slice(&block_align.to_le_bytes());
    out_buf.extend_from_slice(&wav_bits_per_sample.to_le_bytes());
    out_buf.extend_from_slice(b"data");
    out_buf.extend_from_slice(&data_len.to_le_bytes());
    out_buf.extend_from_slice(&wav_pcm);

    write_wav_cache_atomically(&out_wav, &out_buf)
}

/// Type-Safe Windows Media Foundation (`mfplat.dll` + `mfreadwrite.dll` + `ole32.dll`) COM Bindings
/// - Uses explicit `#[repr(C)]` vtable structs matching `unknwn.h`, `mfobjects.h`, and `mfreadwrite.h`
///   instead of raw magic-number pointer offsets.
/// - Fixes `IMFSample::ConvertToContiguousBuffer` to its exact slot (vtable index 41; index 42 is `AddBuffer`).
/// - Enforces RAII `CoInitializeEx` / `CoUninitialize` and `MFStartup` / `MFShutdown` teardown via `MfComSessionGuard`.
/// - Enforces strict `MF_SOURCE_READERF_ENDOFSTREAM` completion and atomic `.tmp.wav` -> `.wav` caching.
#[cfg(target_os = "windows")]
#[allow(non_snake_case, dead_code)]
mod mf_com_safe {
    use std::ffi::c_void;
    use std::ptr;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Guid {
        pub data1: u32,
        pub data2: u16,
        pub data3: u16,
        pub data4: [u8; 8],
    }

    pub const COINIT_MULTITHREADED: u32 = 0x0;
    pub const RPC_E_CHANGED_MODE: i32 = 0x8001_0106u32 as i32;
    pub const MF_VERSION: u32 = 0x0002_0070;
    pub const MF_SOURCE_READER_FIRST_AUDIO_STREAM: u32 = 0xFFFF_FFFD;
    pub const MF_SOURCE_READERF_ERROR: u32 = 0x0000_0001;
    pub const MF_SOURCE_READERF_ENDOFSTREAM: u32 = 0x0000_0002;
    pub const MF_SOURCE_READERF_NATIVEMEDIATYPECHANGED: u32 = 0x0000_0010;
    pub const MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED: u32 = 0x0000_0020;

    pub const MF_MT_MAJOR_TYPE: Guid = Guid {
        data1: 0x48EBA18E,
        data2: 0xF8C9,
        data3: 0x4687,
        data4: [0xBF, 0x11, 0x0A, 0x74, 0xC9, 0xF9, 0x6A, 0x8F],
    };
    pub const MF_MT_SUBTYPE: Guid = Guid {
        data1: 0xF7E34C9A,
        data2: 0x42E8,
        data3: 0x4714,
        data4: [0xB7, 0x4B, 0xCB, 0x29, 0xD7, 0x2C, 0x35, 0xE5],
    };
    pub const MF_MT_AUDIO_NUM_CHANNELS: Guid = Guid {
        data1: 0x37E48BF5,
        data2: 0x645E,
        data3: 0x4C5B,
        data4: [0x89, 0xDE, 0xAD, 0xA9, 0xE2, 0x9B, 0x69, 0x6A],
    };
    pub const MF_MT_AUDIO_SAMPLES_PER_SECOND: Guid = Guid {
        data1: 0x5FAEEAE7,
        data2: 0x0290,
        data3: 0x4C31,
        data4: [0x9E, 0x8A, 0xC5, 0x34, 0xF6, 0x8D, 0x9D, 0xBA],
    };
    pub const MF_MT_AUDIO_BITS_PER_SAMPLE: Guid = Guid {
        data1: 0xF2DEB57F,
        data2: 0x40FA,
        data3: 0x4764,
        data4: [0xAA, 0x33, 0xED, 0x4F, 0x2D, 0x1F, 0xF6, 0x69],
    };
    pub const MF_MT_AUDIO_BLOCK_ALIGNMENT: Guid = Guid {
        data1: 0x322DE230,
        data2: 0x9EEB,
        data3: 0x43BD,
        data4: [0xAB, 0x7A, 0xFF, 0x41, 0x22, 0x51, 0x54, 0x1D],
    };
    pub const MF_MT_AUDIO_AVG_BYTES_PER_SECOND: Guid = Guid {
        data1: 0x1AAB75C8,
        data2: 0xCFED,
        data3: 0x451C,
        data4: [0xAB, 0x95, 0xAC, 0x03, 0x4B, 0x8E, 0x17, 0x31],
    };
    pub const MF_MT_ALL_SAMPLES_INDEPENDENT: Guid = Guid {
        data1: 0xC9173739,
        data2: 0x5E56,
        data3: 0x461C,
        data4: [0xB7, 0x13, 0x46, 0xFB, 0x99, 0x5C, 0xB9, 0x5F],
    };
    pub const MF_MEDIATYPE_AUDIO: Guid = Guid {
        data1: 0x73647561,
        data2: 0x0000,
        data3: 0x0010,
        data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
    };
    pub const MF_AUDIO_FORMAT_PCM: Guid = Guid {
        data1: 0x00000001,
        data2: 0x0000,
        data3: 0x0010,
        data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
    };

    #[repr(C)]
    pub struct IUnknownVtbl {
        pub QueryInterface: unsafe extern "system" fn(*mut c_void, *const Guid, *mut *mut c_void) -> i32,
        pub AddRef: unsafe extern "system" fn(*mut c_void) -> u32,
        pub Release: unsafe extern "system" fn(*mut c_void) -> u32,
    }

    #[repr(C)]
    pub struct IMFAttributesVtbl {
        pub base: IUnknownVtbl, // 0..=2
        pub GetItem: usize, // 3
        pub GetItemType: usize, // 4
        pub CompareItem: usize, // 5
        pub Compare: usize, // 6
        pub GetUINT32: unsafe extern "system" fn(*mut c_void, *const Guid, *mut u32) -> i32, // 7
        pub GetUINT64: unsafe extern "system" fn(*mut c_void, *const Guid, *mut u64) -> i32, // 8
        pub GetDouble: usize, // 9
        pub GetGUID: unsafe extern "system" fn(*mut c_void, *const Guid, *mut Guid) -> i32, // 10
        pub GetStringLength: usize, // 11
        pub GetString: usize, // 12
        pub GetAllocatedString: usize, // 13
        pub GetBlobSize: usize, // 14
        pub GetBlob: usize, // 15
        pub GetAllocatedBlob: usize, // 16
        pub GetUnknown: usize, // 17
        pub SetItem: usize, // 18
        pub DeleteItem: usize, // 19
        pub DeleteAllItems: usize, // 20
        pub SetUINT32: unsafe extern "system" fn(*mut c_void, *const Guid, u32) -> i32, // 21
        pub SetUINT64: unsafe extern "system" fn(*mut c_void, *const Guid, u64) -> i32, // 22
        pub SetDouble: usize, // 23
        pub SetGUID: unsafe extern "system" fn(*mut c_void, *const Guid, *const Guid) -> i32, // 24
        pub SetString: usize, // 25
        pub SetBlob: usize, // 26
        pub SetUnknown: usize, // 27
        pub LockStore: usize, // 28
        pub UnlockStore: usize, // 29
        pub GetCount: usize, // 30
        pub GetItemByIndex: usize, // 31
        pub CopyAllItems: usize, // 32
    }

    #[repr(C)]
    pub struct IMFMediaType {
        pub lpVtbl: *const IMFAttributesVtbl,
    }

    #[repr(C)]
    pub struct IMFMediaBufferVtbl {
        pub base: IUnknownVtbl, // 0..=2
        pub Lock: unsafe extern "system" fn(*mut IMFMediaBuffer, *mut *mut u8, *mut u32, *mut u32) -> i32, // 3
        pub Unlock: unsafe extern "system" fn(*mut IMFMediaBuffer) -> i32, // 4
        pub GetCurrentLength: unsafe extern "system" fn(*mut IMFMediaBuffer, *mut u32) -> i32, // 5
        pub SetCurrentLength: unsafe extern "system" fn(*mut IMFMediaBuffer, u32) -> i32, // 6
        pub GetMaxLength: unsafe extern "system" fn(*mut IMFMediaBuffer, *mut u32) -> i32, // 7
    }

    #[repr(C)]
    pub struct IMFMediaBuffer {
        pub lpVtbl: *const IMFMediaBufferVtbl,
    }

    #[repr(C)]
    pub struct IMFSampleVtbl {
        pub base: IMFAttributesVtbl, // 0..=32 (33 methods)
        pub GetSampleFlags: unsafe extern "system" fn(*mut IMFSample, *mut u32) -> i32, // 33
        pub SetSampleFlags: unsafe extern "system" fn(*mut IMFSample, u32) -> i32, // 34
        pub GetSampleTime: unsafe extern "system" fn(*mut IMFSample, *mut i64) -> i32, // 35
        pub SetSampleTime: unsafe extern "system" fn(*mut IMFSample, i64) -> i32, // 36
        pub GetSampleDuration: unsafe extern "system" fn(*mut IMFSample, *mut i64) -> i32, // 37
        pub SetSampleDuration: unsafe extern "system" fn(*mut IMFSample, i64) -> i32, // 38
        pub GetBufferCount: unsafe extern "system" fn(*mut IMFSample, *mut u32) -> i32, // 39
        pub GetBufferByIndex: unsafe extern "system" fn(*mut IMFSample, u32, *mut *mut IMFMediaBuffer) -> i32, // 40
        pub ConvertToContiguousBuffer: unsafe extern "system" fn(*mut IMFSample, *mut *mut IMFMediaBuffer) -> i32, // 41
        pub AddBuffer: unsafe extern "system" fn(*mut IMFSample, *mut IMFMediaBuffer) -> i32, // 42
        pub RemoveBufferByIndex: unsafe extern "system" fn(*mut IMFSample, u32) -> i32, // 43
        pub RemoveAllBuffers: unsafe extern "system" fn(*mut IMFSample) -> i32, // 44
        pub GetTotalLength: unsafe extern "system" fn(*mut IMFSample, *mut u32) -> i32, // 45
        pub CopyToBuffer: unsafe extern "system" fn(*mut IMFSample, *mut IMFMediaBuffer) -> i32, // 46
    }

    #[repr(C)]
    pub struct IMFSample {
        pub lpVtbl: *const IMFSampleVtbl,
    }

    #[repr(C)]
    pub struct IMFSourceReaderVtbl {
        pub base: IUnknownVtbl, // 0..=2
        pub GetStreamSelection: unsafe extern "system" fn(*mut IMFSourceReader, u32, *mut i32) -> i32, // 3
        pub SetStreamSelection: unsafe extern "system" fn(*mut IMFSourceReader, u32, i32) -> i32, // 4
        pub GetNativeMediaType: unsafe extern "system" fn(*mut IMFSourceReader, u32, u32, *mut *mut IMFMediaType) -> i32, // 5
        pub GetCurrentMediaType: unsafe extern "system" fn(*mut IMFSourceReader, u32, *mut *mut IMFMediaType) -> i32, // 6
        pub SetCurrentMediaType: unsafe extern "system" fn(*mut IMFSourceReader, u32, *mut u32, *mut IMFMediaType) -> i32, // 7
        pub SetCurrentPosition: unsafe extern "system" fn(*mut IMFSourceReader, *const Guid, *const c_void) -> i32, // 8
        pub ReadSample: unsafe extern "system" fn(
            *mut IMFSourceReader,
            u32,
            u32,
            *mut u32,
            *mut u32,
            *mut i64,
            *mut *mut IMFSample,
        ) -> i32, // 9
        pub Flush: unsafe extern "system" fn(*mut IMFSourceReader, u32) -> i32, // 10
        pub GetServiceForStream: usize, // 11
        pub GetPresentationAttribute: usize, // 12
    }

    #[repr(C)]
    pub struct IMFSourceReader {
        pub lpVtbl: *const IMFSourceReaderVtbl,
    }

    #[link(name = "ole32")]
    extern "system" {
        pub fn CoInitializeEx(pvReserved: *mut c_void, dwCoInit: u32) -> i32;
        pub fn CoUninitialize();
    }

    #[link(name = "mfplat")]
    extern "system" {
        pub fn MFStartup(version: u32, dwFlags: u32) -> i32;
        pub fn MFShutdown() -> i32;
        pub fn MFCreateMediaType(ppMFType: *mut *mut IMFMediaType) -> i32;
    }

    #[link(name = "mfreadwrite")]
    extern "system" {
        pub fn MFCreateSourceReaderFromURL(
            pwszURL: *const u16,
            pAttributes: *mut c_void,
            ppSourceReader: *mut *mut IMFSourceReader,
        ) -> i32;
    }

    /// RAII Guard for COM Apartment (`CoInitializeEx`/`CoUninitialize`) and Media Foundation (`MFStartup`/`MFShutdown`)
    pub struct MfComSessionGuard {
        should_uninit_com: bool,
        mf_started: bool,
    }

    impl MfComSessionGuard {
        pub fn new() -> Result<Self, String> {
            unsafe {
                let hr_co = CoInitializeEx(ptr::null_mut(), COINIT_MULTITHREADED);
                let should_uninit_com = if hr_co >= 0 {
                    true
                } else if hr_co == RPC_E_CHANGED_MODE {
                    false
                } else {
                    return Err(format!("CO_INITIALIZE_FAILED_0x{:08X}", hr_co as u32));
                };

                let hr_mf = MFStartup(MF_VERSION, 0);
                if hr_mf < 0 {
                    if should_uninit_com {
                        CoUninitialize();
                    }
                    return Err(format!("MF_STARTUP_FAILED_0x{:08X}", hr_mf as u32));
                }

                Ok(Self {
                    should_uninit_com,
                    mf_started: true,
                })
            }
        }
    }

    impl Drop for MfComSessionGuard {
        fn drop(&mut self) {
            unsafe {
                if self.mf_started {
                    let _ = MFShutdown();
                    self.mf_started = false;
                }
                if self.should_uninit_com {
                    CoUninitialize();
                    self.should_uninit_com = false;
                }
            }
        }
    }

    /// Generic RAII COM Interface Pointer Guard calling `IUnknown::Release` on drop
    pub struct ComPtrGuard<T> {
        pub ptr: *mut T,
    }

    impl<T> ComPtrGuard<T> {
        pub fn new(ptr: *mut T) -> Self {
            Self { ptr }
        }
        pub fn is_null(&self) -> bool {
            self.ptr.is_null()
        }
    }

    impl<T> Drop for ComPtrGuard<T> {
        fn drop(&mut self) {
            unsafe {
                if !self.ptr.is_null() {
                    let unk_vtbl = *(self.ptr as *const *const IUnknownVtbl);
                    ((*unk_vtbl).Release)(self.ptr as *mut c_void);
                    self.ptr = ptr::null_mut();
                }
            }
        }
    }

    pub unsafe fn mt_get_u32(mt: *mut IMFMediaType, key: &Guid, default_val: u32) -> u32 {
        if mt.is_null() {
            return default_val;
        }
        let mut val = default_val;
        let hr = ((*(*mt).lpVtbl).GetUINT32)(mt as *mut c_void, key as *const Guid, &mut val);
        if hr >= 0 && val > 0 {
            val
        } else {
            default_val
        }
    }

    pub unsafe fn mt_set_u32(mt: *mut IMFMediaType, key: &Guid, val: u32) -> i32 {
        ((*(*mt).lpVtbl).SetUINT32)(mt as *mut c_void, key as *const Guid, val)
    }

    pub unsafe fn mt_set_guid(mt: *mut IMFMediaType, key: &Guid, val: &Guid) -> i32 {
        ((*(*mt).lpVtbl).SetGUID)(mt as *mut c_void, key as *const Guid, val as *const Guid)
    }
}

#[cfg(target_os = "windows")]
fn transcode_via_windows_media_foundation(path: &Path) -> Result<PathBuf, String> {
    use mf_com_safe::*;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;

    // Compile-time verification that IMFSampleVtbl::ConvertToContiguousBuffer is at exact vtable slot 41
    const _: () = assert!(
        std::mem::size_of::<IMFAttributesVtbl>() == 33 * std::mem::size_of::<usize>(),
        "IMFAttributesVtbl must have exactly 33 methods (indices 0..=32)"
    );
    const _: () = assert!(
        std::mem::size_of::<IMFSampleVtbl>() == 47 * std::mem::size_of::<usize>(),
        "IMFSampleVtbl must have exactly 47 methods (indices 0..=46)"
    );

    let out_wav = compute_audio_cache_path(path, "wav");
    if is_valid_cached_wav(&out_wav) {
        return Ok(out_wav);
    }
    let path_lock = acquire_transcode_path_lock(&out_wav);
    let _lock_guard = path_lock.lock().unwrap_or_else(|e| e.into_inner());
    if is_valid_cached_wav(&out_wav) {
        return Ok(out_wav);
    }

    // 1. Initialize COM (`CoInitializeEx`) and Media Foundation (`MFStartup`) with RAII teardown guard
    let _session_guard = MfComSessionGuard::new()?;

    unsafe {
        let win_path = path.to_string_lossy().replace('/', "\\");
        let wide_url: Vec<u16> = std::ffi::OsStr::new(&win_path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let mut raw_reader: *mut IMFSourceReader = ptr::null_mut();
        let hr_reader = MFCreateSourceReaderFromURL(wide_url.as_ptr(), ptr::null_mut(), &mut raw_reader);
        if hr_reader < 0 || raw_reader.is_null() {
            return Err(format!("MF_SOURCE_READER_OPEN_FAILED_0x{:08X}", hr_reader as u32));
        }
        let reader = ComPtrGuard::new(raw_reader);

        // Select ONLY the first audio stream
        let _ = ((*(*reader.ptr).lpVtbl).SetStreamSelection)(
            reader.ptr,
            MF_SOURCE_READER_FIRST_AUDIO_STREAM,
            1,
        );

        // Query stream's initial channels & sample rate
        let mut init_channels = 2u32;
        let mut init_sr = 44100u32;
        let mut raw_native_mt: *mut IMFMediaType = ptr::null_mut();
        if ((*(*reader.ptr).lpVtbl).GetCurrentMediaType)(
            reader.ptr,
            MF_SOURCE_READER_FIRST_AUDIO_STREAM,
            &mut raw_native_mt,
        ) >= 0
            && !raw_native_mt.is_null()
        {
            let native_mt = ComPtrGuard::new(raw_native_mt);
            init_channels = mt_get_u32(native_mt.ptr, &MF_MT_AUDIO_NUM_CHANNELS, 2).clamp(1, 8);
            init_sr = mt_get_u32(native_mt.ptr, &MF_MT_AUDIO_SAMPLES_PER_SECOND, 44100).clamp(8000, 192000);
        }

        // 1st attempt: Request explicit 16-bit PCM matching the stream's sample rate & channels
        let mut raw_target_mt: *mut IMFMediaType = ptr::null_mut();
        let hr_mt = MFCreateMediaType(&mut raw_target_mt);
        if hr_mt < 0 || raw_target_mt.is_null() {
            return Err("MF_CREATE_MEDIA_TYPE_FAILED".to_string());
        }
        let target_mt = ComPtrGuard::new(raw_target_mt);
        mt_set_guid(target_mt.ptr, &MF_MT_MAJOR_TYPE, &MF_MEDIATYPE_AUDIO);
        mt_set_guid(target_mt.ptr, &MF_MT_SUBTYPE, &MF_AUDIO_FORMAT_PCM);
        mt_set_u32(target_mt.ptr, &MF_MT_AUDIO_NUM_CHANNELS, init_channels);
        mt_set_u32(target_mt.ptr, &MF_MT_AUDIO_SAMPLES_PER_SECOND, init_sr);
        mt_set_u32(target_mt.ptr, &MF_MT_AUDIO_BITS_PER_SAMPLE, 16);
        let init_block_align = init_channels * 2;
        mt_set_u32(target_mt.ptr, &MF_MT_AUDIO_BLOCK_ALIGNMENT, init_block_align);
        mt_set_u32(
            target_mt.ptr,
            &MF_MT_AUDIO_AVG_BYTES_PER_SECOND,
            init_sr * init_block_align,
        );
        mt_set_u32(target_mt.ptr, &MF_MT_ALL_SAMPLES_INDEPENDENT, 1);

        let mut hr_set = ((*(*reader.ptr).lpVtbl).SetCurrentMediaType)(
            reader.ptr,
            MF_SOURCE_READER_FIRST_AUDIO_STREAM,
            ptr::null_mut(),
            target_mt.ptr,
        );
        drop(target_mt);

        // Fallback: partial media type (MajorType=Audio, SubType=PCM) so MFT chooses its preferred PCM layout
        if hr_set < 0 {
            let mut raw_partial_mt: *mut IMFMediaType = ptr::null_mut();
            if MFCreateMediaType(&mut raw_partial_mt) >= 0 && !raw_partial_mt.is_null() {
                let partial_mt = ComPtrGuard::new(raw_partial_mt);
                mt_set_guid(partial_mt.ptr, &MF_MT_MAJOR_TYPE, &MF_MEDIATYPE_AUDIO);
                mt_set_guid(partial_mt.ptr, &MF_MT_SUBTYPE, &MF_AUDIO_FORMAT_PCM);
                mt_set_u32(partial_mt.ptr, &MF_MT_AUDIO_BITS_PER_SAMPLE, 16);
                hr_set = ((*(*reader.ptr).lpVtbl).SetCurrentMediaType)(
                    reader.ptr,
                    MF_SOURCE_READER_FIRST_AUDIO_STREAM,
                    ptr::null_mut(),
                    partial_mt.ptr,
                );
            }
        }
        if hr_set < 0 {
            let mut raw_min_mt: *mut IMFMediaType = ptr::null_mut();
            if MFCreateMediaType(&mut raw_min_mt) >= 0 && !raw_min_mt.is_null() {
                let min_mt = ComPtrGuard::new(raw_min_mt);
                mt_set_guid(min_mt.ptr, &MF_MT_MAJOR_TYPE, &MF_MEDIATYPE_AUDIO);
                mt_set_guid(min_mt.ptr, &MF_MT_SUBTYPE, &MF_AUDIO_FORMAT_PCM);
                hr_set = ((*(*reader.ptr).lpVtbl).SetCurrentMediaType)(
                    reader.ptr,
                    MF_SOURCE_READER_FIRST_AUDIO_STREAM,
                    ptr::null_mut(),
                    min_mt.ptr,
                );
            }
        }

        if hr_set < 0 {
            return Err(format!("MF_SET_PCM_TYPE_FAILED_0x{:08X}", hr_set as u32));
        }

        // Query the negotiated output media type
        let mut out_channels = init_channels as u16;
        let mut out_sr = init_sr;
        let mut out_bits = 16u16;
        let mut raw_cur_mt: *mut IMFMediaType = ptr::null_mut();
        if ((*(*reader.ptr).lpVtbl).GetCurrentMediaType)(
            reader.ptr,
            MF_SOURCE_READER_FIRST_AUDIO_STREAM,
            &mut raw_cur_mt,
        ) >= 0
            && !raw_cur_mt.is_null()
        {
            let cur_mt = ComPtrGuard::new(raw_cur_mt);
            out_channels = mt_get_u32(cur_mt.ptr, &MF_MT_AUDIO_NUM_CHANNELS, init_channels) as u16;
            out_sr = mt_get_u32(cur_mt.ptr, &MF_MT_AUDIO_SAMPLES_PER_SECOND, init_sr);
            out_bits = mt_get_u32(cur_mt.ptr, &MF_MT_AUDIO_BITS_PER_SAMPLE, 16) as u16;
        }
        if out_channels == 0 {
            out_channels = 2;
        }
        if out_sr == 0 {
            out_sr = 44100;
        }
        if !matches!(out_bits, 8 | 16 | 24 | 32) {
            out_bits = 16;
        }

        let mut pcm_bytes: Vec<u8> = Vec::with_capacity(4 * 1024 * 1024);
        let mut eos_reached = false;

        loop {
            let mut stream_idx = 0u32;
            let mut flags = 0u32;
            let mut timestamp = 0i64;
            let mut raw_sample: *mut IMFSample = ptr::null_mut();

            let hr_read = ((*(*reader.ptr).lpVtbl).ReadSample)(
                reader.ptr,
                MF_SOURCE_READER_FIRST_AUDIO_STREAM,
                0,
                &mut stream_idx,
                &mut flags,
                &mut timestamp,
                &mut raw_sample,
            );
            let sample = ComPtrGuard::new(raw_sample);

            // Never ignore mid-stream errors or save a partial WAV on error!
            if hr_read < 0 {
                return Err(format!("MF_READ_SAMPLE_FAILED_0x{:08X}", hr_read as u32));
            }
            if (flags & MF_SOURCE_READERF_ERROR) != 0 {
                return Err("MF_SOURCE_READER_STREAM_ERROR".to_string());
            }
            if (flags & MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED) != 0 {
                let mut raw_changed_mt: *mut IMFMediaType = ptr::null_mut();
                if ((*(*reader.ptr).lpVtbl).GetCurrentMediaType)(
                    reader.ptr,
                    MF_SOURCE_READER_FIRST_AUDIO_STREAM,
                    &mut raw_changed_mt,
                ) >= 0
                    && !raw_changed_mt.is_null()
                {
                    let changed_mt = ComPtrGuard::new(raw_changed_mt);
                    out_channels = mt_get_u32(changed_mt.ptr, &MF_MT_AUDIO_NUM_CHANNELS, out_channels as u32) as u16;
                    out_sr = mt_get_u32(changed_mt.ptr, &MF_MT_AUDIO_SAMPLES_PER_SECOND, out_sr);
                    out_bits = mt_get_u32(changed_mt.ptr, &MF_MT_AUDIO_BITS_PER_SAMPLE, out_bits as u32) as u16;
                }
            }

            if !sample.is_null() {
                // IMFSample::ConvertToContiguousBuffer (type-safe vtable slot 41)
                let mut raw_media_buf: *mut IMFMediaBuffer = ptr::null_mut();
                let mut hr_buf = ((*(*sample.ptr).lpVtbl).ConvertToContiguousBuffer)(
                    sample.ptr,
                    &mut raw_media_buf,
                );
                if (hr_buf < 0 || raw_media_buf.is_null()) && raw_media_buf.is_null() {
                    hr_buf = ((*(*sample.ptr).lpVtbl).GetBufferByIndex)(
                        sample.ptr,
                        0,
                        &mut raw_media_buf,
                    );
                }
                let media_buf = ComPtrGuard::new(raw_media_buf);
                if hr_buf < 0 || media_buf.is_null() {
                    return Err(format!("MF_SAMPLE_BUFFER_FAILED_0x{:08X}", hr_buf as u32));
                }

                let mut raw_ptr: *mut u8 = ptr::null_mut();
                let mut max_len = 0u32;
                let mut cur_len = 0u32;
                let hr_lock = ((*(*media_buf.ptr).lpVtbl).Lock)(
                    media_buf.ptr,
                    &mut raw_ptr,
                    &mut max_len,
                    &mut cur_len,
                );
                if hr_lock < 0 {
                    return Err(format!("MF_BUFFER_LOCK_FAILED_0x{:08X}", hr_lock as u32));
                }
                if !raw_ptr.is_null() && cur_len > 0 {
                    let slice = std::slice::from_raw_parts(raw_ptr, cur_len as usize);
                    pcm_bytes.extend_from_slice(slice);
                }
                let _ = ((*(*media_buf.ptr).lpVtbl).Unlock)(media_buf.ptr);
            }

            if (flags & MF_SOURCE_READERF_ENDOFSTREAM) != 0 {
                eos_reached = true;
                break;
            }
        }

        // Release reader BEFORE MFShutdown runs in `_session_guard` drop
        drop(reader);

        if !eos_reached || pcm_bytes.is_empty() {
            return Err("MF_DECODED_INCOMPLETE_OR_EMPTY_PCM".to_string());
        }

        let bytes_per_sample = (out_bits / 8).max(1);
        let block_align = out_channels * bytes_per_sample;
        let byte_rate = out_sr * (block_align as u32);
        let data_len = pcm_bytes.len() as u32;
        let riff_len = 36u32.saturating_add(data_len);

        let mut wav_out = Vec::with_capacity(44 + pcm_bytes.len());
        wav_out.extend_from_slice(b"RIFF");
        wav_out.extend_from_slice(&riff_len.to_le_bytes());
        wav_out.extend_from_slice(b"WAVE");
        wav_out.extend_from_slice(b"fmt ");
        wav_out.extend_from_slice(&16u32.to_le_bytes());
        wav_out.extend_from_slice(&1u16.to_le_bytes()); // PCM = 1
        wav_out.extend_from_slice(&out_channels.to_le_bytes());
        wav_out.extend_from_slice(&out_sr.to_le_bytes());
        wav_out.extend_from_slice(&byte_rate.to_le_bytes());
        wav_out.extend_from_slice(&block_align.to_le_bytes());
        wav_out.extend_from_slice(&out_bits.to_le_bytes());
        wav_out.extend_from_slice(b"data");
        wav_out.extend_from_slice(&data_len.to_le_bytes());
        wav_out.extend_from_slice(&pcm_bytes);

        write_wav_cache_atomically(&out_wav, &wav_out)
    }
}

#[cfg(not(target_os = "windows"))]
fn transcode_via_windows_media_foundation(_path: &Path) -> Result<PathBuf, String> {
    Err("MEDIA_FOUNDATION_WINDOWS_ONLY".to_string())
}

/// Resolves a bundled or system CLI audio decoder (`ffmpeg.exe`, `wvunpack.exe`, `mac.exe`, `mpcdec.exe`)
/// across:
/// 1) `<exe_dir>\tools\<bin>` and `<exe_dir>\resources\tools\<bin>` (Zero-user-install NSIS bundled path)
/// 2) `<exe_dir>\<bin>`
/// 3) `%LOCALAPPDATA%\droem\tools\<bin>`
/// 4) WinGet (`Links` & `Packages`), Scoop (`shims`), Chocolatey (`bin`), `C:\ffmpeg\bin`, and system `PATH`.
fn find_bundled_or_system_tool(tool_stem: &str) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let exe_name = if tool_stem.to_ascii_lowercase().ends_with(".exe") {
            tool_stem.to_string()
        } else {
            format!("{}.exe", tool_stem)
        };
        let mut candidates: Vec<PathBuf> = Vec::new();
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                candidates.push(exe_dir.join("tools").join(&exe_name));
                candidates.push(exe_dir.join("resources").join("tools").join(&exe_name));
                candidates.push(exe_dir.join("bin").join(&exe_name));
                candidates.push(exe_dir.join(&exe_name));
            }
        }
        if let Some(local_app) = std::env::var_os("LOCALAPPDATA").map(PathBuf::from) {
            candidates.push(local_app.join("droem").join("tools").join(&exe_name));
            candidates.push(local_app.join("Microsoft").join("WinGet").join("Links").join(&exe_name));
            let winget_pkgs = local_app.join("Microsoft").join("WinGet").join("Packages");
            if let Ok(entries) = fs::read_dir(&winget_pkgs) {
                let stem_lower = tool_stem.to_ascii_lowercase();
                for entry in entries.filter_map(|e| e.ok()) {
                    let fname = entry.file_name().to_string_lossy().to_lowercase();
                    if fname.contains(&stem_lower) || fname.contains("ffmpeg") || fname.contains("wavpack") {
                        for sub in WalkDir::new(entry.path()).max_depth(4).into_iter().filter_map(|e| e.ok()) {
                            if sub.file_name().to_string_lossy().eq_ignore_ascii_case(&exe_name) {
                                candidates.push(sub.path().to_path_buf());
                            }
                        }
                    }
                }
            }
        }
        if let Some(user_prof) = std::env::var_os("USERPROFILE").map(PathBuf::from) {
            candidates.push(user_prof.join("scoop").join("shims").join(&exe_name));
        }
        candidates.push(PathBuf::from(r"C:\ProgramData\chocolatey\bin").join(&exe_name));
        candidates.push(PathBuf::from(r"C:\ffmpeg\bin").join(&exe_name));

        for cand in candidates {
            if cand.is_file() {
                return cand;
            }
        }
    }
    PathBuf::from(tool_stem)
}

fn find_ffmpeg_executable() -> PathBuf {
    find_bundled_or_system_tool("ffmpeg")
}

/// Optional FFmpeg CLI fallback for formats requiring external codec (`.wv`, `.ape`, `.mpc`, or `.wma` fallback)
/// Serializes per target path, writes to a unique temporary `.fftmp.<pid>.<seq>.wav` file, and commits atomically.
fn try_ffmpeg_transcode_to_wav(path: &Path) -> Option<PathBuf> {
    let out_wav = compute_audio_cache_path(path, "wav");
    if is_valid_cached_wav(&out_wav) {
        return Some(out_wav);
    }
    let path_lock = acquire_transcode_path_lock(&out_wav);
    let _lock_guard = path_lock.lock().unwrap_or_else(|e| e.into_inner());
    if is_valid_cached_wav(&out_wav) {
        return Some(out_wav);
    }
    let tmp_wav = make_unique_temp_cache_path(&out_wav, "fftmp", "wav");

    let ffmpeg_bin = find_ffmpeg_executable();
    let mut cmd = Command::new(&ffmpeg_bin);
    cmd.args([
        "-y",
        "-v",
        "error",
        "-i",
        &path.to_string_lossy(),
        "-vn",
        "-acodec",
        "pcm_s16le",
        "-f",
        "wav",
        &tmp_wav.to_string_lossy(),
    ]);
    #[cfg(target_os = "windows")]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    if let Ok(status) = cmd.status() {
        if status.success() {
            if let Ok(committed) = commit_temp_wav_atomically(&tmp_wav, &out_wav) {
                return Some(committed);
            }
        }
    }
    let _ = fs::remove_file(&tmp_wav);
    None
}

/// Format-Specific Official Lightweight CLI Decoder (`wvunpack`, `mac`, `mpcdec`)
/// Automatically uses bundled `C:\Program Files\Droem\tools\wvunpack.exe`, `mac.exe`, `mpcdec.exe` when present,
/// serializing concurrent requests and committing validated `.wav` files atomically without pre-delete or `fs::copy`.
fn try_native_cli_decoder_to_wav(path: &Path, ext_upper: &str) -> Option<PathBuf> {
    let out_wav = compute_audio_cache_path(path, "wav");
    if is_valid_cached_wav(&out_wav) {
        return Some(out_wav);
    }
    let path_lock = acquire_transcode_path_lock(&out_wav);
    let _lock_guard = path_lock.lock().unwrap_or_else(|e| e.into_inner());
    if is_valid_cached_wav(&out_wav) {
        return Some(out_wav);
    }
    let tmp_wav = make_unique_temp_cache_path(&out_wav, "clitmp", "wav");

    let mut cmd = match ext_upper {
        "WV" => {
            let bin = find_bundled_or_system_tool("wvunpack");
            let mut c = Command::new(bin);
            c.args(["-y", "-q", &path.to_string_lossy(), "-o", &tmp_wav.to_string_lossy()]);
            c
        }
        "APE" => {
            let bin = find_bundled_or_system_tool("mac");
            let mut c = Command::new(bin);
            c.args([&path.to_string_lossy(), &tmp_wav.to_string_lossy(), "-d"]);
            c
        }
        "MPC" => {
            let bin = find_bundled_or_system_tool("mpcdec");
            let mut c = Command::new(bin);
            c.arg(&path).arg(&tmp_wav);
            c
        }
        _ => return None,
    };
    #[cfg(target_os = "windows")]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    if let Ok(status) = cmd.status() {
        if status.success() {
            if let Ok(committed) = commit_temp_wav_atomically(&tmp_wav, &out_wav) {
                return Some(committed);
            }
        }
    }
    let _ = fs::remove_file(&tmp_wav);
    None
}

fn build_resolution_with_wav_probe(
    resolved: &Path,
    format: String,
    method: &str,
) -> PlayableAudioResolution {
    let mut duration_sec = None;
    let mut sample_rate = None;
    let mut bitrate_kbps = None;
    if let Some((dur_ms, kbps, sr_str)) = probe_wav_header_summary(resolved) {
        duration_sec = Some((dur_ms as f64) / 1000.0);
        bitrate_kbps = Some(kbps);
        sample_rate = Some(sr_str);
    }
    PlayableAudioResolution {
        playable: true,
        resolved_path: resolved.to_string_lossy().replace('\\', "/"),
        format,
        method: method.to_string(),
        reason: "OK".to_string(),
        duration_sec,
        sample_rate,
        bitrate_kbps,
    }
}

/// Fixes `.aac` files that either:
/// 1) Are actually MP4/M4A containers (`ftyp` box at offset 4) renamed to `.aac`, or
/// 2) Have a leading `ID3`v2 tag (with embedded album cover) before the first `0xFFF` ADTS frame syncword.
fn prepare_aac_playable_path(path: &Path) -> Result<(PathBuf, String), String> {
    let data = fs::read(path).map_err(|e| format!("AAC_READ_ERROR: {}", e))?;
    if data.len() < 16 {
        return Err("AAC_FILE_TOO_SMALL".to_string());
    }

    if data.len() >= 12 && &data[4..8] == b"ftyp" {
        let out_m4a = compute_audio_cache_path(path, "m4a");
        let path_lock = acquire_transcode_path_lock(&out_m4a);
        let _guard = path_lock.lock().unwrap_or_else(|e| e.into_inner());
        if !out_m4a.is_file() {
            let tmp_m4a = make_unique_temp_cache_path(&out_m4a, "tmp", "m4a");
            fs::write(&tmp_m4a, &data).map_err(|e| {
                let _ = fs::remove_file(&tmp_m4a);
                e.to_string()
            })?;
            if let Err(e) = fs::rename(&tmp_m4a, &out_m4a) {
                let _ = fs::remove_file(&tmp_m4a);
                if !out_m4a.is_file() {
                    return Err(e.to_string());
                }
            }
        }
        return Ok((out_m4a, "aac-ftyp-m4a".to_string()));
    }

    let mut audio_offset = 0usize;
    if &data[0..3] == b"ID3" && data.len() >= 10 {
        let id3_sz = (((data[6] & 0x7F) as usize) << 21)
            | (((data[7] & 0x7F) as usize) << 14)
            | (((data[8] & 0x7F) as usize) << 7)
            | ((data[9] & 0x7F) as usize);
        audio_offset = (10 + id3_sz).min(data.len());
    }

    let mut adts_pos = None;
    let search_limit = (audio_offset + 8192).min(data.len().saturating_sub(4));
    for i in audio_offset..search_limit {
        if data[i] == 0xFF && (data[i + 1] & 0xF6) == 0xF0 {
            adts_pos = Some(i);
            break;
        }
    }

    if let Some(pos) = adts_pos {
        if pos > 0 {
            let out_clean_aac = compute_audio_cache_path(path, "aac");
            let path_lock = acquire_transcode_path_lock(&out_clean_aac);
            let _guard = path_lock.lock().unwrap_or_else(|e| e.into_inner());
            if !out_clean_aac.is_file() {
                let tmp_aac = make_unique_temp_cache_path(&out_clean_aac, "tmp", "aac");
                fs::write(&tmp_aac, &data[pos..]).map_err(|e| {
                    let _ = fs::remove_file(&tmp_aac);
                    e.to_string()
                })?;
                if let Err(e) = fs::rename(&tmp_aac, &out_clean_aac) {
                    let _ = fs::remove_file(&tmp_aac);
                    if !out_clean_aac.is_file() {
                        return Err(e.to_string());
                    }
                }
            }
            return Ok((out_clean_aac, "aac-id3-stripped".to_string()));
        }
        return Ok((path.to_path_buf(), "native-adts-aac".to_string()));
    }

    if let Ok(mf_wav) = transcode_via_windows_media_foundation(path) {
        return Ok((mf_wav, "windows-media-foundation-wav".to_string()));
    }
    if let Some(wav_path) = try_ffmpeg_transcode_to_wav(path) {
        return Ok((wav_path, "ffmpeg-wav-fallback".to_string()));
    }

    Ok((path.to_path_buf(), "native-aac-attempt".to_string()))
}

#[tauri::command]
fn prepare_playable_audio_source(file_path: String) -> Result<PlayableAudioResolution, String> {
    let trimmed = file_path.trim().trim_matches('"');
    if trimmed.is_empty() {
        return Ok(PlayableAudioResolution {
            playable: false,
            resolved_path: String::new(),
            format: "UNKNOWN".to_string(),
            method: "none".to_string(),
            reason: "EMPTY_PATH".to_string(),
            duration_sec: None,
            sample_rate: None,
            bitrate_kbps: None,
        });
    }
    let norm = if cfg!(target_os = "windows") {
        trimmed.replace('/', "\\")
    } else {
        trimmed.to_string()
    };
    let path = if Path::new(&norm).is_file() {
        PathBuf::from(&norm)
    } else {
        PathBuf::from(trimmed)
    };
    if !path.is_file() {
        return Ok(PlayableAudioResolution {
            playable: false,
            resolved_path: file_path,
            format: "MISSING".to_string(),
            method: "none".to_string(),
            reason: "FILE_NOT_FOUND".to_string(),
            duration_sec: None,
            sample_rate: None,
            bitrate_kbps: None,
        });
    }

    let ext_upper = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_uppercase();

    match ext_upper.as_str() {
        "MP3" | "FLAC" | "WAV" | "M4A" | "OGG" | "OPUS" => Ok(PlayableAudioResolution {
            playable: true,
            resolved_path: path.to_string_lossy().replace('\\', "/"),
            format: ext_upper,
            method: "native-webview2".to_string(),
            reason: "OK".to_string(),
            duration_sec: None,
            sample_rate: None,
            bitrate_kbps: None,
        }),
        "AIF" | "AIFF" => match transcode_aiff_to_wav_cached(&path) {
            Ok(wav_path) => Ok(build_resolution_with_wav_probe(
                &wav_path,
                ext_upper,
                "rust-aiff-to-wav",
            )),
            Err(err_reason) => {
                if let Ok(mf_wav) = transcode_via_windows_media_foundation(&path) {
                    Ok(build_resolution_with_wav_probe(
                        &mf_wav,
                        ext_upper,
                        "windows-media-foundation-wav",
                    ))
                } else if let Some(ff_wav) = try_ffmpeg_transcode_to_wav(&path) {
                    Ok(build_resolution_with_wav_probe(
                        &ff_wav,
                        ext_upper,
                        "ffmpeg-wav-fallback",
                    ))
                } else {
                    Ok(PlayableAudioResolution {
                        playable: false,
                        resolved_path: path.to_string_lossy().replace('\\', "/"),
                        format: ext_upper,
                        method: "unsupported".to_string(),
                        reason: err_reason,
                        duration_sec: None,
                        sample_rate: None,
                        bitrate_kbps: None,
                    })
                }
            }
        },
        "AAC" => match prepare_aac_playable_path(&path) {
            Ok((res_path, method_name)) => Ok(PlayableAudioResolution {
                playable: true,
                resolved_path: res_path.to_string_lossy().replace('\\', "/"),
                format: ext_upper,
                method: method_name,
                reason: "OK".to_string(),
                duration_sec: None,
                sample_rate: None,
                bitrate_kbps: None,
            }),
            Err(err_reason) => Ok(PlayableAudioResolution {
                playable: false,
                resolved_path: path.to_string_lossy().replace('\\', "/"),
                format: ext_upper,
                method: "unsupported".to_string(),
                reason: err_reason,
                duration_sec: None,
                sample_rate: None,
                bitrate_kbps: None,
            }),
        },
        "WMA" => {
            if let Ok(mf_wav) = transcode_via_windows_media_foundation(&path) {
                let mut res = build_resolution_with_wav_probe(
                    &mf_wav,
                    ext_upper,
                    "windows-media-foundation-wma-to-wav",
                );
                if let Some(wma_meta) = parse_asf_wma_metadata_and_cover(&path, false) {
                    if wma_meta.bitrate_kbps > 0 {
                        res.bitrate_kbps = Some(wma_meta.bitrate_kbps);
                    }
                }
                Ok(res)
            } else if let Some(ff_wav) = try_ffmpeg_transcode_to_wav(&path) {
                let mut res = build_resolution_with_wav_probe(
                    &ff_wav,
                    ext_upper,
                    "ffmpeg-wma-to-wav",
                );
                if let Some(wma_meta) = parse_asf_wma_metadata_and_cover(&path, false) {
                    if wma_meta.bitrate_kbps > 0 {
                        res.bitrate_kbps = Some(wma_meta.bitrate_kbps);
                    }
                }
                Ok(res)
            } else {
                Ok(PlayableAudioResolution {
                    playable: false,
                    resolved_path: path.to_string_lossy().replace('\\', "/"),
                    format: ext_upper,
                    method: "wma-decode-failed".to_string(),
                    reason: "WMA_DECODE_FAILED".to_string(),
                    duration_sec: None,
                    sample_rate: None,
                    bitrate_kbps: None,
                })
            }
        }
        "WV" | "APE" | "MPC" => {
            // Phase 38-F: WV, APE, and MPC are excluded from official v0.1.0 supported formats.
            // Decoder functions (`try_native_cli_decoder_to_wav`, `try_ffmpeg_transcode_to_wav`) are retained in source
            // for future porting, while v0.1.0 cleanly reports `UNSUPPORTED_FORMAT_V010` so the UI displays the
            // unsupported format notice and skips to the next track without crashing.
            let _ = try_native_cli_decoder_to_wav;
            Ok(PlayableAudioResolution {
                playable: false,
                resolved_path: path.to_string_lossy().replace('\\', "/"),
                format: ext_upper,
                method: "unsupported-v010".to_string(),
                reason: "UNSUPPORTED_FORMAT_V010".to_string(),
                duration_sec: None,
                sample_rate: None,
                bitrate_kbps: None,
            })
        }
        _ => Ok(PlayableAudioResolution {
            playable: false,
            resolved_path: path.to_string_lossy().replace('\\', "/"),
            format: ext_upper,
            method: "unsupported".to_string(),
            reason: "UNSUPPORTED_EXTENSION".to_string(),
            duration_sec: None,
            sample_rate: None,
            bitrate_kbps: None,
        }),
    }
}

#[tauri::command]
fn open_windows_default_apps_settings() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        // Check if "Drøm" or "Droem" is registered in RegisteredApplications for direct deep-link
        let mut target_uri = "ms-settings:defaultapps?registeredAppMachine=Dr%C3%B8m";
        if let Ok(out) = Command::new("reg")
            .args(["query", r"HKLM\SOFTWARE\RegisteredApplications"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
        {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if stdout.contains("Dr\u{00f8}m") {
                target_uri = "ms-settings:defaultapps?registeredAppMachine=Dr%C3%B8m";
            } else if stdout.contains("Droem") {
                target_uri = "ms-settings:defaultapps?registeredAppMachine=Droem";
            } else {
                target_uri = "ms-settings:defaultapps";
            }
        }
        if Command::new("explorer.exe")
            .arg(target_uri)
            .spawn()
            .is_err()
        {
            Command::new("explorer.exe")
                .arg("ms-settings:defaultapps")
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[cfg(target_os = "windows")]
#[repr(C)]
struct DwmMargins {
    cx_left_width: i32,
    cx_right_width: i32,
    cy_top_height: i32,
    cy_bottom_height: i32,
}

#[cfg(target_os = "windows")]
#[link(name = "dwmapi")]
extern "system" {
    fn DwmExtendFrameIntoClientArea(
        hwnd: *mut std::ffi::c_void,
        pmargins: *const DwmMargins,
    ) -> i32;
    fn DwmSetWindowAttribute(
        hwnd: *mut std::ffi::c_void,
        dwattribute: u32,
        pvattribute: *const std::ffi::c_void,
        cbattribute: u32,
    ) -> i32;
}

#[cfg(target_os = "windows")]
fn apply_win32_backdrop_to_window(window: &WebviewWindow, effect: &str, is_dark: bool) -> bool {
    let valid_effect = matches!(effect, "glass" | "acrylic" | "mica");
    if !valid_effect {
        return false;
    }
    let Ok(hwnd) = window.hwnd() else {
        return true;
    };
    let raw_hwnd = hwnd.0 as *mut std::ffi::c_void;
    if raw_hwnd.is_null() {
        return true;
    }

    unsafe {
        let dark_val: i32 = if is_dark { 1 } else { 0 };
        // DWMWA_USE_IMMERSIVE_DARK_MODE = 20
        let _ = DwmSetWindowAttribute(
            raw_hwnd,
            20,
            &dark_val as *const i32 as *const std::ffi::c_void,
            std::mem::size_of::<i32>() as u32,
        );

        // Phase 38-E.2 Fix for 1st Window Acrylic/Mica Rectangular Box & Top Border Artifact:
        // Because #stageFull is a multi-island floating layout with transparent desktop gaps between
        // .left-pill-bar, .main-window-island, .right-queue-island, and .bottom-player-bar,
        // enabling HWND-wide DWMWA_SYSTEMBACKDROP_TYPE (2=Mica, 3=Acrylic) or DwmExtendFrameIntoClientArea(-1)
        // causes Windows 11 DWM to paint a solid rectangular sheet and top accent border across the entire HWND bounding box.
        // We keep the native HWND strictly frameless and transparent (margins=0, DWMSBT_NONE=1, DWMWA_COLOR_NONE=0xFFFFFFFE)
        // so that each individual rounded island renders its own clipped Glass / Acrylic / Mica material cleanly!
        let zero_margins = DwmMargins {
            cx_left_width: 0,
            cx_right_width: 0,
            cy_top_height: 0,
            cy_bottom_height: 0,
        };
        let _ = DwmExtendFrameIntoClientArea(raw_hwnd, &zero_margins);

        // DWMWA_SYSTEMBACKDROP_TYPE = 38 -> 1 (DWMSBT_NONE)
        let disable_backdrop: i32 = 1;
        let _ = DwmSetWindowAttribute(
            raw_hwnd,
            38,
            &disable_backdrop as *const i32 as *const std::ffi::c_void,
            std::mem::size_of::<i32>() as u32,
        );

        // DWMWA_WINDOW_CORNER_PREFERENCE = 33 -> 1 (DWMWCP_DONOTROUND on outer transparent container)
        let corner_pref: i32 = 1;
        let _ = DwmSetWindowAttribute(
            raw_hwnd,
            33,
            &corner_pref as *const i32 as *const std::ffi::c_void,
            std::mem::size_of::<i32>() as u32,
        );

        // DWMWA_BORDER_COLOR = 34 -> 0xFFFFFFFE (DWMWA_COLOR_NONE: suppresses Windows 11 top/outer border line)
        let border_none: u32 = 0xFFFFFFFE;
        let _ = DwmSetWindowAttribute(
            raw_hwnd,
            34,
            &border_none as *const u32 as *const std::ffi::c_void,
            std::mem::size_of::<u32>() as u32,
        );

        let _ = window.set_shadow(false);
        true
    }
}

#[tauri::command]
fn set_window_backdrop_effect(
    window: WebviewWindow,
    pos_state: State<'_, Mutex<WindowPositionMemory>>,
    _effect: String,
    is_dark: Option<bool>,
) -> Result<bool, String> {
    if let Ok(mut mem) = pos_state.lock() {
        mem.backdrop_effect = "glass".to_string();
    }

    #[cfg(target_os = "windows")]
    {
        let applied_ok = apply_win32_backdrop_to_window(&window, "glass", is_dark.unwrap_or(true));
        return Ok(applied_ok);
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (window, is_dark);
        Ok(true)
    }
}

#[tauri::command]
fn check_file_exists(file_path: String) -> bool {
    let trimmed = file_path.trim().trim_matches('"');
    if trimmed.is_empty() {
        return false;
    }
    let norm = if cfg!(target_os = "windows") {
        trimmed.replace('/', "\\")
    } else {
        trimmed.to_string()
    };
    Path::new(&norm).is_file() || Path::new(trimmed).is_file()
}

fn prepare_smtc_thumbnail_file(audio_or_img_path: &str) -> Option<String> {
    let norm = audio_or_img_path.replace('/', "\\");
    let p = Path::new(&norm);
    if !p.is_file() {
        return None;
    }

    let ext = p
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "webp" | "bmp") {
        return Some(norm);
    }

    if let Ok(tagged_file) = Probe::open(p).and_then(|pr| pr.read()) {
        for tag in tagged_file.tags() {
            if let Some(pic) = tag.pictures().first() {
                let tmp_path = std::env::temp_dir().join("droem_native_smtc_cover.jpg");
                if fs::write(&tmp_path, pic.data()).is_ok() {
                    return Some(tmp_path.to_string_lossy().replace('/', "\\"));
                }
            }
        }
    }
    None
}

#[tauri::command]
fn update_native_smtc(
    window: WebviewWindow,
    title: String,
    artist: String,
    album: String,
    is_playing: bool,
    cover_path: Option<String>,
) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let thumb_file_path = cover_path
            .as_deref()
            .and_then(prepare_smtc_thumbnail_file);

        let win_clone = window.clone();
        let _ = window.run_on_main_thread(move || {
            unsafe {
                let win_title_lower = HSTRING::from("dr\u{00f8}m");
                let hwnd = FindWindowW(None, &win_title_lower).unwrap_or_else(|_| GetForegroundWindow());
                if hwnd.0.is_null() {
                    return;
                }

                let Ok(interop) = RoGetActivationFactory::<ISystemMediaTransportControlsInterop>(
                    &HSTRING::from("Windows.Media.SystemMediaTransportControls"),
                ) else {
                    return;
                };

                let Ok(smtc): Result<SystemMediaTransportControls, _> = interop.GetForWindow(hwnd) else {
                    return;
                };

                let _ = smtc.SetIsEnabled(true);
                let _ = smtc.SetIsPlayEnabled(true);
                let _ = smtc.SetIsPauseEnabled(true);
                let _ = smtc.SetIsNextEnabled(true);
                let _ = smtc.SetIsPreviousEnabled(true);
                let _ = smtc.SetIsStopEnabled(true);

                if !SMTC_BUTTON_HANDLER_REGISTERED.swap(true, Ordering::SeqCst) {
                    let app_win = win_clone.clone();
                    let handler = TypedEventHandler::<
                        SystemMediaTransportControls,
                        SystemMediaTransportControlsButtonPressedEventArgs,
                    >::new(move |_, args| {
                        if let Some(args) = args {
                            if let Ok(btn) = args.Button() {
                                let act = match btn {
                                    SystemMediaTransportControlsButton::Play => Some("play"),
                                    SystemMediaTransportControlsButton::Pause => Some("pause"),
                                    SystemMediaTransportControlsButton::Next => Some("next"),
                                    SystemMediaTransportControlsButton::Previous => Some("prev"),
                                    SystemMediaTransportControlsButton::Stop => Some("stop"),
                                    _ => None,
                                };
                                if let Some(a) = act {
                                    let _ = app_win.emit("droem://smtc-action", a);
                                }
                            }
                        }
                        Ok(())
                    });
                    let _ = smtc.ButtonPressed(&handler);
                }

                let status = if title.trim().is_empty() {
                    MediaPlaybackStatus::Stopped
                } else if is_playing {
                    MediaPlaybackStatus::Playing
                } else {
                    MediaPlaybackStatus::Paused
                };
                let _ = smtc.SetPlaybackStatus(status);

                if let Ok(updater) = smtc.DisplayUpdater() {
                    let _ = updater.SetType(MediaPlaybackType::Music);
                    if let Ok(props) = updater.MusicProperties() {
                        let _ = props.SetTitle(&HSTRING::from(title.as_str()));
                        let _ = props.SetArtist(&HSTRING::from(artist.as_str()));
                        let album_str = if album.trim().is_empty() {
                            "dr\u{00f8}m"
                        } else {
                            album.as_str()
                        };
                        let _ = props.SetAlbumTitle(&HSTRING::from(album_str));
                    }
                    if let Some(ref tp) = thumb_file_path {
                        if let Ok(op) = StorageFile::GetFileFromPathAsync(&HSTRING::from(tp.as_str())) {
                            if let Ok(sf) = op.get() {
                                if let Ok(stream_ref) = RandomAccessStreamReference::CreateFromFile(&sf) {
                                    let _ = updater.SetThumbnail(&stream_ref);
                                }
                            }
                        }
                    }
                    let _ = updater.Update();
                }
            }
        });
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = (window, title, artist, album, is_playing, cover_path);
    }

    Ok(())
}

async fn pick_music_folder(dialog_title: Option<String>) -> Result<Option<String>, String> {
    let picked = rfd::FileDialog::new()
        .set_title(dialog_title.unwrap_or_else(|| "Drøm".to_string()))
        .pick_folder();
    Ok(picked.map(|p| p.to_string_lossy().replace('\\', "/")))
}

#[tauri::command]
async fn pick_and_scan_folder(dialog_title: Option<String>) -> Result<Option<FolderScanResponse>, String> {
    let Some(folder_path) = pick_music_folder(dialog_title).await? else {
        return Ok(None);
    };
    let tracks = scan_music_folders(vec![folder_path.clone()])?;
    Ok(Some(FolderScanResponse {
        folder_path,
        tracks,
    }))
}

async fn pick_audio_files(dialog_title: Option<String>, filter_label: Option<String>) -> Result<Vec<String>, String> {
    let picked = rfd::FileDialog::new()
        .set_title(dialog_title.unwrap_or_else(|| "Drøm".to_string()))
        .add_filter(filter_label.unwrap_or_else(|| "Audio".to_string()), &["mp3", "flac", "wav", "m4a", "aac", "ogg", "opus", "wma", "aif", "aiff"])
        .pick_files()
        .unwrap_or_default();
    Ok(picked
        .into_iter()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .collect())
}

#[tauri::command]
async fn pick_and_scan_files(dialog_title: Option<String>, filter_label: Option<String>) -> Result<Vec<ScannedTrack>, String> {
    let files = pick_audio_files(dialog_title, filter_label).await?;
    if files.is_empty() {
        return Ok(Vec::new());
    }
    scan_audio_files(files)
}

fn scan_music_folders(folders: Vec<String>) -> Result<Vec<ScannedTrack>, String> {
    let specs: Vec<MonitoredFolderSpec> = folders
        .into_iter()
        .map(|path| MonitoredFolderSpec {
            path,
            include_subfolders: true,
        })
        .collect();
    scan_monitored_folders(specs)
}

fn scan_audio_files(files: Vec<String>) -> Result<Vec<ScannedTrack>, String> {
    let paths: Vec<PathBuf> = files.into_iter().map(PathBuf::from).collect();
    let mut tracks: Vec<ScannedTrack> = paths
        .par_iter()
        .filter_map(|p| parse_audio_file_fast(p))
        .collect();
    if tracks.len() > 1 {
        sort_tracks_by_filename_natural(&mut tracks);
    }
    Ok(tracks)
}

#[tauri::command]
fn scan_dropped_paths(paths: Vec<String>) -> Result<DroppedScanResult, String> {
    let mut has_folders = false;
    let mut folder_paths: Vec<String> = Vec::new();
    let mut audio_paths: Vec<PathBuf> = Vec::new();
    let mut seen_audio_keys: HashSet<String> = HashSet::new();

    for raw in paths {
        let trimmed = raw.trim().trim_matches('"');
        if trimmed.is_empty() {
            continue;
        }
        let norm_input = if cfg!(target_os = "windows") {
            trimmed.replace('/', "\\")
        } else {
            trimmed.to_string()
        };
        let p = if Path::new(&norm_input).exists() {
            PathBuf::from(&norm_input)
        } else {
            PathBuf::from(trimmed)
        };
        if !p.exists() {
            continue;
        }
        if p.is_dir() {
            has_folders = true;
            let norm_dir = p.to_string_lossy().replace('\\', "/");
            if !folder_paths.contains(&norm_dir) {
                folder_paths.push(norm_dir);
            }
            for entry in WalkDir::new(&p)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let ep = entry.path();
                if ep.is_file() && is_supported_audio(ep) {
                    let key = ep.to_string_lossy().replace('\\', "/").to_lowercase();
                    if seen_audio_keys.insert(key) {
                        audio_paths.push(ep.to_path_buf());
                    }
                }
            }
        } else if p.is_file() && is_recognized_external_audio(&p) {
            let key = p.to_string_lossy().replace('\\', "/").to_lowercase();
            if seen_audio_keys.insert(key) {
                audio_paths.push(p);
            }
        }
    }

    let mut tracks: Vec<ScannedTrack> = audio_paths
        .par_iter()
        .filter_map(|p| parse_audio_file_fast(p))
        .collect();

    if has_folders {
        sort_tracks_for_album_folder(&mut tracks);
    } else if tracks.len() > 1 {
        sort_tracks_by_filename_natural(&mut tracks);
    }

    Ok(DroppedScanResult {
        has_folders,
        folder_paths,
        tracks,
    })
}

#[tauri::command]
fn get_pending_startup_files(
    pending: State<'_, PendingStartupFiles>,
) -> Result<Vec<String>, String> {
    if let Ok(mut guard) = pending.0.lock() {
        let files = guard.drain(..).collect();
        return Ok(files);
    }
    Ok(Vec::new())
}

/// Part 1-3 & Rule 2: Deadlock-Free `set_window_mode`
/// - Reads current `outer_position()` BEFORE locking `pos_state`.
/// - Copies target coordinates (`shared_main_pos`, `mini_pos`, `prev_mode`) inside a short lock scope
///   and immediately calls `drop(mem)` BEFORE invoking any `window.set_size` or `window.set_position`!
/// - Guards synchronous Windows `WM_MOVE` callbacks with `IS_SWITCHING_MODE` so `on_window_event`
///   never contends for `pos_state` or overwrites coordinates mid-transition.
#[tauri::command]
fn set_window_mode(
    window: WebviewWindow,
    pos_state: State<'_, Mutex<WindowPositionMemory>>,
    mode: String,
    custom_width: Option<f64>,
    custom_height: Option<f64>,
    mini_always_on_top: Option<bool>,
) -> Result<(), String> {
    IS_SWITCHING_MODE.store(true, Ordering::SeqCst);
    let _ = window.set_shadow(false);

    let current_outer_pos = window.outer_position().ok().filter(|p| p.x > -16000 && p.y > -16000);

    let (prev_mode, saved_main_pos, saved_mini_pos) = if let Ok(mut mem) = pos_state.lock() {
        let prev = mem.current_mode.clone();
        if let Some(cur_pos) = current_outer_pos {
            if prev == "mini" {
                mem.mini_pos = Some(cur_pos);
            } else {
                mem.shared_main_pos = Some(cur_pos);
            }
        }
        mem.current_mode = mode.clone();
        let main_p = mem.shared_main_pos;
        let mini_p = mem.mini_pos;
        drop(mem);
        (prev, main_p, mini_p)
    } else {
        ("full".to_string(), None, None)
    };

    match mode.as_str() {
        "full" => {
            let w = custom_width.unwrap_or(1280.0).max(860.0);
            let h = custom_height.unwrap_or(820.0).max(560.0);
            let _ = window.set_always_on_top(false);
            let _ = window.set_resizable(true);
            let _ = window.set_min_size(Some(Size::Logical(LogicalSize {
                width: 860.0,
                height: 560.0,
            })));
            let _ = window.set_size(Size::Logical(LogicalSize {
                width: w,
                height: h,
            }));
            if let Some(saved_pos) = saved_main_pos {
                let _ = window.set_position(Position::Physical(saved_pos));
            }
        }
        "compact" => {
            let _ = window.set_always_on_top(false);
            let _ = window.set_resizable(true);
            let _ = window.set_min_size(Some(Size::Logical(LogicalSize {
                width: 398.0,
                height: 568.0,
            })));
            let _ = window.set_size(Size::Logical(LogicalSize {
                width: 440.0,
                height: 664.0,
            }));
            if let Some(saved_pos) = saved_main_pos {
                let _ = window.set_position(Position::Physical(saved_pos));
            }
        }
        "mini" => {
            let pin_top = mini_always_on_top.unwrap_or(true);
            let _ = window.set_min_size(Some(Size::Logical(LogicalSize {
                width: 296.0,
                height: 296.0,
            })));
            let _ = window.set_size(Size::Logical(LogicalSize {
                width: 296.0,
                height: 296.0,
            }));
            let _ = window.set_resizable(false);
            let _ = window.set_always_on_top(pin_top);
            if prev_mode != "mini" {
                if let Some(saved_mini) = saved_mini_pos {
                    let _ = window.set_position(Position::Physical(saved_mini));
                }
            }
        }
        _ => {}
    }

    #[cfg(target_os = "windows")]
    {
        let saved_eff = if let Ok(mem) = pos_state.lock() {
            mem.backdrop_effect.clone()
        } else {
            "glass".to_string()
        };
        let eff_for_mode = if mode == "full" { saved_eff.as_str() } else { "glass" };
        if !apply_win32_backdrop_to_window(&window, eff_for_mode, true) {
            if let Ok(mut mem) = pos_state.lock() {
                mem.backdrop_effect = "glass".to_string();
            }
        }
    }

    IS_SWITCHING_MODE.store(false, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
fn set_mini_always_on_top(
    window: WebviewWindow,
    pos_state: State<'_, Mutex<WindowPositionMemory>>,
    always_on_top: bool,
) -> Result<(), String> {
    let is_mini = if let Ok(mem) = pos_state.lock() {
        mem.current_mode == "mini"
    } else {
        true
    };
    if is_mini {
        window.set_always_on_top(always_on_top).map_err(|e| e.to_string())?;
    } else {
        let _ = window.set_always_on_top(false);
    }
    Ok(())
}

#[tauri::command]
fn start_window_drag(window: WebviewWindow) -> Result<(), String> {
    window.start_dragging().map_err(|e| e.to_string())
}

#[tauri::command]
fn start_native_file_drag(
    window: WebviewWindow,
    file_paths: Vec<String>,
) -> Result<(), String> {
    let valid_paths: Vec<PathBuf> = file_paths
        .into_iter()
        .map(|p| PathBuf::from(p.replace('/', "\\")))
        .filter(|p| p.exists() && p.is_file())
        .collect();

    if valid_paths.is_empty() {
        return Ok(());
    }

    let win_clone = window.clone();
    window
        .run_on_main_thread(move || {
            let item = drag::DragItem::Files(valid_paths);
            let preview_icon = drag::Image::Raw(DRAG_PREVIEW_PNG.to_vec());
            #[cfg(not(target_os = "linux"))]
            {
                let _ = drag::start_drag(
                    &win_clone,
                    item,
                    preview_icon,
                    |_result, _cursor_pos| {},
                    drag::Options::default(),
                );
            }
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn control_window(window: WebviewWindow, action: String) -> Result<(), String> {
    let _ = window.set_shadow(false);
    match action.as_str() {
        "minimize" => {
            let _ = window.minimize();
        }
        "maximize" => {
            if window.is_maximized().unwrap_or(false) {
                let _ = window.unmaximize();
            } else {
                let _ = window.maximize();
            }
        }
        "close" => {
            let _ = window.close();
            window.app_handle().exit(0);
        }
        _ => {}
    }
    Ok(())
}

/// Part 1-5: Supports both folders (opens folder directly in Explorer) and audio files (`/select,`)
#[tauri::command]
fn reveal_in_explorer(file_path: String) -> Result<(), String> {
    let trimmed = file_path.trim().trim_matches('"');
    if trimmed.is_empty() {
        return Err("Empty path".to_string());
    }
    let win_path = trimmed.replace('/', "\\");
    let p = Path::new(&win_path);
    let is_dir = p.is_dir() || Path::new(trimmed).is_dir();

    #[cfg(target_os = "windows")]
    {
        if is_dir {
            Command::new("explorer.exe")
                .arg(&win_path)
                .spawn()
                .map_err(|e| e.to_string())?;
        } else if p.exists() {
            Command::new("explorer.exe")
                .raw_arg(format!("/select,\"{}\"", win_path))
                .spawn()
                .map_err(|e| e.to_string())?;
        } else if let Some(parent) = p.parent() {
            if parent.exists() {
                Command::new("explorer.exe")
                    .arg(parent.as_os_str())
                    .spawn()
                    .map_err(|e| e.to_string())?;
            } else {
                Command::new("explorer.exe")
                    .raw_arg(format!("/select,\"{}\"", win_path))
                    .spawn()
                    .map_err(|e| e.to_string())?;
            }
        } else {
            Command::new("explorer.exe")
                .raw_arg(format!("/select,\"{}\"", win_path))
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }
    #[cfg(target_os = "macos")]
    {
        if is_dir {
            Command::new("open")
                .arg(&file_path)
                .spawn()
                .map_err(|e| e.to_string())?;
        } else {
            Command::new("open")
                .args(["-R", &file_path])
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }
    #[cfg(target_os = "linux")]
    {
        let target = if is_dir {
            Path::new(&file_path)
        } else {
            Path::new(&file_path).parent().unwrap_or_else(|| Path::new(&file_path))
        };
        Command::new("xdg-open")
            .arg(target)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Phase 37 Security Hardening: Validates that the target path is an existing supported audio file
/// before moving it to the OS Recycle Bin (`trash::delete`). Arbitrary non-audio files or directories are rejected.
#[tauri::command]
fn move_audio_file_to_trash(file_path: String) -> Result<(), String> {
    let trimmed = file_path.trim().trim_matches('"');
    if trimmed.is_empty() {
        return Err("INVALID_EMPTY_PATH".to_string());
    }
    let norm = if cfg!(target_os = "windows") {
        trimmed.replace('/', "\\")
    } else {
        trimmed.to_string()
    };
    let path = if Path::new(&norm).is_file() {
        Path::new(&norm)
    } else {
        Path::new(trimmed)
    };
    if !path.is_file() {
        return Ok(());
    }
    if !is_supported_audio(path) {
        return Err("UNSUPPORTED_FILE_TYPE".to_string());
    }
    trash::delete(path).map_err(|e| format!("Failed to move audio file to Recycle Bin: {}", e))?;
    Ok(())
}

/// Phase 38-C Security-Hardened External URL Opener:
/// - Strictly allowlists the official public repository (`https://github.com/so1fr1d/droem` and safe `/releases` subpaths)
/// - Rejects any URL containing shell metacharacters, whitespace, quotes, or control characters
/// - Uses `rundll32.exe url.dll,FileProtocolHandler` on Windows instead of `cmd.exe /C start`,
///   completely eliminating `cmd.exe` command injection / argument splitting vectors.
#[tauri::command]
fn open_external_url(url: String) -> Result<(), String> {
    let trimmed = url.trim();
    const ALLOWED_BASE: &str = "https://github.com/so1fr1d/droem";
    if trimmed != ALLOWED_BASE && !trimmed.starts_with("https://github.com/so1fr1d/droem/") {
        return Err("BLOCKED_EXTERNAL_URL".to_string());
    }
    if trimmed.len() > 200
        || !trimmed
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, ':' | '/' | '.' | '-' | '_'))
    {
        return Err("INVALID_URL_CHARACTERS".to_string());
    }
    #[cfg(target_os = "windows")]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        Command::new("rundll32.exe")
            .args(["url.dll,FileProtocolHandler", trimmed])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(trimmed)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(trimmed)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

const EMBEDDED_THIRD_PARTY_NOTICES: &str = r#####"# THIRD-PARTY OPEN SOURCE NOTICES

**drøm v1.0.0** (`Windows x64`)의 자체 소스 코드(`index.html`, `src-tauri/src/main.rs`, `vite.config.ts`, 빌드·설치 스크립트)와 수학적 SDF/벡터 알고리즘으로 직접 생성한 자체 아이콘 자산(`src-tauri/icons/*`)은 **so1fr1d**가 개발하였으며 **MIT-0 (MIT No Attribution, `SPDX-License-Identifier: MIT-0`)** 라이선스로 공개됩니다.

단, 본 소프트웨어가 사용하는 **제3자 오픈소스 라이브러리(`src-tauri/Cargo.lock`, `package-lock.json`) 및 런타임·설치 도구**에는 각 원저작자의 라이선스가 그대로 적용됩니다. 또한 외부 저작물(타인의 소스 코드, 외부 폰트 파일, 제3자 아이콘, 사용자의 음원 및 앨범아트)에는 MIT-0가 자동 적용되지 않으며, 권한이 확인되지 않은 외부 자산은 공개 패키지에 일절 포함하지 않습니다.

---

## 1. 직접 의존성 및 핵심 프레임워크 (`src-tauri/Cargo.toml` / `package.json`)

| 컴포넌트 / 패키지 | 기준 버전 (`Cargo.toml` / `package.json`) | 라이선스 (SPDX) | 저작권자 / 출처 | 용도 및 배포 포함 여부 |
| :--- | :--- | :--- | :--- | :--- |
| **`tauri`** | `2.x` (`Cargo.lock` 고정) | `Apache-2.0 OR MIT` | Copyright (c) 2019-present Tauri Programme within The Commons Conservancy | 데스크톱 애플리케이션 코어 프레임워크 (런타임 포함) |
| **`tauri-build`** | `2.x` (`Cargo.lock` 고정) | `Apache-2.0 OR MIT` | Copyright (c) 2019-present Tauri Programme within The Commons Conservancy | Windows 리소스·매니페스트 빌드 스크립트 (빌드 시점 사용) |
| **`tauri-plugin-single-instance`** | `2.x` (`Cargo.lock` 고정) | `Apache-2.0 OR MIT` | Copyright (c) 2019-present Tauri Programme within The Commons Conservancy | 단일 인스턴스 실행 및 탐색기 다중 파일 인자 전달 (런타임 포함) |
| **`serde` / `serde_json`** | `1.x` (`Cargo.lock` 고정) | `MIT OR Apache-2.0` | Copyright (c) Erickson, David Tolnay & Serde Contributors | 직렬화 및 JSON 데이터 파싱 (런타임 포함) |
| **`lofty`** | `0.21.x` (`Cargo.lock` 고정) | `MIT OR Apache-2.0` | Copyright (c) Serial-ATA & Lofty Contributors | 오디오 메타데이터(ID3v2, Vorbis, MP4, AIFF 등) 및 커버 파싱 (런타임 포함) |
| **`walkdir`** | `2.5.x` (`Cargo.lock` 고정) | `Unlicense OR MIT` | Copyright (c) Andrew Gallant (BurntSushi) | 고속 로컬 음악 디렉터리 순회 (런타임 포함) |
| **`rayon`** | `1.10.x` (`Cargo.lock` 고정) | `MIT OR Apache-2.0` | Copyright (c) Niko Matsakis, Josh Stone & Rayon Contributors | 멀티코어 병렬 오디오 파일 스캔 (런타임 포함) |
| **`base64`** | `0.22.x` (`Cargo.lock` 고정) | `MIT OR Apache-2.0` | Copyright (c) Alice Maz, Marshall Pierce & Contributors | 앨범 커버 이미지 Base64 인코딩 (런타임 포함) |
| **`rfd` (Rusty File Dialogs)** | `0.15.x` (`Cargo.lock` 고정) | `MIT` | Copyright (c) PolyMeilex & RFD Contributors | Windows 네이티브 파일/폴더 열기 대화상자 (런타임 포함) |
| **`trash`** | `5.x` (`Cargo.lock` 고정) | `MIT` | Copyright (c) Byron (Sebastian Thiel), Artururas & Contributors | Windows 시스템 휴지통(Recycle Bin) 이동 (런타임 포함) |
| **`drag`** | `2.x` (`Cargo.lock` 고정) | `MIT OR Apache-2.0` | Copyright (c) CrabNebula / Tauri Contributors | 외부 프로그램으로 네이티브 파일 드래그 아웃 (런타임 포함) |
| **`windows` / `windows-core`** | `0.58.x` (`Cargo.lock` 고정) | `MIT OR Apache-2.0` | Copyright (c) Microsoft Corporation | Windows WinRT SMTC, DWM 및 Win32 API 바인딩 (런타임 포함) |
| **`vite` (빌드 전용)** | `8.3.4` (`package-lock.json` 고정) | `MIT` | Copyright (c) 2019-present VoidZero Inc., Yuxi (Evan) You and Vite contributors | 프론트엔드 번들러 (`vite.config.ts`, `tsc && vite build` — 최종 바이너리 미포함) |
| **`typescript` (빌드 전용)** | `6.0.3` (`package-lock.json` 고정) | `Apache-2.0` | Copyright (c) Microsoft Corporation | TypeScript 타입 검사기 (`tsc && vite build` — 최종 바이너리 미포함) |
| **`@tauri-apps/cli` / `@tauri-apps/api`** | `2.x` (`package-lock.json` 고정) | `Apache-2.0 OR MIT` | Copyright (c) 2019-present Tauri Programme within The Commons Conservancy | Tauri CLI 번들링 도구 및 IPC 타입 바인딩 |

---

## 2. 잠금 파일(`src-tauri/Cargo.lock` & `package-lock.json`) 기반 전이 의존성 대조

`src-tauri/Cargo.lock` 및 `package-lock.json`에 의해 해석되는 하위 전이(Transitive) 구성요소들과 적용 라이선스는 다음과 같습니다:

| 분류 | 주요 구성요소 (`Cargo.lock` / `package-lock.json`) | 적용 라이선스 (SPDX) | 저작권자 및 비고 |
| :--- | :--- | :--- | :--- |
| **Tauri 창·WebView2 런타임 바인딩** | `tao`, `wry`, `webview2-com`, `webview2-com-sys`, `webview2-com-macros`, `dpi`, `muda`, `tray-icon`, `window-vibrancy` | `Apache-2.0 OR MIT` | Copyright (c) Tauri Programme within The Commons Conservancy |
| **Windows Win32 / WinRT 시스템 바인딩** | `windows`, `windows-core`, `windows-result`, `windows-strings`, `windows-sys`, `windows-targets`, `windows-implement`, `windows-interface`, `windows_x86_64_msvc` | `MIT OR Apache-2.0` | Copyright (c) Microsoft Corporation |
| **비동기 런타임 & 동시성** | `tokio`, `tokio-macros`, `mio`, `bytes`, `pin-project-lite`, `parking_lot`, `parking_lot_core`, `lock_api`, `scopeguard`, `smallvec`, `once_cell` | `MIT` 또는 `MIT OR Apache-2.0` | Copyright (c) Tokio Contributors, Amanieu d'Antras & Contributors |
| **멀티코어 병렬 처리 (`rayon` 하위)** | `rayon-core`, `crossbeam-deque`, `crossbeam-epoch`, `crossbeam-utils`, `either` | `MIT OR Apache-2.0` | Copyright (c) Crossbeam & Rayon Contributors |
| **오디오 태그 & 이미지/압축 처리 (`lofty` 하위)** | `lofty_attr`, `ogg_pager`, `byteorder`, `flate2`, `miniz_oxide`, `adler2`, `crc32fast`, `png`, `fdeflate`, `simd-adler32`, `bitflags`, `cfg-if` | `MIT OR Apache-2.0` / `Unlicense OR MIT` / `Zlib` | Copyright (c) Serial-ATA, BurntSushi, Image-rs & Contributors |
| **파일시스템 & 문자열/정규식 유틸리티** | `same-file`, `memchr`, `aho-corasick`, `regex`, `regex-automata`, `regex-syntax`, `itoa`, `ryu`, `log` | `Unlicense OR MIT` / `MIT OR Apache-2.0` | Copyright (c) Andrew Gallant (BurntSushi), David Tolnay & Contributors |
| **URL · 유니코드 · 윈도우 핸들 표준** | `url`, `idna`, `form_urlencoded`, `percent-encoding`, `unicode-bidi`, `unicode-normalization`, `raw-window-handle`, `unicode-ident` | `MIT OR Apache-2.0` / `Unicode-3.0` / `BSD-2-Clause` / `BSD-3-Clause` | Copyright (c)The Servo Project Developers, Unicode, Inc. & Contributors |
| **프로시저 매크로 & 컴파일러 보조** | `syn`, `quote`, `proc-macro2`, `thiserror`, `thiserror-impl`, `anyhow`, `serde_derive`, `serde_spanned`, `toml` | `MIT OR Apache-2.0` | Copyright (c) David Tolnay & Contributors |
| **Vite 8.3.4 / TypeScript 빌드 체인 (`package-lock.json`)** | `vite` (`8.3.4`), `typescript`, `esbuild`, `rollup`, `postcss`, `picocolors`, `nanoid`, `source-map-js` | `MIT` / `Apache-2.0` / `ISC` / `BSD-2-Clause` | 빌드 전용(`devDependencies`) — 최종 `droem.exe` 바이너리에 미포함 |

<!-- BEGIN_LOCKFILE_INVENTORY -->
### 2.1 실제 잠금 파일(`src-tauri/Cargo.lock` 및 `package-lock.json`) 대조 요약

- **`package-lock.json` 파싱**: 완료 (lockfileVersion 3; 루트 프로젝트 제외 **58개** 잠금 패키지)
- **프론트엔드 잠금 버전**: `vite 8.3.4`, `typescript 6.0.3`, `@tauri-apps/cli 2.12.2`, `@tauri-apps/api 2.12.3`
- **`src-tauri/Cargo.lock` 파싱**: 완료 (잠금 패키지 항목 **501개**; 동일 이름의 서로 다른 버전을 별도로 계산)
- **개별 Rust 크레이트 라이선스 메타데이터 전수 확인**: **미완료**. `Cargo.lock`에는 라이선스 필드가 없어 501개 항목에 대한 라이선스 준수 여부를 잠금 파일만으로 확정할 수 없습니다. 릴리즈 공개 전 실제 `cargo metadata` 및 의존 크레이트 배포물의 라이선스·고지문을 추가로 확인해야 합니다.

**주의:** 위의 핵심·전이 구성요소 표는 대표 의존성에 대한 안내이며, 모든 잠금 패키지의 SPDX 라이선스를 독립 검증한 전체 목록은 아닙니다.
<!-- END_LOCKFILE_INVENTORY -->

---

## 3. 외부 자산·폰트·아이콘·런타임 구분 고지

1. **폰트 파일 미번들 (No Bundled Fonts)**:
   - drøm은 `.ttf`, `.otf`, `.woff2` 폰트 파일을 패키지나 실행 파일 내부에 포함·재배포하지 않습니다.
   - CSS `local(...)` 선언으로 사용자 OS에 설치된 시스템/로컬 폰트(`Inter`, `Noto Sans CJK`, `Segoe UI Variable Text`, `Malgun Gothic`, `Yu Gothic UI`, `Microsoft YaHei UI`, `Microsoft JhengHei UI`)만 호출합니다.
2. **자체 제작 아이콘 자산 (`src-tauri/icons/*`)**:
   - 다크 메인 아이콘(`icon.ico`, `1_main_ref_clean.ico`, `icon.svg`, `32x32.png`, `128x128.png`, `128x128@2x.png`), 라이트 보조 아이콘(`5_ui_return_light.ico`, `icon_light.*`), UI 회귀 아이콘(`4_ui_return_dark.ico`), 공식 배포 10종 오디오 파일 아이콘(`audio_mp3.ico`, `audio_flac.ico`, `audio_wav.ico`, `audio_m4a.ico`, `audio_aac.ico`, `audio_ogg.ico`, `audio_opus.ico`, `audio_wma.ico`, `audio_aif.ico`, `audio_aiff.ico` 및 개발 자산 `audio_wv.ico`, `audio_ape.ico`, `audio_mpc.ico`)은 타인의 그래픽 자산을 복제하지 않고 코드 내 수학적 벡터/SDF 수식으로 직접 생성한 자체 저작물이며 **MIT-0**가 적용됩니다.
3. **사용자 음원 및 앨범아트**:
   - 사용자가 앱에서 불러오는 로컬 오디오 파일, 가사, 앨범 커버 이미지의 저작권은 각 권리자에게 있으며, 공개 소스 및 배포 패키지에는 어떠한 개인 음원이나 테스트용 상업 앨범 커버도 포함되지 않습니다.
4. **런타임 및 설치 도구**:
   - **Microsoft Edge WebView2 Runtime**: Windows OS의 Evergreen WebView2 런타임을 동적 호출합니다 (Copyright (c) Microsoft Corporation).
   - **NSIS (Nullsoft Scriptable Install System)**: 설치 마법사 스텁은 `Zlib` / `CPL-1.0` 등 NSIS 라이선스가 적용됩니다.

---

## 4. 제3자 오픈소스 라이선스 전문 (Unabridged Full License Texts)

### 4.1 The MIT License (`MIT`)

```text
MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### 4.2 Apache License, Version 2.0 (`Apache-2.0` — 전문)

```text
                                 Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
      other entities that control, are controlled by, or are under common
      control with that entity. For the purposes of this definition,
      "control" means (i) the power, direct or indirect, to cause the
      direction or management of such entity, whether by contract or
      otherwise, or (ii) ownership of fifty percent (50%) or more of the
      outstanding shares, or (iii) beneficial ownership of such entity.

      "You" (or "Your") shall mean an individual or Legal Entity
      exercising permissions granted by this License.

      "Source" form shall mean the preferred form for making modifications,
      including but not limited to software source code, documentation
      source, and configuration files.

      "Object" form shall mean any form resulting from mechanical
      transformation or translation of a Source form, including but
      not limited to compiled object code, generated documentation,
      and conversions to other media types.

      "Work" shall mean the work of authorship, whether in Source or
      Object form, made available under the License, as indicated by a
      copyright notice that is included in or attached to the work
      (an example is provided in the Appendix below).

      "Derivative Works" shall mean any work, whether in Source or Object
      form, that is based on (or derived from) the Work and for which the
      editorial revisions, annotations, elaborations, or other modifications
      represent, as a whole, an original work of authorship. For the purposes
      of this License, Derivative Works shall not include works that remain
      separable from, or merely link (or bind by name) to the interfaces of,
      the Work and Derivative Works thereof.

      "Contribution" shall mean any work of authorship, including
      the original version of the Work and any modifications or additions
      to that Work or Derivative Works thereof, that is intentionally
      submitted to Licensor for inclusion in the Work by the copyright owner
      or by an individual or Legal Entity authorized to submit on behalf of
      the copyright owner. For the purposes of this definition, "submitted"
      means any form of electronic, verbal, or written communication sent
      to the Licensor or its representatives, including but not limited to
      communication on electronic mailing lists, source code control systems,
      and issue tracking systems that are managed by, or on behalf of, the
      Licensor for the purpose of discussing and improving the Work, but
      excluding communication that is conspicuously marked or otherwise
      designated in writing by the copyright owner as "Not a Contribution."

      "Contributor" shall mean Licensor and any individual or Legal Entity
      on behalf of whom a Contribution has been received by Licensor and
      subsequently incorporated within the Work.

   2. Grant of Copyright License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      copyright license to reproduce, prepare Derivative Works of,
      publicly display, publicly perform, sublicense, and distribute the
      Work and such Derivative Works in Source or Object form.

   3. Grant of Patent License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      (except as stated in this section) patent license to make, have made,
      use, offer to sell, sell, import, and otherwise transfer the Work,
      where such license applies only to those patent claims licensable
      by such Contributor that are necessarily infringed by their
      Contribution(s) alone or by combination of their Contribution(s)
      with the Work to which such Contribution(s) was submitted. If You
      institute patent litigation against any entity (including a
      cross-claim or counterclaim in a lawsuit) alleging that the Work
      or a Contribution incorporated within the Work constitutes direct
      or contributory patent infringement, then any patent licenses
      granted to You under this License for that Work shall terminate
      as of the date such litigation is filed.

   4. Redistribution. You may reproduce and distribute copies of the
      Work or Derivative Works thereof in any medium, with or without
      modifications, and in Source or Object form, provided that You
      meet the following conditions:

      (a) You must give any other recipients of the Work or
          Derivative Works a copy of this License; and

      (b) You must cause any modified files to carry prominent notices
          stating that You changed the files; and

      (c) You must retain, in the Source form of any Derivative Works
          that You distribute, all copyright, patent, trademark, and
          attribution notices from the Source form of the Work,
          excluding those notices that do not pertain to any part of
          the Derivative Works; and

      (d) If the Work includes a "NOTICE" text file as part of its
          distribution, then any Derivative Works that You distribute must
          include a readable copy of the attribution notices contained
          within such NOTICE file, excluding those notices that do not
          pertain to any part of the Derivative Works, in at least one
          of the following places: within a NOTICE text file distributed
          as part of the Derivative Works; within the Source form or
          documentation, if provided along with the Derivative Works; or,
          within a display generated by the Derivative Works, if and
          wherever such third-party notices normally appear. The contents
          of the NOTICE file are for informational purposes only and
          do not modify the License. You may add Your own attribution
          notices within Derivative Works that You distribute, alongside
          or as an addendum to the NOTICE text from the Work, provided
          that such additional attribution notices cannot be construed
          as modifying the License.

      You may add Your own copyright statement to Your modifications and
      may provide additional or different license terms and conditions
      for use, reproduction, or distribution of Your modifications, or
      for any such Derivative Works as a whole, provided Your use,
      reproduction, and distribution of the Work otherwise complies with
      the conditions stated in this License.

   5. Submission of Contributions. Unless You explicitly state otherwise,
      any Contribution intentionally submitted for inclusion in the Work
      by You to the Licensor shall be under the terms and conditions of
      this License, without any additional terms or conditions.
      Notwithstanding the above, nothing herein shall supersede or modify
      the terms of any separate license agreement you may have executed
      with Licensor regarding such Contributions.

   6. Trademarks. This License does not grant permission to use the trade
      names, trademarks, service marks, or product names of the Licensor,
      except as required for reasonable and customary use in describing the
      origin of the Work and reproducing the content of the NOTICE file.

   7. Disclaimer of Warranty. Unless required by applicable law or
      agreed to in writing, Licensor provides the Work (and each
      Contributor provides its Contributions) on an "AS IS" BASIS,
      WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
      implied, including, without limitation, any warranties or conditions
      of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
      PARTICULAR PURPOSE. You are solely responsible for determining the
      appropriateness of using or redistributing the Work and assume any
      risks associated with Your exercise of permissions under this License.

   8. Limitation of Liability. In no event and under no legal theory,
      whether in tort (including negligence), contract, or otherwise,
      unless required by applicable law (such as deliberate and grossly
      negligent acts) or agreed to in writing, shall any Contributor be
      liable to You for damages, including any direct, indirect, special,
      incidental, or consequential damages of any character arising as a
      result of this License or out of the use or inability to use the
      Work (including but not limited to damages for loss of goodwill,
      work stoppage, computer failure or malfunction, or any and all
      other commercial damages or losses), even if such Contributor
      has been advised of the possibility of such damages.

   9. Accepting Warranty or Additional Liability. While redistributing
      the Work or Derivative Works thereof, You may choose to offer,
      and charge a fee for, acceptance of support, warranty, indemnity,
      or other liability obligations and/or rights consistent with this
      License. However, in accepting such obligations, You may act only
      on Your own behalf and on Your sole responsibility, not on behalf
      of any other Contributor, and only if You agree to indemnify,
      defend, and hold each Contributor harmless for any liability
      incurred by, or claims asserted against, such Contributor by reason
      of your accepting any such warranty or additional liability.

   END OF TERMS AND CONDITIONS

   APPENDIX: How to apply the Apache License to your work.

      To apply the Apache License to your work, attach the following
      boilerplate notice, with the fields enclosed by brackets "[]"
      replaced with your own identifying information. (Don't include
      the brackets!)  The text should be enclosed in the appropriate
      comment syntax for the file format. We also recommend that a
      file or class name and description of purpose be included on the
      same "printed page" as the copyright notice for easier
      identification within third-party archives.

   Copyright [yyyy] [name of copyright owner]

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```

### 4.3 BSD 2-Clause "Simplified" License (`BSD-2-Clause`)

```text
BSD 2-Clause License

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

### 4.4 BSD 3-Clause "New" or "Revised" License (`BSD-3-Clause`)

```text
BSD 3-Clause License

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

### 4.5 ISC License (`ISC`)

```text
ISC License

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE.
```

### 4.6 Zlib / libpng License (`Zlib` — NSIS Stub & Compression Crates)

```text
zlib License

This software is provided 'as-is', without any express or implied
warranty. In no event will the authors be held liable for any damages
arising from the use of this software.

Permission is granted to anyone to use this software for any purpose,
including commercial applications, and to alter it and redistribute it
freely, subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not
   claim that you wrote the original software. If you use this software
   in a product, an acknowledgment in the product documentation would be
   appreciated but is not required.
2. Altered source versions must be plainly marked as such, and must not be
   misrepresented as being the original software.
3. This notice may not be removed or altered from any source distribution.
```

### 4.7 The Unlicense (`Unlicense`)

```text
This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <https://unlicense.org>
```

### 4.8 Unicode License v3 (`Unicode-3.0`)

```text
UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 1991-2026 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
Data Files or Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, and/or sell
copies of the Data Files or Software, and to permit persons to whom the
Data Files or Software are furnished to do so, provided that either (a)
this copyright and permission notice appear with all copies of the Data
Files or Software, or (b) this copyright and permission notice appear in
associated Documentation.

THE DATA FILES AND SOFTWARE ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY
KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF
THIRD PARTY RIGHTS. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS
INCLUDED IN THIS NOTICE BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT
OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THE DATA FILES OR SOFTWARE.

Except as contained in this notice, the name of a copyright holder shall
not be used in advertising or otherwise to promote the sale, use or other
dealings in these Data Files or Software without prior written
authorization of the copyright holder.
```

### 4.9 Mozilla Public License Version 2.0 (`MPL-2.0` — 조건부 전이 의존성 고지)

```text
Cargo.lock 전이 의존성 중 MPL-2.0이 적용되는 크레이트(예: option-ext 등)가 포함되는 경우,
해당 크레이트는 수정 없이 원본 상태 그대로 링크되며 MPL-2.0 제1.10조 및 제3.2조에 따라
원본 소스 코드는 https://crates.io 및 각 크레이트의 공식 저장소에서 제공됩니다.
전문: https://www.mozilla.org/en-US/MPL/2.0/
```
"#####;

/// Phase 38-E.4: Opens THIRD_PARTY_NOTICES.md directly in the user's local viewer
/// Works in both installed (`C:\Program Files\Droem`) and standalone portable (`droem.exe`) builds.
#[tauri::command]
fn open_third_party_notices() -> Result<String, String> {
    let mut candidate_paths: Vec<PathBuf> = Vec::new();
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            candidate_paths.push(exe_dir.join("THIRD_PARTY_NOTICES.md"));
            candidate_paths.push(exe_dir.join("_up_").join("THIRD_PARTY_NOTICES.md"));
            candidate_paths.push(exe_dir.join("resources").join("THIRD_PARTY_NOTICES.md"));
            candidate_paths.push(exe_dir.join("resources").join("_up_").join("THIRD_PARTY_NOTICES.md"));
        }
    }
    candidate_paths.push(PathBuf::from("THIRD_PARTY_NOTICES.md"));

    let resolved_path = if let Some(existing) = candidate_paths.into_iter().find(|p| p.is_file()) {
        existing
    } else {
        let fallback_dir = std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .filter(|p| p.is_dir())
            .map(|p| p.join("droem"))
            .unwrap_or_else(std::env::temp_dir);
        let _ = fs::create_dir_all(&fallback_dir);
        let target_file = fallback_dir.join("THIRD_PARTY_NOTICES.md");
        let _ = write_file_if_changed(&target_file, EMBEDDED_THIRD_PARTY_NOTICES.as_bytes());
        target_file
    };

    let path_str = resolved_path.to_string_lossy().to_string();

    #[cfg(target_os = "windows")]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let win_path = path_str.replace('/', "\\");
        // Open with default associated viewer (or Notepad fallback if ShellExecute protocol handler fails)
        let launched = Command::new("rundll32.exe")
            .args(["url.dll,FileProtocolHandler", &win_path])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
        if launched.is_err() {
            let _ = Command::new("notepad.exe")
                .arg(&win_path)
                .spawn();
        }
    }
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg(&path_str).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(&path_str).spawn();
    }

    Ok(path_str.replace('\\', "/"))
}


// =============================================================================
// Phase 26: Redefined Lunar Icon System (Group 1 Single Main Icon & Group 2 Common Family)
// - Shared Core Geometry (`sample_lunar_core_symbol_masks`):
//   Approved Lunar Crescent Moon + Star Dot + 2 Tapered S-Curve Waves (Master Lunar Reference)
// - Group 1 (Single Main App Icon: `icon.ico` & `1_main_ref_clean.ico`):
//   Subtle semi-transparent high-radius rounded square (`cr = 0.245` on `0.445` half-size),
//   very thin soft outer border, resolution-aware rendering (16/24/32px crisp legibility,
//   48/64px subtle ambient highlight).
// - Group 2 (UI Return Dark `4_ui_return_dark.ico`, UI Return Light `5_ui_return_light.ico`,
//   and 13 Audio Format Icons `audio_<ext>.ico`):
//   100% identical master geometry, canvas, corner radius (`cr = 0.195`), border thickness,
//   and symbol placement via `sample_group2_master_rgba(pal, u, v)` — differing ONLY in palette!
// - Legacy unused icon candidates (`2_main_subtle_glow.ico`, `3_main_subtle_glass.ico`,
//   `6_audio_file_badge.ico`, `droem-monogram-do.svg`) are no longer regenerated.
// =============================================================================

fn clamp01(v: f32) -> f32 {
    v.max(0.0).min(1.0)
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp01((x - edge0) / (edge1 - edge0));
    t * t * (3.0 - 2.0 * t)
}

fn lerp_rgb(a: (f32, f32, f32), b: (f32, f32, f32), t: f32) -> (f32, f32, f32) {
    let k = clamp01(t);
    (
        a.0 + (b.0 - a.0) * k,
        a.1 + (b.1 - a.1) * k,
        a.2 + (b.2 - a.2) * k,
    )
}

/// Signed distance to a rounded rectangle centered at (0.5, 0.5) with half-extents `hx, hy` and corner radius `cr`
fn sd_rounded_box(u: f32, v: f32, hx: f32, hy: f32, cr: f32) -> f32 {
    let qx = (u - 0.5).abs() - (hx - cr);
    let qy = (v - 0.5).abs() - (hy - cr);
    let out_dist = (qx.max(0.0).powi(2) + qy.max(0.0).powi(2)).sqrt();
    let in_dist = qx.max(qy).min(0.0);
    out_dist + in_dist - cr
}

/// Shared Master Lunar Symbol Mask Evaluator (`(primary_mask, secondary_wave_mask)`)
/// Master Lunar geometry specification:
/// - Crescent Moon: outer `(0.53765, 0.42025, r=0.20356)`, inner cutout `(0.59952, 0.37119, r=0.18004)`
/// - Tiny Star Dot: `(0.6763, 0.3397, r=0.0095)`
/// - Two diagonal crescent-hugging waves with perpendicular distance evaluation
fn sample_lunar_core_symbol_masks_scaled(u_raw: f32, v_raw: f32, size: u32, sym_scale: f32) -> (f32, f32) {
    let mut primary_mask = 0.0f32;
    let mut secondary_mask = 0.0f32;

    let u_mid = 0.4575f32;
    let v_mid = 0.4976f32;
    let u = u_mid + (u_raw - 0.50) / sym_scale;
    let v = v_mid + (v_raw - 0.50) / sym_scale;

    let px_unit = 1.0f32 / (size.max(16) as f32);
    let aa_sym = (0.55 * px_unit) / sym_scale;
    let small_wave_floor = if size <= 16 {
        0.0055
    } else if size <= 24 {
        0.0035
    } else if size <= 32 {
        0.0018
    } else {
        0.0
    };
    let small_star_boost = if size <= 16 {
        0.0040
    } else if size <= 24 {
        0.0022
    } else {
        0.0
    };

    // 1) Crescent Moon (exact sub-pixel fit)
    let (m_cx, m_cy, m_r1, cut_cx, cut_cy, m_r2) =
        (0.53765f32, 0.42025f32, 0.20356f32, 0.59952f32, 0.37119f32, 0.18004f32);
    let d1 = ((u - m_cx).powi(2) + (v - m_cy).powi(2)).sqrt();
    let d2 = ((u - cut_cx).powi(2) + (v - cut_cy).powi(2)).sqrt();
    let moon_m = (1.0 - smoothstep(m_r1 - aa_sym, m_r1 + aa_sym, d1))
        * smoothstep(m_r2 - aa_sym, m_r2 + aa_sym, d2);
    primary_mask = primary_mask.max(moon_m);

    // 2) Single Round Star Dot in upper-right concave hollow (100% identical across all icons)
    let star_r = 0.0102 + small_star_boost;
    let star_d = ((u - 0.6763).powi(2) + (v - 0.3397).powi(2)).sqrt();
    let star_m = 1.0 - smoothstep(star_r - aa_sym, star_r + aa_sym, star_d);
    primary_mask = primary_mask.max(star_m);

    // 3) Upper Diagonal Crescent-Hugging Wave (`u in [0.19033, 0.71473]`)
    let u1_min = 0.19033f32;
    let u1_max = 0.71473f32;
    if u >= u1_min && u <= u1_max {
        let span1 = u1_max - u1_min;
        let t1 = (u - u1_min) / span1;
        let v1 = 0.60374 - 0.46471 * t1 + 1.68955 * t1.powi(2) - 1.11900 * t1.powi(3);
        let dv1_du = (-0.46471 + 3.37910 * t1 - 3.35700 * t1.powi(2)) / span1;
        let perp1 = (v - v1).abs() / (1.0 + dv1_du.powi(2)).sqrt();
        let hw1 = (0.0126 + small_wave_floor) * (std::f32::consts::PI * t1).sin().max(0.0).powf(0.70);
        if hw1 > 0.0005 {
            let w1_m = 1.0 - smoothstep(hw1 - aa_sym, hw1 + aa_sym, perp1);
            primary_mask = primary_mask.max(w1_m);
        }
    }

    // 4) Lower Diagonal Crescent-Hugging Wave (`u in [0.22757, 0.61857]`)
    let u2_min = 0.22757f32;
    let u2_max = 0.61857f32;
    if u >= u2_min && u <= u2_max {
        let span2 = u2_max - u2_min;
        let t2 = (u - u2_min) / span2;
        let v2 = 0.66699 - 0.36531 * t2 + 1.19197 * t2.powi(2) - 0.73156 * t2.powi(3);
        let dv2_du = (-0.36531 + 2.38394 * t2 - 2.19468 * t2.powi(2)) / span2;
        let perp2 = (v - v2).abs() / (1.0 + dv2_du.powi(2)).sqrt();
        let hw2 = (0.0118 + small_wave_floor) * (std::f32::consts::PI * t2).sin().max(0.0).powf(0.70);
        if hw2 > 0.0005 {
            let w2_m = 1.0 - smoothstep(hw2 - aa_sym, hw2 + aa_sym, perp2);
            secondary_mask = secondary_mask.max(w2_m);
        }
    }

    (clamp01(primary_mask), clamp01(secondary_mask))
}

#[derive(Clone, Copy)]
struct Group2PaletteSpec {
    file_stem: &'static str,
    bg_rgb: (f32, f32, f32),
    border_rgb: (f32, f32, f32),
    fg_main: (f32, f32, f32),
    fg_sub: (f32, f32, f32),
    bg_alpha: f32,
    sym_scale: f32,
}

const MAIN_DARK_ICON_SPEC: Group2PaletteSpec = Group2PaletteSpec {
    file_stem: "icon.ico",
    bg_rgb: (20.0, 29.0, 46.0),
    border_rgb: (64.0, 82.0, 112.0),
    fg_main: (248.0, 250.0, 252.0),
    fg_sub: (226.0, 232.0, 240.0),
    bg_alpha: 1.0,
    sym_scale: 1.36,
};

const GROUP2_UI_RETURN_DARK: Group2PaletteSpec = Group2PaletteSpec {
    file_stem: "4_ui_return_dark.ico",
    bg_rgb: (224.0, 234.0, 248.0),
    border_rgb: (100.0, 116.0, 139.0),
    fg_main: (10.0, 20.0, 44.0),
    fg_sub: (24.0, 40.0, 76.0),
    bg_alpha: 196.0 / 255.0,
    sym_scale: 1.36,
};

const GROUP2_UI_RETURN_LIGHT: Group2PaletteSpec = Group2PaletteSpec {
    file_stem: "5_ui_return_light.ico",
    bg_rgb: (224.0, 234.0, 248.0),
    border_rgb: (100.0, 116.0, 139.0),
    fg_main: (10.0, 20.0, 44.0),
    fg_sub: (24.0, 40.0, 76.0),
    bg_alpha: 196.0 / 255.0,
    sym_scale: 1.36,
};

/// 13 Audio Format Icons:
/// - 100% identical geometry & mask (`sample_lunar_core_symbol_masks_scaled`, `sym_scale = 1.40`):
///   Approved Crescent Moon + 1 Round Star Dot + 2-Line Diagonal Wave.
/// - Preserves the exact 13 color palettes (including WAV `#7A9299`).
const GROUP2_AUDIO_PALETTES: [Group2PaletteSpec; 13] = [
    Group2PaletteSpec { file_stem: "audio_mp3.ico",  bg_rgb: (181.0, 101.0, 152.0), border_rgb: (136.0, 68.0, 111.0),  fg_main: (255.0, 255.0, 255.0), fg_sub: (242.0, 246.0, 252.0), bg_alpha: 1.0, sym_scale: 1.40 },
    Group2PaletteSpec { file_stem: "audio_aac.ico",  bg_rgb: (220.0, 107.0, 90.0),  border_rgb: (168.0, 73.0, 58.0),   fg_main: (255.0, 255.0, 255.0), fg_sub: (242.0, 246.0, 252.0), bg_alpha: 1.0, sym_scale: 1.40 },
    Group2PaletteSpec { file_stem: "audio_flac.ico", bg_rgb: (70.0, 163.0, 126.0),  border_rgb: (46.0, 122.0, 92.0),   fg_main: (255.0, 255.0, 255.0), fg_sub: (242.0, 246.0, 252.0), bg_alpha: 1.0, sym_scale: 1.40 },
    Group2PaletteSpec { file_stem: "audio_wav.ico",  bg_rgb: (122.0, 146.0, 153.0), border_rgb: (86.0, 108.0, 115.0),  fg_main: (255.0, 255.0, 255.0), fg_sub: (242.0, 246.0, 252.0), bg_alpha: 1.0, sym_scale: 1.40 },
    Group2PaletteSpec { file_stem: "audio_aif.ico",  bg_rgb: (204.0, 168.0, 72.0),  border_rgb: (153.0, 123.0, 44.0),  fg_main: (255.0, 255.0, 255.0), fg_sub: (242.0, 246.0, 252.0), bg_alpha: 1.0, sym_scale: 1.40 },
    Group2PaletteSpec { file_stem: "audio_aiff.ico", bg_rgb: (196.0, 122.0, 59.0),  border_rgb: (145.0, 84.0, 32.0),   fg_main: (255.0, 255.0, 255.0), fg_sub: (242.0, 246.0, 252.0), bg_alpha: 1.0, sym_scale: 1.40 },
    Group2PaletteSpec { file_stem: "audio_wv.ico",   bg_rgb: (200.0, 100.0, 100.0), border_rgb: (148.0, 66.0, 66.0),   fg_main: (255.0, 255.0, 255.0), fg_sub: (242.0, 246.0, 252.0), bg_alpha: 1.0, sym_scale: 1.40 },
    Group2PaletteSpec { file_stem: "audio_ogg.ico",  bg_rgb: (104.0, 120.0, 180.0), border_rgb: (72.0, 86.0, 138.0),   fg_main: (255.0, 255.0, 255.0), fg_sub: (242.0, 246.0, 252.0), bg_alpha: 1.0, sym_scale: 1.40 },
    Group2PaletteSpec { file_stem: "audio_opus.ico", bg_rgb: (72.0, 178.0, 196.0),  border_rgb: (46.0, 131.0, 146.0),  fg_main: (255.0, 255.0, 255.0), fg_sub: (242.0, 246.0, 252.0), bg_alpha: 1.0, sym_scale: 1.40 },
    Group2PaletteSpec { file_stem: "audio_m4a.ico",  bg_rgb: (148.0, 120.0, 196.0), border_rgb: (106.0, 80.0, 150.0),  fg_main: (255.0, 255.0, 255.0), fg_sub: (242.0, 246.0, 252.0), bg_alpha: 1.0, sym_scale: 1.40 },
    Group2PaletteSpec { file_stem: "audio_mpc.ico",  bg_rgb: (132.0, 168.0, 88.0),  border_rgb: (94.0, 124.0, 58.0),   fg_main: (255.0, 255.0, 255.0), fg_sub: (242.0, 246.0, 252.0), bg_alpha: 1.0, sym_scale: 1.40 },
    Group2PaletteSpec { file_stem: "audio_ape.ico",  bg_rgb: (104.0, 120.0, 180.0), border_rgb: (72.0, 86.0, 138.0),   fg_main: (255.0, 255.0, 255.0), fg_sub: (242.0, 246.0, 252.0), bg_alpha: 1.0, sym_scale: 1.40 },
    Group2PaletteSpec { file_stem: "audio_wma.ico",  bg_rgb: (164.0, 130.0, 116.0), border_rgb: (120.0, 91.0, 79.0),   fg_main: (255.0, 255.0, 255.0), fg_sub: (242.0, 246.0, 252.0), bg_alpha: 1.0, sym_scale: 1.40 },
];

fn sample_flat_brand_icon_subpixel(pal: &Group2PaletteSpec, size: u32, u: f32, v: f32) -> (f32, f32, f32, f32) {
    let dist_box = sd_rounded_box(u, v, 0.50, 0.50, 0.263);
    let px_unit = 1.0f32 / (size.max(16) as f32);
    let aa_box = 0.55 * px_unit;
    let outer_cov = 1.0 - smoothstep(-aa_box, aa_box, dist_box);
    if outer_cov <= 0.0005 {
        return (0.0, 0.0, 0.0, 0.0);
    }

    let mut rgb = pal.bg_rgb;
    let border_w = (1.05 * px_unit).max(0.018);
    let border_mask = (1.0 - smoothstep(0.0, aa_box, (dist_box + border_w * 0.5).abs() - border_w * 0.5)) * 0.88;
    if border_mask > 0.001 {
        rgb = lerp_rgb(rgb, pal.border_rgb, border_mask);
    }

    let (prim_m, sec_m) = sample_lunar_core_symbol_masks_scaled(u, v, size, pal.sym_scale);
    if sec_m > 0.001 {
        rgb = lerp_rgb(rgb, pal.fg_sub, sec_m);
    }
    if prim_m > 0.001 {
        rgb = lerp_rgb(rgb, pal.fg_main, prim_m);
    }

    let sym_union = prim_m.max(sec_m);
    let interior_alpha = pal.bg_alpha + (1.0 - pal.bg_alpha) * sym_union;
    let final_alpha = clamp01(outer_cov * interior_alpha);
    (rgb.0, rgb.1, rgb.2, final_alpha)
}

fn sample_flat_brand_icon_ssaa_rgba(pal: &Group2PaletteSpec, size: u32, px: u32, py: u32) -> [u8; 4] {
    let ss: u32 = if size <= 64 { 4 } else { 2 };
    let inv_n = 1.0f32 / ((size * ss) as f32);
    let mut r_pre = 0.0f32;
    let mut g_pre = 0.0f32;
    let mut b_pre = 0.0f32;
    let mut a_sum = 0.0f32;
    let count = (ss * ss) as f32;

    for sy in 0..ss {
        let v = ((py * ss + sy) as f32 + 0.5) * inv_n;
        for sx in 0..ss {
            let u = ((px * ss + sx) as f32 + 0.5) * inv_n;
            let (r, g, b, a) = sample_flat_brand_icon_subpixel(pal, size, u, v);
            r_pre += r * a;
            g_pre += g * a;
            b_pre += b * a;
            a_sum += a;
        }
    }

    let a_avg = a_sum / count;
    if a_avg <= 1e-4 {
        return [0, 0, 0, 0];
    }
    let r_out = ((r_pre / count) / a_avg).round().clamp(0.0, 255.0) as u8;
    let g_out = ((g_pre / count) / a_avg).round().clamp(0.0, 255.0) as u8;
    let b_out = ((b_pre / count) / a_avg).round().clamp(0.0, 255.0) as u8;
    let a_out = (a_avg * 255.0).round().clamp(0.0, 255.0) as u8;
    [r_out, g_out, b_out, a_out]
}

fn build_dib_frame_from_palette(size: u32, pal: &Group2PaletteSpec) -> Vec<u8> {
    let xor_bytes_len = (size * size * 4) as usize;
    let mask_row_bytes = (((size + 31) / 32) * 4) as usize;
    let and_bytes_len = mask_row_bytes * (size as usize);
    let mut buf = Vec::with_capacity(40 + xor_bytes_len + and_bytes_len);

    buf.extend_from_slice(&40u32.to_le_bytes());
    buf.extend_from_slice(&(size as i32).to_le_bytes());
    buf.extend_from_slice(&((size * 2) as i32).to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&32u16.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&((xor_bytes_len + and_bytes_len) as u32).to_le_bytes());
    buf.extend_from_slice(&0i32.to_le_bytes());
    buf.extend_from_slice(&0i32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());

    let mut and_mask = vec![0u8; and_bytes_len];

    for row_bottom_up in 0..size {
        let py = (size - 1) - row_bottom_up;
        for px in 0..size {
            let [r, g, b, a] = sample_flat_brand_icon_ssaa_rgba(pal, size, px, py);
            buf.push(b);
            buf.push(g);
            buf.push(r);
            buf.push(a);
            if a < 16 {
                let byte_idx = (row_bottom_up as usize) * mask_row_bytes + (px as usize / 8);
                let bit_idx = 7 - (px % 8);
                and_mask[byte_idx] |= 1 << bit_idx;
            }
        }
    }

    buf.extend_from_slice(&and_mask);
    buf
}

fn encode_multi_size_ico_from_palette(pal: &Group2PaletteSpec) -> Vec<u8> {
    let sizes: [u32; 7] = [256, 128, 64, 48, 32, 24, 16];
    let frames: Vec<Vec<u8>> = sizes
        .iter()
        .map(|&sz| build_dib_frame_from_palette(sz, pal))
        .collect();

    let count = sizes.len() as u16;
    let mut out = Vec::new();
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&count.to_le_bytes());

    let mut offset = 6u32 + (count as u32) * 16;
    for (i, &sz) in sizes.iter().enumerate() {
        let frame_len = frames[i].len() as u32;
        let w_byte = if sz >= 256 { 0u8 } else { sz as u8 };
        let h_byte = if sz >= 256 { 0u8 } else { sz as u8 };
        out.push(w_byte);
        out.push(h_byte);
        out.push(0);
        out.push(0);
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&32u16.to_le_bytes());
        out.extend_from_slice(&frame_len.to_le_bytes());
        out.extend_from_slice(&offset.to_le_bytes());
        offset += frame_len;
    }

    for frame in frames {
        out.extend_from_slice(&frame);
    }
    out
}

fn build_group1_main_ico_bytes() -> Vec<u8> {
    encode_multi_size_ico_from_palette(&MAIN_DARK_ICON_SPEC)
}

fn build_group2_ico_bytes(pal: &Group2PaletteSpec) -> Vec<u8> {
    encode_multi_size_ico_from_palette(pal)
}

fn generate_static_brand_icons_offline() {
    let manifest_icons = PathBuf::from("src-tauri").join("icons");
    let target_dir = if manifest_icons.is_dir() {
        manifest_icons
    } else if Path::new("icons").is_dir() {
        PathBuf::from("icons")
    } else {
        return;
    };

    let main_ico_bytes = build_group1_main_ico_bytes();
    let ui_dark_bytes = build_group2_ico_bytes(&GROUP2_UI_RETURN_DARK);
    let ui_light_bytes = build_group2_ico_bytes(&GROUP2_UI_RETURN_LIGHT);

    let _ = write_file_if_changed(&target_dir.join("icon.svg"), DROEM_BRAND_ICON_SVG.as_bytes());
    let _ = write_file_if_changed(&target_dir.join("icon_light.svg"), DROEM_BRAND_ICON_LIGHT_SVG.as_bytes());
    let _ = write_file_if_changed(&target_dir.join("icon.ico"), &main_ico_bytes);
    let _ = write_file_if_changed(&target_dir.join("1_main_ref_clean.ico"), &main_ico_bytes);
    let _ = write_file_if_changed(&target_dir.join("icon_light.ico"), &ui_light_bytes);
    let _ = write_file_if_changed(&target_dir.join(GROUP2_UI_RETURN_DARK.file_stem), &ui_dark_bytes);
    let _ = write_file_if_changed(&target_dir.join(GROUP2_UI_RETURN_LIGHT.file_stem), &ui_light_bytes);

    for pal in &GROUP2_AUDIO_PALETTES {
        let fmt_ico = build_group2_ico_bytes(pal);
        let _ = write_file_if_changed(&target_dir.join(pal.file_stem), &fmt_ico);
    }
}

/// Part 1-1: 21 Locale Files (pure language names without `- ko` or `(ko)` codes, including `hi` & `is`)
fn default_locales_map() -> Vec<(&'static str, &'static str)> {
    vec![
        ("en", include_str!("../locales/en.json")),
        ("ko", include_str!("../locales/ko.json")),
        ("ja", include_str!("../locales/ja.json")),
        ("zh-CN", include_str!("../locales/zh-CN.json")),
        ("zh-TW", include_str!("../locales/zh-TW.json")),
        ("hi", include_str!("../locales/hi.json")),
        ("ar", include_str!("../locales/ar.json")),
        ("fr", include_str!("../locales/fr.json")),
        ("es", include_str!("../locales/es.json")),
        ("pt", include_str!("../locales/pt.json")),
        ("it", include_str!("../locales/it.json")),
        ("de", include_str!("../locales/de.json")),
        ("nl", include_str!("../locales/nl.json")),
        ("da", include_str!("../locales/da.json")),
        ("no", include_str!("../locales/no.json")),
        ("sv", include_str!("../locales/sv.json")),
        ("is", include_str!("../locales/is.json")),
        ("fi", include_str!("../locales/fi.json")),
        ("ru", include_str!("../locales/ru.json")),
        ("pl", include_str!("../locales/pl.json")),
        ("cs", include_str!("../locales/cs.json")),
    ]
}

/// Phase 34 P0: Read-only locale loader that never overwrites `locales/*.json` at runtime.
/// - Uses verified built-in locale resources (`default_locales_map()`) as the baseline.
/// - If a user-customized locale file exists in `locales/<code.json>` (or resource dir),
///   validates that it parses as a JSON object, contains no U+FFFD replacement characters,
///   and preserves the canonical `langName` (preventing previously CP949-corrupted files from poisoning the UI),
///   then merges user overrides on top without touching the filesystem.
fn ensure_and_read_locales_dir() -> HashMap<String, serde_json::Value> {
    let candidate_dirs = [
        PathBuf::from("locales"),
        PathBuf::from("src-tauri").join("locales"),
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|dir| dir.join("locales")))
            .unwrap_or_else(|| PathBuf::from("locales")),
    ];

    let mut result = HashMap::new();
    for (code, raw_json) in default_locales_map() {
        let builtin_val = serde_json::from_str::<serde_json::Value>(raw_json).ok();
        let expected_lang_name = builtin_val
            .as_ref()
            .and_then(|v| v.get("langName"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut resolved_val = builtin_val.clone();

        for dir in &candidate_dirs {
            let file_path = dir.join(format!("{}.json", code));
            if !file_path.is_file() {
                continue;
            }
            if let Ok(content) = fs::read_to_string(&file_path) {
                if content.contains('\u{FFFD}') {
                    continue;
                }
                if let Ok( serde_json::Value::Object(disk_map) ) = serde_json::from_str::<serde_json::Value>(&content) {
                    let disk_lang_name = disk_map
                        .get("langName")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if !expected_lang_name.is_empty() && disk_lang_name == expected_lang_name {
                        if let Some(serde_json::Value::Object(mut merged)) = builtin_val.clone() {
                            for (k, v) in disk_map {
                                merged.insert(k, v);
                            }
                            resolved_val = Some(serde_json::Value::Object(merged));
                        } else {
                            resolved_val = Some(serde_json::Value::Object(disk_map));
                        }
                        break;
                    }
                }
            }
        }

        if let Some(val) = resolved_val {
            result.insert(code.to_string(), val);
        }
    }
    result
}

#[tauri::command]
fn load_locales() -> Result<HashMap<String, serde_json::Value>, String> {
    Ok(ensure_and_read_locales_dir())
}

fn extract_cli_audio_paths(args: &[String]) -> Vec<String> {
    args.iter()
        .skip(1)
        .filter_map(|arg| {
            let cleaned = arg.trim_matches('"');
            if cleaned.starts_with('-') {
                return None;
            }
            let p = Path::new(cleaned);
            if p.exists() && (is_recognized_external_audio(p) || p.is_dir()) {
                Some(p.to_string_lossy().replace('\\', "/"))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.iter().any(|a| a == "--generate-static-assets") {
        generate_static_brand_icons_offline();
        return;
    }

    #[cfg(target_os = "windows")]
    {
        std::env::set_var(
            "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
            "--disable-features=HardwareMediaKeyHandling,MediaSessionService",
        );
        unsafe {
            let aumid = HSTRING::from("dr\u{00f8}m.MusicPlayer");
            let _ = SetCurrentProcessExplicitAppUserModelID(&aumid);
        }
    }

    let initial_args: Vec<String> = std::env::args().collect();
    let initial_files = extract_cli_audio_paths(&initial_args);

    tauri::Builder::default()
        .manage(Mutex::new(WindowPositionMemory::default()))
        .manage(PendingStartupFiles(Mutex::new(initial_files)))
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            let ext_paths = extract_cli_audio_paths(&argv);
            if !ext_paths.is_empty() {
                if let Some(pending_state) = app.try_state::<PendingStartupFiles>() {
                    if let Ok(mut guard) = pending_state.0.lock() {
                        for p in &ext_paths {
                            if !guard.contains(p) {
                                guard.push(p.clone());
                            }
                        }
                    }
                }
                let _ = app.emit("droem://external-files", ext_paths);
            }
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.set_shadow(false);
                let _ = w.unminimize();
                let _ = w.set_focus();
            }
        }))
        .on_window_event(|window, event| {
            if let WindowEvent::Moved(pos) = event {
                // Part 1-3: Ignore synchronous WM_MOVE events triggered during set_window_mode transitions
                // and use `try_lock()` so the Windows UI message loop can never deadlock!
                if IS_SWITCHING_MODE.load(Ordering::SeqCst) {
                    return;
                }
                if pos.x > -16000 && pos.y > -16000 {
                    if let Some(state) = window.try_state::<Mutex<WindowPositionMemory>>() {
                        if let Ok(mut mem) = state.try_lock() {
                            if mem.current_mode == "mini" {
                                mem.mini_pos = Some(*pos);
                            } else {
                                mem.shared_main_pos = Some(*pos);
                            }
                        }
                    }
                }
            }
        })
        .setup(|app| {
            std::thread::spawn(|| {
                prune_cover_cache_if_needed();
                let outputs = query_native_audio_outputs_sync();
                if let Ok(mut guard) = get_audio_cache_mutex().lock() {
                    if guard.is_none() {
                        *guard = Some(outputs);
                    }
                }
            });

            if let Some(main_win) = app.get_webview_window("main") {
                let _ = main_win.set_shadow(false);
                let _ = main_win.set_title("dr\u{00f8}m");
                if let Ok(pos) = main_win.outer_position() {
                    if let Some(state) = app.try_state::<Mutex<WindowPositionMemory>>() {
                        if let Ok(mut mem) = state.lock() {
                            mem.shared_main_pos = Some(pos);
                        }
                    }
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_system_drives,
            list_subdirectories,
            list_native_audio_outputs,
            scan_monitored_folders,
            smart_sync_library,
            pick_and_scan_folder,
            pick_and_scan_files,
            scan_dropped_paths,
            get_pending_startup_files,
            get_track_detail,
            check_file_exists,
            update_native_smtc,
            set_window_mode,
            set_mini_always_on_top,
            start_window_drag,
            start_native_file_drag,
            control_window,
            reveal_in_explorer,
            move_audio_file_to_trash,
            prepare_playable_audio_source,
            open_windows_default_apps_settings,
            open_external_url,
            open_third_party_notices,
            set_window_backdrop_effect,
            load_locales
        ])
        .run(tauri::generate_context!())
        .expect("error while running dr\u{00f8}m Tauri v2 application");
}
