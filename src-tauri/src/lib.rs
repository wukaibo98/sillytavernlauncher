// ─────────────────────────────────────────────────────────────
// 模块声明
// ─────────────────────────────────────────────────────────────
pub mod character;
pub mod chat;
pub mod config;
pub mod elevation;
pub mod extensions;
pub mod finderst;
pub mod git;
pub mod node;
pub mod presets;
pub mod sillytavern;
pub mod types;
pub mod utils;
pub mod worldinfo;
pub mod secrets;
pub mod tavern_api;
pub mod state;
pub mod events;
pub mod routes;

// ─────────────────────────────────────────────────────────────
// 顶层 use
// ─────────────────────────────────────────────────────────────
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    Router,
    routing::{get, post},
};
use axum::response::sse::{Event as AxumSseEvent, KeepAlive, Sse};
use axum::response::IntoResponse;
use futures_util::StreamExt;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use state::{AppHandle, AppState};
use utils::init_logger;

// ─────────────────────────────────────────────────────────────
// 命令注册宏
// ─────────────────────────────────────────────────────────────

/// 注册单个 command 为 axum POST 路由
macro_rules! route_cmd {
    ($router:expr, $path:expr, $handler:expr) => {
        $router.route($path, post($handler))
    };
}

// ─────────────────────────────────────────────────────────────
// 应用入口
// ─────────────────────────────────────────────────────────────
pub async fn run() {
    // 1. 确定 base_path
    let base_path = resolve_base_path();
    let handle = AppHandle::new(base_path.clone());

    // 2. 确保目录结构
    if !base_path.exists() {
        if let Err(e) = std::fs::create_dir_all(&base_path) {
            eprintln!("创建应用数据目录失败: {e}");
            std::process::exit(1);
        }
    }
    if let Err(e) = std::env::set_current_dir(&base_path) {
        eprintln!("设置工作目录失败: {e}");
    }
    if let Err(e) = handle.ensure_dirs() {
        eprintln!("确保目录结构失败: {e}");
    }

    // 3. 初始化日志 & SSE 事件广播
    init_logger(&base_path.join("data"));
    let _ = events::init_events();
    tracing::info!("应用启动（fnOS 移植版）");

    // 4. 初始化配置（自动配置内置酒馆和 Node.js）
    init_bundled_assets(&handle);

    // 5. 构建路由
    let app_state = AppState::new(handle);
    let app = build_router(app_state, &base_path);

    // 6. 启动 HTTP server
    let port = std::env::var("FNOS_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8010u16);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("服务端启动，监听: http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn resolve_base_path() -> PathBuf {
    // fnOS 环境变量优先级最高
    if let Ok(p) = std::env::var("TRIM_APPDEST") {
        return PathBuf::from(p);
    }
    if let Ok(p) = std::env::var("TRIM_PKGVAR") {
        return PathBuf::from(p);
    }

    // 开发模式：当前目录
    let exe_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    let dot_buf = PathBuf::from(".");
    let exe_dir = exe_path.parent().unwrap_or(&dot_buf);

    // 检查是否在 target/debug 或 target/release 下（开发构建）
    let components: Vec<String> = exe_dir
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_lowercase())
        .collect();
    let is_target_build = components
        .windows(2)
        .any(|pair| pair[0] == "target" && (pair[1] == "debug" || pair[1] == "release"));

    if is_target_build || cfg!(debug_assertions) {
        let mut cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        if cwd.ends_with("src-tauri") {
            cwd.pop();
        }
        return cwd;
    }

    exe_dir.to_path_buf()
}

fn init_bundled_assets(handle: &AppHandle) {
    let data_path = handle.data_dir();

    // Node.js 自动配置
    let node_exe = if cfg!(target_os = "windows") {
        data_path.join("node").join("node.exe")
    } else {
        data_path.join("node").join("bin").join("node")
    };
    if !node_exe.exists() {
        let resource_base = find_resource_dir();
        if let Ok(entries) = std::fs::read_dir(&resource_base) {
            for entry in entries.flatten() {
                let name_str = entry.file_name().to_string_lossy().to_string();
                if name_str.starts_with("node-v") {
                    let node_to = data_path.join("node");
                    let _ = std::fs::remove_dir_all(&node_to);
                    if utils::copy_dir_all(&entry.path(), &node_to).is_ok() {
                        tracing::info!("已复制内置 Node.js");
                    }
                    break;
                }
            }
        }
    }

    // 酒馆自动配置
    if let Ok(bundled_path) = sillytavern::get_bundled_tavern_path(handle.clone()) {
        let config = config::read_app_config_from_disk(handle);
        let needs_update = config.sillytavern.version.version.is_empty()
            || !config.initial_setup_completed
            || config.sillytavern.version.path != bundled_path;
        if needs_update {
            let mut config = config;
            config.sillytavern.version = crate::types::LocalTavernItem {
                version: "1.18.0".to_string(),
                path: bundled_path,
                has_node_modules: true,
            };
            config.initial_setup_completed = true;
            config.setup_checkpoint = Some("DONE".to_string());
            let _ = config::write_app_config_to_disk(handle, &config);
            tracing::info!("已自动配置内置酒馆 v1.18.0");
        }
    }
}

fn find_resource_dir() -> PathBuf {
    #[cfg(not(debug_assertions))]
    {
        // 生产环境：与可执行文件同目录
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("."))
    }
    #[cfg(debug_assertions)]
    {
        let mut p = std::env::current_dir().unwrap_or_default();
        if !p.ends_with("src-tauri") {
            p.push("src-tauri");
        }
        p.join("resources")
    }
}

// ─────────────────────────────────────────────────────────────
// 路由构建
// ─────────────────────────────────────────────────────────────
fn build_router(state: AppState, base_path: &PathBuf) -> Router {
    let api_routes = routes::api_router(Arc::new(state.clone()));

    // 静态文件服务（前端 SPA）
    let ui_dir = base_path.join("dist");
    let static_service = ServeDir::new(&ui_dir).fallback(
        ServeDir::new(&ui_dir).append_index_html_on_directories(true),
    );

    Router::new()
        .route("/api/events", get(sse_handler))
        .nest("/api", api_routes)
        .fallback_service(static_service)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}

/// SSE handler: streams events to frontend
async fn sse_handler() -> impl IntoResponse {
    let tx = events::init_events();
    let tx_clone = tx.clone();
    let rx = tx.subscribe();

    let stream = futures_util::stream::unfold(rx, move |mut rx| {
        let tx = tx_clone.clone();
        async move {
            match rx.recv().await {
                Ok(ev) => Some((
                    Ok::<_, std::convert::Infallible>(AxumSseEvent::default()
                        .event(ev.event)
                        .data(ev.data)),
                    rx,
                )),
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                    rx = tx.subscribe();
                    Some((
                        Ok(AxumSseEvent::default().data("reconnected")),
                        rx,
                    ))
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => None,
            }
        }
    });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}
