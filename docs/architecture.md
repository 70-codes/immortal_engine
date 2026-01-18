# Architecture Documentation

This document describes the technical architecture of the Immortal Engine, a visual prototyping system for building applications in Rust.

## High-Level Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           USER INTERFACE                                │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    Visual Editor (imortal-editor)                │   │
│  │  ┌───────────┐  ┌─────────────────┐  ┌───────────────────────┐  │   │
│  │  │  Palette  │  │     Canvas      │  │   Properties Panel    │  │   │
│  │  └───────────┘  └─────────────────┘  └───────────────────────┘  │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    CLI (imortal_cli)                             │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────┬───────────────────────────────────────┘
                                  │
┌─────────────────────────────────▼───────────────────────────────────────┐
│                           CORE ENGINE                                   │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────┐ │
│  │   imortal_ir    │  │ imortal_components│  │    imortal_codegen     │ │
│  │  (Graph Model)  │  │   (Registry)    │  │   (Code Generation)    │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────────────┘ │
│  ┌─────────────────────────────────────────────────────────────────────┐│
│  │                        imortal_core                                 ││
│  │                    (Shared Types & Traits)                          ││
│  └─────────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────┬───────────────────────────────────────┘
                                  │
┌─────────────────────────────────▼───────────────────────────────────────┐
│                        GENERATED OUTPUT                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐ │
│  │ Rust Structs │  │  API Routes  │  │  Migrations  │  │   Config    │ │
│  └──────────────┘  └──────────────┘  └──────────────┘  └─────────────┘ │
└─────────────────────────────────────────────────────────────────────────┘
```

## Workspace Structure

The project is organized as a Cargo workspace with the following crates:

```
imortal_engine/
├── Cargo.toml              # Workspace root + imortal-editor binary
├── src/
│   └── main.rs             # Visual editor entry point
│
├── crates/
│   ├── core/               # imortal_core - Shared types and traits
│   ├── ir/                 # imortal_ir - Intermediate representation
│   ├── components/         # imortal_components - Component definitions
│   ├── codegen/            # imortal_codegen - Code generation
│   ├── ui/                 # imortal_ui - Visual editor UI
│   └── cli/                # imortal_cli - Command-line interface
│
└── docs/                   # Documentation
```

## Crate Descriptions

### imortal_core

**Purpose:** Shared types, traits, and error definitions used across all crates.

**Key Types:**
```rust
// Node and Edge identifiers
pub type NodeId = Uuid;
pub type EdgeId = Uuid;

// Data types for fields
pub enum DataType {
    String, Text, Int32, Int64, Float32, Float64,
    Bool, Uuid, DateTime, Date, Time, Bytes, Json,
    Optional(Box<DataType>),
    Array(Box<DataType>),
    Entity(String),
    Reference(String),
    // ...
}

// Component categories
pub enum ComponentCategory {
    Auth, Data, Api, Storage, Logic, Ui, Embedded, Custom,
}

// Connection types between nodes
pub enum ConnectionType {
    DataFlow,
    Navigation,
    Relationship(RelationType),
    Trigger,
    Dependency,
}
```

**Dependencies:** None (leaf crate)

---

### imortal_ir

**Purpose:** Intermediate Representation (IR) - the graph-based data model that represents a project.

**Key Types:**

```rust
// The complete project graph
pub struct ProjectGraph {
    pub meta: ProjectMeta,
    pub nodes: HashMap<NodeId, Node>,
    pub edges: HashMap<EdgeId, Edge>,
    pub groups: HashMap<Uuid, Group>,
    pub selected_nodes: HashSet<NodeId>,
    pub selected_edges: HashSet<EdgeId>,
    pub viewport: Viewport,
}

// A node (component instance) on the canvas
pub struct Node {
    pub id: NodeId,
    pub component_type: String,
    pub name: String,
    pub position: Position,
    pub size: Size,
    pub fields: Vec<Field>,
    pub ports: PortCollection,
    pub config: HashMap<String, ConfigValue>,
    pub category: ComponentCategory,
    // ...
}

// An edge (connection) between nodes
pub struct Edge {
    pub id: EdgeId,
    pub from_node: NodeId,
    pub from_port: String,
    pub to_node: NodeId,
    pub to_port: String,
    pub connection_type: ConnectionType,
    // ...
}

// A field within a node
pub struct Field {
    pub id: Uuid,
    pub name: String,
    pub data_type: DataType,
    pub required: bool,
    pub constraints: Vec<FieldConstraint>,
    // ...
}
```

**Modules:**
- `graph.rs` - ProjectGraph implementation
- `node.rs` - Node type and builders
- `edge.rs` - Edge type and connection logic
- `field.rs` - Field definitions
- `port.rs` - Port definitions for connections
- `group.rs` - Node grouping
- `project.rs` - Project metadata and configuration
- `serialization.rs` - JSON/TOML serialization
- `validation.rs` - Graph validation rules

**Dependencies:** `imortal_core`

---

### imortal_components

**Purpose:** Component registry and built-in component definitions.

**Key Types:**

```rust
// Component definition (template)
pub struct ComponentDefinition {
    pub id: String,
    pub name: String,
    pub category: ComponentCategory,
    pub description: String,
    pub icon: String,
    pub default_fields: Vec<FieldDefinition>,
    pub config_schema: Vec<ConfigOption>,
    pub ports: PortDefinitions,
}

// Component registry
pub struct ComponentRegistry {
    components: HashMap<String, ComponentDefinition>,
}

impl ComponentRegistry {
    pub fn with_builtins() -> Self;
    pub fn register(&mut self, def: ComponentDefinition);
    pub fn get(&self, id: &str) -> Option<&ComponentDefinition>;
    pub fn by_category(&self, cat: ComponentCategory) -> Vec<&ComponentDefinition>;
    pub fn instantiate(&self, id: &str) -> Option<Node>;
}
```

**Built-in Components:**
- **Auth:** Login, Register, Logout, Session
- **Data:** Entity, Collection, Query
- **API:** REST Endpoint, GraphQL, WebSocket
- **Storage:** Database, Cache, File Storage
- **Logic:** Validator, Transformer, Condition

**Dependencies:** `imortal_core`, `imortal_ir`

---

### imortal_codegen

**Purpose:** Code generation from the IR to Rust source code.

**Key Types:**

```rust
// Code generator
pub struct CodeGenerator {
    config: GeneratorConfig,
    templates: TemplateEngine,
}

impl CodeGenerator {
    pub fn generate(&self, graph: &ProjectGraph) -> Result<GeneratedProject>;
}

// Generated project output
pub struct GeneratedProject {
    pub files: HashMap<PathBuf, GeneratedFile>,
}

pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
    pub file_type: FileType,
}
```

**Modules:**
- `generator.rs` - Main generation orchestrator
- `templates/` - Code templates
- `rust/` - Rust-specific generators
  - `structs.rs` - Struct generation
  - `models.rs` - Database model generation
  - `handlers.rs` - API handler generation
  - `migrations.rs` - Migration generation
  - `auth.rs` - Authentication code
  - `config.rs` - Configuration generation

**Dependencies:** `imortal_core`, `imortal_ir`, `quote`, `syn`, `proc-macro2`

---

### imortal_ui

**Purpose:** Visual editor user interface built with egui.

**Key Types:**

```rust
// Main application
pub struct ImmortalApp {
    pub project: ProjectGraph,
    pub state: EditorState,
    pub registry: ComponentRegistry,
    pub config: UiConfig,
    history: History,
    // ...
}

// Editor state
pub struct EditorState {
    pub selection: SelectionState,
    pub interaction: InteractionState,
    pub view: ViewState,
    pub active_tool: Tool,
    // ...
}

// Undo/Redo history
pub struct History {
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
}
```

**Modules:**
- `app.rs` - Main application and rendering
- `state.rs` - Editor state management
- `canvas/` - Canvas widget and interactions
- `palette.rs` - Component palette
- `properties.rs` - Properties panel
- `toolbar.rs` - Toolbar widgets
- `dialogs.rs` - Modal dialogs
- `theme.rs` - Visual theming

**Dependencies:** `imortal_core`, `imortal_ir`, `imortal_components`, `imortal_codegen`, `eframe`, `egui`

---

### imortal_cli

**Purpose:** Command-line interface for project management and code generation.

**Commands:**
- `new` - Create new project
- `editor` - Open visual editor
- `generate` - Generate code
- `validate` - Validate project
- `components` - List components
- `export` - Export project
- `import` - Import project
- `info` - Show engine info

**Dependencies:** `imortal_core`, `imortal_ir`, `imortal_components`, `imortal_codegen`, `clap`

---

## Data Flow

### 1. Project Creation/Loading

```
User Action (New/Open)
        │
        ▼
┌───────────────────┐
│   ProjectGraph    │  ← Created empty or loaded from .imortal file
│   (imortal_ir)    │
└───────┬───────────┘
        │
        ▼
┌───────────────────┐
│   Visual Editor   │  ← Displays graph on canvas
│   (imortal_ui)    │
└───────────────────┘
```

### 2. Component Addition

```
User clicks component in palette
        │
        ▼
┌───────────────────┐
│ComponentRegistry  │  → Looks up ComponentDefinition
│(imortal_components)│
└───────┬───────────┘
        │ instantiate()
        ▼
┌───────────────────┐
│      Node         │  ← New node with default fields/ports
│   (imortal_ir)    │
└───────┬───────────┘
        │ add_node()
        ▼
┌───────────────────┐
│   ProjectGraph    │  ← Graph updated
│   (imortal_ir)    │
└───────┬───────────┘
        │
        ▼
┌───────────────────┐
│   Canvas Redraw   │  ← UI shows new node
│   (imortal_ui)    │
└───────────────────┘
```

### 3. Connection Creation

```
User clicks output port, then input port
        │
        ▼
┌───────────────────┐
│      Edge         │  ← New edge created
│   (imortal_ir)    │
└───────┬───────────┘
        │ add_edge()
        ▼
┌───────────────────┐
│   ProjectGraph    │  ← Graph validates and stores edge
│   (imortal_ir)    │
└───────┬───────────┘
        │
        ▼
┌───────────────────┐
│   Canvas Redraw   │  ← UI shows connection line
│   (imortal_ui)    │
└───────────────────┘
```

### 4. Code Generation

```
User triggers Generate
        │
        ▼
┌───────────────────┐
│   ProjectGraph    │  → Provides complete graph
│   (imortal_ir)    │
└───────┬───────────┘
        │
        ▼
┌───────────────────┐
│   Validation      │  → Checks for errors/warnings
│   (imortal_ir)    │
└───────┬───────────┘
        │
        ▼
┌───────────────────┐
│  CodeGenerator    │  → Processes each node/edge
│ (imortal_codegen) │
└───────┬───────────┘
        │
        ▼
┌───────────────────┐
│ GeneratedProject  │  → Collection of source files
│ (imortal_codegen) │
└───────┬───────────┘
        │
        ▼
┌───────────────────┐
│   File System     │  ← Files written to output directory
└───────────────────┘
```

---

## Key Design Decisions

### 1. Graph-Based IR

The project uses a graph-based intermediate representation where:
- **Nodes** represent component instances (entities, endpoints, etc.)
- **Edges** represent connections (data flow, relationships, dependencies)

This allows:
- Visual representation on a canvas
- Flexible connection patterns
- Easy validation of the model
- Platform-agnostic code generation

### 2. Component Registry Pattern

Components are defined as templates in a registry:
- Separation of component definition from instance
- Easy to add new component types
- Plugin architecture for custom components
- Consistent instantiation behavior

### 3. Immediate Mode UI (egui)

The UI uses egui's immediate mode paradigm:
- No retained widget tree
- State managed explicitly in `ImmortalApp`
- Simple rendering loop
- Cross-platform support

### 4. Serialization Format

Projects are saved as JSON with:
- IR version for compatibility
- Complete graph serialization
- Human-readable format
- Easy to diff and version control

### 5. Undo/Redo with Snapshots

History is implemented using full state snapshots:
- Simple implementation
- Guaranteed consistency
- Memory trade-off for simplicity
- Limited to 50 states by default

---

## Extension Points

### Adding a New Component

1. Create definition in `crates/components/src/definitions/`:

```rust
pub fn my_component() -> ComponentDefinition {
    ComponentDefinition {
        id: "category.my_component",
        name: "My Component",
        category: ComponentCategory::Custom,
        description: "Description here",
        icon: "🆕",
        default_fields: vec![...],
        config_schema: vec![...],
        ports: PortDefinitions { ... },
    }
}
```

2. Register in `ComponentRegistry::with_builtins()`

3. Add code generation in `crates/codegen/src/rust/`

### Adding a New Code Generator

1. Implement generator in `crates/codegen/src/`:

```rust
pub trait Generator {
    fn generate(&self, graph: &ProjectGraph) -> Result<Vec<GeneratedFile>>;
}
```

2. Add to `CodeGenerator` orchestration

### Adding a New Validation Rule

1. Implement `ValidationRule` trait in `crates/ir/src/validation.rs`:

```rust
pub struct MyRule;

impl ValidationRule for MyRule {
    fn name(&self) -> &'static str { "My Rule" }
    fn validate(&self, graph: &ProjectGraph) -> Vec<ValidationError> {
        // Validation logic
    }
}
```

2. Add to `Validator::new()` default rules

---

## Performance Considerations

### Canvas Rendering

- Nodes are only drawn if visible in viewport
- Port hit detection uses distance calculations
- Edge rendering uses simple line segments

### Undo/Redo

- Full graph cloning on each action
- Consider delta-based approach for large projects
- Memory grows with history size

### Code Generation

- Template-based for simple patterns
- AST manipulation for complex transformations
- Parallel file generation possible

---

## Testing Strategy

### Unit Tests

Each crate has unit tests for:
- Type construction and validation
- Serialization/deserialization
- Component instantiation
- Code generation output

### Integration Tests

- Full project creation → generation flow
- CLI command execution
- File format compatibility

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p imortal_ir

# With output
cargo test --workspace -- --nocapture
```

---

## Future Architecture Considerations

### Planned Improvements

1. **Plugin System** - Dynamic loading of custom components
2. **WASM Support** - Run editor in browser
3. **Language Server** - IDE integration for generated code
4. **Real-time Collaboration** - Multi-user editing
5. **Template Engine** - User-defined code templates

### Dioxus Migration Path

The UI could be migrated to Dioxus for:
- Better web support
- React-like component model
- Tailwind styling

The core crates (ir, components, codegen) would remain unchanged.

---

## Contributing

### Development Setup

```bash
git clone https://github.com/yourusername/imortal_engine.git
cd imortal_engine
cargo build --workspace
cargo test --workspace
```

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use `clippy` for linting (`cargo clippy`)
- Document public APIs
- Write tests for new functionality

### Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `cargo fmt` and `cargo clippy`
5. Submit PR with description of changes