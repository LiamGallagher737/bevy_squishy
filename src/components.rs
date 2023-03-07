//! All the Bevy Squishy components

use bevy::prelude::*;

// Point

/// A singular physics point
#[derive(Component, Default, PartialEq, Debug)]
pub enum Point {
    /// A physics point that will stayed in a fixed position
    #[default]
    Fixed,
    /// A physics point that is affected by forces
    Dynamic {
        /// The points mass, deault is 1.0
        mass: f32,
        /// The current velocity of this point
        velocity: Vec2,
        /// Current force on a point
        ///
        /// This should only be edited durning the [`UpdateForcesSet`](crate::UpdateForcesSet) set,
        /// and in the [`CoreSchedule::FixedUpdate`] schedule.
        force: Vec2,
    },
}

impl Point {
    /// The deault value for a fixed point
    pub const FIXED: Self = Self::Fixed;
    /// The deault value for a dynamic point
    pub const DYNAMIC: Self = Self::Dynamic {
        mass: 1.0,
        velocity: Vec2::ZERO,
        force: Vec2::ZERO,
    };

    /// A conctructor for [`Point`]
    pub const fn dynamic(mass: f32, velocity: Vec2) -> Self {
        Self::Dynamic {
            mass,
            velocity,
            force: Vec2::ZERO,
        }
    }

    /// If point is dynamic, return its velocity, else return zero
    pub fn velocity_or_zero(&self) -> Vec2 {
        match self {
            Point::Dynamic { velocity, .. } => *velocity,
            Point::Fixed => Vec2::ZERO,
        }
    }
}

/// Bundle for spawning a singular physics point in world space
#[derive(Bundle, Default)]
pub struct SquishyPointBundle {
    /// The physics point
    pub point: Point,
    /// The transforms of the point
    pub transform_bundle: TransformBundle,
}

impl SquishyPointBundle {
    /// Constructor for creating a [`SquishyPointBundle`] at a certain position
    pub fn new(point: Point, translation: Vec2) -> Self {
        Self {
            point,
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
