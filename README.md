# pingdown - Network Connectivity Monitor 

**[简体中文](./README_zh.md)**

A Rust-based utility that initiates system shutdown when critical network connections fail.

## Key Features

🛡️ **Fail-Safe Mechanism**  
Continuously monitors specified network targets and triggers emergency actions(shutdown) when connectivity is lost.

## Operational Modes

### Normal Monitoring Mode
- **Default check interval**: 60 seconds
- Verifies connectivity to all specified targets

### Emergency Mode Activation
- Automatically enters emergency mode upon first connection failure
- **Emergency check interval**: 20 seconds
- **Failure threshold**: 3 consecutive failures (60 seconds total)
- Initiates system shutdown when threshold is reached

## Platform Support
✅ **Stable Support**:
- Windows XP/7/8/8.1/10/11
- macOS 10.15+
- Linux (kernel 5.4+)

🔧 **Experimental Support**:
- VMware ESXi (v7.0+)

## Installation & Building
```bash
cargo build --release
```
## Configuration Options

### Basic Usage
```bash
pingdown [TARGETS...]
```
### Advanced Configuration

| Option | Description | Default |
|--------|-------------|---------|
| `-s`, `--strict` | Require **all** targets to be reachable | `false` |
| `-n` | Regular check interval (seconds) | `60` |
| `-e` | Emergency check interval (seconds) | `20` |
| `-t` | Emergency retry attempts | `3` |
| `-V`, `--version` | Show version information | - |
| `-h`, `--help` | Display full help documentation | - |

## Usage Example

```bash
pingdown -n 45 -e 15 -t 4 192.168.1.1 example.com
```

## Have a nice day. ヾ(✿ﾟ▽ﾟ)ノ
