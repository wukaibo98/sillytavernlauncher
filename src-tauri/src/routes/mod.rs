// Auto-generated axum route wrappers for #[allow(unused)] commands

use std::sync::Arc;
use axum::{Router, Json, routing::post};
use axum::extract::State;

use crate::state::AppState;
use crate::config::GithubTestResultItem;
use crate::config::DownloadSpeedResult;
use crate::presets::PresetFile;
use crate::presets::RegexFile;
use crate::types::*;

/// Simple base64 decode (no external crate needed for standard base64)
fn base64_decode(input: &str) -> Option<Vec<u8>> {
    use std::collections::HashMap;
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".chars().collect();
    let mut map = HashMap::new();
    for (i, &c) in chars.iter().enumerate() {
        map.insert(c, i as u8);
    }
    let mut result = Vec::new();
    let bytes: Vec<u8> = input.bytes().filter(|&b| b != b'\n' && b != b'\r').collect();
    let n = (bytes.len() / 4) * 4;
    if n == 0 { return Some(result); }
    for chunk in bytes[..n].chunks(4) {
        let a = *map.get(&(chunk[0] as char))?;
        let b = *map.get(&(chunk[1] as char))?;
        let c = *map.get(&(chunk[2] as char))?;
        let d = *map.get(&(chunk[3] as char))?;
        result.push((a.wrapping_sub(64) & 63) << 2 | (b.wrapping_sub(64) & 63) >> 4);
        if c < 64 {
            result.push((b.wrapping_sub(64) & 63) << 4 | (c.wrapping_sub(64) & 63) >> 2);
            if d < 64 {
                result.push((c.wrapping_sub(64) & 63) << 6 | (d.wrapping_sub(64) & 63));
            }
        }
    }
    Some(result)
}

/// Simple base64 encode
fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let n = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((n >> 18) & 63) as usize] as char);
        result.push(CHARS[((n >> 12) & 63) as usize] as char);
        result.push(if chunk.len() > 1 { CHARS[((n >> 6) & 63) as usize] as char } else { '=' });
        result.push(if chunk.len() > 2 { CHARS[(n & 63) as usize] as char } else { '=' });
    }
    result
}

// config::get_app_config
#[allow(unused)]
pub async fn axum_get_app_config(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::config::get_app_config(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// config::save_app_config
#[allow(unused)]
pub async fn axum_save_app_config(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let config: AppConfig = payload.get("config")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("Missing or invalid field: config"))?;
    crate::config::save_app_config(app.clone(), config).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// config::get_app_version
#[allow(unused)]
pub async fn axum_get_app_version(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::config::get_app_version(app.clone());
    Ok(Json(serde_json::json!({"result": result})))
}

// config::open_directory
#[allow(unused)]
pub async fn axum_open_directory(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let dir_type: String = payload.get("dir_type")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: dir_type"))?;
    let custom_path: Option<String> = payload.get("custom_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    crate::config::open_directory(app.clone(), dir_type, custom_path).map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// config::fetch_github_proxies
#[allow(unused)]
pub async fn axum_fetch_github_proxies(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let result = crate::config::fetch_github_proxies().await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// config::greet
#[allow(unused)]
pub async fn axum_greet(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let name: &str = payload.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Missing field: name"))?;
    let result = crate::config::greet(name);
    Ok(Json(serde_json::json!({"result": result})))
}

// config::get_system_cpu_cores
#[allow(unused)]
pub async fn axum_get_system_cpu_cores(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let result = crate::config::get_system_cpu_cores();
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// config::get_system_proxy_info
#[allow(unused)]
pub async fn axum_get_system_proxy_info(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let result = crate::config::get_system_proxy_info();
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// config::test_network_proxy
#[allow(unused)]
pub async fn axum_test_network_proxy(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let mode: String = payload.get("mode")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: mode"))?;
    let host: String = payload.get("host")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: host"))?;
    let port: u16 = payload.get("port")
        .and_then(|v| v.as_u64())
        .map(|v| v as u16)
        .ok_or_else(|| format!("Missing field: port"))?;
    let result = crate::config::test_network_proxy(mode, host, port).await?;
    Ok(Json(serde_json::json!({"result": result})))
}

// config::test_github_connection
#[allow(unused)]
pub async fn axum_test_github_connection(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let mode: String = payload.get("mode")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: mode"))?;
    let host: String = payload.get("host")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: host"))?;
    let port: u16 = payload.get("port")
        .and_then(|v| v.as_u64())
        .map(|v| v as u16)
        .ok_or_else(|| format!("Missing field: port"))?;
    let result = crate::config::test_github_connection(app.clone(), mode, host, port).await?;
    Ok(Json(serde_json::json!({"result": result})))
}

// config::test_github_multi
#[allow(unused)]
pub async fn axum_test_github_multi(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let mode: String = payload.get("mode")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: mode"))?;
    let host: String = payload.get("host")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: host"))?;
    let port: u16 = payload.get("port")
        .and_then(|v| v.as_u64())
        .map(|v| v as u16)
        .ok_or_else(|| format!("Missing field: port"))?;
    let include_api: bool = payload.get("include_api")
        .and_then(|v| v.as_bool())
        .ok_or_else(|| format!("Missing field: include_api"))?;
    let result = crate::config::test_github_multi(app.clone(), mode, host, port, include_api).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// config::test_download_speed
#[allow(unused)]
pub async fn axum_test_download_speed(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let mode: String = payload.get("mode")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: mode"))?;
    let host: String = payload.get("host")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: host"))?;
    let result = crate::config::test_download_speed(mode, host).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// chat::list_chats
#[allow(unused)]
pub async fn axum_list_chats(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::chat::list_chats(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// chat::read_chat
#[allow(unused)]
pub async fn axum_read_chat(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let char_folder: String = payload.get("char_folder")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: char_folder"))?;
    let file_name: String = payload.get("file_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: file_name"))?;
    let result = crate::chat::read_chat(app.clone(), char_folder, file_name).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// chat::delete_chats
#[allow(unused)]
pub async fn axum_delete_chats(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let items: Vec<ChatDeleteItem> = payload.get("items")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    crate::chat::delete_chats(app.clone(), items).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::get_bundled_tavern_path
#[allow(unused)]
pub async fn axum_get_bundled_tavern_path(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::sillytavern::get_bundled_tavern_path(app.clone())?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::fetch_sillytavern_releases
#[allow(unused)]
pub async fn axum_fetch_sillytavern_releases(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let result = crate::sillytavern::fetch_sillytavern_releases().await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::get_installed_sillytavern_versions
#[allow(unused)]
pub async fn axum_get_installed_sillytavern_versions(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::sillytavern::get_installed_sillytavern_versions(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::get_installed_versions_info
#[allow(unused)]
pub async fn axum_get_installed_versions_info(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::sillytavern::get_installed_versions_info(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::switch_sillytavern_version
#[allow(unused)]
pub async fn axum_switch_sillytavern_version(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let version: LocalTavernItem = payload.get("version")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("Missing or invalid field: version"))?;
    crate::sillytavern::switch_sillytavern_version(app.clone(), version).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::cancel_install
#[allow(unused)]
pub async fn axum_cancel_install(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    crate::sillytavern::cancel_install();
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::install_sillytavern_version
#[allow(unused)]
pub async fn axum_install_sillytavern_version(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let version: String = payload.get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: version"))?;
    let url: String = payload.get("url")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: url"))?;
    crate::sillytavern::install_sillytavern_version(app.clone(), version, url).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::check_local_tavern_dependencies
#[allow(unused)]
pub async fn axum_check_local_tavern_dependencies(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let path: String = payload.get("path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: path"))?;
    let result = crate::sillytavern::check_local_tavern_dependencies(app.clone(), path).await?;
    Ok(Json(serde_json::json!({"result": result})))
}

// sillytavern::install_sillytavern_dependencies
#[allow(unused)]
pub async fn axum_install_sillytavern_dependencies(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let version: String = payload.get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: version"))?;
    crate::sillytavern::install_sillytavern_dependencies(app.clone(), version).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::delete_sillytavern_version
#[allow(unused)]
pub async fn axum_delete_sillytavern_version(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let version: String = payload.get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: version"))?;
    crate::sillytavern::delete_sillytavern_version(app.clone(), version).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::check_sillytavern_empty
#[allow(unused)]
pub async fn axum_check_sillytavern_empty(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::sillytavern::check_sillytavern_empty(app.clone()).await?;
    Ok(Json(serde_json::json!({"result": result})))
}

// sillytavern::link_existing_sillytavern
#[allow(unused)]
pub async fn axum_link_existing_sillytavern(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let package_json_path: String = payload.get("package_json_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: package_json_path"))?;
    let result = crate::sillytavern::link_existing_sillytavern(app.clone(), package_json_path).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::get_tavern_version
#[allow(unused)]
pub async fn axum_get_tavern_version(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::sillytavern::get_tavern_version(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::read_sillytavern_config
#[allow(unused)]
pub async fn axum_read_sillytavern_config(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let version: String = payload.get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: version"))?;
    let result = crate::sillytavern::read_sillytavern_config(app.clone(), version).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::write_sillytavern_config
#[allow(unused)]
pub async fn axum_write_sillytavern_config(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let version: String = payload.get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: version"))?;
    let content: String = payload.get("content")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: content"))?;
    crate::sillytavern::write_sillytavern_config(app.clone(), version, content).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::get_sillytavern_config_path
#[allow(unused)]
pub async fn axum_get_sillytavern_config_path(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let version: String = payload.get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: version"))?;
    let result = crate::sillytavern::get_sillytavern_config_path(app.clone(), version)?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::get_sillytavern_config_options
#[allow(unused)]
pub async fn axum_get_sillytavern_config_options(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let version: String = payload.get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: version"))?;
    let result = crate::sillytavern::get_sillytavern_config_options(app.clone(), version).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::update_sillytavern_config_options
#[allow(unused)]
pub async fn axum_update_sillytavern_config_options(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let version: String = payload.get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: version"))?;
    let config: TavernConfigPayload = payload.get("config")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("Missing or invalid field: config"))?;
    let result = crate::sillytavern::update_sillytavern_config_options(app.clone(), version, config).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::open_sillytavern_config_file
#[allow(unused)]
pub async fn axum_open_sillytavern_config_file(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let version: String = payload.get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: version"))?;
    crate::sillytavern::open_sillytavern_config_file(app.clone(), version).map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::get_sillytavern_global_config_options
#[allow(unused)]
pub async fn axum_get_sillytavern_global_config_options(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::sillytavern::get_sillytavern_global_config_options(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::update_sillytavern_global_config_options
#[allow(unused)]
pub async fn axum_update_sillytavern_global_config_options(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let config: TavernConfigPayload = payload.get("config")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("Missing or invalid field: config"))?;
    let result = crate::sillytavern::update_sillytavern_global_config_options(app.clone(), config).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::open_sillytavern_global_config_file
#[allow(unused)]
pub async fn axum_open_sillytavern_global_config_file(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    crate::sillytavern::open_sillytavern_global_config_file(app.clone()).map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::list_config_migration_sources
#[allow(unused)]
pub async fn axum_list_config_migration_sources(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::sillytavern::list_config_migration_sources(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::migrate_tavern_config
#[allow(unused)]
pub async fn axum_migrate_tavern_config(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let source_path: String = payload.get("source_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: source_path"))?;
    crate::sillytavern::migrate_tavern_config(app.clone(), source_path).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::list_resource_migration_sources
#[allow(unused)]
pub async fn axum_list_resource_migration_sources(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::sillytavern::list_resource_migration_sources(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::scan_migration_conflicts
#[allow(unused)]
pub async fn axum_scan_migration_conflicts(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let source_paths: Vec<String> = payload.get("source_paths")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let source_displays: Vec<String> = payload.get("source_displays")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let exclude_categories_per_source: Option<Vec<Vec<String>>> = payload.get("exclude_categories_per_source")
        .and_then(|v| serde_json::from_value(v.clone()).ok());
    let priority_source_path: Option<String> = payload.get("priority_source_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let result = crate::sillytavern::scan_migration_conflicts(app.clone(), source_paths, source_displays, exclude_categories_per_source, priority_source_path).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::execute_resource_migration
#[allow(unused)]
pub async fn axum_execute_resource_migration(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let source_paths: Vec<String> = payload.get("source_paths")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let _source_displays: Vec<String> = payload.get("_source_displays")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let overwrite_rel_paths: Vec<String> = payload.get("overwrite_rel_paths")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let skip_rel_paths: Vec<String> = payload.get("skip_rel_paths")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let exclude_categories_per_source: Option<Vec<Vec<String>>> = payload.get("exclude_categories_per_source")
        .and_then(|v| serde_json::from_value(v.clone()).ok());
    let priority_source_path: Option<String> = payload.get("priority_source_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    crate::sillytavern::execute_resource_migration(app.clone(), source_paths, _source_displays, overwrite_rel_paths, skip_rel_paths, exclude_categories_per_source, priority_source_path).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::start_sillytavern
#[allow(unused)]
pub async fn axum_start_sillytavern(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    crate::sillytavern::start_sillytavern(app.clone()).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::stop_sillytavern
#[allow(unused)]
pub async fn axum_stop_sillytavern(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    crate::sillytavern::stop_sillytavern(app.clone()).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::check_sillytavern_status
#[allow(unused)]
pub async fn axum_check_sillytavern_status(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let result = crate::sillytavern::check_sillytavern_status().await?;
    Ok(Json(serde_json::json!({"result": result})))
}

// sillytavern::open_tavern_desktop_window
#[allow(unused)]
pub async fn axum_open_tavern_desktop_window(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let url: String = payload.get("url")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: url"))?;
    crate::sillytavern::open_tavern_desktop_window(app.clone(), url).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// sillytavern::get_local_ip_addresses
#[allow(unused)]
pub async fn axum_get_local_ip_addresses(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let result = crate::sillytavern::get_local_ip_addresses().await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::get_public_ip_addresses
#[allow(unused)]
pub async fn axum_get_public_ip_addresses(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let result = crate::sillytavern::get_public_ip_addresses().await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::check_network_availability
#[allow(unused)]
pub async fn axum_check_network_availability(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let ipv4_host: Option<String> = payload.get("ipv4_host")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let ipv6_host: Option<String> = payload.get("ipv6_host")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let port: u16 = payload.get("port")
        .and_then(|v| v.as_u64())
        .map(|v| v as u16)
        .ok_or_else(|| format!("Missing field: port"))?;
    let result = crate::sillytavern::check_network_availability(app.clone(), ipv4_host, ipv6_host, port).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// sillytavern::repair_missing_deps
#[allow(unused)]
pub async fn axum_repair_missing_deps(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let packages: Vec<String> = payload.get("packages")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let st_dir: String = payload.get("st_dir")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: st_dir"))?;
    crate::sillytavern::repair_missing_deps(app.clone(), packages, st_dir).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// node::check_nodejs
#[allow(unused)]
pub async fn axum_check_nodejs(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::node::check_nodejs(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// node::check_nodejs_both
#[allow(unused)]
pub async fn axum_check_nodejs_both(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::node::check_nodejs_both(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// node::check_npm
#[allow(unused)]
pub async fn axum_check_npm(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::node::check_npm(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// node::install_nodejs
#[allow(unused)]
pub async fn axum_install_nodejs(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    crate::node::install_nodejs(app.clone()).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// git::cancel_git_node_install
#[allow(unused)]
pub async fn axum_cancel_git_node_install(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    crate::git::cancel_git_node_install().map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// git::check_git_both
#[allow(unused)]
pub async fn axum_check_git_both(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::git::check_git_both(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// git::check_git
#[allow(unused)]
pub async fn axum_check_git(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::git::check_git(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// git::install_git
#[allow(unused)]
pub async fn axum_install_git(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    crate::git::install_git(app.clone()).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// character::list_character_card_pngs
#[allow(unused)]
pub async fn axum_list_character_card_pngs(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::character::list_character_card_pngs(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// character::read_character_card_png
#[allow(unused)]
pub async fn axum_read_character_card_png(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let file_name: String = payload.get("file_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: file_name"))?;
    let result = crate::character::read_character_card_png(app.clone(), file_name).await.map_err(|e| e.to_string())?;
    let b64 = base64_encode(&result);
    Ok(Json(serde_json::json!({"data": b64})))
}

// character::delete_character_cards
#[allow(unused)]
pub async fn axum_delete_character_cards(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let file_names: Vec<String> = payload.get("file_names")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    crate::character::delete_character_cards(app.clone(), file_names).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// character::import_character_card
#[allow(unused)]
pub async fn axum_import_character_card(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let source_path: String = payload.get("source_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: source_path"))?;
    crate::character::import_character_card(app.clone(), source_path).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// character::read_local_file
#[allow(unused)]
pub async fn axum_read_local_file(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let path: String = payload.get("path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: path"))?;
    let result = crate::character::read_local_file(path).await.map_err(|e| e.to_string())?;
    let b64 = base64_encode(&result);
    Ok(Json(serde_json::json!({"data": b64})))
}

// character::import_character_card_from_bytes
#[allow(unused)]
pub async fn axum_import_character_card_from_bytes(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    // Vec<u8> expects base64-encoded JSON string field: "bytes"
    let bytes: Vec<u8> = payload.get("bytes")
        .and_then(|v| v.as_str())
        .and_then(|s| base64_decode(s))
        .ok_or_else(|| format!("Missing or invalid base64 field: bytes"))?;
    let filename: String = payload.get("filename")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: filename"))?;
    crate::character::import_character_card_from_bytes(app.clone(), bytes, filename).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// character::list_bundled_presets
#[allow(unused)]
pub async fn axum_list_bundled_presets(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::character::list_bundled_presets(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// character::import_bundled_preset
#[allow(unused)]
pub async fn axum_import_bundled_preset(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let category: String = payload.get("category")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: category"))?;
    let file_name: String = payload.get("file_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: file_name"))?;
    crate::character::import_bundled_preset(app.clone(), category, file_name).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// character::list_bundled_cards
#[allow(unused)]
pub async fn axum_list_bundled_cards(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::character::list_bundled_cards(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// character::read_bundled_card_thumb
#[allow(unused)]
pub async fn axum_read_bundled_card_thumb(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let category: String = payload.get("category")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: category"))?;
    let file_name: String = payload.get("file_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: file_name"))?;
    let result = crate::character::read_bundled_card_thumb(app.clone(), category, file_name).await.map_err(|e| e.to_string())?;
    let b64 = base64_encode(&result);
    Ok(Json(serde_json::json!({"data": b64})))
}

// character::import_bundled_card
#[allow(unused)]
pub async fn axum_import_bundled_card(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let category: String = payload.get("category")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: category"))?;
    let file_name: String = payload.get("file_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: file_name"))?;
    crate::character::import_bundled_card(app.clone(), category, file_name).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// presets::list_presets
#[allow(unused)]
pub async fn axum_list_presets(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::presets::list_presets(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// presets::import_preset_file
#[allow(unused)]
pub async fn axum_import_preset_file(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let source_path: String = payload.get("source_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: source_path"))?;
    let category: String = payload.get("category")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: category"))?;
    let file_name: Option<String> = payload.get("file_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    crate::presets::import_preset_file(app.clone(), source_path, category, file_name).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// presets::read_preset_file
#[allow(unused)]
pub async fn axum_read_preset_file(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let category: String = payload.get("category")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: category"))?;
    let file_name: String = payload.get("file_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: file_name"))?;
    let result = crate::presets::read_preset_file(app.clone(), category, file_name).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// presets::delete_presets
#[allow(unused)]
pub async fn axum_delete_presets(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let items: Vec<PresetFile> = payload.get("items")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    crate::presets::delete_presets(app.clone(), items).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// presets::list_regex_scripts
#[allow(unused)]
pub async fn axum_list_regex_scripts(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::presets::list_regex_scripts(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// presets::import_regex_script
#[allow(unused)]
pub async fn axum_import_regex_script(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let source_path: String = payload.get("source_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: source_path"))?;
    let file_name: Option<String> = payload.get("file_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    crate::presets::import_regex_script(app.clone(), source_path, file_name).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// presets::delete_regex_scripts
#[allow(unused)]
pub async fn axum_delete_regex_scripts(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let file_names: Vec<String> = payload.get("file_names")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    crate::presets::delete_regex_scripts(app.clone(), file_names).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// worldinfo::list_world_infos
#[allow(unused)]
pub async fn axum_list_world_infos(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::worldinfo::list_world_infos(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// worldinfo::read_world_info
#[allow(unused)]
pub async fn axum_read_world_info(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let file_name: String = payload.get("file_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: file_name"))?;
    let result = crate::worldinfo::read_world_info(app.clone(), file_name).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// worldinfo::delete_world_infos
#[allow(unused)]
pub async fn axum_delete_world_infos(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let file_names: Vec<String> = payload.get("file_names")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    crate::worldinfo::delete_world_infos(app.clone(), file_names).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// worldinfo::import_world_info
#[allow(unused)]
pub async fn axum_import_world_info(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let source_path: String = payload.get("source_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: source_path"))?;
    crate::worldinfo::import_world_info(app.clone(), source_path).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// worldinfo::import_world_info_from_bytes
#[allow(unused)]
pub async fn axum_import_world_info_from_bytes(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    // Vec<u8> expects base64-encoded JSON string field: "bytes"
    let bytes: Vec<u8> = payload.get("bytes")
        .and_then(|v| v.as_str())
        .and_then(|s| base64_decode(s))
        .ok_or_else(|| format!("Missing or invalid base64 field: bytes"))?;
    let filename: String = payload.get("filename")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: filename"))?;
    crate::worldinfo::import_world_info_from_bytes(app.clone(), bytes, filename).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// secrets::read_secrets
#[allow(unused)]
pub async fn axum_read_secrets(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let result = crate::secrets::read_secrets(app.clone()).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// secrets::write_secrets
#[allow(unused)]
pub async fn axum_write_secrets(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let secrets: serde_json::Value = payload.get("secrets")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("Missing or invalid field: secrets"))?;
    crate::secrets::write_secrets(app.clone(), secrets).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// secrets::test_api_connection
#[allow(unused)]
pub async fn axum_test_api_connection(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let endpoint: String = payload.get("endpoint")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: endpoint"))?;
    let api_key: String = payload.get("api_key")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: api_key"))?;
    let result = crate::secrets::test_api_connection(endpoint, api_key).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// secrets::fetch_model_list
#[allow(unused)]
pub async fn axum_fetch_model_list(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let endpoint: String = payload.get("endpoint")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: endpoint"))?;
    let api_key: String = payload.get("api_key")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: api_key"))?;
    let result = crate::secrets::fetch_model_list(endpoint, api_key).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::tavern_register
#[allow(unused)]
pub async fn axum_tavern_register(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let username: String = payload.get("username")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: username"))?;
    let password: String = payload.get("password")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: password"))?;
    let email: String = payload.get("email")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: email"))?;
    let verification_code: String = payload.get("verification_code")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: verification_code"))?;
    let aff_code: Option<String> = payload.get("aff_code")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let result = crate::tavern_api::tavern_register(username, password, email, verification_code, aff_code).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::tavern_login
#[allow(unused)]
pub async fn axum_tavern_login(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let username: String = payload.get("username")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: username"))?;
    let password: String = payload.get("password")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: password"))?;
    let result = crate::tavern_api::tavern_login(username, password).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::tavern_send_verification_code
#[allow(unused)]
pub async fn axum_tavern_send_verification_code(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let email: String = payload.get("email")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: email"))?;
    let result = crate::tavern_api::tavern_send_verification_code(email).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::tavern_get_self
#[allow(unused)]
pub async fn axum_tavern_get_self(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let session_cookie: String = payload.get("session_cookie")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: session_cookie"))?;
    let user_id: String = payload.get("user_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: user_id"))?;
    let result = crate::tavern_api::tavern_get_self(session_cookie, user_id).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::tavern_get_tokens
#[allow(unused)]
pub async fn axum_tavern_get_tokens(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let session_cookie: String = payload.get("session_cookie")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: session_cookie"))?;
    let user_id: String = payload.get("user_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: user_id"))?;
    let page: Option<u32> = payload.get("page")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);
    let size: Option<u32> = payload.get("size")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);
    let result = crate::tavern_api::tavern_get_tokens(session_cookie, user_id, page, size).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::tavern_create_token
#[allow(unused)]
pub async fn axum_tavern_create_token(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let session_cookie: String = payload.get("session_cookie")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: session_cookie"))?;
    let user_id: String = payload.get("user_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: user_id"))?;
    let name: String = payload.get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: name"))?;
    let remain_quota: Option<f64> = payload.get("remain_quota")
        .and_then(|v| v.as_f64());
    let unlimited_quota: Option<bool> = payload.get("unlimited_quota")
        .and_then(|v| v.as_bool());
    let result = crate::tavern_api::tavern_create_token(session_cookie, user_id, name, remain_quota, unlimited_quota).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::tavern_delete_token
#[allow(unused)]
pub async fn axum_tavern_delete_token(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let session_cookie: String = payload.get("session_cookie")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: session_cookie"))?;
    let user_id: String = payload.get("user_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: user_id"))?;
    let key_id: u64 = payload.get("key_id")
        .and_then(|v| v.as_u64())
        .map(|v| v as u64)
        .ok_or_else(|| format!("Missing field: key_id"))?;
    let result = crate::tavern_api::tavern_delete_token(session_cookie, user_id, key_id).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::tavern_update_token_status
#[allow(unused)]
pub async fn axum_tavern_update_token_status(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let session_cookie: String = payload.get("session_cookie")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: session_cookie"))?;
    let user_id: String = payload.get("user_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: user_id"))?;
    let key_id: u64 = payload.get("key_id")
        .and_then(|v| v.as_u64())
        .map(|v| v as u64)
        .ok_or_else(|| format!("Missing field: key_id"))?;
    let status: u8 = payload.get("status")
        .and_then(|v| v.as_u64())
        .map(|v| v as u8)
        .ok_or_else(|| format!("Missing field: status"))?;
    let result = crate::tavern_api::tavern_update_token_status(session_cookie, user_id, key_id, status).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::tavern_get_token_by_name
#[allow(unused)]
pub async fn axum_tavern_get_token_by_name(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let session_cookie: String = payload.get("session_cookie")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: session_cookie"))?;
    let user_id: String = payload.get("user_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: user_id"))?;
    let name: String = payload.get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: name"))?;
    let result = crate::tavern_api::tavern_get_token_by_name(session_cookie, user_id, name).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::tavern_topup
#[allow(unused)]
pub async fn axum_tavern_topup(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let session_cookie: String = payload.get("session_cookie")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: session_cookie"))?;
    let user_id: String = payload.get("user_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: user_id"))?;
    let key: String = payload.get("key")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: key"))?;
    let result = crate::tavern_api::tavern_topup(session_cookie, user_id, key).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::tavern_calc_amount
#[allow(unused)]
pub async fn axum_tavern_calc_amount(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let session_cookie: String = payload.get("session_cookie")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: session_cookie"))?;
    let user_id: String = payload.get("user_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: user_id"))?;
    let amount: u64 = payload.get("amount")
        .and_then(|v| v.as_u64())
        .map(|v| v as u64)
        .ok_or_else(|| format!("Missing field: amount"))?;
    let result = crate::tavern_api::tavern_calc_amount(session_cookie, user_id, amount).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::tavern_create_payment
#[allow(unused)]
pub async fn axum_tavern_create_payment(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let session_cookie: String = payload.get("session_cookie")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: session_cookie"))?;
    let user_id: String = payload.get("user_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: user_id"))?;
    let amount: u64 = payload.get("amount")
        .and_then(|v| v.as_u64())
        .map(|v| v as u64)
        .ok_or_else(|| format!("Missing field: amount"))?;
    let payment_method: String = payload.get("payment_method")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: payment_method"))?;
    let result = crate::tavern_api::tavern_create_payment(session_cookie, user_id, amount, payment_method).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::tavern_get_token_detail
#[allow(unused)]
pub async fn axum_tavern_get_token_detail(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let session_cookie: String = payload.get("session_cookie")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: session_cookie"))?;
    let user_id: String = payload.get("user_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: user_id"))?;
    let key_id: u64 = payload.get("key_id")
        .and_then(|v| v.as_u64())
        .map(|v| v as u64)
        .ok_or_else(|| format!("Missing field: key_id"))?;
    let result = crate::tavern_api::tavern_get_token_detail(session_cookie, user_id, key_id).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// tavern_api::open_tavern_key_webview
#[allow(unused)]
pub async fn axum_open_tavern_key_webview(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    crate::tavern_api::open_tavern_key_webview().await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// tavern_api::tavern_get_models
#[allow(unused)]
pub async fn axum_tavern_get_models(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let session_cookie: String = payload.get("session_cookie")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: session_cookie"))?;
    let user_id: String = payload.get("user_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: user_id"))?;
    let result = crate::tavern_api::tavern_get_models(session_cookie, user_id).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// extensions::verify_extension_zip
#[allow(unused)]
pub async fn axum_verify_extension_zip(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let zip_path: String = payload.get("zip_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: zip_path"))?;
    let result = crate::extensions::verify_extension_zip(zip_path)?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// extensions::install_extension_zip
#[allow(unused)]
pub async fn axum_install_extension_zip(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let zip_path: String = payload.get("zip_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: zip_path"))?;
    let scope: String = payload.get("scope")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: scope"))?;
    let version: LocalTavernItem = payload.get("version")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("Missing or invalid field: version"))?;
    crate::extensions::install_extension_zip(app.clone(), zip_path, scope, version).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// extensions::get_extensions
#[allow(unused)]
pub async fn axum_get_extensions(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let version: LocalTavernItem = payload.get("version")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("Missing or invalid field: version"))?;
    let result = crate::extensions::get_extensions(app.clone(), version).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// extensions::toggle_extension_enable
#[allow(unused)]
pub async fn axum_toggle_extension_enable(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let _id: String = payload.get("_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: _id"))?;
    let enable: bool = payload.get("enable")
        .and_then(|v| v.as_bool())
        .ok_or_else(|| format!("Missing field: enable"))?;
    let dir_path: String = payload.get("dir_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: dir_path"))?;
    crate::extensions::toggle_extension_enable(app.clone(), _id, enable, dir_path).map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// extensions::delete_extension
#[allow(unused)]
pub async fn axum_delete_extension(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let _id: String = payload.get("_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: _id"))?;
    let dir_path: String = payload.get("dir_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: dir_path"))?;
    crate::extensions::delete_extension(app.clone(), _id, dir_path).map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// extensions::toggle_extension_auto_update
#[allow(unused)]
pub async fn axum_toggle_extension_auto_update(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let _id: String = payload.get("_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: _id"))?;
    let auto_update: bool = payload.get("auto_update")
        .and_then(|v| v.as_bool())
        .ok_or_else(|| format!("Missing field: auto_update"))?;
    let dir_path: String = payload.get("dir_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: dir_path"))?;
    crate::extensions::toggle_extension_auto_update(app.clone(), _id, auto_update, dir_path).map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// extensions::open_extension_folder
#[allow(unused)]
pub async fn axum_open_extension_folder(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let scope: String = payload.get("scope")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: scope"))?;
    let version: LocalTavernItem = payload.get("version")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("Missing or invalid field: version"))?;
    crate::extensions::open_extension_folder(app.clone(), scope, version).map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// extensions::open_specific_extension_folder
#[allow(unused)]
pub async fn axum_open_specific_extension_folder(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let dir_path: String = payload.get("dir_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: dir_path"))?;
    crate::extensions::open_specific_extension_folder(app.clone(), dir_path).map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// extensions::verify_extension_zip_from_bytes
#[allow(unused)]
pub async fn axum_verify_extension_zip_from_bytes(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    // Vec<u8> expects base64-encoded JSON string field: "bytes"
    let bytes: Vec<u8> = payload.get("bytes")
        .and_then(|v| v.as_str())
        .and_then(|s| base64_decode(s))
        .ok_or_else(|| format!("Missing or invalid base64 field: bytes"))?;
    let result = crate::extensions::verify_extension_zip_from_bytes(bytes).await?;
    Ok(Json(serde_json::to_value(result).map_err(|e| e.to_string())?))
}

// extensions::install_extension_zip_from_bytes
#[allow(unused)]
pub async fn axum_install_extension_zip_from_bytes(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    // Vec<u8> expects base64-encoded JSON string field: "bytes"
    let bytes: Vec<u8> = payload.get("bytes")
        .and_then(|v| v.as_str())
        .and_then(|s| base64_decode(s))
        .ok_or_else(|| format!("Missing or invalid base64 field: bytes"))?;
    let filename: String = payload.get("filename")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: filename"))?;
    let scope: String = payload.get("scope")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: scope"))?;
    let version: LocalTavernItem = payload.get("version")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("Missing or invalid field: version"))?;
    crate::extensions::install_extension_zip_from_bytes(app.clone(), bytes, filename, scope, version).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// extensions::install_extension_git
#[allow(unused)]
pub async fn axum_install_extension_git(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let url: String = payload.get("url")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: url"))?;
    let branch_opt: Option<String> = payload.get("branch_opt")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let scope: String = payload.get("scope")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: scope"))?;
    let version: LocalTavernItem = payload.get("version")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("Missing or invalid field: version"))?;
    crate::extensions::install_extension_git(app.clone(), url, branch_opt, scope, version).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// extensions::repair_extension_git
#[allow(unused)]
pub async fn axum_repair_extension_git(State(state): State<Arc<AppState>>, Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    let id: String = payload.get("id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: id"))?;
    let scope: String = payload.get("scope")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field: scope"))?;
    let dir_path: Option<String> = payload.get("dir_path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let result = crate::extensions::repair_extension_git(app.clone(), id, scope, dir_path).await?;
    Ok(Json(serde_json::json!({"result": result})))
}

// elevation::is_elevated
#[allow(unused)]
pub async fn axum_is_elevated(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    let result = crate::elevation::is_elevated();
    Ok(Json(serde_json::json!({"result": result})))
}

// elevation::elevate_process
#[allow(unused)]
pub async fn axum_elevate_process(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    crate::elevation::elevate_process(app.clone()).map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// finderst::cancel_scan_local_sillytavern
#[allow(unused)]
pub async fn axum_cancel_scan_local_sillytavern(Json(payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, String> {
    crate::finderst::cancel_scan_local_sillytavern().await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// finderst::scan_local_sillytavern
#[allow(unused)]
pub async fn axum_scan_local_sillytavern(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, String> {
    let app = state.handle.clone();
    crate::finderst::scan_local_sillytavern(app.clone()).await.map_err(|e| e.to_string())?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// ─── Router ───
pub fn api_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/config/get_app_config", post(axum_get_app_config))
        .route("/config/save_app_config", post(axum_save_app_config))
        .route("/config/get_app_version", post(axum_get_app_version))
        .route("/config/open_directory", post(axum_open_directory))
        .route("/config/fetch_github_proxies", post(axum_fetch_github_proxies))
        .route("/config/greet", post(axum_greet))
        .route("/config/get_system_cpu_cores", post(axum_get_system_cpu_cores))
        .route("/config/get_system_proxy_info", post(axum_get_system_proxy_info))
        .route("/config/test_network_proxy", post(axum_test_network_proxy))
        .route("/config/test_github_connection", post(axum_test_github_connection))
        .route("/config/test_github_multi", post(axum_test_github_multi))
        .route("/config/test_download_speed", post(axum_test_download_speed))
        .route("/chat/list_chats", post(axum_list_chats))
        .route("/chat/read_chat", post(axum_read_chat))
        .route("/chat/delete_chats", post(axum_delete_chats))
        .route("/sillytavern/get_bundled_tavern_path", post(axum_get_bundled_tavern_path))
        .route("/sillytavern/fetch_sillytavern_releases", post(axum_fetch_sillytavern_releases))
        .route("/sillytavern/get_installed_sillytavern_versions", post(axum_get_installed_sillytavern_versions))
        .route("/sillytavern/get_installed_versions_info", post(axum_get_installed_versions_info))
        .route("/sillytavern/switch_sillytavern_version", post(axum_switch_sillytavern_version))
        .route("/sillytavern/cancel_install", post(axum_cancel_install))
        .route("/sillytavern/install_sillytavern_version", post(axum_install_sillytavern_version))
        .route("/sillytavern/check_local_tavern_dependencies", post(axum_check_local_tavern_dependencies))
        .route("/sillytavern/install_sillytavern_dependencies", post(axum_install_sillytavern_dependencies))
        .route("/sillytavern/delete_sillytavern_version", post(axum_delete_sillytavern_version))
        .route("/sillytavern/check_sillytavern_empty", post(axum_check_sillytavern_empty))
        .route("/sillytavern/link_existing_sillytavern", post(axum_link_existing_sillytavern))
        .route("/sillytavern/get_tavern_version", post(axum_get_tavern_version))
        .route("/sillytavern/read_sillytavern_config", post(axum_read_sillytavern_config))
        .route("/sillytavern/write_sillytavern_config", post(axum_write_sillytavern_config))
        .route("/sillytavern/get_sillytavern_config_path", post(axum_get_sillytavern_config_path))
        .route("/sillytavern/get_sillytavern_config_options", post(axum_get_sillytavern_config_options))
        .route("/sillytavern/update_sillytavern_config_options", post(axum_update_sillytavern_config_options))
        .route("/sillytavern/open_sillytavern_config_file", post(axum_open_sillytavern_config_file))
        .route("/sillytavern/get_sillytavern_global_config_options", post(axum_get_sillytavern_global_config_options))
        .route("/sillytavern/update_sillytavern_global_config_options", post(axum_update_sillytavern_global_config_options))
        .route("/sillytavern/open_sillytavern_global_config_file", post(axum_open_sillytavern_global_config_file))
        .route("/sillytavern/list_config_migration_sources", post(axum_list_config_migration_sources))
        .route("/sillytavern/migrate_tavern_config", post(axum_migrate_tavern_config))
        .route("/sillytavern/list_resource_migration_sources", post(axum_list_resource_migration_sources))
        .route("/sillytavern/scan_migration_conflicts", post(axum_scan_migration_conflicts))
        .route("/sillytavern/execute_resource_migration", post(axum_execute_resource_migration))
        .route("/sillytavern/start_sillytavern", post(axum_start_sillytavern))
        .route("/sillytavern/stop_sillytavern", post(axum_stop_sillytavern))
        .route("/sillytavern/check_sillytavern_status", post(axum_check_sillytavern_status))
        .route("/sillytavern/open_tavern_desktop_window", post(axum_open_tavern_desktop_window))
        .route("/sillytavern/get_local_ip_addresses", post(axum_get_local_ip_addresses))
        .route("/sillytavern/get_public_ip_addresses", post(axum_get_public_ip_addresses))
        .route("/sillytavern/check_network_availability", post(axum_check_network_availability))
        .route("/sillytavern/repair_missing_deps", post(axum_repair_missing_deps))
        .route("/node/check_nodejs", post(axum_check_nodejs))
        .route("/node/check_nodejs_both", post(axum_check_nodejs_both))
        .route("/node/check_npm", post(axum_check_npm))
        .route("/node/install_nodejs", post(axum_install_nodejs))
        .route("/git/cancel_git_node_install", post(axum_cancel_git_node_install))
        .route("/git/check_git_both", post(axum_check_git_both))
        .route("/git/check_git", post(axum_check_git))
        .route("/git/install_git", post(axum_install_git))
        .route("/character/list_character_card_pngs", post(axum_list_character_card_pngs))
        .route("/character/read_character_card_png", post(axum_read_character_card_png))
        .route("/character/delete_character_cards", post(axum_delete_character_cards))
        .route("/character/import_character_card", post(axum_import_character_card))
        .route("/character/read_local_file", post(axum_read_local_file))
        .route("/character/import_character_card_from_bytes", post(axum_import_character_card_from_bytes))
        .route("/character/list_bundled_presets", post(axum_list_bundled_presets))
        .route("/character/import_bundled_preset", post(axum_import_bundled_preset))
        .route("/character/list_bundled_cards", post(axum_list_bundled_cards))
        .route("/character/read_bundled_card_thumb", post(axum_read_bundled_card_thumb))
        .route("/character/import_bundled_card", post(axum_import_bundled_card))
        .route("/presets/list_presets", post(axum_list_presets))
        .route("/presets/import_preset_file", post(axum_import_preset_file))
        .route("/presets/read_preset_file", post(axum_read_preset_file))
        .route("/presets/delete_presets", post(axum_delete_presets))
        .route("/presets/list_regex_scripts", post(axum_list_regex_scripts))
        .route("/presets/import_regex_script", post(axum_import_regex_script))
        .route("/presets/delete_regex_scripts", post(axum_delete_regex_scripts))
        .route("/worldinfo/list_world_infos", post(axum_list_world_infos))
        .route("/worldinfo/read_world_info", post(axum_read_world_info))
        .route("/worldinfo/delete_world_infos", post(axum_delete_world_infos))
        .route("/worldinfo/import_world_info", post(axum_import_world_info))
        .route("/worldinfo/import_world_info_from_bytes", post(axum_import_world_info_from_bytes))
        .route("/secrets/read_secrets", post(axum_read_secrets))
        .route("/secrets/write_secrets", post(axum_write_secrets))
        .route("/secrets/test_api_connection", post(axum_test_api_connection))
        .route("/secrets/fetch_model_list", post(axum_fetch_model_list))
        .route("/tavern_api/tavern_register", post(axum_tavern_register))
        .route("/tavern_api/tavern_login", post(axum_tavern_login))
        .route("/tavern_api/tavern_send_verification_code", post(axum_tavern_send_verification_code))
        .route("/tavern_api/tavern_get_self", post(axum_tavern_get_self))
        .route("/tavern_api/tavern_get_tokens", post(axum_tavern_get_tokens))
        .route("/tavern_api/tavern_create_token", post(axum_tavern_create_token))
        .route("/tavern_api/tavern_delete_token", post(axum_tavern_delete_token))
        .route("/tavern_api/tavern_update_token_status", post(axum_tavern_update_token_status))
        .route("/tavern_api/tavern_get_token_by_name", post(axum_tavern_get_token_by_name))
        .route("/tavern_api/tavern_topup", post(axum_tavern_topup))
        .route("/tavern_api/tavern_calc_amount", post(axum_tavern_calc_amount))
        .route("/tavern_api/tavern_create_payment", post(axum_tavern_create_payment))
        .route("/tavern_api/tavern_get_token_detail", post(axum_tavern_get_token_detail))
        .route("/tavern_api/open_tavern_key_webview", post(axum_open_tavern_key_webview))
        .route("/tavern_api/tavern_get_models", post(axum_tavern_get_models))
        .route("/extensions/verify_extension_zip", post(axum_verify_extension_zip))
        .route("/extensions/install_extension_zip", post(axum_install_extension_zip))
        .route("/extensions/get_extensions", post(axum_get_extensions))
        .route("/extensions/toggle_extension_enable", post(axum_toggle_extension_enable))
        .route("/extensions/delete_extension", post(axum_delete_extension))
        .route("/extensions/toggle_extension_auto_update", post(axum_toggle_extension_auto_update))
        .route("/extensions/open_extension_folder", post(axum_open_extension_folder))
        .route("/extensions/open_specific_extension_folder", post(axum_open_specific_extension_folder))
        .route("/extensions/verify_extension_zip_from_bytes", post(axum_verify_extension_zip_from_bytes))
        .route("/extensions/install_extension_zip_from_bytes", post(axum_install_extension_zip_from_bytes))
        .route("/extensions/install_extension_git", post(axum_install_extension_git))
        .route("/extensions/repair_extension_git", post(axum_repair_extension_git))
        .route("/elevation/is_elevated", post(axum_is_elevated))
        .route("/elevation/elevate_process", post(axum_elevate_process))
        .route("/finderst/cancel_scan_local_sillytavern", post(axum_cancel_scan_local_sillytavern))
        .route("/finderst/scan_local_sillytavern", post(axum_scan_local_sillytavern))
        .with_state(state)
}
