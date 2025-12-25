<objective>
Update all dependencies in Cargo.toml to their latest stable versions and fix any breaking changes in the codebase.

Current dependencies to update:
- bevy = "0.12.1" (major update expected - check migration guides)
- rand = "0.8.5"
- serde = "1.0.196"
- serde_json = "1.0.112"
</objective>

<context>
This is a Bevy-based 2D game project with a physics-based character controller.
@Cargo.toml - current dependency versions
@src/main.rs - main game logic using Bevy ECS
@src/collisions.rs - collision detection system
@src/level.rs - level loading and rendering

Bevy updates frequently with breaking changes between minor versions. Pay special attention to:
- ECS API changes (queries, systems, resources)
- Rendering/Gizmos API changes
- Input handling changes
- Plugin system changes
</context>

<research>
Before updating, research the latest versions:
1. Check crates.io for latest stable versions of each dependency
2. For Bevy specifically, review the migration guide from 0.12 to the latest version
3. Note any deprecated APIs or breaking changes that affect this codebase
</research>

<requirements>
1. Update Cargo.toml with the latest stable versions of all dependencies
2. Fix ALL breaking changes in the source code to ensure compilation
3. Maintain existing functionality - the game should work identically after updates
4. Follow idiomatic patterns for the new Bevy version
5. Update any deprecated API usage to the recommended alternatives
</requirements>

<implementation>
Step-by-step process:
1. Research latest versions on crates.io
2. Update Cargo.toml with new versions
3. Run `cargo build` to identify breaking changes
4. Fix each compilation error, consulting migration guides as needed
5. Test that the application runs correctly with `cargo run`

For Bevy migration, pay attention to:
- System scheduling API changes
- Query syntax changes  
- Resource access patterns
- Gizmos API (if changed)
- Input handling (KeyCode changes)
- Window/PresentMode configuration
</implementation>

<output>
Modify the following files as needed:
- `./Cargo.toml` - updated dependency versions
- `./src/main.rs` - fix any breaking changes
- `./src/collisions.rs` - fix any breaking changes
- `./src/level.rs` - fix any breaking changes
</output>

<verification>
Before declaring complete:
1. Run `cargo build` - must compile without errors
2. Run `cargo run` - application must launch and be functional
3. Verify player movement and collision detection still work
4. Check that level rendering displays correctly
</verification>

<success_criteria>
- All dependencies updated to latest stable versions
- Code compiles without errors or warnings related to deprecated APIs
- Application runs and maintains all existing functionality
- Code follows idiomatic patterns for the updated dependency versions
</success_criteria>

