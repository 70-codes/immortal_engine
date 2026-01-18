<div align="center">

# 🔮 Immortal Engine

**A Visual Prototyping System for Building Applications in Rust**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

[Features](#-features) •
[Quick Start](#-quick-start) •
[Documentation](#-documentation) •
[Contributing](#-contributing)

---

</div>

## 🎯 What is Immortal Engine?

Immortal Engine is a **visual prototyping system** that allows you to design applications by dragging components onto a canvas and connecting them together. Instead of writing boilerplate code, you visually model your application's architecture and generate production-ready Rust code.

```
┌─────────────────────────────────────────────────────────────────────────┐
│  Components    │              Canvas                    │  Properties   │
│  ────────────  │  ┌────────────┐     ┌──────────────┐  │  ──────────── │
│  ▼ Auth        │  │ 📊 User    │────▶│ 🔌 REST API  │  │  Name: User   │
│    • Login     │  │ 🔑 id      │     └──────────────┘  │  Type: Entity │
│    • Register  │  │ email      │            │          │               │
│  ▼ Data        │  │ name       │            ▼          │  ▼ Fields     │
│    • Entity    │  └────────────┘     ┌──────────────┐  │    id: Uuid   │
│    • Query     │                     │ 💾 Database  │  │    email: Str │
│  ▼ API         │                     └──────────────┘  │    name: Str  │
│    • REST      │                                       │               │
└─────────────────────────────────────────────────────────────────────────┘
```

## ✨ Features

### Visual Editor
- **🎨 Drag & Drop Components** - Add entities, APIs, storage, and more from the palette
- **🔗 Visual Connections** - Click ports to connect components together
- **📝 Inline Editing** - Configure fields and properties directly in the UI
- **📊 Entity Field Display** - See your data model fields right on the canvas
- **↩️ Undo/Redo** - Full history support with Ctrl+Z / Ctrl+Y

### Component Library
- **🔐 Authentication** - Login, Register, Logout, Session management
- **📊 Data Modeling** - Entity, Collection, Query components
- **🔌 API Layer** - REST Endpoints, GraphQL, WebSocket
- **💾 Storage** - Database, Cache, File Storage
- **⚙️ Logic** - Validator, Transformer, Condition

### Developer Experience
- **🖥️ Native Desktop App** - Fast, responsive egui-based editor
- **⌨️ CLI Tools** - Create, validate, and generate from command line
- **📁 Project Files** - Human-readable JSON format, version control friendly
- **🧪 Comprehensive Tests** - 159+ tests covering core functionality

## 🚀 Quick Start

### Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Linux dependencies** (Ubuntu/Debian):
  ```bash
  sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev
  ```

### Installation

```bash
# Clone the repository
git clone https://github.com/70-codes/immortal_engine.git
cd immortal_engine

# Build the project
cargo build --release

# Run tests
cargo test --workspace
```

### Launch the Visual Editor

```bash
cargo run --bin imortal-editor
```

### Or Use the CLI

```bash
# Create a new project
cargo run -p imortal_cli -- new my_app

# List available components
cargo run -p imortal_cli -- components

# Validate a project
cargo run -p imortal_cli -- validate my_app/my_app.imortal
```

## 🎮 Basic Usage

### Creating a Todo App

1. **Launch the editor**
   ```bash
   cargo run --bin imortal-editor
   ```

2. **Add an Entity** - Click "Entity" in the Data section of the palette

3. **Configure fields** - In the Properties panel:
   - Rename to "Todo"
   - Add fields: `title` (String), `completed` (Boolean)

4. **Add a REST Endpoint** - Click "REST Endpoint" in the API section

5. **Connect them** - Click the green port on Todo → click the blue port on REST Endpoint

6. **Save your project** - File → Save As

## 📁 Project Structure

```
imortal_engine/
├── src/main.rs              # Visual editor entry point
├── crates/
│   ├── core/                # Shared types and traits
│   ├── ir/                  # Intermediate representation (graph model)
│   ├── components/          # Component definitions and registry
│   ├── codegen/             # Code generation engine
│   ├── ui/                  # Visual editor (egui)
│   └── cli/                 # Command-line interface
├── docs/                    # Documentation
└── target/                  # Build output
```

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Z` | Undo |
| `Ctrl+Y` | Redo |
| `Delete` | Delete selected |
| `Escape` | Cancel action |
| `Shift+Click` | Multi-select |
| `Middle-drag` | Pan canvas |

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [Getting Started](docs/getting-started.md) | Installation and first project tutorial |
| [UI Guide](docs/ui-guide.md) | Complete visual editor reference |
| [Components](docs/components.md) | All available components and their configuration |
| [Keyboard Shortcuts](docs/shortcuts.md) | Full shortcut reference |
| [CLI Reference](docs/cli.md) | Command-line interface documentation |
| [Architecture](docs/architecture.md) | Technical architecture for contributors |
| [Changelog](docs/CHANGELOG.md) | Version history and roadmap |

## 🛠️ Technology Stack

- **Language**: Rust
- **UI Framework**: [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe)
- **Serialization**: serde (JSON/TOML)
- **Code Generation**: quote, syn, proc-macro2
- **CLI**: clap

## 🗺️ Roadmap

### Current (v0.1.0)
- ✅ Visual editor with drag-and-drop
- ✅ Component palette with 16 built-in components
- ✅ Connection drawing between nodes
- ✅ Entity field display on canvas
- ✅ Undo/Redo system
- ✅ Project save/load
- ✅ CLI tools

### Upcoming
- 🔲 Complete code generation
- 🔲 Database migration generation
- 🔲 Copy/paste nodes
- 🔲 Minimap
- 🔲 Zoom with mouse wheel
- 🔲 Dark/light theme toggle
- 🔲 Custom component plugins

### Future
- 🔲 Web-based editor (Dioxus)
- 🔲 Real-time collaboration
- 🔲 Template marketplace
- 🔲 Embedded systems domain

## 🤝 Contributing

Contributions are welcome! Here's how to get started:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Make** your changes
4. **Run** tests (`cargo test --workspace`)
5. **Format** code (`cargo fmt`)
6. **Lint** (`cargo clippy`)
7. **Commit** (`git commit -m 'Add amazing feature'`)
8. **Push** (`git push origin feature/amazing-feature`)
9. **Open** a Pull Request

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [egui](https://github.com/emilk/egui) - Immediate mode GUI library
- [serde](https://serde.rs/) - Serialization framework
- [clap](https://github.com/clap-rs/clap) - CLI argument parser

---

<div align="center">

**[⬆ Back to Top](#-immortal-engine)**

Made with ❤️ in Rust by [Stephen Kinuthia](mailto:kinuthiasteve098@gmail.com)

Copyright © 2026 Stephen Kinuthia | [GitHub](https://github.com/70-codes/immortal_engine)

</div>