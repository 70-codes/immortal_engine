# User Interface Guide

This guide provides a comprehensive overview of the Immortal Engine visual editor interface.

## Window Layout

The editor window is divided into four main areas:

```
┌─────────────────────────────────────────────────────────────────┐
│  File   Edit   View   Generate   Help                    │ Menu │
├───────────────┬─────────────────────────┬───────────────────────┤
│               │                         │                       │
│   Components  │        Canvas           │     Properties        │
│    Palette    │                         │       Panel           │
│               │   ┌─────────┐           │                       │
│  ▼ Auth       │   │  Node   │───────────│  Name: [Entity]       │
│    • Login    │   └─────────┘           │  Type: data.entity    │
│    • Register │                         │                       │
│               │                         │  ▼ Fields             │
│  ▼ Data       │                         │  ▼ Ports              │
│    • Entity   │                         │  ▼ Configuration      │
│               │                         │                       │
├───────────────┴─────────────────────────┴───────────────────────┤
│  Zoom: 100% │ Nodes: 2 │ Edges: 1                  │ Status Bar │
└─────────────────────────────────────────────────────────────────┘
```

## Menu Bar

### File Menu

| Item | Shortcut | Description |
|------|----------|-------------|
| New Project | - | Create a new empty project |
| Open... | - | Open an existing project file |
| Save | - | Save current project |
| Save As... | - | Save project to a new location |
| Export... | - | Export project (coming soon) |
| Quit | - | Exit the application |

### Edit Menu

| Item | Shortcut | Description |
|------|----------|-------------|
| Undo | `Ctrl+Z` | Undo the last action |
| Redo | `Ctrl+Y` | Redo the last undone action |
| Cut | - | Cut selected items (coming soon) |
| Copy | - | Copy selected items (coming soon) |
| Paste | - | Paste from clipboard (coming soon) |
| Select All | - | Select all nodes |
| Deselect All | - | Clear selection |
| Delete Selected | `Delete` | Delete selected items |

### View Menu

| Item | Description |
|------|-------------|
| Show Grid | Toggle canvas grid visibility |
| Snap to Grid | Enable/disable grid snapping |
| Show Minimap | Toggle minimap (coming soon) |
| Zoom In | Increase zoom level |
| Zoom Out | Decrease zoom level |
| Reset Zoom | Reset to 100% zoom |
| Fit to Screen | Fit all nodes in view |

### Generate Menu

| Item | Description |
|------|-------------|
| Generate Code | Generate Rust code from the project |
| Validate | Check project for errors |
| Preview | Preview generated code (coming soon) |

### Help Menu

| Item | Description |
|------|-------------|
| About | Show application information |
| Settings | Open settings dialog |

## Components Palette (Left Panel)

The Components Palette contains all available components organized by category.

### Categories

#### 🔐 Authentication
- **Login** - User login with email/password
- **Register** - User registration
- **Logout** - End user session
- **Session** - Session management

#### 📊 Data
- **Entity** - Define a data model/table
- **Collection** - Queryable collection of entities
- **Query** - Build database queries

#### 🔌 API
- **REST Endpoint** - RESTful API endpoint
- **GraphQL** - GraphQL API
- **WebSocket** - Real-time WebSocket connection

#### 💾 Storage
- **Database** - Database connection
- **Cache** - In-memory caching
- **File Storage** - File/blob storage

#### ⚙ Logic
- **Validator** - Data validation
- **Transformer** - Data transformation
- **Condition** - Conditional logic

### Adding Components

1. **Click** on a component in the palette
2. The component is added to the canvas center
3. **Drag** to reposition as needed

### Searching Components

Use the search box at the top of the palette to filter components by name or description.

## Canvas (Center Panel)

The canvas is your main workspace where you design your application visually.

### Navigation

| Action | Method |
|--------|--------|
| Pan | Middle-mouse drag OR Shift + left-drag |
| Zoom | Mouse wheel (coming soon) OR View menu |

### Selecting Nodes

| Action | Method |
|--------|--------|
| Select single | Click on node |
| Add to selection | Shift + click |
| Deselect | Click empty canvas |
| Select all | Edit → Select All |

### Moving Nodes

1. **Select** one or more nodes
2. **Drag** to move them
3. Release to place

### Deleting Nodes

| Method | Steps |
|--------|-------|
| Keyboard | Select node(s) → Press `Delete` or `Backspace` |
| Properties Panel | Select node → Click "🗑 Delete Component" |
| Menu | Select node(s) → Edit → Delete Selected |

### Grid

- Toggle grid visibility in **View → Show Grid**
- Grid helps align components
- Snap to grid available (View → Snap to Grid)

## Node Anatomy

### Standard Node

```
┌─────────────────────────┐
│   🔌 REST Endpoint      │  ← Header (colored by category)
├─────────────────────────┤
│                         │
│      (Node Body)        │  ← Content area
│                         │
└─────────────────────────┘
●                         ●
↑                         ↑
Input Port              Output Port
(blue)                  (green)
```

### Entity Node (with fields)

Entity nodes automatically display their fields:

```
┌─────────────────────────┐
│       📊 Todo           │  ← Header (blue for Data)
├─────────────────────────┤
│ 🔑 id              Uuid │  ← Primary key (gold icon)
│ created_at     DateTime │  
│ updated_at     DateTime │  ← System fields
│ title           String  │  
│ completed       Boolean │  ← Custom fields
│ description       Text  │  
└─────────────────────────┘
●                         ●
```

### Node Colors by Category

| Category | Header Color |
|----------|--------------|
| Authentication | Red |
| Data | Blue |
| API | Green |
| Storage | Orange |
| Logic | Purple |

## Connection Ports

Ports are the connection points on nodes.

### Port Types

| Port | Position | Color | Purpose |
|------|----------|-------|---------|
| Input | Left side | 🔵 Blue | Receives data |
| Output | Right side | 🟢 Green | Sends data |

### Creating Connections

1. **Click** on a port (it highlights when hovered)
2. A line follows your mouse
3. **Click** on another port to complete the connection
4. Press **Escape** or **right-click** to cancel

### Connection Rules

- Connect **output** (green) → **input** (blue)
- Cannot connect a node to itself
- Multiple connections to the same port are allowed

### Deleting Connections

Currently, delete the node and recreate connections. Edge selection coming soon.

## Properties Panel (Right Panel)

When a node is selected, the Properties Panel shows its configuration.

### Header Section

- **Name**: Editable node name
- **Type**: Component type (read-only)
- **🗑 Delete Component**: Button to delete the node

### Fields Section (Entity nodes)

Displays all fields with:
- Field name
- Data type
- Required indicator (*)
- Delete button (🗑) for custom fields

**System fields** (id, created_at, updated_at) are locked (🔒).

#### Adding Fields

1. Enter **Name** in the text field
2. Select **Type** from dropdown:
   - String, Text, Integer, BigInt
   - Float, Double, Boolean
   - DateTime, Date, JSON
3. Click **+ Add Field**

### Ports Section

Lists all input and output ports with their names.

### Configuration Section

Shows component-specific settings like:
- `table_name` - Database table name
- `id_type` - Primary key type
- `timestamps` - Auto timestamp fields
- `soft_delete` - Soft delete support

## Status Bar

The bottom status bar displays:

```
Zoom: 100% │ Nodes: 3 │ Edges: 2          Untitled │ 3 nodes │ 2 edges
```

- **Zoom level**: Current canvas zoom
- **Nodes**: Number of nodes in project
- **Edges**: Number of connections
- **Status messages**: Temporary feedback (e.g., "Connection created")

## Undo/Redo System

The editor maintains a history of your actions for undo/redo.

### Supported Actions

| Action | Can Undo? |
|--------|-----------|
| Add component | ✅ Yes |
| Delete component | ✅ Yes |
| Add field | ✅ Yes |
| Delete field | ✅ Yes |
| Create connection | ✅ Yes |
| Move node | ❌ Not yet |
| Rename node | ❌ Not yet |

### Using Undo/Redo

| Method | Undo | Redo |
|--------|------|------|
| Keyboard | `Ctrl+Z` | `Ctrl+Y` or `Ctrl+Shift+Z` |
| Menu | Edit → Undo | Edit → Redo |

### History Capacity

- Stores up to **50** undo states
- Shows action name in Edit menu (e.g., "Undo: Add Todo")
- Grayed out when unavailable

## Dialogs

### New Project Dialog

Create a new project with custom settings.

### Settings Dialog

Configure editor preferences:
- Grid settings
- Theme (dark/light)
- Auto-save options

### About Dialog

Shows application version and credits.

## Tips & Best Practices

### Layout Tips

1. **Group related components** together
2. **Use the grid** for alignment
3. **Leave space** between components for readability
4. **Position data flow** left-to-right

### Workflow Tips

1. **Start with entities** - Define your data models first
2. **Add storage** - Connect to a database
3. **Create APIs** - Add endpoints to expose your data
4. **Add logic** - Include validation and transformations
5. **Save often** - Use Ctrl+S or File → Save

### Performance Tips

1. **Minimize nodes** on canvas for better performance
2. **Use groups** to organize large projects (coming soon)
3. **Close unused panels** if needed

## Accessibility

- **Keyboard navigation** for all major actions
- **High contrast** port colors
- **Status messages** for action feedback
- **Tooltips** on hover (coming soon)

## Known Limitations

Current limitations that will be addressed in future updates:

1. No zoom with mouse wheel (use View menu)
2. Cannot select/delete edges directly
3. Node move not undoable
4. No copy/paste functionality yet
5. No minimap yet
6. No dark/light theme toggle yet