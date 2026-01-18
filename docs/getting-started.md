# Getting Started with Immortal Engine

This guide will help you get up and running with Immortal Engine, a visual prototyping system for building applications in Rust.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (1.70 or later) - [Install Rust](https://rustup.rs/)
- **Cargo** (comes with Rust)
- **Git** (for cloning the repository)

### Linux Dependencies

On Linux, you may need additional packages for the GUI:

```bash
# Ubuntu/Debian
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev

# Fedora
sudo dnf install libxcb-devel libxkbcommon-devel openssl-devel
```

## Installation

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/yourusername/imortal_engine.git
cd imortal_engine

# Build the project
cargo build --release

# Run tests to verify installation
cargo test --workspace
```

## Running the Visual Editor

Launch the visual editor with:

```bash
cargo run --bin imortal-editor
```

This opens the main editor window with three panels:
- **Left**: Component Palette
- **Center**: Canvas (workspace)
- **Right**: Properties Panel

## Your First Project

### Step 1: Add an Entity

1. In the **Components** panel (left), expand **Data**
2. Click on **Entity** to add it to the canvas
3. A new "Entity" node appears on the canvas

### Step 2: Configure the Entity

1. Click on the Entity node to select it
2. In the **Properties** panel (right):
   - Change the **Name** to "Todo"
   - Add fields using the **Add New Field** form:
     - `title` (String)
     - `completed` (Boolean)
     - `description` (Text)

### Step 3: Add a REST Endpoint

1. Expand **API** in the Components panel
2. Click **REST Endpoint** to add it
3. Position it next to your Todo entity

### Step 4: Connect Components

1. Click the **green port** (●) on the right side of the Todo entity
2. Click the **blue port** (●) on the left side of the REST Endpoint
3. A connection line appears, linking the two components

### Step 5: Save Your Project

1. Go to **File → Save As...**
2. Choose a location and filename (e.g., `todo_app.imortal`)
3. Click Save

## Understanding the Interface

### Canvas Navigation

| Action | How To |
|--------|--------|
| Pan canvas | Middle-mouse drag OR Shift + drag |
| Select node | Click on it |
| Multi-select | Shift + click |
| Move node | Drag selected node |
| Deselect all | Click empty canvas |

### Node Ports

Each node has connection ports:
- **🟢 Green (right side)** - Output port (data flows OUT)
- **🔵 Blue (left side)** - Input port (data flows IN)

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Z` | Undo |
| `Ctrl+Y` | Redo |
| `Delete` | Delete selected |
| `Escape` | Cancel current action |

## Project Structure

When you save a project, it creates a `.imortal` file containing:

```json
{
  "ir_version": "1.0.0",
  "format": "imortal",
  "project": {
    "meta": {
      "name": "todo_app",
      "version": "0.1.0"
    },
    "nodes": { ... },
    "edges": { ... }
  }
}
```

## Using the CLI

The CLI provides command-line access to Immortal Engine features:

```bash
# Create a new project
cargo run -p imortal_cli -- new my_project

# View available components
cargo run -p imortal_cli -- components

# Validate a project file
cargo run -p imortal_cli -- validate my_project/my_project.imortal

# Show engine info
cargo run -p imortal_cli -- info
```

## Next Steps

- Read the [UI Guide](./ui-guide.md) for detailed interface documentation
- Explore [Components Reference](./components.md) to learn about all available components
- Check [Keyboard Shortcuts](./shortcuts.md) for productivity tips
- Learn about the [Architecture](./architecture.md) if you want to contribute

## Troubleshooting

### Editor won't start

1. Ensure all dependencies are installed (see Prerequisites)
2. Try rebuilding: `cargo clean && cargo build`
3. Check for error messages in the terminal

### Can't create connections

1. Make sure you're clicking on the port circles (●), not the node body
2. Click the green port first (output), then the blue port (input)
3. Press Escape to cancel and try again

### Changes not saving

1. Use **File → Save** or **File → Save As...**
2. Check that you have write permissions to the directory
3. Look for error messages in the status bar

## Getting Help

- Check the [documentation](./README.md)
- Open an issue on GitHub
- Join our community discussions