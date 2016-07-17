use rand::Rng;
use std::f64;

/// A `Size` represents a region in space
#[derive(Clone, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

/// A `Vector` represents a position in space with a radius and direction
/// Used by most game objects
#[derive(Clone, Default)]
pub struct Vector {
    /// x position
    pub x: f64,
    /// y position
    pub y: f64,
    /// The direction angle, in radians
    pub direction: f64,
}

/// The radius for collision detection
pub type Radius = f64;

impl Vector {
    /// Returns a new `Vector`
    pub fn new(x: f64, y: f64, direction: f64) -> Vector {
        Vector { x: x, y: y, direction: direction }
    }

    /// Returns a random `Position` within the given bounds
    pub fn random<R: Rng>(rng: &mut R, direction: f64, bounds: &Size) -> Vector {
        Vector {
            x: rng.gen_range(0.0, bounds.width),
            y: rng.gen_range(0.0, bounds.height),
            direction: direction,
        }
    }
}
