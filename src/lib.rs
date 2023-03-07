#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::prelude::*;

pub use components::*;

mod components;

type EitherPoint = Or<(With<Point>, With<FixedPoint>)>;

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

fn add_gravity_force(mut query: Query<(&mut Force, &Point)>, gravity: Res<Gravity>) {
    for (mut force, point) in &mut query {
        **force += **gravity / point.mass;
    }
}

fn add_spring_force(
    mut points: Query<(&mut Force, &Transform, Option<&FixedPoint>), EitherPoint>,
    springs: Query<&Spring>,
) {
    for spring in &springs {
        let (mut force_a, transform_a, fixed_a) = points.get(spring.entity_a).unwrap();
        let (mut force_b, transform_b, fixed_b) = points.get(spring.entity_b).unwrap();

        // Skip if both points are fixed
        if fixed_a.is_some() && fixed_b.is_some() {
            continue;
        }

        let spring_force = ((transform_b.translation - transform_a.translation)
            - spring.rest_length)
            * spring.stiffness;
    }
}

fn apply_forces(mut query: Query<(&mut Transform, &mut Point, &mut Force)>, time: Res<FixedTime>) {
    let delta_time = time.period.as_secs_f32();
    for (mut transform, mut point, mut force) in &mut query {
        transform.translation += point.velocity.extend(0.0) * delta_time;
        point.velocity += **force * delta_time;
        force.reset();
    }
}
