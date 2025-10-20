# pingdown - 网络连接监控守护程序

## [English](./README.md)

基于 Rust 的网络监控工具，按间隔持续 ping 多个目标。当连接持续丢失时，可触发系统关机（需要管理员权限）。

使用前请确保系统已安装 ping 命令。

```bash
# 编译构建
cargo build --release

# 基本运行（默认 60s 间隔）
cargo run -- 8.8.8.8
```

---

## 平台支持
✅ 稳定支持:
- Windows XP/7/8/8.1/10/11/Windows Server
- macOS 10.15+
- Linux (内核 5.4+)

🔧 即将支持:
- VMware ESXi (v7.0+)

---

## 命令行总览

以下为 `--help` 摘要（完整帮助可通过 scripts/update-help.sh 生成 docs/CLI_HELP.md）：

```
用法: pingdown [选项] [目标]...

参数:
  [目标]...  需要检测的 IP 或域名（可多个）

选项:
  -s, --strict                 严格模式（所有目标均需成功）
  -c, --config <文件>         从 JSON 配置文件读取参数（与本文档一致）
  -r, --read-json              兼容旧参数：读取当前目录 ./config.json（已不推荐）
  -n, --normal <秒>           常规检测间隔，默认 60
  -e, --emergency <秒>        应急重试间隔，默认 20
  -t, --tries <次数>          应急最大重试次数，默认 3
  -v, --verbose...             增加日志详细程度（-v、-vv）。与 --quiet / --status-only 冲突
  -q, --quiet                  仅输出汇总信息（无逐目标日志）
      --status-only            仅输出结构化汇总（无逐目标日志）
      --progress               等待间隔期间显示进度指示器
  -h, --help                   查看帮助
  -V, --version                查看版本
```

示例：

```bash
# 默认 60s 间隔，检查一个目标
pingdown 8.8.8.8

# 严格模式，两个目标，常规=30s，应急=10s，重试=5 次
pingdown -s -n 30 -e 10 -t 5 1.1.1.1 8.8.8.8

# 指定配置文件
pingdown --config ./config.json

# 仅汇总输出，并显示进度指示器
pingdown --status-only --progress 1.1.1.1
```

---

## 配置

### 优先级规则
```bash
# 使用 --config（配置优先）
config.json > 命令行参数 > 默认值

# 未使用 --config（纯命令行）
命令行参数 > 默认值
```

### JSON 规范
```json
{
  "address": ["127.0.0.1", "192.168.1.1:8443", "bing.com"],
  "strict": false,
  "secs-for-normal-loop": 60,
  "secs-for-emergency-loop": 20,
  "times-for-emergency-loop": 3,
  // 可选：也可在 JSON 中固化以下 UX 开关
  "quiet": false,
  "status_only": false,
  "progress": false,
  "verbose": 0
}
```

---

## 输出与报告

- 常规循环输出结构化行：`[NORMAL] OK | up: 2 | down: 0 | next: 60s`
- 进入应急循环时使用 `[EMERGENCY]` 前缀并清晰区分剩余重试与下次间隔
- 退出时（Ctrl-C）打印最终汇总，并将计数写入 `pingdown_runtime_info.txt`
- 添加 `--progress` 可在等待间隔显示简单进度指示

---

## 测试与性能建议

- 功能测试可缩短间隔：`-n 5 -e 2 -t 2`
- 通过系统 ping 命令检测，需确保命令可用且权限足够
- Unix 下关机需要管理员权限，若失败请以管理员身份运行

---

## 参数速览

- `-c, --config`：从 JSON 文件加载配置（默认禁用）
- `-r, --read-json`：兼容旧参数（默认禁用）
- `-s, --strict`：严格模式（默认 false）
- `-n, --normal`：常规检测间隔（秒，默认 60）
- `-e, --emergency`：应急重试间隔（秒，默认 20）
- `-t, --tries`：应急重试次数（默认 3）
- `-v, --verbose`：增加日志详细程度（默认 0）
- `-q, --quiet`：仅汇总输出（默认 false）
- `--status-only`：仅结构化汇总（默认 false）
- `--progress`：显示进度指示器（默认 false）

---

最佳实践：生产环境建议使用 `--config` 固化配置，本地调试通过命令行参数快速覆盖。

如关机失败，请以管理员权限运行。
