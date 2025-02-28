# pingdown - Network Connectivity Monitoring Daemon

## **[简体中文](./README_zh.md)**

A Rust-based intelligent network monitoring tool that triggers system shutdown when critical connections are lost.

```bash
# Build from source
cargo build --release
```

---

## Platform Support
✅ **Stable Support**:
- Windows XP/7/8/8.1/10/11/Windows Server
- macOS 10.15+
- Linux (Kernel 5.4+)

🔧 **Coming Soon**:
- VMware ESXi (v7.0+)

---

## ⚙️ Configuration System

### 1. Priority Rules
```bash
# When using -r (config file priority)
config.json > CLI arguments > Default values

# Without -r (CLI-only mode)
CLI arguments > Default values
```

### 2. Configuration File Specification
```json
{
  "address": [
    "127.0.0.1",
    "192.168.1.1:8443",
    "bing.com"
  ],
  "strict": false,
  "secs-for-normal-loop": 60,
  "secs-for-emergency-loop": 20,
  "times-for-emergency-loop": 3
}
```

### 3. Parameter Interaction
```bash
# Scenario 1: CLI-only mode
pingdown -n 30 8.8.8.8

# Scenario 2: Config file mode
pingdown -r
```

---

## 📦 Command Reference

| Flag | Function                     | Default Value |
|------|------------------------------|---------------|
| `-r` | Enable config file           | Disabled      |
| `-s` | Strict check mode            | false         |
| `-n` | Normal check interval (sec)  | 60            |
| `-e` | Emergency check interval (sec)| 20           |
| `-t` | Emergency failure threshold  | 3             |

---

> 📌 **Best Practice**: Use `-r` for production environments, CLI mode for development

## **If shutdown fails, run with administrator privileges!**

### Have a nice day! ヾ(✿ﾟ▽ﾟ)ノ
