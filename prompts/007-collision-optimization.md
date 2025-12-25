<objective>
Optimize the collision detection system for performance, fix collision-related bugs, and polish the overall movement feel. This is the final phase focusing on performance and polish.
</objective>

<context>
This is Phase 4 (final phase) of the physics system improvement. Prerequisites:
- `./docs/physics-analysis.md` - Phase 1 analysis
- Phase 2 completed - Data structures refactored
- Phase 3 completed - Delta time physics implemented

Key collision issues from Phase 1:
- O(n*m) iteration over all polygons and line segments
- Unused `intersection_points` vector allocation every frame
- Potential tunneling at high velocities
- Debug gizmos mixed with collision logic

Tech stack: Bevy 0.17.3, Rust 2021 edition.
</context>

<requirements>
Implement the following optimizations and improvements:

1. **Remove Unused Code**
   - `intersection_points` vector is allocated but never used meaningfully
   - Clean up any other dead code identified in Phase 1

2. **Reduce Per-Frame Allocations**
   - Avoid `Vec::new()` in hot loops
   - Consider pre-allocated buffers or capacity hints
   - Profile-guided optimization where applicable

3. **Spatial Partitioning (if beneficial)**
   - Consider simple grid-based spatial hashing for large levels
   - Only implement if level size warrants it
   - AABB pre-check before detailed collision

4. **Broad Phase Optimization**
   - Add AABB (bounding box) check before line-segment tests
   - Early-out when player is far from polygon
   - Cache polygon bounding boxes in Level resource

5. **Collision Resolution Improvements**
   - Review collision response for edge cases
   - Ensure no jittering or vibration
   - Handle corner cases (literal corners) better

6. **Separate Debug Rendering**
   - Move debug gizmo drawing out of collision system
   - Create optional debug system that runs after collision
   - Use feature flag or runtime toggle for debug rendering

7. **Movement Feel Polish**
   - Tune acceleration/deceleration curves
   - Ensure responsive controls
   - Smooth out any remaining rough edges
   - Consider adding subtle features like:
     - Landing squash anticipation
     - Apex hang time (reduced gravity at jump peak)
     - Better air control tuning
</requirements>

<constraints>
- Maintain correctness - optimizations must not break collision
- Don't over-engineer - simple optimizations first
- Keep code readable - avoid micro-optimizations that hurt clarity
- Profile before optimizing - identify actual bottlenecks
- Preserve the improved delta-time physics from Phase 3
</constraints>

<implementation>
1. Review Phase 1 analysis for collision-specific issues
2. Remove unused `intersection_points` code
3. Add AABB pre-check for polygons
4. Consider caching polygon bounds in Level resource
5. Separate debug gizmos into optional system
6. Test performance with timing
7. Polish movement feel

Performance testing approach:
- Use `std::time::Instant` to measure collision system duration
- Compare before/after optimization
- Test with varying level complexity
</implementation>

<output>
Modified files:
- `./src/collisions.rs` - Optimized collision detection
- `./src/main.rs` - Polish and optional debug system
- `./src/level.rs` - AABB caching if implemented
</output>

<verification>
Before completing:
- [ ] `cargo build` succeeds
- [ ] `cargo clippy --all-targets --all-features -D warnings` passes
- [ ] Unused code removed
- [ ] No unnecessary allocations in collision loop
- [ ] AABB pre-check implemented
- [ ] Debug rendering separated from collision logic
- [ ] Collision behavior correct (no new bugs)
- [ ] Movement feels responsive and polished
</verification>

<success_criteria>
- Collision system is measurably faster
- No per-frame allocations in hot paths
- Debug rendering is cleanly separated
- Movement feels tight and responsive
- Code is clean, well-documented, and maintainable
- All clippy warnings resolved
</success_criteria>

<final_checklist>
After completing all 4 phases, the physics system should have:
- [ ] Frame-rate independent physics
- [ ] Clean component separation (Player vs Physics)
- [ ] Time-based timers (seconds, not frames)
- [ ] Proper floating point handling
- [ ] Optimized collision detection
- [ ] Separated debug rendering
- [ ] Well-documented constants with units
- [ ] Responsive, polished controls
</final_checklist>

