//! All the Bevy Squishy components

use bevy::prelude::*;

// Point

/// A singular soft body point
#[derive(Component, Debug)]
pub struct Point {
    /// The points mass, deault is 1.0
    pub mass: f32,
    /// The current velocity of this point
    pub velocity: Vec2,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            mass: 1.0,
            velocity: Vec2::ZERO,
        }
    }
}

impl Point {
    /// A conctructor for [`Point`]
    pub fn new(mass: f32, velocity: Vec2) -> Self {
        Self { mass, velocity }
    }
}

/// Current force on a point
///
/// This should only be edited durning the [`UpdateForcesSet`](crate::UpdateForcesSet) set,
/// and in the [`CoreSchedule::FixedUpdate`] schedule.
#[derive(Component, Default, Deref, DerefMut)]
pub struct Force(Vec2);

impl Force {
    pub(crate) fn reset(&mut self) {
        self.0 = Vec2::ZERO;
    }
}

/// Bundle for spawning a singular soft body point in world space
#[derive(Bundle, Default)]
pub struct SquishyPointBundle {
    /// The soft body point
    pub point: Point,
    /// The current force of the point
    pub force: Force,
    /// The transforms of the point
    pub transform_bundle: TransformBundle,
}

impl SquishyPointBundle {
    /// Constructor for creating a [`SquishyPointBundle`] at a certain position
    pub fn new(translation: Vec2) -> Self {
        Self {
            point: Point::default(),
            force: Default::default(),
            transform_bundle: TransformBundle::from_transform(Transform::from_translation(
                translation.extend(0.0),
            )),
        }
    }
}

// Fixed Point

/// A point that will not move
#[derive(Component, Default)]
pub struct FixedPoint;

/// Bundle for spawning a singular fixed point in world space
#[derive(Bundle, Default)]
pub struct FixedSquishyPointBundle {
    /// The soft body point
    pub fixed_point: FixedPoint,
    /// The transforms of the point
    pub transform_bundle: TransformBundle,
}

impl FixedSquishyPointBundle {
    /// Constructor for creating a [`FixedSquishyPointBundle`] at a certain position
    pub fn new(translation: Vec2) -> Self {
        Self {
            fixed_point: FixedPoint,
            transform_bundle: TransformBundle::from_transform(Transform::from_translation(
                translation.extend(0.0),
            )),
        }
    }
}

// Spring

/// A spring use to create a connection between two soft body points
#[derive(Component, Debug)]
pub struct Spring {
    /// The first soft body point, entity must have a [`Point`] component
    pub entity_a: Entity,
    /// The second soft body point, entity must have a [`Point`] component
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
