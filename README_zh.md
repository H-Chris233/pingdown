# pingdown - 网络连接监控守护程序

## **[English](./README.md)**

基于 Rust 的智能网络监控工具，在连接丢失时触发系统关机。

**要想使用，你的设备必须已经安装*ping*.**

```bash
# 编译构建
cargo build --release
```

---

## 平台支持
✅ **稳定支持**:
- Windows XP/7/8/8.1/10/11/Windows server
- macOS 10.15+
- Linux (内核5.4+)

🔧 **真的在做了呜呜**:
- VMware ESXi (v7.0+)

---

## ⚙️ 配置体系

### 1. 优先级规则
```bash
# 启用 -r 时的层级 (配置文件优先)
config.json > 命令行参数 > 默认值

# 禁用 -r 时的层级 (纯命令行模式)
命令行参数 > 默认值
```

### 2. 配置文件规范
```json
// config.json
{
  "address": ["127.0.0.1", 
            "192.168.1.1:8443",
            "bing.com"
  ],
  "strict": false,
  "secs-for-normal-loop": 60,
  "secs-for-emergency-loop": 20,
  "times-for-emergency-loop": 3
}
```

### 3. 参数交互逻辑
```bash
# 场景1：纯命令行模式
pingdown -n 30 8.8.8.8  # 完全忽略配置文件

# 场景2：配置文件模式 (-r 必选)
pingdown -r  # 完全遵从 config.json
```

---

## 📦 操作指令集

| 参数 | 作用                   | 默认值          |
|------|-------------------------|------------------|
| `-r` | 启用配置文件            | 禁用             |
| `-s` | 严格模式 (全目标检测)   | 禁用              |
| `-n` | 设置常规检测间隔 (秒)   | 60              |
| `-e` | 设置应急检测间隔 (秒)   | 20              |
| `-t` | 设置熔断阈值 (次)       | 3                |

---

> 📌 **最佳实践**：生产环境建议通过 `-r` 固化配置，开发调试使用纯命令行模式

## **如果不能顺利关机，请以管理员权限运行！**

### 祝您使用愉快！ヾ(✿ﾟ▽ﾟ)ノ
