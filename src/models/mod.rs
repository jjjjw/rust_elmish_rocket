/// A game entity, implements various components as traits
}
struct Entity;

/// A `Position` represents a position in space
#[derive(Clone, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64
}

/// A `Size` represents a region in space
#[derive(Clone, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64
}

/// A `Vector`
#[derive(Clone, Default)]
pub struct Vector {
    /// The position of the vector
    pub position: Position,
    /// The direction angle, in radians
    pub direction: f64
}

/// The `Player` is the rocket controlled by the user
#[derive(Default)]
pub struct Player {
    pub vector: Vector
}

impl Player {
    pub fn new () -> Player {
        Player {
            vector: Vector {
                position: Position {
                    x: 50.0,
                    y: 50.0
                },
                direction: 0.0
            }
        }
    }
}

/// Enemies follow the player in order to cause a collision and let him explode
pub struct Enemy {
    pub vector: Vector
}

/// Bullets are spawned when the player shoots
///
/// When an enemy is reached by a bullet, it will explode
pub struct Bullet {
    pub vector: Vector
}

/// A model representing a particle
///
/// Particles are visible objects that have a time to live and move around
/// in a given direction until their time is up. They are spawned when the
/// player or an enemy is killed
pub struct Particle {
    pub vector: Vector,
    pub ttl: f64
}

impl Particle {
    /// Create a particle with the given vector and time to live in seconds
    pub fn new(vector: Vector, ttl: f64) -> Particle {
        Particle { vector: vector, ttl: ttl }
    }
}

/// Timers to handle creation of bullets, enemies and particles
#[derive(Default)]
pub struct Timers {
    pub current_time: f64,
    pub last_tail_particle: f64,
    pub last_shoot: f64,
    pub last_spawned_enemy: f64
}

/// A model that contains the other models and renders them
pub struct World {
    pub player: Player,
    pub particles: Vec<Particle>,
    pub bullets: Vec<Bullet>,
    pub enemies: Vec<Enemy>,
    pub size: Size,
    pub timers: Timers
}

impl World {
    /// Returns a new `World` of the given `Size`
    pub fn new(size: Size) -> World {
        World {
            player: Player::new(),
            particles: vec![],
            bullets: vec![],
            enemies: vec![],
            size: size,
            timers: Timers::default()
        }
    }
}

/// Active actions (toggled by user input)
#[derive(Default)]
pub struct Actions {
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub boost: bool,
    pub shoot: bool
}

pub struct Game {
    pub world: World,
    pub actions: Actions
}

impl Game {
    /// Returns a new `Game` containing a `World` of the given `Size` and a default action state
    pub fn new(size: Size) -> Game {
        Game {
            world: World::new(size),
            actions: Actions::default()
        }
    }
}


