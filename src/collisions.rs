use crate::components::*;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

pub(super) fn add_collision_forces(
    mut set: ParamSet<(
        Query<(Entity, &mut DynamicPoint, &mut Transform)>,
        Query<(&Point, &Transform)>,
    )>,
    shapes: Query<&Shape>,
) {
    for shape in &shapes {
        let shape_points: Vec<_> = set
            .p1()
            .iter_many(&shape.points)
            .map(|(_, t)| t.translation.xy())
            .collect();

        let bounds = bounding_box(&shape_points);

        for (dyn_entity, mut dyn_point, mut dyn_transform) in &mut set.p0() {
            if shape.points.contains(&dyn_entity) {
                continue;
            }

            let dyn_pos = dyn_transform.translation.xy();
            if !bounds.contains(dyn_pos) {
                continue;
            }

            if point_in_shape(dyn_pos, &shape_points, bounds) == true {
                let new_position = closest_point_on_shape(dyn_pos, &shape_points);

                let vector = (new_position - dyn_transform.translation.xy()).normalize();
                dyn_point.velocity =
                    dyn_point.velocity - 2.0 * (dyn_point.velocity * vector) * vector;

                dyn_transform.translation = new_position.extend(0.0);
            }
        }
    }
}

fn bounding_box(shape: &Vec<Vec2>) -> Rect {
    let mut min = Vec2::new(f32::INFINITY, f32::INFINITY);
    let mut max = Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);

    for point in shape {
        if point.x < min.x {
            min.x = point.x;
        }
        if point.y < min.y {
            min.y = point.y;
        }
        if point.x > max.x {
            max.x = point.x;
        }
        if point.y > max.y {
            max.y = point.y;
        }
    }

    Rect { min, max }
}

fn point_in_shape(point: Vec2, shape: &Vec<Vec2>, bounds: Rect) -> bool {
    let test_point = Vec2::new(bounds.max.x + 1.0, point.y);

    let mut num_collisions = 0;
    for i in 0..shape.len() {
        let j = (i + 1) % shape.len();
        if intersect_line_segments(point, test_point, shape[i], shape[j]) {
            num_collisions += 1;
        }
    }

    num_collisions % 2 == 1
}

fn intersect_line_segments(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    let denom = (d.y - c.y) * (b.x - a.x) - (d.x - c.x) * (b.y - a.y);
    if denom == 0.0 {
        return false;
    }

    let ua = ((d.x - c.x) * (a.y - c.y) - (d.y - c.y) * (a.x - c.x)) / denom;
    let ub = ((b.x - a.x) * (a.y - c.y) - (b.y - a.y) * (a.x - c.x)) / denom;

    ua >= 0.0 && ua <= 1.0 && ub >= 0.0 && ub <= 1.0
}

fn closest_point_on_shape(point: Vec2, shape: &Vec<Vec2>) -> Vec2 {
    let mut closest_point = Vec2::ZERO;
    let mut closest_distance = f32::MAX;

    for i in 0..shape.len() {
        let a = shape[i];
        let b = shape[(i + 1) % shape.len()];

        let p = closest_point_on_line_segment(a, b, point);
        let distance = (point - p).length_squared();

        if distance < closest_distance {
            closest_distance = distance;
            closest_point = p;
        }
    }

    closest_point
}

fn closest_point_on_line_segment(a: Vec2, b: Vec2, p: Vec2) -> Vec2 {
    let ap = p - a;
    let ab = b - a;
    let ab_length = ab.length_squared();
    let ap_dot_ab = ap.dot(ab);

    let t = if ab_length > 0.0 {
        ap_dot_ab / ab_length
    } else {
        0.0
    };

    if t < 0.0 {
        a
    } else if t > 1.0 {
        b
    } else {
        a + ab * t
    }
}
