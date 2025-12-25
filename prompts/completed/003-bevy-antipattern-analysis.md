<objective>
Thoroughly analyze this small Bevy ECS game codebase for antipatterns and implement practical design pattern improvements.

**Important**: This is a ~1,100 line codebase for a 2D character controller game. Focus on practical improvements that make the code cleaner, more idiomatic Bevy/Rust, and easier to extend. Avoid over-engineering - no enterprise patterns, excessive abstraction layers, or generalization for hypothetical future requirements.
</objective>

<context>
**Project**: bevy-advanced-cc - A Bevy 0.17.3 game with advanced 2D character controller, custom physics, and collision detection.

**Source Files**:
- `./src/main.rs` - App setup, core systems, components, resources (~305 lines)
- `./src/collisions.rs` - CollisionPlugin, collision detection system (~207 lines)  
- `./src/level.rs` - Level loading, polygon generation (~581 lines)

**Tech Stack**: Rust 2021 + Bevy 0.17.3 + serde_json + rand

Read the project conventions first:
@AGENTS.md
@src/AGENTS.md
</context>

<analysis_requirements>
Thoroughly analyze the codebase for these categories of antipatterns:

**1. Bevy ECS Antipatterns**
- Inappropriate component/resource boundaries (data that should be grouped or separated)
- Systems doing too much (violation of single responsibility)
- Unnecessary resource mutability (`ResMut` where `Res` would suffice)
- Missing or excessive system ordering constraints
- Query inefficiencies (overly broad queries, missing filters)
- Startup vs Update system misplacements

**2. Rust Code Quality**
- Unnecessary allocations in hot paths (per-frame allocations)
- Magic numbers not captured as constants
- Code duplication that could be extracted into functions
- Overly complex control flow that could be simplified
- Missing or misleading documentation
- Commented-out code that should be removed

**3. Structural Issues**
- Module organization (is the split between files logical?)
- Public API boundaries (what should be pub vs private?)
- Dependency between modules (circular dependencies, tight coupling)

**4. Bevy-Idiomatic Opportunities**
- Where Bevy's built-in features could replace custom code
- Where events might improve decoupling
- Where marker components could simplify queries
</analysis_requirements>

<implementation_guidelines>
For each issue found, evaluate:
1. **Severity**: Is this a real problem or just a style preference?
2. **Scope**: How much code needs to change?
3. **Risk**: Could this change introduce bugs?
4. **Benefit**: What tangible improvement does this provide?

**DO implement improvements for:**
- Clear antipatterns with obvious fixes
- Code that's harder to understand than necessary
- Violations of Bevy best practices documented in AGENTS.md
- Magic numbers that should be constants
- Unnecessary allocations in per-frame systems
- Dead/commented-out code that should be removed

**DO NOT implement:**
- Architectural rewrites (keep the existing plugin structure)
- Dependency injection frameworks
- Event-driven architecture overhauls (unless clearly beneficial)
- Generic abstractions for single-use code
- Config file systems (compile-time constants are fine for this scope)
- Full error handling refactors (simple unwrap is acceptable for asset loading in a small game)
</implementation_guidelines>

<output>
**Phase 1: Analysis Document**
Create a brief analysis summary (not a separate file, just summarize findings before implementing).

**Phase 2: Implementation**
Apply improvements directly to the source files:
- `./src/main.rs`
- `./src/collisions.rs`  
- `./src/level.rs`

**Phase 3: Verification**
After implementing changes, run:
```bash
cargo fmt
cargo clippy --all-targets --all-features -D warnings
cargo build
```

Document what changed and why in your response.
</output>

<success_criteria>
- [ ] All three source files analyzed for antipatterns
- [ ] Practical improvements implemented (not over-engineered)
- [ ] Code compiles without errors
- [ ] No new clippy warnings introduced
- [ ] Game functionality preserved (no behavioral changes unless fixing bugs)
- [ ] Changes documented with clear rationale
</success_criteria>

<constraints>
- **Preserve existing system naming convention** (`s_` prefix)
- **Preserve existing module structure** (3 files)
- **Preserve CollisionPlugin pattern** (plugin-first architecture)
- **Use Bevy 0.17.3 APIs** (see tech-stack in AGENTS.md for specific API patterns)
- **Remove large blocks of commented-out code** (level.rs has ~200 lines of dead code)
- **Keep improvements proportional** to the codebase size - this is a small project
</constraints>

