use crate::ai;
use crate::config::{get_all_foods, load_config, Config};
use chrono::{Local, Timelike};
use rand::seq::SliceRandom;

fn get_meal_time() -> &'static str {
    let hour = Local::now().hour();
    match hour {
        5..=10 => "早餐",
        11..=14 => "午餐",
        15..=17 => "下午茶",
        18..=21 => "晚餐",
        _ => "夜宵",
    }
}

fn build_prompt(config: &Config, mood: Option<&str>) -> String {
    let foods = get_all_foods();
    let meal_time = get_meal_time();

    let mut conditions = vec![format!("时间:{}", meal_time)];

    if config.preferences.spicy {
        conditions.push("爱辣".to_string());
    }
    if config.preferences.vegetarian {
        conditions.push("素食".to_string());
    }
    if let Some(m) = mood {
        conditions.push(format!("心情:{}", m));
    }

    format!(
        "{}。从[{}]选一个推荐，格式:食物名（10字内理由）",
        conditions.join(","),
        foods.join(",")
    )
}

pub async fn ai_recommend(mood: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let config = load_config();

    let api_key = match config.api.provider.as_str() {
        "zhipu" => &config.api.zhipu_key,
        _ => &config.api.deepseek_key,
    };

    if api_key.is_empty() {
        return Err(format!(
            "请先配置 {} API Key。运行 'ttchd config' 查看配置文件路径。",
            if config.api.provider == "zhipu" { "智谱AI" } else { "DeepSeek" }
        ).into());
    }

    let prompt = build_prompt(&config, mood);
    let result = ai::chat(&config.api.provider, api_key, &prompt).await?;

    Ok(result.trim().to_string())
}

pub fn random_recommend() -> String {
    let foods = get_all_foods();
    let mut rng = rand::thread_rng();
    foods
        .choose(&mut rng)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "火锅".to_string())
}
