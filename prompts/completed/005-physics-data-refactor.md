<objective>
Refactor the player physics data structures, components, and constants based on the analysis from Phase 1. Improve separation of concerns, eliminate data coupling, and prepare the foundation for improved physics integration.
</objective>

<context>
This is Phase 2 of the physics system improvement. Before starting, read:
- `./docs/physics-analysis.md` - The Phase 1 analysis document
- `src/main.rs` - Current components and constants
- `src/collisions.rs` - Current collision system

This phase focuses on DATA STRUCTURE improvements only. Do not change physics calculations or collision algorithms yet - those come in later phases.

Tech stack: Bevy 0.17.3, Rust 2021 edition.
</context>

<requirements>
Thoroughly analyze the Phase 1 document, then implement these structural improvements:

1. **Separate Timer Data from Frame Counts**
   - Create a proper timer abstraction using `f32` seconds instead of `i32` frame counts
   - This enables frame-rate independent physics in Phase 3
   - Consider a `JumpBuffer`, `CoyoteTime`, `WallCoyoteTime` approach

2. **Decouple Walled Direction from Timer**
   - Currently `walled_timer` sign encodes wall direction (coupling)
   - Separate into distinct fields: timer value and wall direction

3. **Organize Physics Constants**
   - Group related constants logically
   - Consider a `PhysicsConfig` resource for runtime tuning
   - Document units (pixels/frame vs pixels/second)

4. **Component Responsibility Cleanup**
   - Review what belongs in `Player` vs `Physics` components
   - `Player` should be gameplay state (timers, jump state)
   - `Physics` should be pure physics state (velocity, position, collision)

5. **Add Missing State**
   - Consider adding `is_grounded: bool` for cleaner checks
   - Consider `last_wall_normal: Option<Vec2>` for wall jump direction

6. **Improve Type Safety**
   - Replace magic `i32` signs with enums where appropriate
   - Use newtypes for distinct concepts if beneficial
</requirements>

<constraints>
- Maintain backward compatibility with existing system logic (systems should still work after refactor)
- Do NOT change physics calculations yet - only data structures
- Do NOT change collision algorithm - only the data it reads/writes
- Follow existing code style (s_ prefix for systems, PascalCase components)
- Keep changes minimal and focused on data structures
- Update all systems that use the changed data structures to compile and work correctly
</constraints>

<implementation>
1. First, read the Phase 1 analysis document thoroughly
2. Create/modify components in `src/main.rs`
3. Update all systems that reference changed fields
4. Update collision system in `src/collisions.rs` for new data structures
5. Ensure `cargo build` succeeds
6. Ensure `cargo clippy` passes

Testing approach:
- Run `cargo build` after each significant change
- Run `cargo clippy --all-targets --all-features -D warnings` at the end
- Manual playtest to verify behavior is unchanged
</implementation>

<output>
Modified files:
- `./src/main.rs` - Updated components, constants, and systems
- `./src/collisions.rs` - Updated to use new data structures
</output>

<verification>
Before completing:
- [ ] `cargo build` succeeds
- [ ] `cargo clippy --all-targets --all-features -D warnings` passes
- [ ] All systems compile and use new data structures
- [ ] Timer values are now `f32` (seconds) not `i32` (frames)
- [ ] Wall direction is separate from wall timer
- [ ] Constants are organized and documented
- [ ] Player component has gameplay state, Physics has physics state
</verification>

<success_criteria>
- Code compiles without warnings
- Gameplay behavior is unchanged (systems adapted correctly)
- Data structures are cleaner and ready for delta-time physics
- No remaining coupling between unrelated concerns
- Constants are well-organized with clear documentation
</success_criteria>

