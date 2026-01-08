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

    /// 指定心情或需求，如 "想吃辣的"
    #[arg(short, long)]
    mood: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// 随机推荐（不使用 AI）
    Random,
    /// 初始化或查看配置文件
    Config,
    /// 添加自定义食物
    Add {
        /// 食物名称
        food: String,
    },
    /// 列出所有可选食物
    List,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Random) => {
            let food = recommend::random_recommend();
            println!("今天吃：{}", food);
        }
        Some(Commands::Config) => {
            match config::init_config() {
                Ok(path) => {
                    println!("请编辑配置文件设置 API Key 和偏好：");
                    println!("{}", path.display());
                }
                Err(e) => eprintln!("错误: {}", e),
            }
        }
        Some(Commands::Add { food }) => {
            if let Err(e) = config::add_food(&food) {
                eprintln!("添加失败: {}", e);
            }
        }
        Some(Commands::List) => {
            let foods = config::get_all_foods();
            println!("可选食物列表 ({} 种)：", foods.len());
            for (i, food) in foods.iter().enumerate() {
                print!("{:<8}", food);
                if (i + 1) % 5 == 0 {
                    println!();
                }
            }
            println!();
        }
        None => {
            // 默认：AI 智能推荐
            match recommend::ai_recommend(cli.mood.as_deref()).await {
                Ok(result) => println!("今天吃：{}", result),
                Err(e) => {
                    eprintln!("AI 推荐失败: {}", e);
                    eprintln!("使用随机推荐作为备选...");
                    let food = recommend::random_recommend();
                    println!("今天吃：{}", food);
                }
            }
        }
    }
}
