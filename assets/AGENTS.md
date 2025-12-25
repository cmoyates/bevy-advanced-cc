# assets/AGENTS.md - Asset Conventions

## Overview

This directory contains game assets: level data, images, audio, shaders, etc. Currently, the project uses JSON for level geometry data.

---

## Level Data Format

### `level.json`

**Format**: 2D array of integers (`Vec<Vec<u32>>`)

**Tile Types**:
- `0`: Empty/air
- `1`: Solid square
- `2-5`: Right triangles (2=bottom-left, 3=bottom-right, 4=top-left, 5=top-right)
- `6-9`: Isosceles triangles (currently commented out in `level.rs`)

**Example** (`level.json`):
```json
[
  [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
  [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 1],
  ...
]
```

**Loading** (`level.rs:10-16`):
```rust
const LEVEL_DATA: &'static [u8] = include_bytes!("../assets/level.json");
let json_data: Vec<Vec<u32>> = serde_json::from_str(&res.unwrap()).unwrap();
```

**Grid Size**: Level tiles are rendered at `32.0` units per tile (defined in `s_init`, `main.rs:97`).

---

## Asset Loading Conventions

### Compile-Time Embedding

**Pattern**: Use `include_bytes!` for small assets loaded at startup.

```rust
const LEVEL_DATA: &'static [u8] = include_bytes!("../assets/level.json");
```

**Pros**: No runtime file I/O, assets bundled in binary.

**Cons**: Requires rebuild to change assets; not suitable for large assets or hot-reload.

### Runtime Loading (Future)

For larger assets or hot-reload, use Bevy's `AssetServer`:

```rust
let handle: Handle<Image> = asset_server.load("textures/player.png");
```

**Note**: Currently not used; project uses compile-time embedding for level data.

---

## Asset Organization (Future)

When adding more assets, organize by type:

```
assets/
├── levels/
│   └── level.json
├── textures/
│   ├── player.png
│   └── tiles.png
├── audio/
│   ├── jump.ogg
│   └── music.ogg
└── shaders/
    └── custom.wgsl
```

---

## Level Generation

Level geometry is generated from tile data in `level.rs:generate_level_polygons()`:

1. **Tile Processing**: Iterates through tile grid, generates line segments for edges
2. **Line Optimization**: Removes superfluous points (parallel lines sharing endpoints)
3. **Polygon Construction**: Groups lines into closed polygons
4. **Winding Order**: Calculates collision side based on polygon winding
5. **Color Assignment**: Random colors per polygon (for debug rendering)

**Grid Size**: `32.0` units per tile (configurable in `s_init`).

---

## Asset Validation (Future)

Consider adding validation:

- **Level bounds**: Ensure level fits within reasonable size
- **Tile validity**: Validate tile IDs are in expected range
- **Polygon validity**: Ensure polygons are closed and non-self-intersecting

---

## Common Commands

```bash
# Find asset loads
rg -n "include_bytes!|asset_server\.load|Handle<" src/

# Find level references
rg -n "level\.json|LEVEL_DATA|generate_level_polygons" src/

# Validate JSON (if jq installed)
jq . assets/level.json
```

