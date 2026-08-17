# Calculator (Rust + egui) 🧮

A modern, fast, and lightweight cross-platform desktop calculator written in **Rust** using the **eframe / egui** GUI framework. Designed with a clean dark theme, keyboard navigation support, and clipboard integration.

---

## ✨ Features

* **Standard Arithmetic:** Addition (`+`), Subtraction (`-`), Multiplication (`*`), and Division (`/`).
* **Additional Functions:** Percentage (`%`), Sign toggling (`+/-`), and Clear (`C`).
* **Formatted Display:** Automatic thousands separator commas (e.g., `1,000,000`) for readability.
* **Repeat Calculation:** Pressing `=` continuously repeats the last operated value and operator.
* **Keyboard Shortcuts:** Full numpad and keyboard support for fast calculations.
* **Clipboard Integration:**
* `Ctrl + C` / `Cmd + C`: Copy the current value or result to the clipboard.
* `Ctrl + V` / `Cmd + V`: Paste numeric strings into the calculator.


* **Dark Mode UI:** Built-in clean dark visual appearance with responsive window resizing.

---

## ⌨️ Keyboard Shortcuts

| Key | Action |
| --- | --- |
| `0` - `9`, `.` | Numeric input / Decimal point |
| `+`, `-`, `*`, `/` | Arithmetic operators |
| `Enter` / `=` | Calculate result |
| `Backspace` | Delete last entered digit |
| `Escape` / `C` | Clear all (`C`) |
| `%` | Calculate percentage |
| `Ctrl + C` / `Cmd + C` | Copy result to clipboard |
| `Ctrl + V` / `Cmd + V` | Paste number from clipboard |

---

## 🛠️ Prerequisites & Dependencies

Before building, ensure you have the following installed on your system:

* **Rust toolchain** (1.70+ recommended): [Install Rust](https://www.rust-lang.org/tools/install)

### Cargo Dependencies (`Cargo.toml`)

Make sure your `Cargo.toml` includes the following crates:

```toml
[package]
name = "calculator"
version = "0.1.0"
edition = "2021"

[dependencies]
eframe = "0.29"
egui = "0.29"
arboard = "3.4"
image = "0.25"

```

> **Note:** Place an `icon.ico` file in the root directory (or update the path in `include_bytes!("../icon.ico")` accordingly) for application icon loading.

---

## 🚀 Building and Running

### 1. Clone the Repository

```bash
git clone https://github.com/rust-bd/Calculator.git
cd Calculator

```

### 2. Run in Development Mode

```bash
cargo run

```

### 3. Build for Production (Release)

```bash
cargo build --release

```

The compiled executable will be available in the `target/release/` directory.

---

## 📁 Project Structure

```text
├── src/
│   └── main.rs         # Application entry point and egui logic
├── icon.ico            # Windows icon asset
├── Cargo.toml          # Project configuration and dependencies
└── README.md           # Documentation

```

---

## 📜 License

This project is open-source and available under the [MIT License](https://www.google.com/search?q=LICENSE).
