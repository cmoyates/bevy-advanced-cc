use bevy::{
    app::{App, Plugin, Update},
    color::Color,
    ecs::{
        schedule::IntoScheduleConfigs,
        system::{Query, Res},
    },
    gizmos::gizmos::Gizmos,
    math::{Vec2, Vec3Swizzles},
    transform::components::Transform,
};

use crate::{s_movement, Level, Physics, Player, MAX_GROUNDED_TIMER, MAX_WALLED_TIMER};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, s_collision.after(s_movement));
    }
}

pub fn s_collision(
    mut player_query: Query<(&mut Transform, &mut Physics, &mut Player)>,
    level: Res<Level>,
    mut gizmos: Gizmos,
) {
    if let Ok((mut player_transform, mut player_physics, mut player_data)) =
        player_query.single_mut()
    {
        let mut adjustment = Vec2::ZERO;

        let mut new_player_normal = Vec2::ZERO;

        for polygon in &level.polygons {
            let mut intersect_counter = 0;
            let mut colliding_with_polygon = false;

            let mut intersection_points: Vec<Vec2> = Vec::new();

            for i in 1..polygon.points.len() {
                let start = polygon.points[i - 1];
                let end = polygon.points[i];

                let intersection = line_intersect(
                    start,
                    end,
                    player_transform.translation.xy(),
                    player_transform.translation.xy() + Vec2::new(2.0, 1.0) * 10000.0,
                );

                if let Some(point) = intersection {
                    intersection_points.push(point);
                    intersect_counter += 1;
                }

                let previous_side_of_line =
                    side_of_line_detection(start, end, player_physics.prev_position);

                if previous_side_of_line != polygon.collision_side {
                    continue;
                }

                let (distance_sq, projection) = find_projection(
                    start,
                    end,
                    player_transform.translation.xy(),
                    player_physics.radius,
                );

                let colliding_with_line = distance_sq <= player_physics.radius.powi(2);
                colliding_with_polygon = colliding_with_polygon || colliding_with_line;

                let touching_line = distance_sq <= (player_physics.radius + 0.5).powi(2);

                if touching_line {
                    let normal_dir =
                        (player_transform.translation.xy() - projection).normalize_or_zero();

                    // If the line is not above the player
                    if normal_dir.y >= -0.01 {
                        gizmos.line_2d(
                            player_transform.translation.xy(),
                            player_transform.translation.xy() - normal_dir * 12.0,
                            Color::WHITE,
                        );

                        // Add the normal dir to the players new normal
                        new_player_normal -= normal_dir;

                        // If the player is on a wall
                        if normal_dir.x.abs() >= 0.8 {
                            player_data.walled_timer = MAX_WALLED_TIMER * normal_dir.x as i32;
                            player_data.has_wall_jumped = false;
                        }

                        // If the player is on the ground
                        if normal_dir.y > 0.01 {
                            player_data.grounded_timer = MAX_GROUNDED_TIMER;
                            player_data.walled_timer = 0;
                            player_data.has_wall_jumped = false;
                        }
                    }
                }

                if colliding_with_line {
                    let mut delta =
                        (player_transform.translation.xy() - projection).normalize_or_zero();

                    if delta.y < -0.01 {
                        // println!("Hit ceiling");
                        // dbg!(delta);
                        player_physics.velocity.y = 0.0;
                    }

                    delta *= player_physics.radius - distance_sq.sqrt();

                    if delta.x.abs() > adjustment.x.abs() {
                        adjustment.x = delta.x;
                    }
                    if delta.y.abs() > adjustment.y.abs() {
                        adjustment.y = delta.y;
                    }
                }
            }
            if colliding_with_polygon && intersect_counter % 2 == 1 {
                player_transform.translation = player_physics.prev_position.extend(0.0);
            }
        }

        // Update the players normal
        new_player_normal = new_player_normal.normalize_or_zero();
        player_physics.normal = new_player_normal;

        // Remove the players velocity in the direction of the normal
        let velocity_adjustment =
            player_physics.velocity.dot(new_player_normal) * new_player_normal;

        // dbg!(player_physics.normal);

        player_physics.velocity -= velocity_adjustment;

        // Update the players position
        player_transform.translation += adjustment.extend(0.0);
    }
}

pub fn find_projection(start: Vec2, end: Vec2, point: Vec2, radius: f32) -> (f32, Vec2) {
    let point_vec = point - start;
    let line_vec = end - start;

    let line_vec_normalized = line_vec.normalize();

    let dot = point_vec.dot(line_vec_normalized);

    let projection_point = line_vec_normalized * dot + start;

    if dot < 0.0 {
        return (point_vec.length_squared() + radius * 2.0, projection_point);
    }

    if dot.powi(2) > (end - start).length_squared() {
        return (
            (point - end).length_squared() + radius * 2.0,
            projection_point,
        );
    }

    let dist = (point - projection_point).length_squared();

    (dist, projection_point)
}

pub fn side_of_line_detection(line_start: Vec2, line_end: Vec2, point: Vec2) -> f32 {
    let determinant = (line_end.x - line_start.x) * (point.y - line_start.y)
        - (line_end.y - line_start.y) * (point.x - line_start.x);

    determinant.signum()
}

pub fn line_intersect(
    line_1_start: Vec2,
    line_1_end: Vec2,
    line_2_start: Vec2,
    line_2_end: Vec2,
) -> Option<Vec2> {
    let line_1 = line_1_end - line_1_start;
    let line_2 = line_2_end - line_2_start;
    let r_cross_s = cross_product(line_1, line_2);
    let a_to_c = line_2_start - line_1_start;
    let t = cross_product(a_to_c, line_2) / r_cross_s;
    let u = cross_product(a_to_c, line_1) / r_cross_s;

    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        Some(Vec2::new(
            line_1_start.x + t * line_1.x,
            line_1_start.y + t * line_1.y,
        ))
    } else {
        None
    }
}

pub fn cross_product(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}
