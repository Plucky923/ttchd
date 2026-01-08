use reqwest::Client;
use serde::{Deserialize, Serialize};

const DEEPSEEK_API_URL: &str = "https://api.deepseek.com/chat/completions";
const ZHIPU_API_URL: &str = "https://open.bigmodel.cn/api/paas/v4/chat/completions";

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

pub async fn chat(provider: &str, api_key: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let (api_url, model) = match provider {
        "zhipu" => (ZHIPU_API_URL, "GLM-4.5-Flash"),
        _ => (DEEPSEEK_API_URL, "deepseek-chat"),  // 默认使用 deepseek
    };

    let request = ChatRequest {
        model: model.to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
        max_tokens: 50,
        temperature: 1.0,
    };

    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("API 请求失败: {}", error_text).into());
    }

    let chat_response: ChatResponse = response.json().await?;

    if let Some(choice) = chat_response.choices.first() {
        Ok(choice.message.content.clone())
    } else {
        Err("没有收到回复".into())
    }
}
