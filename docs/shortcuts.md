# Keyboard Shortcuts Reference

A complete reference of all keyboard shortcuts available in the Immortal Engine visual editor.

## Quick Reference Card

| Shortcut | Action |
|----------|--------|
| `Ctrl+Z` | Undo |
| `Ctrl+Y` | Redo |
| `Ctrl+Shift+Z` | Redo (alternative) |
| `Delete` | Delete selected |
| `Backspace` | Delete selected |
| `Escape` | Cancel current action |

## General Shortcuts

### File Operations

| Shortcut | Action | Notes |
|----------|--------|-------|
| `Ctrl+S` | Save | Coming soon |
| `Ctrl+Shift+S` | Save As | Coming soon |
| `Ctrl+O` | Open | Coming soon |
| `Ctrl+N` | New Project | Coming soon |

### Edit Operations

| Shortcut | Action | Notes |
|----------|--------|-------|
| `Ctrl+Z` | Undo | Undoes last action |
| `Ctrl+Y` | Redo | Redoes undone action |
| `Ctrl+Shift+Z` | Redo | Alternative redo shortcut |
| `Ctrl+X` | Cut | Coming soon |
| `Ctrl+C` | Copy | Coming soon |
| `Ctrl+V` | Paste | Coming soon |
| `Ctrl+A` | Select All | Coming soon |
| `Delete` | Delete selected | Deletes all selected nodes/edges |
| `Backspace` | Delete selected | Same as Delete |

## Canvas Navigation

### View Controls

| Shortcut | Action | Notes |
|----------|--------|-------|
| `Middle Mouse Drag` | Pan canvas | Hold and drag to move view |
| `Shift + Left Drag` | Pan canvas | Alternative pan method |
| `Ctrl++` | Zoom in | Coming soon |
| `Ctrl+-` | Zoom out | Coming soon |
| `Ctrl+0` | Reset zoom | Coming soon |

### Selection

| Shortcut | Action | Notes |
|----------|--------|-------|
| `Left Click` | Select node | Selects single node |
| `Shift + Left Click` | Toggle selection | Add/remove from selection |
| `Click Empty Space` | Deselect all | Clears selection |

## Connection Drawing

| Shortcut | Action | Notes |
|----------|--------|-------|
| `Left Click Port` | Start/complete connection | Click green then blue port |
| `Escape` | Cancel connection | While drawing a connection |
| `Right Click` | Cancel connection | Alternative cancel method |

## Node Operations

| Shortcut | Action | Notes |
|----------|--------|-------|
| `Left Drag` | Move selected nodes | Drag to reposition |
| `Delete` | Delete selected | Works with multi-selection |

## Modifiers

### Shift Key

| With Action | Effect |
|-------------|--------|
| + Click on node | Add/remove from selection |
| + Drag on canvas | Pan view (instead of selecting) |
| + Ctrl+Z | Redo |

### Ctrl Key (Cmd on Mac)

| With Action | Effect |
|-------------|--------|
| + Z | Undo |
| + Y | Redo |

## Action State Shortcuts

### While Drawing Connection

| Shortcut | Action |
|----------|--------|
| `Escape` | Cancel drawing |
| `Right Click` | Cancel drawing |
| `Click Port` | Complete connection |

### While Node Selected

| Shortcut | Action |
|----------|--------|
| `Delete` | Delete node |
| `Backspace` | Delete node |
| Drag | Move node |

## Platform-Specific Notes

### Windows/Linux

- Use `Ctrl` for all control shortcuts

### macOS

- Use `Cmd` (⌘) instead of `Ctrl`
- `Cmd+Z` for Undo
- `Cmd+Y` or `Cmd+Shift+Z` for Redo

## Shortcuts by Feature

### Undo/Redo System

```
Ctrl+Z          → Undo last action
Ctrl+Y          → Redo last undone action
Ctrl+Shift+Z    → Redo (alternative)
```

**Undoable actions:**
- Adding components
- Deleting components
- Adding fields
- Deleting fields
- Creating connections

### Node Management

```
Click           → Select node
Shift+Click     → Multi-select
Delete          → Delete selected
Drag            → Move selected
```

### Canvas Navigation

```
Middle-Drag     → Pan canvas
Shift+Drag      → Pan canvas
Click Empty     → Deselect all
```

### Connection Management

```
Click Green Port   → Start connection from output
Click Blue Port    → Complete connection to input
Escape             → Cancel connection
Right-Click        → Cancel connection
```

## Customization

Keyboard shortcuts are currently not customizable. Custom keybindings will be available in a future release.

## Upcoming Shortcuts

The following shortcuts are planned for future releases:

| Shortcut | Planned Action |
|----------|----------------|
| `Ctrl+S` | Save project |
| `Ctrl+O` | Open project |
| `Ctrl+N` | New project |
| `Ctrl+A` | Select all |
| `Ctrl+D` | Duplicate selection |
| `Ctrl+G` | Group selected |
| `Ctrl+F` | Find/search |
| `Space` | Toggle pan mode |
| `F2` | Rename selected |
| `Tab` | Cycle selection |

## Troubleshooting

### Shortcuts Not Working?

1. **Ensure canvas has focus** - Click on the canvas area first
2. **Check for conflicts** - Other applications may intercept shortcuts
3. **Verify modifier keys** - Make sure Ctrl/Cmd is held properly

### Common Issues

| Problem | Solution |
|---------|----------|
| Ctrl+Z does nothing | Ensure there are actions to undo |
| Delete doesn't work | Make sure a node is selected |
| Can't pan canvas | Try middle-mouse or Shift+drag |

## Quick Reference Printable

```
╔═══════════════════════════════════════════════════╗
║        IMMORTAL ENGINE - KEYBOARD SHORTCUTS       ║
╠═══════════════════════════════════════════════════╣
║  EDIT                                             ║
║    Ctrl+Z .............. Undo                     ║
║    Ctrl+Y .............. Redo                     ║
║    Delete .............. Delete selected          ║
║                                                   ║
║  CANVAS                                           ║
║    Middle-Drag ......... Pan view                 ║
║    Shift+Drag .......... Pan view                 ║
║    Click ............... Select                   ║
║    Shift+Click ......... Multi-select             ║
║                                                   ║
║  CONNECTIONS                                      ║
║    Click Port .......... Start/end connection     ║
║    Escape .............. Cancel connection        ║
╚═══════════════════════════════════════════════════╝
```
