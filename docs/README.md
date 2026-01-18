# Immortal Engine Documentation

Welcome to the Immortal Engine documentation. This visual prototyping system allows you to design applications by dragging components onto a canvas and connecting them.

## Table of Contents

1. [Getting Started](./getting-started.md)
2. [User Interface Guide](./ui-guide.md)
3. [Components Reference](./components.md)
4. [Keyboard Shortcuts](./shortcuts.md)
5. [CLI Reference](./cli.md)
6. [Architecture](./architecture.md)

## Quick Start

### Running the Visual Editor

```bash
cargo run --bin imortal-editor
```

### Using the CLI

```bash
# Create a new project
cargo run -p imortal_cli -- new my_project

# List available components
cargo run -p imortal_cli -- components

# Validate a project
cargo run -p imortal_cli -- validate my_project/my_project.imortal
```

## Features Overview

### Visual Editor
- **Drag & Drop Components** - Add components from the palette to the canvas
- **Visual Connections** - Connect components by clicking ports
- **Properties Panel** - Configure component fields and settings
- **Entity Field Display** - See entity fields directly on nodes
- **Undo/Redo** - Full history support with Ctrl+Z/Ctrl+Y

### Component Types
- **Authentication** - Login, Register, Logout, Session
- **Data** - Entity, Collection, Query
- **API** - REST Endpoint, GraphQL, WebSocket
- **Storage** - Database, Cache, File Storage
- **Logic** - Validator, Transformer, Condition

### Project Management
- Create, save, and load projects
- Export to JSON/TOML formats
- Validate project structure

## Current Version

- **Engine Version**: 0.1.0
- **IR Version**: 1.0.0
- **Component Version**: 1.0.0

## Author

**Stephen Kinuthia**  
📧 [kinuthiasteve098@gmail.com](mailto:kinuthiasteve098@gmail.com)  
🐙 [GitHub: 70-codes](https://github.com/70-codes)

## License

MIT License - Copyright © 2026 Stephen Kinuthia