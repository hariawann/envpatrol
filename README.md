# envguard

A CLI utility for managing environment files. It checks synchronization between `.env` and `.env.example`, detects leaked credentials and API keys, and generates sanitized `.env.example` templates.

[![Crates.io](https://img.shields.io/crates/v/envguard.svg)](https://crates.io/crates/envguard)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

```
$ envguard check
🔍 Comparing environment files...
  Target:  .env
  Example: .env.example

❌ OUT OF SYNC DETECTED!
[+] Missing in example template: (2 keys)
    + REDIS_HOST
    + REDIS_PASSWORD
```

---

## Privacy & Security (Zero Telemetry)

EnvGuard is designed with a strict **local-only, zero-trust** philosophy:
- **100% Offline:** Runs entirely on your local machine.
- **Zero Network Calls:** Contains no HTTP client, telemetry, or remote analytics of any kind. You can inspect `Cargo.toml` to verify zero networking dependencies.
- **Memory-Only Processing:** Secret scans run strictly in-memory using local regex matching. No sensitive data is ever written to disk, sent over sockets, or exfiltrated.
- **Masked Previews:** Terminal output automatically masks detected tokens (e.g., `sk-...348`) to prevent shoulder surfing or terminal session recording leaks.

## Installation

### From crates.io
```bash
cargo install envguard
```

### Pre-built Binaries
Download the binary for your platform from [Releases](https://github.com/username/envguard/releases).

**Linux / macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/username/envguard/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/username/envguard/main/install.ps1 | iex
```

*Manual Windows Setup (Standalone .exe):*
1. Download `envguard-windows.zip` from [Releases](https://github.com/username/envguard/releases) and extract `envguard.exe`.
2. Place `envguard.exe` into a folder of your choice (e.g., `C:\tools`).
3. Add that folder to your User `PATH` (run once in PowerShell):
   ```powershell
   [System.Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\tools", "User")
   ```

### From Source
```bash
git clone https://github.com/username/envguard.git
cd envguard
cargo install --path .
```

---

## Commands

### `envguard check`
Compares keys between two environment files to ensure they remain synchronized.

```bash
envguard check
```

**Options:**
- `-e, --env <PATH>`: Path to local `.env` file (default: `.env`)
- `-x, --example <PATH>`: Path to example file (default: `.env.example`)

**Exit codes:**
- `0`: Files are synchronized.
- `1`: Missing keys detected or file read error.

---

### `envguard scan`
Inspects an environment file locally for plaintext credentials before committing to version control. Uses deterministic regular expressions matched in-memory—no data leaves your machine.

```bash
envguard scan --path .env.example
```

**Detects:**
- OpenAI API keys (`sk-...`, `sk-proj-...`)
- AWS access key IDs (`AKIA...`)
- Stripe keys (`sk_live_...`, `rk_live_...`)
- GitHub tokens (`ghp_...`, `github_pat_...`)
- Google Cloud / Firebase API keys (`AIza...`)
- Slack tokens (`xoxb-...`, `xoxp-...`)
- Private keys (`BEGIN PRIVATE KEY`)
- Suspicious high-length passwords in example files

---

### `envguard generate`
Generates a sanitized `.env.example` from an existing `.env` file. Retains comments, structure, and non-sensitive defaults (e.g., `PORT`, `localhost`, booleans), while replacing secrets with `your_<key>_here`.

```bash
envguard generate
```

**Options:**
- `-s, --source <PATH>`: Source `.env` file (default: `.env`)
- `-o, --output <PATH>`: Destination template (default: `.env.example`)
- `-f, --force`: Overwrite destination file if it already exists
- `--keep-defaults <BOOL>`: Retain harmless defaults like ports and hosts (default: `true`)

---

## CI / CD Integration

### GitHub Actions
To verify environment file synchronization on every pull request:

```yaml
name: CI

on:
  pull_request:
  push:
    branches: [main]

jobs:
  env-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install envguard
        run: cargo install envguard
      - name: Check env sync
        run: envguard check
      - name: Scan example for leaks
        run: envguard scan --path .env.example
```

---

## License

Dual-licensed under either:
- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
