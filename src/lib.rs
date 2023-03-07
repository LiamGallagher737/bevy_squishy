#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::{math::Vec3Swizzles, prelude::*};

pub use components::*;

mod components;

/// Adds surport for 2D soft bodies in your application
pub struct SquishyPlugin {
    /// The gravity to apply to every soft body, use [`None`] to disable gravity
    pub gravity: Option<Vec2>,
}

impl Default for SquishyPlugin {
    fn default() -> Self {
        Self {
            gravity: Some(Vec2::Y * -9.81),
        }
    }
}

impl Plugin for SquishyPlugin {
    fn build(&self, app: &mut App) {
        if let Some(gravity) = self.gravity {
            app.insert_resource(Gravity(gravity));
        }

        app.add_systems(
            (
                add_gravity_force.run_if(resource_exists::<Gravity>()),
                add_spring_force,
            )
                .in_set(UpdateForcesSet)
                .in_schedule(CoreSchedule::FixedUpdate),
        );

        app.add_system(
            apply_forces
                .in_schedule(CoreSchedule::FixedUpdate)
                .after(UpdateForcesSet),
        );
    }
}

/// Forces should only be updated in this set to avoid single frame delays
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UpdateForcesSet;

/// Use this resource to set the gravity applied to your soft bodies
#[derive(Resource, Debug, Deref, DerefMut)]
pub struct Gravity(pub Vec2);

fn add_gravity_force(mut query: Query<&mut Point>, gravity: Res<Gravity>) {
    for mut point in &mut query {
        if let Point::Dynamic {
            ref mut force,
            mass: _,
            velocity: _,
        } = *point
        {
            *force += **gravity;
        }
    }
}

fn add_spring_force(mut points: Query<(&mut Point, &Transform)>, springs: Query<&Spring>) {
    for spring in &springs {
        let [(point_a, transform_a), (point_b, transform_b)] = points
            .get_many_mut([spring.entity_a, spring.entity_b])
            .unwrap();

        // Skip if both points are fixed
        if *point_a == Point::Fixed && *point_b == Point::Fixed {
            continue;
        }

        let spring_force = (transform_b.translation.distance(transform_a.translation)
            - spring.rest_length)
            * spring.stiffness;

        let direction = (transform_b.translation - transform_a.translation)
            .normalize()
            .xy();
        let velocity_diff = point_b.velocity_or_zero() - point_a.velocity_or_zero();
        let dot_prod = direction.dot(velocity_diff);
        let damp = spring.damping * dot_prod;

        for mut point in [point_a, point_b] {
            if let Point::Dynamic {
                ref mut force,
                mass: _,
                velocity: _,
            } = *point
            {
                // let direction = (transform_a.translation - transform_b.translation)
                //     .normalize()
                //     .xy();
                force.y += (spring_force + damp);
            }
        }
    }
}

fn apply_forces(mut query: Query<(&mut Transform, &mut Point)>, time: Res<FixedTime>) {
    let delta_time = time.period.as_secs_f32();
    for (mut transform, mut point) in &mut query {
        if let Point::Dynamic {
            ref mut force,
            ref mut velocity,
            mass: _,
        } = *point
        {
            transform.translation += velocity.extend(0.0) * delta_time;
            *velocity += *force * delta_time;
            *force = Vec2::ZERO;
        }
    }
}
