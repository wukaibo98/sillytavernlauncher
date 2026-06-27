// ─────────────────────────────────────────────────────────────
// AppHandle 垫片：替代 tauri::AppHandle，存储应用根路径和共享状态
// ─────────────────────────────────────────────────────────────
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::types::{InstallState, ProcessState};

/// 飞牛 fnOS 移植版的 AppHandle 垫片。
/// 原 tauri::AppHandle 提供 `path().app_data_dir()` 等路径解析，
/// 这里我们用 `base_path` 替代。
#[derive(Clone)]
pub struct AppHandle {
    /// 应用数据根目录（对应原 app.path().app_data_dir()）
    pub base_path: PathBuf,
    /// 进程状态
    pub process_state: ProcessState,
    /// 安装状态
    pub install_state: InstallState,
}

impl AppHandle {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            process_state: ProcessState {
                kill_tx: Arc::new(Mutex::new(None)),
                child_pid: Arc::new(Mutex::new(None)),
            },
            install_state: InstallState {
                cancel_flag: Arc::new(std::sync::atomic::AtomicBool::new(false)),
                git_child_pid: Arc::new(Mutex::new(None)),
            },
        }
    }

    /// 模拟 tauri::AppHandle.path().app_data_dir()
    pub fn app_data_dir(&self) -> PathBuf {
        self.base_path.clone()
    }

    /// 配置文件目录 = base_path/data
    pub fn data_dir(&self) -> PathBuf {
        self.base_path.join("data")
    }

    /// SillyTavern data 目录 = base_path/data/st_data
    pub fn st_data_dir(&self) -> PathBuf {
        self.data_dir().join("st_data")
    }

    /// 日志目录 = base_path/data/logs
    pub fn logs_dir(&self) -> PathBuf {
        self.data_dir().join("logs")
    }

    /// 确保数据和子目录存在
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.data_dir())?;
        std::fs::create_dir_all(self.st_data_dir())?;
        std::fs::create_dir_all(self.logs_dir())?;
        std::fs::create_dir_all(self.data_dir().join("sillytavern"))?;
        // 初始 config.json
        let config_path = self.data_dir().join("config.json");
        if !config_path.exists() {
            let default_config = crate::types::AppConfig::default();
            let s = serde_json::to_string_pretty(&default_config).unwrap();
            std::fs::write(&config_path, s)?;
        }
        // 初始 st_data/config.yaml
        let st_cfg = self.st_data_dir().join("config.yaml");
        if !st_cfg.exists() {
            // SillyTavern 默认配置模板在 sillytavern 模块中
            let _ = std::fs::write(
                &st_cfg,
                "listen: true\nwhitelist: []\nwhitelistMode: false\n",
            );
        }
        Ok(())
    }
}

/// Axum 共享状态
#[derive(Clone)]
pub struct AppState {
    pub handle: AppHandle,
}

impl AppState {
    pub fn new(handle: AppHandle) -> Self {
        Self { handle }
    }
}
