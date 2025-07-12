# Rust Command-Line To-Do Manager

A simple, persistent to-do list application for your terminal, written in Rust.

This tool allows you to add, list, and complete tasks. Your list is saved to a `todos.json` file in the same directory, so your data persists between sessions.

## Features

*   **Command-Line Interface**: Manage your to-do list directly from the terminal.
*   **Data Persistence**: Tasks are saved to a local `todos.json` file.
*   **Simple Commands**: Intuitive `add`, `list`, and `done` commands.
*   **Safe and Modern**: Built with Rust, using the popular `serde` crate for robust JSON handling.

## Prerequisites

To build and run this project, you need to have the Rust toolchain installed. If you don't have it, you can install it using `rustup`.

*   [Install Rust](https://www.rust-lang.org/tools/install)

This will provide you with `rustc` (the compiler) and `cargo` (the package manager and build tool).

## Getting Started

### 1. Create a New Project

First, create a new binary project using Cargo:

```bash
cargo new todo_cli
cd todo_cli
