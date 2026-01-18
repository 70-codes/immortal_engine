# CLI Reference

The Immortal Engine CLI (`imortal`) provides command-line access to project management, code generation, and utility functions.

## Installation

The CLI is built as part of the workspace:

```bash
cargo build -p imortal_cli --release
```

The binary is located at `target/release/imortal`.

## Usage

```bash
imortal [OPTIONS] <COMMAND>
```

### Global Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Enable verbose output |
| `--config <PATH>` | `-c` | Configuration file path |
| `--help` | `-h` | Print help information |
| `--version` | `-V` | Print version information |

---

## Commands

### new

Create a new Immortal Engine project.

```bash
imortal new <NAME> [OPTIONS]
```

**Arguments:**
| Argument | Required | Description |
|----------|----------|-------------|
| `<NAME>` | Yes | Project name |

**Options:**
| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--path <PATH>` | `-p` | Current directory | Directory to create the project in |
| `--template <TEMPLATE>` | `-t` | "default" | Project template to use |

**Examples:**

```bash
# Create a new project in current directory
imortal new my_app

# Create project in specific directory
imortal new my_app --path /projects/

# Create project with template
imortal new my_app --template web_api
```

**Output:**
```
📦 Creating new project: my_app
   Template: default
   Location: my_app
✅ Project created successfully!

   To open the editor, run:
   imortal editor --project my_app/my_app.imortal
```

---

### editor

Open the visual editor.

```bash
imortal editor [OPTIONS]
```

**Options:**
| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--project <FILE>` | `-p` | None | Project file to open |
| `--port <PORT>` | | 3000 | Port for the editor server |

**Examples:**

```bash
# Open editor with empty project
imortal editor

# Open existing project
imortal editor --project my_app/my_app.imortal
```

**Note:** The visual editor is also available as a standalone binary:
```bash
cargo run --bin imortal-editor
```

---

### generate

Generate code from a project file.

```bash
imortal generate <PROJECT> [OPTIONS]
```

**Arguments:**
| Argument | Required | Description |
|----------|----------|-------------|
| `<PROJECT>` | Yes | Project file to generate from |

**Options:**
| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--output <DIR>` | `-o` | "generated" | Output directory |
| `--target <LANG>` | `-t` | "rust" | Target language |
| `--watch` | `-w` | false | Watch for changes and regenerate |

**Examples:**

```bash
# Generate Rust code
imortal generate my_app/my_app.imortal

# Generate to specific directory
imortal generate my_app/my_app.imortal --output src/generated

# Watch mode
imortal generate my_app/my_app.imortal --watch
```

**Output:**
```
⚙️  Generating code from: my_app/my_app.imortal
   Output: generated
   Target: rust
   Loaded 5 nodes and 3 edges
✅ Code generated successfully!
```

---

### validate

Validate a project file for errors and warnings.

```bash
imortal validate <PROJECT> [OPTIONS]
```

**Arguments:**
| Argument | Required | Description |
|----------|----------|-------------|
| `<PROJECT>` | Yes | Project file to validate |

**Options:**
| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--format <FORMAT>` | `-f` | "text" | Output format (text, json) |

**Examples:**

```bash
# Validate project
imortal validate my_app/my_app.imortal

# JSON output for CI/CD
imortal validate my_app/my_app.imortal --format json
```

**Output (text):**
```
🔍 Validating project: my_app/my_app.imortal

   Nodes: 5
   Edges: 3
   Groups: 0

✅ No issues found!
```

**Output (with errors):**
```
🔍 Validating project: my_app/my_app.imortal

   Nodes: 5
   Edges: 3
   Groups: 0

❌ Entity 'User' has no primary key field
⚠️  REST Endpoint 'GetUsers' has no connected entity

   1 error(s), 1 warning(s)
```

**Output (json):**
```json
{"errors": 1, "warnings": 1}
```

---

### components

List available components.

```bash
imortal components [OPTIONS]
```

**Options:**
| Option | Short | Description |
|--------|-------|-------------|
| `--category <CAT>` | `-C` | Filter by category (auth, data, api, storage, logic) |
| `--search <QUERY>` | `-s` | Search query |

**Examples:**

```bash
# List all components
imortal components

# Filter by category
imortal components --category auth

# Search components
imortal components --search "endpoint"
```

**Output:**
```
📦 Available Components

🔐 Authentication
   🔐 Login - User login component with email and password authentication
      ID: auth.login
   🚪 Logout - User logout component for ending sessions
      ID: auth.logout
   📝 Register - User registration component for creating new accounts
      ID: auth.register
   🎫 Session - Session management component
      ID: auth.session

📊 Data
   📊 Entity - Define a data model with fields and relationships
      ID: data.entity
   🔍 Query - Build and execute database queries
      ID: data.query
   📚 Collection - A queryable collection of entities
      ID: data.collection

🔌 API
   🔌 REST Endpoint - Define a RESTful API endpoint
      ID: api.rest
   ◈ GraphQL - Define a GraphQL API
      ID: api.graphql
   🔄 WebSocket - WebSocket connection for real-time communication
      ID: api.websocket

💾 Storage
   💾 Database - Database connection and configuration
      ID: storage.database
   ⚡ Cache - In-memory or distributed caching
      ID: storage.cache
   📁 File Storage - File and blob storage
      ID: storage.files

⚙ Logic
   ✅ Validator - Validate data against configurable rules
      ID: logic.validator
   🔄 Transformer - Transform and map data between formats
      ID: logic.transformer
   🔀 Condition - Conditional branching
      ID: logic.condition

Total: 16 components
```

---

### export

Export a project to different formats.

```bash
imortal export <PROJECT> <OUTPUT> [OPTIONS]
```

**Arguments:**
| Argument | Required | Description |
|----------|----------|-------------|
| `<PROJECT>` | Yes | Project file to export |
| `<OUTPUT>` | Yes | Output file path |

**Options:**
| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--format <FMT>` | `-f` | "json" | Export format (json, json-compact, toml) |

**Examples:**

```bash
# Export as formatted JSON
imortal export my_app/my_app.imortal export.json

# Export as compact JSON
imortal export my_app/my_app.imortal export.json --format json-compact

# Export as TOML
imortal export my_app/my_app.imortal export.toml --format toml
```

**Output:**
```
📤 Exporting project: my_app/my_app.imortal
   Output: export.json
   Format: json
✅ Exported successfully!
```

---

### import

Import a project from external formats.

```bash
imortal import <INPUT> [OPTIONS]
```

**Arguments:**
| Argument | Required | Description |
|----------|----------|-------------|
| `<INPUT>` | Yes | Input file to import |

**Options:**
| Option | Short | Description |
|--------|-------|-------------|
| `--output <FILE>` | `-o` | Output project file |
| `--format <FMT>` | `-f` | Input format (json, toml, openapi, prisma) |

**Examples:**

```bash
# Import from JSON
imortal import schema.json --output my_app.imortal

# Import from OpenAPI spec
imortal import openapi.yaml --format openapi
```

**Note:** Import from OpenAPI and Prisma schemas is planned for future releases.

---

### info

Show information about the engine.

```bash
imortal info
```

**Output:**
```
🔧 Immortal Engine

   Version: 0.1.0
   IR Version: 1.0.0
   Component Version: 1.0.0

📦 Built-in Components: 16
   🔐 Authentication: 4
   📊 Data: 3
   🔌 API: 3
   💾 Storage: 3
   ⚙ Logic: 3

🌐 Project Home: https://github.com/yourusername/imortal_engine
📖 Documentation: https://docs.imortal-engine.dev
```

---

## Configuration File

You can specify a configuration file with the `--config` option. The configuration file is in TOML format:

```toml
# imortal.toml

[project]
default_template = "web_api"
output_dir = "generated"

[generation]
target = "rust"
framework = "axum"

[database]
default_backend = "postgres"

[editor]
port = 3000
theme = "dark"
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Project not found |
| 4 | Validation failed |

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `IMORTAL_CONFIG` | Default configuration file path |
| `IMORTAL_LOG` | Log level (error, warn, info, debug, trace) |
| `RUST_BACKTRACE` | Enable backtraces on errors (1 or full) |

**Example:**
```bash
IMORTAL_LOG=debug imortal validate my_app/my_app.imortal
```

---

## Shell Completion

Generate shell completion scripts:

```bash
# Bash
imortal completions bash > /etc/bash_completion.d/imortal

# Zsh
imortal completions zsh > ~/.zsh/completions/_imortal

# Fish
imortal completions fish > ~/.config/fish/completions/imortal.fish
```

**Note:** Shell completions are planned for a future release.

---

## Examples

### Create and Build a Complete Project

```bash
# Create new project
imortal new todo_api

# Open in editor, design your app
imortal editor --project todo_api/todo_api.imortal

# Validate the project
imortal validate todo_api/todo_api.imortal

# Generate code
imortal generate todo_api/todo_api.imortal --output todo_api/src/generated

# Build and run (standard Rust)
cd todo_api
cargo run
```

### CI/CD Integration

```bash
#!/bin/bash
# validate.sh - CI validation script

set -e

echo "Validating project..."
if ! imortal validate $PROJECT_FILE --format json | jq -e '.errors == 0'; then
    echo "Validation failed!"
    exit 1
fi

echo "Generating code..."
imortal generate $PROJECT_FILE --output src/generated

echo "Building..."
cargo build --release

echo "Success!"
```

### Batch Processing

```bash
# Validate all projects in a directory
for file in projects/*.imortal; do
    echo "Validating $file..."
    imortal validate "$file"
done
```

---

## Troubleshooting

### Command Not Found

Ensure the CLI is in your PATH:
```bash
export PATH="$PATH:/path/to/imortal_engine/target/release"
```

### Permission Denied

Make sure the binary is executable:
```bash
chmod +x target/release/imortal
```

### Project File Errors

If you get "Failed to load project" errors:
1. Check the file path is correct
2. Ensure the file is valid JSON
3. Verify the IR version is compatible

### Verbose Mode

Use `-v` for detailed output:
```bash
imortal -v validate my_app/my_app.imortal
```
