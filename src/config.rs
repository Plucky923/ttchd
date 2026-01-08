use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub api: ApiConfig,
    pub preferences: Preferences,
    pub foods: FoodList,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiConfig {
    pub provider: String,  // "deepseek" 或 "zhipu"
    pub deepseek_key: String,
    pub zhipu_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Preferences {
    pub spicy: bool,
    pub vegetarian: bool,
    pub budget: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FoodList {
    pub favorites: Vec<String>,
    pub blacklist: Vec<String>,
    pub custom: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api: ApiConfig {
                provider: "deepseek".to_string(),
                deepseek_key: String::new(),
                zhipu_key: String::new(),
            },
            preferences: Preferences {
                spicy: false,
                vegetarian: false,
                budget: "medium".to_string(),
            },
            foods: FoodList {
                favorites: vec![
                    "火锅".to_string(),
                    "烧烤".to_string(),
                    "拉面".to_string(),
                ],
                blacklist: vec![],
                custom: vec![],
            },
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ttchd");
    config_dir.join("config.toml")
}

pub fn load_config() -> Config {
    let path = get_config_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        Config::default()
    }
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(config)?;
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

pub fn add_food(food: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config();
    if !config.foods.custom.contains(&food.to_string()) {
        config.foods.custom.push(food.to_string());
        save_config(&config)?;
        println!("已添加: {}", food);
    } else {
        println!("食物已存在: {}", food);
    }
    Ok(())
}

pub fn get_all_foods() -> Vec<String> {
    let config = load_config();
    let mut foods = get_default_foods();
    foods.extend(config.foods.custom.clone());
    foods.extend(config.foods.favorites.clone());
    foods.retain(|f| !config.foods.blacklist.contains(f));
    foods.sort();
    foods.dedup();
    foods
}

pub fn get_default_foods() -> Vec<String> {
    vec![
        "火锅", "烧烤", "麻辣烫", "炸鸡", "披萨", "汉堡", "寿司", "拉面", "饺子", "小龙虾",
        "麻辣香锅", "黄焖鸡", "螺蛳粉", "酸菜鱼", "烤鱼", "炒饭", "盖浇饭", "牛肉面", "米线",
        "煲仔饭", "烤肉", "日料", "韩餐", "泰餐", "越南粉", "沙拉", "三明治", "粥", "面包",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}
