# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A 2D platformer with advanced character controller physics built using Bevy 0.12.1. Features include:
- Circle-based player collision with polygon level geometry
- Advanced movement mechanics: wall jumping, variable jump height, surface alignment
- Custom physics system with velocity, acceleration, and surface normals
- Level geometry loaded from JSON and rendered using Bevy's Gizmos

## Build & Run Commands

```bash
# Build the project
cargo build

# Run the game (native)
cargo run

# Build for release
cargo build --release

# Build for WASM
cargo build --target wasm32-unknown-unknown

# Run WASM (requires wasm-server-runner)
cargo run --target wasm32-unknown-unknown
```

## Architecture

### Module Structure

- **main.rs**: App initialization, ECS systems, player input, movement logic, and rendering
- **collisions.rs**: Collision detection and resolution with polygon geometry
- **level.rs**: Level loading from JSON, polygon generation, and geometry optimization

### ECS System Execution Order

Systems run in this order each frame:
1. `s_input` - Captures keyboard input and sets jump timers
2. `s_movement` - Applies physics (acceleration, gravity, jumping)
3. `s_collision` - Detects and resolves collisions, updates surface normals
4. `s_timers` - Decrements jump/grounded/walled timers
5. `s_render` - Draws player and level geometry using Gizmos
6. `s_wait_for_next_frame` - Caps framerate to 60 FPS (native only)

### Core Components

**Player** (main.rs:56-62): Tracks jump state and ground/wall contact timers
- `jump_timer`: Frames remaining to execute jump input
- `grounded_timer`: Frames since last ground contact (coyote time)
- `walled_timer`: Frames since last wall contact (signed by direction)
- `has_wall_jumped`: Reduces air control after wall jump

**Physics** (main.rs:64-71): Custom physics simulation per entity
- `prev_position`: Used for collision detection
- `velocity`: Current movement speed
- `acceleration`: Applied each frame
- `radius`: Collision circle radius
- `normal`: Current surface normal (zero when airborne)

### Level Format

Level geometry is defined in `assets/level.json` as a 2D grid where:
- `0` = empty space
- `1` = solid square tile
- `2-5` = right triangles (bottom-left, bottom-right, top-left, top-right)
- `6-9` = isosceles triangles (currently commented out)

The level loader:
1. Extracts tile edges based on neighboring tiles
2. Merges collinear edges to reduce line segments
3. Groups connected edges into closed polygons
4. Calculates winding order to determine collision side
5. Assigns random colors for visualization

### Collision System

Circle-to-polygon collision detection:
- Projects player position onto each line segment in each polygon
- Uses ray casting to determine if player is inside polygon
- Calculates surface normals from nearby edges
- Resolves penetration by adjusting player position
- Modifies velocity to prevent sinking into surfaces
- Updates `grounded_timer` and `walled_timer` based on surface orientation

### Movement Physics

**Surface-Aligned Movement**: Input direction rotates to align with surface normal when grounded (unless moving directly toward/away from surface)

**Acceleration**: Interpolates velocity toward target speed
- Moving: `PLAYER_ACCELERATION_SCALERS.0` (0.2)
- Stopping: `PLAYER_ACCELERATION_SCALERS.1` (0.4)
- Reduced by 50% after wall jump

**Gravity**: Always pulls down at 0.5 units/frame, except when moving off a wall

**Jumping**:
- Ground jump: velocity.y = 9.0
- Wall jump: velocity = (±7.8, 4.5) away from wall
- Variable height: releasing jump early reduces upward velocity by 66%
- Timers provide input buffering (10 frames) and coyote time (10 frames)

## WASM Support

Configured via `.cargo/config.toml` with `wasm-server-runner`. Frame limiting is disabled for WASM builds using `#[cfg(not(target_arch = "wasm32"))]`.

## Controls

- Arrow Keys: Move
- Space: Jump (hold for higher jump)
- Escape: Exit
