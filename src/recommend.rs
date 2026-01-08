use crate::ai;
use crate::config::{load_config, Config};
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
    let meal_time = get_meal_time();
    let mut parts = vec![];

    // 时间
    parts.push(format!("时间:{}", meal_time));

    // 用户口味
    if config.user.spicy >= 4 {
        parts.push("爱吃辣".to_string());
    } else if config.user.spicy <= 1 {
        parts.push("不吃辣".to_string());
    }
    if config.user.sweet >= 4 {
        parts.push("喜甜".to_string());
    }
    if config.user.sour >= 4 {
        parts.push("喜酸".to_string());
    }
    if config.user.vegetarian {
        parts.push("素食".to_string());
    }
    if config.user.halal {
        parts.push("清真".to_string());
    }

    // 预算
    match config.user.budget.as_str() {
        "low" => parts.push("预算低".to_string()),
        "high" => parts.push("预算高".to_string()),
        _ => {}
    }

    // 偏好菜系
    if !config.user.cuisine.is_empty() {
        parts.push(format!("偏好:{}", config.user.cuisine.join("/")));
    }

    // 过敏
    if !config.user.allergies.is_empty() {
        parts.push(format!("过敏:{}", config.user.allergies.join("/")));
    }

    // 黑名单
    if !config.rules.blacklist.is_empty() {
        parts.push(format!("不吃:{}", config.rules.blacklist.join("/")));
    }

    // 最近吃过
    if !config.rules.recent.is_empty() {
        parts.push(format!("最近吃过:{}", config.rules.recent.join("/")));
    }

    // 心情
    if let Some(m) = mood {
        parts.push(format!("心情:{}", m));
    }

    // 自定义提示
    let custom = if config.rules.custom_prompt.is_empty() {
        String::new()
    } else {
        format!("。{}", config.rules.custom_prompt)
    };

    format!(
        "{}{}。推荐一道美食，格式:食物名（15字内理由）",
        parts.join("，"),
        custom
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
    let foods = vec![
        "火锅", "烧烤", "麻辣烫", "炸鸡", "披萨", "汉堡", "寿司", "拉面",
        "饺子", "小龙虾", "麻辣香锅", "黄焖鸡", "螺蛳粉", "酸菜鱼",
    ];
    let mut rng = rand::thread_rng();
    foods
        .choose(&mut rng)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "火锅".to_string())
}
