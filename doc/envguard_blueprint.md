# 🛡️ EnvGuard — Blueprint & Strategi Proyek

> **Tagline:** Pelindung File `.env` Cepat, Cerdas, dan Ringan Berbasis Rust untuk Mencegah Kebocoran API Key dan Menjaga Sinkronisasi Tim.

---

## 📌 1. Latar Belakang & Problem Statement

Dalam pengembangan software modern (baik tim kecil maupun skala enterprise):
1. **Out-of-Sync `.env` vs `.env.example`**
   - Developer menambahkan environment variable baru di `.env` lokal, tetapi lupa memasukkannya ke `.env.example`.
   - Anggota tim lain atau pipeline CI/CD yang menarik kode terbaru langsung mengalami *runtime crash* karena variabel tersebut tidak ada.
2. **Kebocoran Kredensial / Secret Leak**
   - Developer tidak sengaja menyalin kredensial asli (API Key OpenAI, AWS Access Key, Stripe Secret, GitHub Token) ke dalam `.env.example` atau ter-commit ke Git.
   - Bot scanner publik di GitHub dapat mendeteksi dan mengeksploitasi token tersebut dalam hitungan detik.
3. **Pembuatan `.env.example` Manual yang Membosankan**
   - Developer harus menyalin file `.env`, lalu menghapus satu per satu nilainya secara manual agar aman di-commit.

---

## 🎯 2. Fitur Utama EnvGuard

### A. `envguard check` (Diff & Sync Engine)
- Membaca dan membandingkan file `.env` dan `.env.example`.
- Menampilkan status dengan pewarnaan terminal yang rapi:
  - Variabel yang ada di `.env` tetapi belum ada di `.env.example` (Missing in example).
  - Variabel yang ada di `.env.example` tetapi belum didefinisikan di `.env` lokal.
- Mengembalikan exit code:
  - `0`: Semua variabel sinkron.
  - `1`: Tidak sinkron (cocok untuk integrasi pre-commit hook dan GitHub Action).

### B. `envguard scan` (Secret Leak Detector)
- Memindai file `.env.example`, `.env`, atau file kodingan untuk mendeteksi apakah terdapat token rahasia asli.
- Deteksi pola (Pattern & Entropy Recognition):
  - OpenAI API Key (`sk-...`, `sk-proj-...`)
  - AWS Access Key (`AKIA...`)
  - Stripe Secret (`sk_live_...`, `rk_live_...`)
  - GitHub Token (`ghp_...`, `github_pat_...`)
  - Google Cloud / Firebase API Key
  - Private Key blocks (`BEGIN RSA PRIVATE KEY`, dll)
- Memberikan peringatan tegas dengan nomor baris jika ditemukan token asli.

### C. `envguard generate` (Auto-Generator `.env.example`)
- Membaca file `.env` yang ada.
- Menghasilkan file `.env.example` baru secara otomatis:
  - Mempertahankan struktur komentar dan urutan baris.
  - Mengosongkan atau mengganti nilai rahasia dengan placeholder yang aman (contoh: `your_openai_api_key_here` atau nilai default non-sensitif seperti `PORT=8080`).

---

## 🛠️ 3. Arsitektur & Tech Stack (Rust)

| Komponen | Pustaka / Crate | Alasan Pemilihan |
| :--- | :--- | :--- |
| **CLI Argument Parser** | `clap` (derive feature) | Standar industri di Rust, parsing argument & subcommands sangat rapi. |
| **Terminal Styling** | `colored` / `owo-colors` | Tampilan visual warna merah/hijau/kuning yang ramah mata. |
| **Pattern Matching** | `regex` | Pencocokan token sensitif dengan performa tinggi. |
| **Data Structure** | `std::collections::{HashMap, BTreeMap}` | Operasi set diffing instan (< 5ms). |
| **Testing** | Built-in Rust unit & integration test (`cargo test`) | Menjamin keandalan parser untuk berbagai format `.env`. |

---

## 🚀 4. Strategi Distribusi ke End-User

Rust mengompilasi kode menjadi **single binary executable** tanpa butuh runtime Node.js/Python di mesin pengguna.

### 1. Registry Resmi Rust (`crates.io`)
- Target: Pengguna Rust.
- Perintah instalasi:
  ```bash
  cargo install envguard
  ```

### 2. Standalone Binary di GitHub Releases + One-Line Installer
- Target: Semua developer (tidak perlu install Rust).
- Otomatisasi rilis cross-platform (Windows `.exe`, Linux, macOS Apple Silicon & Intel) via **GitHub Actions** / **`cargo-dist`**.
- Perintah instalasi satu baris:
  - **Linux / macOS:**
    ```bash
    curl -sSL https://raw.githubusercontent.com/<username>/envguard/main/install.sh | bash
    ```
  - **Windows (PowerShell):**
    ```powershell
    irm https://raw.githubusercontent.com/<username>/envguard/main/install.ps1 | iex
    ```

### 3. Package Manager Bawaan OS
- **Windows:** `winget install envguard` atau `scoop install envguard`
- **macOS:** `brew install envguard`

### 4. NPM Wrapper / `npx` (Viral Booster)
- Bungkus binary dengan paket npm tipis.
- Web developer bisa langsung menjalankan tanpa instalasi apa pun:
  ```bash
  npx envguard check
  ```

### 5. GitHub Action (CI/CD Pipeline)
- Disediakan action reusable di GitHub Marketplace:
  ```yaml
  - name: Check Environment Sync
    uses: <username>/envguard-action@v1
  ```

---

## 🗓️ 5. Rencana Langkah Implementasi

1. **Langkah 1: Setup Proyek & Parser `.env` Sederhana**
   - Menambahkan dependensi awal (`clap`, `colored`).
   - Membuat fungsi pembaca format `.env` menjadi Key-Value pair.
2. **Langkah 2: Membangun Subcommand `check`**
   - Logika diff antara 2 file `.env`.
   - Formatting output terminal berwarna.
3. **Langkah 3: Membangun Subcommand `generate`**
   - Logika sanitasi nilai secret menjadi format dummy / placeholder.
   - Menyimpan output ke `.env.example`.
4. **Langkah 4: Membangun Subcommand `scan`**
   - Rule matcher untuk berbagai pola API key / token populer.
5. **Langkah 5: Dokumentasi, Pengujian, & Rilis**
   - Menulis `README.md` yang menarik dengan badge & demo visual.
   - Konfigurasi `cargo-dist` / GitHub Actions untuk rilis multi-OS.
