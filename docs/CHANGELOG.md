# Changelog

All notable changes to the Immortal Engine project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Visual Editor
- **Component Palette** - Drag and drop components from categorized palette
  - Authentication: Login, Register, Logout, Session
  - Data: Entity, Collection, Query
  - API: REST Endpoint, GraphQL, WebSocket
  - Storage: Database, Cache, File Storage
  - Logic: Validator, Transformer, Condition
- **Canvas Interactions**
  - Pan canvas with middle-mouse drag or Shift+drag
  - Click to select nodes
  - Shift+click for multi-selection
  - Click empty canvas to deselect
  - Drag to move selected nodes
- **Node Selection** - Visual feedback for selected nodes with highlighted borders
- **Node Deletion** - Delete nodes via:
  - `Delete` or `Backspace` keyboard shortcuts
  - "🗑 Delete Component" button in Properties panel
  - Edit menu → Delete Selected
- **Connection Ports** - Visual ports on nodes for creating connections
  - Green output ports (right side)
  - Blue input ports (left side)
  - Hover highlighting on ports
- **Connection Drawing** - Click-to-connect workflow
  - Click output port to start connection
  - Click input port to complete connection
  - Press Escape or right-click to cancel
  - Visual feedback line while drawing
- **Entity Field Display** - Entity nodes show their fields directly on the canvas
  - Primary key fields marked with 🔑 icon
  - Field names and types displayed
  - Auto-sizing nodes based on field count
- **Properties Panel** - Configure selected components
  - Edit node name
  - View component type
  - Manage fields (add/delete)
  - View ports
  - View configuration
- **Field Management** - Add and remove fields on Entity nodes
  - Add new fields with name and type selection
  - Delete custom fields (system fields protected)
  - Supported types: String, Text, Integer, BigInt, Float, Double, Boolean, DateTime, Date, JSON
- **Undo/Redo System** - Full history support
  - `Ctrl+Z` to undo
  - `Ctrl+Y` or `Ctrl+Shift+Z` to redo
  - Edit menu shows action names
  - 50 state history capacity
  - Supports: add/delete components, add/delete fields, create connections

#### CLI
- **new** - Create new projects with templates
- **validate** - Validate project files for errors
- **components** - List all available components with filtering
- **export** - Export projects to JSON/TOML formats
- **import** - Import projects (basic support)
- **info** - Display engine information

#### Core Engine
- **Project Graph IR** - Graph-based intermediate representation
- **Component Registry** - Extensible component system with 16 built-in components
- **Validation System** - Configurable validation rules
- **Serialization** - JSON and TOML project file support

### Fixed
- Fixed CLI short option conflict (`-c` for both config and category)
- Fixed validation tests for entity relationships
- Added missing input ports on Entity nodes for relationship connections
- Fixed port compatibility validation to skip relationship edges
- Fixed node selection not updating Properties panel
- Fixed connection drawing port detection
- Fixed edge drawing to connect at the actual port circle positions (green/blue dots)
  - Edges now properly account for dynamic entity node heights
  - Connection lines start/end exactly at the port circles (+8px outside node)

### Technical
- Workspace structure with 6 crates
- egui-based visual editor
- Comprehensive test suite (159+ tests)

## [0.1.0] - Initial Development

### Added
- Initial project structure
- Core type definitions
- Basic IR implementation
- Component definitions
- Code generation framework (in progress)
- Visual editor foundation
- CLI foundation

---

## Feature Roadmap

### Planned for Next Release
- [ ] Mouse wheel zoom
- [ ] Edge selection and deletion
- [ ] Copy/paste nodes
- [ ] Node move undo support
- [ ] Minimap
- [ ] Dark/light theme toggle

### Future Releases
- [ ] Code generation completion
- [ ] Database migration generation
- [ ] API route generation
- [ ] Custom component plugins
- [ ] Project templates
- [ ] Collaboration features
- [ ] Web-based editor (Dioxus)

---

## Version History

| Version | Date | Description |
|---------|------|-------------|
| 0.1.0 | In Development | Initial release with visual editor |
