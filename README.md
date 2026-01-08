# ttchd - 今天吃什么？

AI 智能美食推荐命令行工具，帮你解决每天「吃什么」的难题。

## 安装

```bash
cargo build --release
sudo cp target/release/ttchd /usr/local/bin/
```

## 配置

首次使用需配置 API Key：

```bash
ttchd config
```

编辑配置文件 `~/.config/ttchd/config.toml`：

```toml
[api]
provider = "deepseek"  # 可选: "deepseek" 或 "zhipu"
deepseek_key = "sk-xxx"
zhipu_key = ""

[preferences]
spicy = false      # 是否爱吃辣
vegetarian = false # 是否素食
budget = "medium"  # 预算: low/medium/high

[foods]
favorites = ["火锅", "烧烤", "拉面"]  # 喜欢的食物
blacklist = []                        # 不吃的食物
custom = []                           # 自定义食物
```

## 使用

```bash
# AI 智能推荐（根据当前时间自动判断餐点）
ttchd

# 指定心情/需求
ttchd --mood "想吃辣的"
ttchd --mood "天冷想吃热乎的"
ttchd -m "减肥中"

# 随机推荐（不调用 AI）
ttchd random

# 添加自定义食物
ttchd add "外婆红烧肉"

# 查看所有可选食物
ttchd list

# 查看帮助
ttchd --help
```

## 示例输出

```
$ ttchd
今天吃：烧烤（烟火气足，夜宵氛围首选）

$ ttchd --mood "心情不好"
今天吃：螺蛳粉（酸辣浓烈，一口驱散烦闷）

$ ttchd random
今天吃：火锅
```

## 支持的 AI 服务

| 服务商 | 模型 | 配置项 |
|--------|------|--------|
| DeepSeek | deepseek-chat | `deepseek_key` |
| 智谱AI | GLM-4.5-Flash | `zhipu_key` |

## License

MIT
