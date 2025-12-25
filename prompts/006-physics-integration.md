<objective>
Implement proper frame-rate independent physics using delta time, improve the integration method, and fix physics calculation issues identified in Phase 1. This phase transforms the physics from frame-based to time-based.
</objective>

<context>
This is Phase 3 of the physics system improvement. Prerequisites:
- `./docs/physics-analysis.md` - Phase 1 analysis (reference for issues)
- Phase 2 completed - Data structures are refactored with f32 timers

Key physics issues to address:
- Currently uses frame-based physics (assumes 60 FPS)
- Uses basic Euler integration
- Floating point comparisons like `== 0.0`
- Gravity applied additively to acceleration (can compound)
- No delta time usage

Tech stack: Bevy 0.17.3, Rust 2021 edition.
</context>

<requirements>
Implement the following physics improvements:

1. **Delta Time Integration**
   - Use `Res<Time>` to get `time.delta_secs()` (Bevy 0.17.3 API)
   - All physics calculations must scale by delta time
   - Velocity units become pixels/second (not pixels/frame)
   - Acceleration units become pixels/second² (not pixels/frame²)

2. **Improved Integration Method**
   - Current: `velocity += acceleration; position += velocity` (Euler)
   - Implement semi-implicit Euler or Verlet for stability
   - Separate acceleration application from position update

3. **Fix Floating Point Comparisons**
   - Replace `== 0.0` with epsilon comparisons or `length_squared() < EPSILON`
   - Use appropriate epsilon values for different comparisons
   - Consider using `Vec2::ZERO.abs_diff_eq()` where available

4. **Gravity Improvements**
   - Gravity should be applied consistently each frame
   - Remove additive gravity to acceleration (apply directly to velocity)
   - Consider gravity as a force, not accumulated acceleration

5. **Timer Updates**
   - Timers now use seconds (f32) - update using delta time
   - Jump buffer, coyote time, wall coyote time all use delta
   - Remove frame-based timer constants, use second-based ones

6. **Remove Frame Rate Capping System**
   - The `s_wait_for_next_frame` system is a workaround for frame-based physics
   - With proper delta time, this becomes unnecessary
   - Let Bevy/VSync handle frame timing

7. **Update Constants**
   - Convert all physics constants to per-second units
   - Example: `PLAYER_MAX_SPEED` from 5.0 pixels/frame to 300.0 pixels/second
   - Document units in comments
</requirements>

<constraints>
- Use Bevy 0.17.3 Time API: `time.delta_secs()` (not `delta().as_secs_f32()`)
- Maintain gameplay feel as close as possible to original
- Constants may need tuning after conversion - that's expected
- Do NOT change collision detection algorithm yet (Phase 4)
- Keep system ordering intact
</constraints>

<implementation>
1. Read Phase 1 analysis for specific issues to address
2. Update `s_movement` system with delta time
3. Update `s_timers` system with delta time
4. Update physics constants with correct units
5. Remove `s_wait_for_next_frame` system
6. Fix floating point comparisons throughout
7. Test and tune constants for similar feel

Suggested constant conversions (assuming 60 FPS baseline):
- Velocity: multiply by 60 (frames/second)
- Acceleration: multiply by 3600 (60²)
- Timer values: divide by 60 (convert frames to seconds)
</implementation>

<output>
Modified files:
- `./src/main.rs` - Updated physics systems and constants
</output>

<verification>
Before completing:
- [ ] `cargo build` succeeds
- [ ] `cargo clippy --all-targets --all-features -D warnings` passes
- [ ] No `== 0.0` comparisons remain (use epsilon)
- [ ] All physics calculations use delta time
- [ ] Constants documented with units (pixels/second, etc.)
- [ ] `s_wait_for_next_frame` removed
- [ ] Game runs at variable frame rates correctly
- [ ] Gameplay feel is similar to original (may need tuning)
</verification>

<success_criteria>
- Physics are frame-rate independent
- Game behaves consistently at 30, 60, 120+ FPS
- No floating point comparison issues
- Constants have clear unit documentation
- Codebase is simpler (no manual frame capping)
- Gameplay feel preserved or improved
</success_criteria>

