# pingdown - Network Connectivity Monitoring Daemon

## [简体中文](./README_zh.md)

Pingdown is a small Rust-based network monitor that pings one or more targets at configurable intervals and can trigger a graceful system shutdown when connectivity is lost for sustained periods.

You must have the ping utility available on your system.

```bash
# Build from source
cargo build --release

# Run basic check (every 60s by default)
cargo run -- 8.8.8.8
```

---

## Platform Support
✅ Stable:
- Windows XP/7/8/8.1/10/11/Windows Server
- macOS 10.15+
- Linux (Kernel 5.4+)

🔧 Coming soon:
- VMware ESXi (v7.0+)

---

## CLI Overview

A quick excerpt of `--help` for convenience (see also docs/CLI_HELP.md which can be regenerated via scripts/update-help.sh):

```
Usage: pingdown [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...  Target IP address(es) or domain name(s) to check

Options:
  -s, --strict                 Enable strict verification mode (all targets must succeed)
  -c, --config <FILE>          Read configuration from a JSON file (same format as README)
  -r, --read-json              Deprecated: read ./config.json from current directory
  -n, --normal <SECS>          Interval (in seconds) between regular checks [default: 60]
  -e, --emergency <SECS>       Interval (in seconds) between emergency retries [default: 20]
  -t, --tries <NUM>            Maximum number of emergency retry attempts before shutdown [default: 3]
  -v, --verbose...             Increase output verbosity (-v, -vv). Conflicts with --quiet and --status-only
  -q, --quiet                  Suppress per-target messages; only summaries are printed
      --status-only            Only print structured status summaries (no per-target ping logs)
      --progress               Show a progress spinner while waiting between checks
  -h, --help                   Print help
  -V, --version                Print version
```

Examples:

```bash
# Check one target every 60s (default)
pingdown 8.8.8.8

# Strict mode, two targets, normal=30s, emergency=10s, attempts=5
pingdown -s -n 30 -e 10 -t 5 1.1.1.1 8.8.8.8

# Use configuration file
pingdown --config ./config.json

# Quiet summary-only output, show progress spinner
pingdown --status-only --progress 1.1.1.1
```

---

## Configuration

### Priority Rules
```bash
# When a config file is provided (via --config, --read-json, or PINGDOWN_CONFIG)
CLI arguments > config file > default values

# CLI-only mode
CLI arguments > default values
```

`PINGDOWN_CONFIG` can be used to point at a configuration file when `--config` is not supplied. CLI flags always take precedence over file values.

### JSON Specification
```json
{
  "address": ["127.0.0.1", "192.168.1.1:8443", "bing.com"],
  "strict": false,
  "secs-for-normal-loop": 60,
  "secs-for-emergency-loop": 20,
  "times-for-emergency-loop": 3,
  // Optional UX flags if you wish to persist them in JSON as well
  "quiet": false,
  "status_only": false,
  "progress": false,
  "verbose": 0
}
```

---

## Output and Reports

- Normal loop emits a structured summary like: `[NORMAL] OK | up: 2 | down: 0 | next: 60s`
- Emergency loop clearly marked as `[EMERGENCY]` and shows retries left and next delay
- A final summary is printed on exit (Ctrl-C), and counters are persisted to `pingdown_runtime_info.txt`
- Use `--progress` to show a simple spinner while waiting between checks

---

## Testing and Performance Notes

- For functional testing, shorten intervals: `-n 5 -e 2 -t 2`
- Pings are executed via the system `ping` command; ensure it is available and usable without privileges
- On Unix, shutdown requires sufficient privileges; run as an administrator if shutdown is desired

---

## Command Reference (quick table)

| Flag | Function | Default |
|------|----------|---------|
| `-c, --config` | Load configuration from JSON file | Disabled |
| `-r, --read-json` | Deprecated legacy config loader | Disabled |
| `-s, --strict` | All targets must succeed | false |
| `-n, --normal` | Normal check interval (sec) | 60 |
| `-e, --emergency` | Emergency check interval (sec) | 20 |
| `-t, --tries` | Emergency retry count | 3 |
| `-v, --verbose` | Increase log verbosity | 0 |
| `-q, --quiet` | Summaries only | false |
| `--status-only` | Summaries only (no per-target logs) | false |
| `--progress` | Show progress spinner | false |

---

## Development and testing

The repository includes comprehensive automated tests covering configuration parsing, monitoring state transitions, CLI behavior, and a smoke-level end-to-end path with stubbed ping execution.

Run the same checks locally that CI runs:

- Format check
  cargo fmt --all -- --check

- Lints (treat warnings as errors)
  cargo clippy --all-targets --all-features -- -D warnings

- Unit, integration, and e2e tests
  cargo test --all-targets

Notes:
- Integration tests use temporary directories and kill spawned CLI processes shortly after startup to verify configuration output without hanging.
- No network access is required; ping execution is stubbed/mocked in tests.
- On Unix you may need to ensure the `ping` command is present for manual runs.

---

Best practice: Pin production configs with `--config` and use CLI flags for local testing.

If shutdown fails, run with administrator privileges.
