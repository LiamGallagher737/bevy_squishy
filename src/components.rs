//! All the Bevy Squishy components

use bevy::prelude::*;

// Point

/// Add this component to and entity for it to become a point
///
/// Use [`DynamicPointBundle`] to spawn a point which can be affected by forces
/// or [`FixedPointBundle`] for a point that can't be affected by forces
#[derive(Component, Default)]
pub struct Point;

// Dynamic Point

/// A component to add to any point which should be affected by forces
#[derive(Component, Debug)]
pub struct DynamicPoint {
    /// The radius of this point used for self collisions
    pub radius: f32,
    /// The points mass, deault is 1.0
    pub mass: f32,
    /// The current velocity of this point
    pub velocity: Vec2,
    /// Current force on a point
    ///
    /// This should only be edited durning the [`UpdateForcesSet`](crate::UpdateForcesSet) set,
    /// and in the [`CoreSchedule::FixedUpdate`] schedule.
    pub force: Vec2,
}

impl Default for DynamicPoint {
    fn default() -> Self {
        Self {
            radius: 0.5,
            mass: 1.0,
            velocity: Vec2::ZERO,
            force: Vec2::ZERO,
        }
    }
}

impl DynamicPoint {
    /// Set this points radius to the given value
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

/// Bundle for spawning a dynamic point in world space
#[derive(Bundle, Default)]
pub struct DynamicPointBundle {
    /// Point marker
    pub point: Point,
    /// The dynamic point
    pub dynamic_point: DynamicPoint,
    /// Required to have a position in world space
    pub transform_bundle: TransformBundle,
}

impl DynamicPointBundle {
    /// Constructor for [`DynamicPointBundle`]
    pub fn new(dynamic_point: DynamicPoint, position: Vec2) -> Self {
        Self {
            dynamic_point,
            transform_bundle: TransformBundle::from_transform(Transform::from_translation(
                position.extend(0.0),
            )),
            ..Default::default()
        }
    }
}

// Fixed Point

/// Bundle for spawning a fixed point in world space
#[derive(Bundle, Default)]
pub struct FixedPointBundle {
    /// Point marker
    pub point: Point,
    /// Required to have a position in world space
    pub transform_bundle: TransformBundle,
}

impl FixedPointBundle {
    /// Constructor for [`FixedPointBundle`]
    pub fn new(position: Vec2) -> Self {
        Self {
            transform_bundle: TransformBundle::from_transform(Transform::from_translation(
                position.extend(0.0),
            )),
            ..Default::default()
        }
    }
}

// Spring

/// A spring use to create a connection between two [`DynamicPoint`]s
#[derive(Component, Debug)]
pub struct Spring {
    /// The first point, entity must have a [`DynamicPoint`] component
    pub entity_a: Entity,
    /// The second point, entity must have a [`DynamicPoint`] component
    pub entity_b: Entity,
    /// How stiff this spring should be
    pub stiffness: f32,
    /// The resting length of this spring
    pub rest_length: f32,
    /// The amount of damping to apply to this spring when in movement
    pub damping: f32,
}

impl Spring {
    /// A constructor for [`Spring`]
    pub fn new(
        entity_a: Entity,
        entity_b: Entity,
        stiffness: f32,
        rest_length: f32,
        damping: f32,
    ) -> Self {
        Self {
            entity_a,
            entity_b,
            stiffness,
            rest_length,
            damping,
        }
    }
}

// Shape

/// Used to define a physics shape
///
/// The final shape must be closed
#[derive(Component)]
pub struct Shape {
    /// The point entities making up this shapes, each entity must have a [`Point`] component
    /// Points must be in a clockwise order or else you risk incorrect collisions
    pub points: Vec<Entity>,
}

impl Shape {
    /// Constructor for [`Shape`]
    pub fn new(points: Vec<Entity>) -> Self {
        if points.len() < 3 {
            warn!("A physics shape has been constructed with less than 3 points, this is going to cause issues.")
        }
        Self { points }
    }
}
