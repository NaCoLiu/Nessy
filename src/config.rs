use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::error::Error;
use once_cell::sync::Lazy;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub language: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            language: "en".to_string(),
        }
    }
}

// 单例模式，使用once_cell存储全局配置实例
static CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| {
    Arc::new(Mutex::new(match load_from_file() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            Config::default()
        }
    }))
});

// 返回配置文件路径
fn get_config_path() -> PathBuf {
    PathBuf::from("config.json")
}

// 从文件加载配置
fn load_from_file() -> Result<Config, Box<dyn Error>> {
    let path = get_config_path();
    if !path.exists() {
        return Ok(Config::default());
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let config: Config = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    Ok(config)
}

// 将配置保存到文件
fn save_to_file(config: &Config) -> Result<(), Box<dyn Error>> {
    let path = get_config_path();
    let dir = path.parent().unwrap_or_else(|| Path::new(""));
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    let json = serde_json::to_string_pretty(config)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    file.flush()?;
    
    Ok(())
}

// 公共API

// 获取当前配置的克隆
pub fn get() -> Config {
    CONFIG.lock().unwrap().clone()
}

// 更新特定字段
pub fn update<F>(updater: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&mut Config),
{
    let mut config = CONFIG.lock().unwrap();
    updater(&mut config);
    save_to_file(&config)?;
    Ok(())
}

// 获取语言设置
pub fn get_language() -> String {
    get().language
}

// 设置语言
pub fn set_language(lang: &str) -> Result<(), Box<dyn Error>> {
    update(|config| {
        config.language = lang.to_string();
    })
}

// 获取用户名
pub fn get_username() -> String {
    get().username
}

// 获取密码
pub fn get_password() -> String {
    get().password
}

// 设置用户凭据
pub fn set_credentials(username: &str, password: &str) -> Result<(), Box<dyn Error>> {
    update(|config| {
        config.username = username.to_string();
        config.password = password.to_string();
    })
}
