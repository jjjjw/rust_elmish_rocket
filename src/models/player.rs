use models::{Vector, Size, Radius};
use rand::Rng;

/// The `Player` is the rocket controlled by the user
#[derive(Default)]
pub struct Player {
    pub last_shoot: f64,
    pub last_tail_particle: f64,
    /// The radius for collision detection
    pub radius: Radius,
    pub score: u32,
    pub vector: Vector,
}

/// The player is drawn as the triangle below
pub const PLAYER_POLYGON: &'static [[f64; 2]] = &[
    [0.0, -8.0],
    [20.0, 0.0],
    [0.0, 8.0],
];

const PLAYER_RADIUS: f64 = 6.0;

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn random<R: Rng>(rng: &mut R, bounds: &Size) -> Player {
        Player {
            last_shoot: 0.0,
            last_tail_particle: 0.0,
            radius: PLAYER_RADIUS,
            score: 0,
            vector: Vector::random(rng, 0.0, bounds),
        }
    }
}
