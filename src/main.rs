mod ai;
mod config;
mod recommend;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ttchd")]
#[command(about = "今天吃什么？AI 智能美食推荐工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// 指定心情或需求
    #[arg(short, long)]
    mood: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// 随机推荐（不使用 AI）
    Random,
    /// 初始化或查看配置文件
    Config,
    /// 加入黑名单（不想再吃的食物）
    Ban {
        /// 食物名称
        food: String,
    },
    /// 不想吃这个，换一个推荐
    Skip {
        /// 不想吃的食物
        food: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Random) => {
            let food = recommend::random_recommend();
            println!("今天吃：{}", food);
        }
        Some(Commands::Config) => match config::init_config() {
            Ok(path) => {
                println!("配置文件路径: {}", path.display());
            }
            Err(e) => eprintln!("错误: {}", e),
        },
        Some(Commands::Ban { food }) => {
            if let Err(e) = config::add_to_blacklist(&food) {
                eprintln!("失败: {}", e);
            }
        }
        Some(Commands::Skip { food }) => {
            // 记录不想吃的，然后重新推荐
            let _ = config::add_skip(&food);
            match recommend::ai_recommend(cli.mood.as_deref()).await {
                Ok(result) => {
                    println!("今天吃：{}", result);
                    let food_name = result.split('（').next().unwrap_or(&result).trim();
                    let _ = config::add_recent(food_name);
                }
                Err(e) => {
                    eprintln!("AI 推荐失败: {}", e);
                    let food = recommend::random_recommend();
                    println!("今天吃：{}", food);
                }
            }
        }
        None => {
            match recommend::ai_recommend(cli.mood.as_deref()).await {
                Ok(result) => {
                    println!("今天吃：{}", result);
                    // 提取食物名并记录到最近吃过
                    let food_name = result.split('（').next().unwrap_or(&result).trim();
                    let _ = config::add_recent(food_name);
                }
                Err(e) => {
                    eprintln!("AI 推荐失败: {}", e);
                    eprintln!("使用随机推荐...");
                    let food = recommend::random_recommend();
                    println!("今天吃：{}", food);
                }
            }
        }
    }
}
