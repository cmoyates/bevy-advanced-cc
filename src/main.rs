mod collisions;
mod level;

use std::{thread::sleep, time::Duration};

use ::bevy::prelude::*;
use bevy::{app::AppExit, input::ButtonInput, window::PresentMode};
use collisions::{s_collision, CollisionPlugin};
use level::{generate_level_polygons, Polygon};

const FRAMERATE: u32 = 60;
const FRAME_DURATION_SECS: f32 = 1.0 / FRAMERATE as f32;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(InputDir { dir: Vec2::ZERO })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Advanced Character Controller".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CollisionPlugin)
        // Startup systems
        .add_systems(Startup, s_init)
        // Update systems
        .add_systems(Update, s_input)
        .add_systems(Update, s_movement.after(s_input))
        .add_systems(Update, s_timers.after(s_collision))
        .add_systems(Update, s_render.after(s_timers))
        .add_systems(Update, s_wait_for_next_frame.after(s_render))
        .run();
}

#[derive(Resource)]
pub struct Level {
    pub polygons: Vec<Polygon>,
}

#[derive(Resource)]
pub struct InputDir {
    pub dir: Vec2,
}

pub const PLAYER_MAX_SPEED: f32 = 5.0;
pub const PLAYER_ACCELERATION_SCALERS: (f32, f32) = (0.2, 0.4);

pub const MAX_JUMP_TIMER: i32 = 10;
pub const MAX_GROUNDED_TIMER: i32 = 10;
pub const MAX_WALLED_TIMER: i32 = 10;

// Physics constants
pub const JUMP_VELOCITY: f32 = 9.0;
pub const WALL_JUMP_VELOCITY_Y: f32 = 4.5;
pub const WALL_JUMP_VELOCITY_X: f32 = 7.8;
pub const GRAVITY_STRENGTH: f32 = 0.5;
pub const WALL_JUMP_ACCELERATION_REDUCTION: f32 = 0.5;
pub const JUMP_RELEASE_VELOCITY_DIVISOR: f32 = 3.0;

// Collision detection thresholds
pub const NORMAL_DOT_THRESHOLD: f32 = 0.8;
pub const GROUND_NORMAL_Y_THRESHOLD: f32 = 0.01;
pub const CEILING_NORMAL_Y_THRESHOLD: f32 = -0.01;

#[derive(Component)]
pub struct Player {
    jump_timer: i32,
    grounded_timer: i32,
    walled_timer: i32,
    has_wall_jumped: bool,
}

#[derive(Component)]
pub struct Physics {
    pub prev_position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub radius: f32,
    pub normal: Vec2,
}

/// Initial setup system
pub fn s_init(mut commands: Commands) {
    // Spawn camera
    commands.spawn((Camera2d, Transform::default()));

    // Spawn player
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, -50.0, 0.0)),
        Physics {
            prev_position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            radius: 12.0,
            normal: Vec2::ZERO,
        },
        Player {
            jump_timer: 0,
            grounded_timer: 0,
            walled_timer: 0,
            has_wall_jumped: false,
        },
    ));

    // Init level
    {
        let grid_size = 32.0;

        let level_polygons = generate_level_polygons(grid_size);

        commands.insert_resource(Level {
            polygons: level_polygons,
        });
    }
}

/// Input system
pub fn s_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
    mut input_dir: ResMut<InputDir>,
    mut player_query: Query<(&mut Player, &mut Physics)>,
) {
    if let Ok((mut player_data, mut player_physics)) = player_query.single_mut() {
        let mut direction = Vec2::ZERO;

        // Escape to exit
        if keyboard_input.just_pressed(KeyCode::Escape) {
            exit.write(AppExit::Success);
        }

        // Arrow keys to move
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        // Space to jump
        if keyboard_input.just_pressed(KeyCode::Space) {
            player_data.jump_timer = MAX_JUMP_TIMER;
        }

        if keyboard_input.just_released(KeyCode::Space) && player_physics.velocity.y > 0.0 {
            player_physics.velocity.y /= JUMP_RELEASE_VELOCITY_DIVISOR;
        }

        // Normalize direction
        direction = direction.normalize_or_zero();

        // Set direction resource
        input_dir.dir = direction;
    }
}

/// Movement system
pub fn s_movement(
    mut player_query: Query<(&mut Transform, &mut Physics, &mut Player)>,
    input_dir: Res<InputDir>,
) {
    if let Ok((mut player_transform, mut player_physics, mut player_data)) =
        player_query.single_mut()
    {
        let player_falling = player_physics.normal.length_squared() == 0.0;
        let no_input = input_dir.dir.length_squared() == 0.0;

        // Rotate input according to the normal (compute locally, don't mutate resource)
        let mut effective_input_dir = input_dir.dir;
        if !no_input
            && !player_falling
            && input_dir.dir.dot(player_physics.normal).abs() < NORMAL_DOT_THRESHOLD
        {
            let mut new_input_dir = Vec2::new(player_physics.normal.y, -player_physics.normal.x);

            if new_input_dir.dot(input_dir.dir) < 0.0 {
                new_input_dir *= -1.0;
            }

            effective_input_dir = new_input_dir;
        }

        // If the player is on a wall and is trying to move away from it
        let player_move_off_wall = player_physics.normal.x.abs() >= NORMAL_DOT_THRESHOLD
            && effective_input_dir.x.abs() >= NORMAL_DOT_THRESHOLD
            && player_physics.normal.x.signum() != effective_input_dir.x.signum();

        // Acceleration
        {
            // Apply acceleration
            player_physics.acceleration = (effective_input_dir * PLAYER_MAX_SPEED
                - player_physics.velocity)
                * if no_input {
                    // Deacceleration
                    PLAYER_ACCELERATION_SCALERS.1
                } else {
                    // Acceleration
                    PLAYER_ACCELERATION_SCALERS.0
                };

            // Wall jump physics
            player_physics.acceleration *= if player_data.has_wall_jumped {
                WALL_JUMP_ACCELERATION_REDUCTION
            } else {
                1.0
            };

            // If the player is falling
            if player_falling {
                // Ignore any other acceleration in the y direction
                player_physics.acceleration.y = 0.0;
            }
            // Unless the player is on a wall and is trying to move away from it
            if !player_move_off_wall {
                // Remove the acceleration in the direction of the normal
                let acceleration_adjustment =
                    player_physics.normal * player_physics.acceleration.dot(player_physics.normal);
                player_physics.acceleration -= acceleration_adjustment;
            }
        }

        // Gravity
        {
            if player_move_off_wall || player_falling {
                // Gravity goes down
                player_physics.acceleration.y = -GRAVITY_STRENGTH;
            } else {
                // Gravity goes towards the normal
                let gravity_normal_dir = player_physics.normal * GRAVITY_STRENGTH;
                player_physics.acceleration += gravity_normal_dir;
            }
        }

        // Jumping
        {
            // If the player is trying to jump
            if player_data.jump_timer > 0 {
                // If on the ground
                if player_data.grounded_timer > 0 {
                    // Jump
                    player_physics.velocity.y = JUMP_VELOCITY;
                    player_data.jump_timer = 0;
                    player_data.grounded_timer = 0;
                }
                // If on a wall
                else if player_data.walled_timer != 0 {
                    // Wall jump
                    player_physics.velocity.y = WALL_JUMP_VELOCITY_Y;
                    player_physics.velocity.x =
                        player_data.walled_timer.signum() as f32 * WALL_JUMP_VELOCITY_X;
                    player_data.jump_timer = 0;
                    player_data.walled_timer = 0;
                    player_data.has_wall_jumped = true;
                }
            }
        }

        // Update physics
        player_physics.prev_position = player_transform.translation.xy();
        let new_velocity = player_physics.velocity + player_physics.acceleration;
        player_physics.velocity = new_velocity;
        player_transform.translation.x += player_physics.velocity.x;
        player_transform.translation.y += player_physics.velocity.y;
    }
}

/// Render system
pub fn s_render(
    mut gizmos: Gizmos,
    player_query: Query<(&Transform, &Physics), With<Player>>,
    level: Res<Level>,
) {
    if let Ok((player_transform, player_physics)) = player_query.single() {
        // Draw player
        gizmos.circle_2d(
            player_transform.translation.xy(),
            player_physics.radius,
            Color::WHITE,
        );

        // Draw level
        for polygon in &level.polygons {
            gizmos.linestrip_2d(polygon.points.iter().copied(), polygon.color);
        }
    }
}

pub fn s_timers(mut player_query: Query<&mut Player>) {
    if let Ok(mut player_data) = player_query.single_mut() {
        if player_data.jump_timer > 0 {
            player_data.jump_timer -= 1;
        }
        if player_data.grounded_timer > 0 {
            player_data.grounded_timer -= 1;
        }
        if player_data.walled_timer.abs() > 0 {
            player_data.walled_timer -= player_data.walled_timer.signum();
        }
    }
}

/// Framerate capping system
pub fn s_wait_for_next_frame(time: Res<Time>) {
    // If not running in wasm
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Sleep to cap framerate
        let time_to_sleep = FRAME_DURATION_SECS - time.delta().as_secs_f32();
        if time_to_sleep > 0.0 {
            sleep(Duration::from_secs_f32(time_to_sleep));
        }
    }
}
