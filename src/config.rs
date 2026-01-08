use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const MAX_CONFIG_SIZE: u64 = 10 * 1024; // 10KB 限制

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub api: ApiConfig,
    pub user: UserProfile,
    pub rules: Rules,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiConfig {
    pub provider: String,
    pub deepseek_key: String,
    pub zhipu_key: String,
    pub openai_key: String,
    pub openai_endpoint: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub name: String,
    pub spicy: u8, // 0-5 辣度接受程度
    pub sweet: u8, // 0-5 甜度喜好
    pub sour: u8,  // 0-5 酸度喜好
    pub vegetarian: bool,
    pub halal: bool,
    pub allergies: Vec<String>,
    pub budget: String,
    pub cuisine: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rules {
    pub favorites: Vec<String>,
    pub blacklist: Vec<String>,
    pub recent: Vec<String>,
    pub custom_prompt: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api: ApiConfig {
                provider: "deepseek".to_string(),
                deepseek_key: String::new(),
                zhipu_key: String::new(),
                openai_key: String::new(),
                openai_endpoint: String::new(),
            },
            user: UserProfile {
                name: String::new(),
                spicy: 3,
                sweet: 3,
                sour: 3,
                vegetarian: false,
                halal: false,
                allergies: vec![],
                budget: "medium".to_string(),
                cuisine: vec![],
            },
            rules: Rules {
                favorites: vec![],
                blacklist: vec![],
                recent: vec![],
                custom_prompt: String::new(),
            },
        }
    }
}

pub fn get_config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".ttchd")
}

pub fn load_config() -> Config {
    let path = get_config_path();
    if path.exists() {
        // 检查文件大小
        if let Ok(meta) = fs::metadata(&path) {
            if meta.len() > MAX_CONFIG_SIZE {
                eprintln!("警告: 配置文件超过 {}KB 限制", MAX_CONFIG_SIZE / 1024);
                return Config::default();
            }
        }
        let content = fs::read_to_string(&path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        Config::default()
    }
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();
    let content = toml::to_string_pretty(config)?;

    // 检查大小限制
    if content.len() as u64 > MAX_CONFIG_SIZE {
        return Err(format!("配置文件超过 {}KB 限制", MAX_CONFIG_SIZE / 1024).into());
    }

    let mut file = fs::File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn init_config() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = get_config_path();
    if !path.exists() {
        save_config(&Config::default())?;
        println!("配置文件已创建: {}", path.display());
    } else {
        println!("配置文件已存在: {}", path.display());
    }
    Ok(path)
}

pub fn add_to_blacklist(food: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config();
    if !config.rules.blacklist.contains(&food.to_string()) {
        config.rules.blacklist.push(food.to_string());
        save_config(&config)?;
        println!("已加入黑名单: {}", food);
    }
    Ok(())
}

pub fn add_recent(food: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config();
    config.rules.recent.retain(|f| f != food);
    config.rules.recent.insert(0, food.to_string());
    config.rules.recent.truncate(5);
    save_config(&config)?;
    Ok(())
}

pub fn add_skip(food: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config();
    if !config.rules.recent.contains(&food.to_string()) {
        config.rules.recent.push(food.to_string());
        config.rules.recent.truncate(10);
        save_config(&config)?;
    }
    Ok(())
}
