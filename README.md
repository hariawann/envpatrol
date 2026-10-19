# 🛡️ EnvGuard

<p align="center">
  <strong>Blazing-fast CLI tool to keep your <code>.env</code> files in sync, detect leaked API secrets, and auto-generate safe templates.</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/envguard"><img src="https://img.shields.io/badge/crates.io-v0.1.0-orange.svg" alt="Crates.io"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg" alt="License"></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.80%2B-blueviolet.svg" alt="Rust 1.80+"></a>
  <img src="https://img.shields.io/badge/speed-%3C5ms-brightgreen.svg" alt="Speed">
</p>

---

## 💥 The Problem

In modern software development across teams:
1. **Out of Sync `.env`:** You add `STRIPE_WEBHOOK_SECRET` to your local `.env`, but forget to update `.env.example`. Your teammates pull the latest code and their app immediately crashes.
2. **Accidental Secret Leaks:** Someone clones `.env` to `.env.example` to commit it, but forgets to clear an active OpenAI or AWS token. Automated scrapers find it within seconds.
3. **Manual Tedium:** Manually copying keys and erasing secret values by hand is repetitive and error-prone.

---

## ✨ Features

- 🔍 **`envguard check`**: Compares `.env` against `.env.example` and highlights missing keys with colored diffs. Returns exit code `1` on failure (ideal for CI/CD and pre-commit hooks).
- 🚨 **`envguard scan`**: Scans files for exposed API keys (OpenAI, Stripe, AWS, GitHub PATs, JWTs, Private Keys, and suspicious passwords) using high-precision regex & entropy heuristics.
- 🪄 **`envguard generate`**: Automatically reads your active `.env` and generates a clean `.env.example`, masking all sensitive credentials while preserving comments and safe defaults (e.g. `PORT=3000`, `127.0.0.1`).

---

## ⚡ Quick Start

### 1. Installation

#### Via Cargo (Rust Users)
```bash
cargo install envguard
```

#### Via One-Line Script (Linux / macOS)
```bash
curl -sSL https://raw.githubusercontent.com/username/envguard/main/install.sh | bash
```

#### Via PowerShell (Windows)
```powershell
irm https://raw.githubusercontent.com/username/envguard/main/install.ps1 | iex
```

---

## 🚀 Usage

### 1. Check Synchronization
Verify that `.env` and `.env.example` have the exact same set of environment keys:
```bash
envguard check
```

Specify custom paths:
```bash
envguard check --env .env.local --example .env.template
```

### 2. Scan for Leaked Secrets
Ensure no active secrets or private keys were accidentally committed or included in your example templates:
```bash
envguard scan --path .env.example
```

### 3. Auto-Generate `.env.example`
Generate a sanitized `.env.example` template straight from your local `.env`:
```bash
envguard generate --force
```

---

## 🤖 CI / CD Integration

### GitHub Actions
Add this step to your `.github/workflows/ci.yml` to prevent broken PRs from missing environment variables:

```yaml
name: CI
on: [push, pull_request]

jobs:
  env-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install EnvGuard
        run: cargo install envguard
      - name: Verify Environment Sync
        run: envguard check
```

---

## 🛠️ Architecture

Built with ❤️ in **Rust** for maximum speed and zero dependencies:
- **CLI Parser:** [clap](https://crates.io/crates/clap) (v4)
- **Styling:** [colored](https://crates.io/crates/colored)
- **Pattern Matcher:** [regex](https://crates.io/crates/regex)

---

## 📄 License

Dual-licensed under either:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
