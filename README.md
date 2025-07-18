# 🦀 Rustle-TUI a Memorable Password Generator

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg?style=for-the-badge)](https://www.gnu.org/licenses/agpl-3.0)
[![Built with Rust](https://img.shields.io/badge/Rust-CE412B?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=for-the-badge)](http://makeapullrequest.com)

A Rust-based TUI application that generates unique, random passwords in the format `Adjective+Symbol+Noun+Symbol+Number(0...9999)`.

---

## Preview

The application generates random passwords in a terminal user interface.

---

## ✨ Features

* Generates unique, random passwords.
* Stores password hashes to prevent duplicates.
* Uses a TUI for a user-friendly experience.
* Includes word lists for adjectives and nouns.
* **Security Note**: Passwords generated by this app follow a predictable pattern and are more vulnerable to dictionary attacks than completely random passwords. They are not recommended for mission-critical security applications.

---

## ⚙️ Installation

1.  **Clone the repository**

    First, clone the repository to your local machine.

    ```bash
    git clone https://github.com/chxrlie/rustle-tui.git
    cd rustle-tui
    ```

2.  **Build the application**

    Next, use Cargo to build the project in release mode. Make sure you have **Rust** and **Cargo** installed.

    ```bash
    cargo build --release
    ```

-----

## 🚀 Usage

```bash
./target/release/rustle-tui
```

Press `g` to generate a new password and `q` to quit.

-----

## 🏛️ Architecture

The application is composed of three primary components:

  * **PasswordGenerator:** Creates new passwords according to the specified format.
  * **StorageManager:** Persistently stores and checks for the existence of password hashes to prevent duplicates.
  * **UIRenderer / App:** Manages the application state, handles user input, and renders the TUI.

See [`Architecture.md`](https://www.google.com/search?q=Architecture.md) for more details.

-----

## 🗂️ Word Lists

The application uses word lists for adjectives and nouns. These lists are embedded directly into the application binary.
You can customise these text files before building the application to tailor the passwords to your preference.

  * [`src/adjectives.txt`](https://www.google.com/search?q=src/adjectives.txt)
  * [`src/nouns.txt`](https://www.google.com/search?q=src/nouns.txt)

-----

## 📦 Dependencies

  * `ratatui`
  * `rand`
  * `blake3`
  * `crossterm`
  * `dirs`
  * `bincode`
  * `serde`
  * `zxcvbn`

-----

## 🛠️ Tech Stack

  * Rust

-----

## 🤝 Contributing

Contributions are welcome\! Please open an issue or submit a pull request.

-----

## 📜 License

This project is licensed under the **GNU AGPLv3** licence.