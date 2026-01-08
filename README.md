# ttchd - 今天吃什么？

AI 智能美食推荐命令行工具。

## 安装

```bash
cargo build --release
sudo cp target/release/ttchd /usr/local/bin/
```

## 配置

配置文件：`~/.ttchd`（最大 10KB）

```bash
ttchd config  # 初始化配置
```

```toml
[api]
provider = "deepseek"  # 或 "zhipu"
deepseek_key = "sk-xxx"
zhipu_key = ""

[user]
name = ""
spicy = 3         # 0-5 辣度接受程度
sweet = 3         # 0-5 甜度喜好
sour = 3          # 0-5 酸度喜好
vegetarian = false
halal = false
allergies = []    # 过敏食材，如 ["花生", "海鲜"]
budget = "medium" # low/medium/high
cuisine = []      # 偏好菜系，如 ["川菜", "日料"]

[rules]
favorites = []
blacklist = []    # 永远不吃
recent = []       # 最近吃过（自动记录）
custom_prompt = "" # 自定义提示词
```

## 使用

```bash
# AI 推荐
ttchd

# 指定心情
ttchd -m "想吃辣的"
ttchd --mood "天冷想暖和的"

# 不想吃这个，换一个
ttchd skip "螺蛳粉"

# 加入黑名单（永远不推荐）
ttchd ban "香菜"

# 随机推荐（不用 AI）
ttchd random

# 查看配置路径
ttchd config
```

## 示例

```
$ ttchd
今天吃：烤冷面（夜市灵魂，咸香筋道）

$ ttchd skip "烤冷面"
今天吃：烤苕皮（焦糯裹满料，夜风中的慰藉）

$ ttchd -m "减肥中"
今天吃：鸡胸肉沙拉（低卡高蛋白，清爽无负担）
```

## License

MIT
