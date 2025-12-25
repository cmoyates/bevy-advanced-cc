<objective>
Thoroughly analyze the current player movement, control, and physics system to identify all performance issues, logic bugs, code smells, and areas for improvement. Create a detailed analysis document that will guide subsequent refactoring phases.
</objective>

<context>
This is a Bevy 0.17.3 2D platformer character controller with:
- Custom physics (no physics engine)
- Circle-vs-polygon collision detection
- Wall jumping, coyote time, variable jump height
- Acceleration-based movement with surface normal alignment

Key files to analyze:
- `src/main.rs` - Movement system, input handling, physics components, timers
- `src/collisions.rs` - Collision detection, position correction, velocity adjustment
- `src/level.rs` - Level geometry generation (for understanding collision data)

This analysis will be used to guide refactoring in subsequent phases. Be rigorous and identify everything that could be improved.
</context>

<analysis_requirements>
Deeply analyze the following aspects:

1. **Performance Issues**
   - Unnecessary allocations in hot paths (per-frame allocations)
   - Computational complexity of collision detection
   - Cache-unfriendly data access patterns
   - Redundant calculations
   - Memory layout concerns

2. **Physics Logic Issues**
   - Integration method quality (currently Euler)
   - Frame-rate independence (or lack thereof)
   - Floating point comparison issues (e.g., `== 0.0`)
   - Gravity application correctness
   - Velocity/acceleration handling

3. **Code Architecture Issues**
   - Separation of concerns (timers stored as frame counts vs time)
   - Data coupling (e.g., walled_timer sign encoding direction)
   - Component responsibilities overlap
   - Magic numbers and unclear constants
   - Missing abstractions

4. **Collision System Issues**
   - Unused code (e.g., `intersection_points` vector)
   - Edge cases in collision resolution
   - Normal calculation correctness
   - Tunneling prevention

5. **Input/Control Issues**
   - Input buffering quality
   - Responsiveness concerns
   - State machine clarity

6. **Bevy-Specific Issues**
   - System ordering dependencies
   - Query efficiency
   - Resource usage patterns
</analysis_requirements>

<output_format>
Create a comprehensive analysis document with:

1. **Executive Summary** - Top 5 most critical issues
2. **Performance Analysis** - Detailed performance issues with severity ratings
3. **Logic Analysis** - Physics and gameplay logic issues
4. **Architecture Analysis** - Code structure and design issues
5. **Collision System Analysis** - Specific collision-related issues
6. **Recommended Fix Priority** - Ordered list of what to fix first
7. **Refactoring Roadmap** - High-level plan for subsequent phases

For each issue, include:
- Description of the problem
- Why it matters (performance impact, bug potential, maintainability)
- Suggested fix approach
- Severity: Critical / High / Medium / Low

Save analysis to: `./docs/physics-analysis.md`
</output_format>

<verification>
Before completing, verify:
- All three source files have been thoroughly examined
- Every system function has been analyzed
- All constants have been reviewed for appropriateness
- Performance hot paths have been identified
- The analysis provides actionable guidance for Phase 2
</verification>

<success_criteria>
- Document identifies at least 15 distinct issues across categories
- Each issue has clear description, impact, and suggested fix
- Priority ordering is logical and justified
- Analysis is specific to this codebase, not generic advice
- Document can serve as a checklist for subsequent refactoring phases
</success_criteria>

