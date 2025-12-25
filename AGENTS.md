# AGENTS.md - bevy-advanced-cc

## Project Snapshot

**bevy-advanced-cc** is a single-crate Rust game project built with Bevy 0.17.3, featuring an advanced 2D character controller with custom physics and collision detection. The project uses a plugin-first architecture with modular systems for input, movement, collision, and rendering.

**Tech Stack**: Rust 2021 edition + Bevy 0.17.3 + serde/serde_json for asset loading + rand for procedural generation.

**Note**: Sub-folders (`src/`, `assets/`) have their own AGENTS.md files with detailed patterns and conventions.

---

## Root Setup Commands

```bash
# Build all
cargo build

# Run game
cargo run

# Build release
cargo build --release

# Format code
cargo fmt

# Lint (treat warnings as errors)
cargo clippy --all-targets --all-features -D warnings

# Test all
cargo test

# Check for unused dependencies
cargo machete

# Security audit (if cargo-audit installed)
cargo audit
```

---

## Universal Conventions

- **Rust Style**: `rustfmt` required; clippy warnings treated as errors (`-D warnings`)
- **Bevy Architecture**: Prefer plugin-first architecture (small plugins like `CollisionPlugin` over monolithic App setup)
- **System Naming**: Systems prefixed with `s_` (e.g., `s_input`, `s_movement`, `s_collision`)
- **Performance**: Avoid tight-loop allocations; profile before optimizing; use `single_mut()` for single-entity queries
- **Bevy 0.17.3 API**: Use `MessageWriter<T>` for events (not `EventWriter`), `Color::srgb()` for colors, `ButtonInput<KeyCode>` for input
- **No Feature Flags**: Currently no custom features; use default Bevy features

---

## Security & Secrets

- **Never commit**: API keys, tokens, or secrets
- **Config**: No `.env` or secrets management currently; all config is compile-time (`include_bytes!` for assets)
- **Logging**: No PII logging; debug prints use `dbg!()` or `println!()` (remove before production)

---

## JIT Index (what to open, not what to paste)

### Directory Map

- **Core game code**: `src/` → [see src/AGENTS.md](src/AGENTS.md)
  - Systems, components, resources, plugins
  - ECS architecture patterns
  - Physics and collision conventions

- **Assets**: `assets/` → [see assets/AGENTS.md](assets/AGENTS.md)
  - Level JSON format
  - Asset loading conventions

### Quick Find Commands

```bash
# Find components
rg -n "derive\(Component\)" src/

# Find systems (functions with Query/Res/Commands)
rg -n "fn s_\w+.*\(.*(Query|Res|Commands|MessageWriter)" src/

# Find plugins
rg -n "impl Plugin for" src/

# Find resources
rg -n "derive\(Resource\)" src/

# Find asset loads
rg -n "include_bytes!|serde_json::from_str" src/

# Find system ordering
rg -n "\.after\(|\.before\(" src/
```

---

## System Execution Order

Systems run in this order each frame (defined in `main.rs`):

1. `s_input` - Keyboard input, jump timers
2. `s_movement` (after `s_input`) - Physics, acceleration, gravity, jumping
3. `s_collision` (after `s_movement`, via `CollisionPlugin`) - Collision detection/resolution
4. `s_timers` (after `s_collision`) - Decrement jump/grounded/walled timers
5. `s_render` (after `s_timers`) - Draw player and level with Gizmos
6. `s_wait_for_next_frame` (after `s_render`) - Cap framerate to 60 FPS (native only)

**Critical**: System ordering matters! Movement must run before collision, timers after collision, render after timers.

