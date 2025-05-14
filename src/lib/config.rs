use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

/// 配置结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub email: String,
    pub password: String,
    pub language: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            language: "zh".to_string(),
        }
    }
}

/// 获取配置文件路径
pub fn get_config_path() -> io::Result<PathBuf> {
    // 获取程序执行路径
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "ERROR : Unable to find executable directory")
    })?;
    
    Ok(exe_dir.join("config.json"))
}

/// 读取或创建配置文件
pub fn load_config() -> io::Result<Config> {
    let config_path = get_config_path()?;
    
    if config_path.exists() {
        println!("NOT FOUND {:?}", config_path);
        // 读取现有配置
        let mut file = fs::File::open(&config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        match serde_json::from_str(&contents) {
            Ok(config) => Ok(config),
            Err(e) => {
                eprintln!("ERROR CONFIG: {}", e);
                Err(io::Error::new(io::ErrorKind::InvalidData, e))
            }
        }
    } else {
        println!("未找到配置文件，创建默认配置: {:?}", config_path);
        // 创建默认配置
        let config = Config::default();
        save_config(&config)?;
        Ok(config)
    }
}

/// 保存配置到文件
pub fn save_config(config: &Config) -> io::Result<()> {
    let config_path = get_config_path()?;
    
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    let mut file = fs::File::create(&config_path)?;
    file.write_all(json.as_bytes())?;
    
    println!("SAVE CONFIG: {:?}", config_path);
    Ok(())
}