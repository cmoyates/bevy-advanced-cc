<objective>
Diagnose and fix an intermittent hang/freeze when pressing Escape to exit the game.

The bug: When pressing Escape to exit, the game sometimes hangs or stops responding instead of exiting cleanly. This happens most of the time but not every time, suggesting a timing or race condition issue.
</objective>

<context>
This is a Bevy 0.17.3 game project with a custom 2D character controller.

Current exit implementation in `src/main.rs`:
- Uses `MessageWriter<AppExit>` (Bevy 0.17.3 API)
- Writes `AppExit::Success` when Escape is pressed
- Located in `s_input` system

System execution order:
1. `s_input` - handles input and exit
2. `s_movement` (after s_input)
3. `s_collision` (after s_movement, via CollisionPlugin)
4. `s_timers` (after s_collision)
5. `s_render` (after s_timers)

Window configuration:
- `PresentMode::AutoNoVsync` is used

@src/main.rs - contains exit logic in s_input
@src/collisions.rs - CollisionPlugin and collision system
</context>

<research>
Thoroughly investigate the following potential causes:

1. **MessageWriter vs EventWriter**: Verify `MessageWriter<AppExit>` is the correct API for Bevy 0.17.3. Check if there's a different pattern for app exit.

2. **System ordering with exit**: Does sending AppExit during a system cause other systems to still run, potentially blocking?

3. **PresentMode interaction**: Could `AutoNoVsync` interact poorly with exit? Does the rendering loop need to complete before exit processes?

4. **Bevy 0.17 exit patterns**: Research the recommended way to exit a Bevy 0.17.3 application. Look for:
   - Any known issues with AppExit
   - Whether to use a dedicated exit system
   - If exit should happen in a specific schedule

5. **Resource/system blocking**: Could any system be waiting on something that blocks exit?

Use `btca ask -t bevy -q "how to properly exit a bevy 0.17 application"` and similar queries to research the correct patterns.
</research>

<diagnosis_steps>
1. Add debug output to trace when AppExit is written and when the app actually exits
2. Check if the hang occurs during a specific system (add timing/logging)
3. Test if removing `AutoNoVsync` changes the behavior
4. Test if moving exit handling to a separate, later system changes behavior
5. Check if the hang is related to specific frame timing
</diagnosis_steps>

<implementation>
After diagnosis, implement a fix. Potential approaches to consider:

1. **Dedicated exit system**: Move exit handling to its own system that runs after all other update systems
2. **Different exit pattern**: Use a different API or pattern if MessageWriter isn't appropriate
3. **Schedule change**: Consider if exit should be in a different schedule (e.g., PostUpdate)
4. **Resource flag pattern**: Set a flag resource, then exit in a later system

Whatever fix is implemented:
- Must work 100% of the time (no intermittent hangs)
- Should exit cleanly without visible delay
- Should follow Bevy 0.17.3 best practices
- Keep the code simple and maintainable
</implementation>

<output>
Modify files as needed:
- `./src/main.rs` - fix the exit logic

If the fix requires new resources or systems, add them following project conventions:
- Systems prefixed with `s_`
- Resources use `#[derive(Resource)]`
</output>

<verification>
Before declaring complete:

1. Run the game with `cargo run`
2. Test pressing Escape immediately after launch (within 1 second)
3. Test pressing Escape during gameplay (while moving/jumping)
4. Test pressing Escape while idle
5. Repeat each test at least 5 times to ensure the hang is fully resolved
6. Run `cargo clippy --all-targets --all-features -D warnings` to ensure no warnings
7. Confirm the game exits cleanly and promptly in all cases
</verification>

<success_criteria>
- Game exits cleanly 100% of the time when Escape is pressed
- No visible delay between pressing Escape and the window closing
- Code follows Bevy 0.17.3 best practices
- No clippy warnings introduced
- Exit works regardless of game state (idle, moving, jumping, colliding)
</success_criteria>

