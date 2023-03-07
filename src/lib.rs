#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::{math::Vec3Swizzles, prelude::*};

pub use components::*;

mod collisions;
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
                collisions::add_collision_forces,
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

fn add_gravity_force(mut query: Query<&mut DynamicPoint>, gravity: Res<Gravity>) {
    for mut point in &mut query {
        point.force += **gravity;
    }
}

fn add_spring_force(mut points: Query<(&mut DynamicPoint, &Transform)>, springs: Query<&Spring>) {
    for spring in &springs {
        let [(mut point_a, transform_a), (mut point_b, transform_b)] = points
            .get_many_mut([spring.entity_a, spring.entity_b])
            .unwrap();

        // Calculate spring force

        let spring_force = (transform_b.translation.distance(transform_a.translation)
            - spring.rest_length)
            * spring.stiffness;

        // Calculate dampening

        let direction = (transform_b.translation - transform_a.translation)
            .normalize()
            .xy();
        let velocity_diff = point_b.velocity - point_a.velocity;
        let dot_prod = direction.dot(velocity_diff);
        let damp = spring.damping * dot_prod;

        // Add forces

        let direction = (transform_b.translation - transform_a.translation)
            .normalize()
            .xy();
        point_a.force += (spring_force + damp) * direction;

        let direction = (transform_a.translation - transform_b.translation)
            .normalize()
            .xy();
        point_b.force += (spring_force + damp) * direction;
    }
}

fn apply_forces(mut query: Query<(&mut Transform, &mut DynamicPoint)>, time: Res<FixedTime>) {
    let delta_time = time.period.as_secs_f32();
    for (mut transform, mut point) in &mut query {
        transform.translation += point.velocity.extend(0.0) * delta_time;

        let force = point.force;
        point.velocity += force * delta_time;
        point.force = Vec2::ZERO;
    }
}
