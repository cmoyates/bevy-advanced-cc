# src/AGENTS.md - Game Code Conventions

## Overview

This directory contains all game logic: ECS systems, components, resources, plugins, and physics. The codebase follows Bevy 0.17.3 conventions with a plugin-first architecture.

---

## Module Structure

- **`main.rs`**: App initialization, core systems (`s_input`, `s_movement`, `s_render`, `s_timers`, `s_wait_for_next_frame`), components (`Player`, `Physics`), resources (`Level`, `InputDir`)
- **`collisions.rs`**: `CollisionPlugin`, collision detection system (`s_collision`), collision utilities
- **`level.rs`**: Level loading from JSON, polygon generation, geometry optimization

---

## ECS Architecture Patterns

### Components

**Naming**: PascalCase, singular nouns (e.g., `Player`, `Physics`)

**Player Component** (`main.rs:56-62`):
```rust
#[derive(Component)]
pub struct Player {
    jump_timer: i32,        // Frames remaining to execute jump
    grounded_timer: i32,    // Frames since last ground contact (coyote time)
    walled_timer: i32,      // Frames since last wall contact (signed by direction)
    has_wall_jumped: bool,  // Reduces air control after wall jump
}
```

**Physics Component** (`main.rs:64-71`):
```rust
#[derive(Component)]
pub struct Physics {
    pub prev_position: Vec2,  // Used for collision detection
    pub velocity: Vec2,       // Current velocity
    pub acceleration: Vec2,   // Current acceleration
    pub radius: f32,          // Collision radius
    pub normal: Vec2,         // Surface normal (for gravity/alignment)
}
```

**Pattern**: Components are data-only; logic lives in systems.

### Resources

**Naming**: PascalCase, singular nouns (e.g., `Level`, `InputDir`)

**Level Resource** (`main.rs:39-41`):
```rust
#[derive(Resource)]
pub struct Level {
    pub polygons: Vec<Polygon>,  // Immutable level geometry
}
```

**InputDir Resource** (`main.rs:44-46`):
```rust
#[derive(Resource)]
pub struct InputDir {
    pub dir: Vec2,  // Mutable input direction (updated each frame)
}
```

**Pattern**: Resources are shared state; prefer `Res<T>` for read-only, `ResMut<T>` for mutable.

### Systems

**Naming**: Prefix with `s_` (e.g., `s_input`, `s_movement`, `s_collision`)

**System Signature Pattern**:
```rust
pub fn s_system_name(
    mut query: Query<(&mut Component, &OtherComponent), With<Marker>>,
    resource: Res<Resource>,
    mut gizmos: Gizmos,  // For debug rendering
) {
    if let Ok((mut comp, other)) = query.single_mut() {
        // System logic
    }
}
```

**Query Patterns**:
- Use `single_mut()` for single-entity queries (replaces deprecated `get_single_mut()`)
- Use `With<T>` / `Without<T>` filters to narrow queries
- Avoid large queries in tight loops; prefer filtering with components

**System Ordering**:
```rust
.add_systems(Update, s_movement.after(s_input))
.add_systems(Update, s_collision.after(s_movement))
```

**Critical**: Always specify ordering with `.after()` / `.before()` when systems depend on each other.

### Plugins

**Naming**: PascalCase + "Plugin" suffix (e.g., `CollisionPlugin`)

**Plugin Pattern** (`collisions.rs:15-21`):
```rust
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, s_collision.after(s_movement));
    }
}
```

**Pattern**: Small, focused plugins over one giant App setup. Each plugin manages related systems.

---

## Physics & Movement Conventions

### Constants (`main.rs:48-53`)

```rust
pub const PLAYER_MAX_SPEED: f32 = 5.0;
pub const PLAYER_ACCELERATION_SCALERS: (f32, f32) = (0.2, 0.4);  // (accel, deaccel)
pub const MAX_JUMP_TIMER: i32 = 10;
pub const MAX_GROUNDED_TIMER: i32 = 10;
pub const MAX_WALLED_TIMER: i32 = 10;
```

**Pattern**: Gameplay constants at module level; use `pub const` for shared values.

### Movement System (`s_movement`, `main.rs:156-258`)

**Key Patterns**:
- Rotate input according to surface normal (for wall-running)
- Apply acceleration/deceleration based on input
- Gravity points toward surface normal (not always down)
- Jump logic: ground jump vs wall jump
- Update `prev_position` before applying velocity

**Anti-Pattern**: Don't modify `prev_position` after movement; collision system needs it.

### Collision System (`s_collision`, `collisions.rs:23-146`)

**Key Patterns**:
- Line-segment collision detection (player circle vs polygon edges)
- Surface normal calculation (for gravity/alignment)
- Timer updates (`grounded_timer`, `walled_timer`) based on collision
- Position adjustment to resolve collisions
- Velocity adjustment based on surface normal

**Critical**: Collision must run after movement (`after(s_movement)`).

---

## Rendering Conventions

### Gizmos (`s_render`, `main.rs:261-282`)

**Pattern**: Use `Gizmos` for debug/prototype rendering (not production sprites).

```rust
gizmos.circle_2d(position, radius, Color::WHITE);
gizmos.linestrip_2d(points, Color::srgb(r, g, b));
gizmos.line_2d(start, end, Color::WHITE);
```

**Color API**: Use `Color::srgb()` (Bevy 0.17.3), not `Color::rgb()`.

---

## Bevy 0.17.3 API Notes

### Input (`s_input`, `main.rs:108-153`)

```rust
keyboard_input: Res<ButtonInput<KeyCode>>  // Not Input<KeyCode>
keyboard_input.just_pressed(KeyCode::Space)
keyboard_input.pressed(KeyCode::ArrowLeft)
```

### Events

```rust
mut exit: MessageWriter<AppExit>  // Not EventWriter
exit.write(AppExit::Success);     // Not send()
```

### Queries

```rust
if let Ok(mut player) = query.single_mut() {  // Not get_single_mut()
    // ...
}
```

### Colors

```rust
Color::srgb(0.0, 0.0, 0.0)  // Not Color::rgb()
```

### Camera

```rust
commands.spawn((Camera2d, Transform::default()));  // Not Camera2dBundle
```

---

## Asset Loading Conventions

### Level Data (`level.rs:10-16`)

**Pattern**: Use `include_bytes!` for compile-time asset embedding.

```rust
const LEVEL_DATA: &'static [u8] = include_bytes!("../assets/level.json");
let json_data: Vec<Vec<u32>> = serde_json::from_str(&res.unwrap()).unwrap();
```

**Note**: Currently uses `unwrap()`; consider proper error handling for production.

---

## Anti-Patterns to Avoid

1. **Monolithic systems**: Don't put all logic in one system; split by responsibility
2. **Excessive `ResMut`**: Prefer `Res` when possible; only use `ResMut` when mutating
3. **Large queries in tight loops**: Filter with components (`With<T>`, `Without<T>`)
4. **Missing system ordering**: Always specify `.after()` / `.before()` for dependencies
5. **Hardcoded magic numbers**: Use constants (e.g., `PLAYER_MAX_SPEED`)
6. **Production `unwrap()`**: Use proper error handling for asset loading
7. **Gizmos in production**: Replace with proper sprites/meshes for release builds

---

## Testing (Future)

Currently no tests. When adding:

- **Unit tests**: `#[test]` functions in modules (e.g., `#[cfg(test)] mod tests`)
- **Integration tests**: `tests/` directory at root
- **System tests**: Use Bevy's test utilities for ECS testing

---

## Common Commands

```bash
# Find all systems
rg -n "fn s_\w+" src/

# Find all components
rg -n "derive\(Component\)" src/

# Find all resources
rg -n "derive\(Resource\)" src/

# Find system ordering
rg -n "\.after\(|\.before\(" src/

# Find plugins
rg -n "impl Plugin" src/
```

