<objective>
Fix all `cargo clippy` linting errors and warnings in the codebase to achieve a clean lint output.

This ensures code quality, consistency, and adherence to Rust best practices. The project treats clippy warnings as errors (`-D warnings`), so all issues must be resolved.
</objective>

<context>
This is a Bevy 0.17.3 game project with a plugin-first architecture.

Key source files:
- `src/main.rs` - Main entry point, system registration
- `src/collisions.rs` - Collision detection plugin
- `src/level.rs` - Level loading and geometry

Refer to the project's AGENTS.md for conventions and the tech stack rules for Bevy 0.17.3 API patterns.
</context>

<requirements>
1. Run `cargo clippy --all-targets --all-features -- -D warnings` to identify all issues
2. Fix each warning/error while preserving existing functionality
3. Follow Rust idioms and the project's existing code style
4. Do not change game behavior - only fix lint issues
5. Ensure fixes are compatible with Bevy 0.17.3 API
</requirements>

<implementation>
Common clippy fixes to apply:
- Use `if let` instead of `match` with single pattern
- Remove unnecessary `clone()` calls
- Use `unwrap_or_default()` where appropriate
- Fix unused variables (prefix with `_` or remove)
- Use proper iterator methods instead of manual loops
- Fix mutable reference patterns
- Address any deprecated API usage

What to avoid:
- Do not suppress warnings with `#[allow(...)]` unless absolutely necessary
- Do not refactor beyond what's needed for the fix
- Do not change public API signatures unless required by clippy
</implementation>

<steps>
1. Run clippy and capture all warnings/errors:
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings 2>&1
   ```

2. For each issue identified:
   - Read the relevant source file
   - Understand the context of the warning
   - Apply the minimal fix that resolves the issue
   - Ensure the fix maintains code clarity

3. After fixing all issues, run clippy again to verify clean output

4. Run `cargo build` to ensure the project still compiles

5. Optionally run `cargo test` if tests exist to verify no regressions
</steps>

<output>
Modify the necessary source files in `./src/` to resolve all clippy warnings.

No new files should be created - only existing files should be modified.
</output>

<verification>
Before declaring complete, verify your work:

1. Run `cargo clippy --all-targets --all-features -- -D warnings` and confirm zero warnings/errors
2. Run `cargo build` and confirm successful compilation
3. Confirm no functional changes were made (only lint fixes)
</verification>

<success_criteria>
- `cargo clippy --all-targets --all-features -- -D warnings` exits with code 0 and produces no warnings
- `cargo build` succeeds without errors
- All changes are minimal and targeted to specific clippy issues
- Code style remains consistent with the rest of the codebase
</success_criteria>

