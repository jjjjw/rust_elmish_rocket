mod grid;
mod player;

pub use self::grid::{Vector, Size, Radius};
pub use self::player::{Player, PLAYER_POLYGON};
use rand::{thread_rng, ThreadRng, Rng};
use opengl_graphics::glyph_cache::GlyphCache;

const ENEMY_RADIUS: f64 = 10.0;
const BULLET_RADIUS: f64 = 3.0;


/// Enemies follow the player in order to cause a collision and let her explode
pub struct Enemy {
    pub radius: Radius,
    pub vector: Vector,
}

impl Enemy {
    /// Create an enemy with the given vector
    pub fn new(vector: Vector) -> Enemy {
        Enemy { vector: vector, radius: ENEMY_RADIUS }
    }

    /// Create a new `Enemy` with a random position and direction
    pub fn random<R: Rng>(rng: &mut R, bounds: &Size) -> Enemy {
        Enemy::new(Vector::random(rng, 0.0, bounds))
    }
}

/// Bullets are spawned when the player shoots
///
/// When an enemy is reached by a bullet, it will explode
pub struct Bullet {
    pub radius: Radius,
    pub vector: Vector,
}

impl Bullet {
    /// Create a buller with the given vector
    pub fn new(vector: Vector) -> Bullet {
        Bullet { vector: vector, radius: BULLET_RADIUS }
    }
}

/// A model representing a particle
///
/// Particles are visible objects that have a time to live and move around
/// in a given direction until their time is up. They are spawned when the
/// player or an enemy is killed
pub struct Particle {
    pub vector: Vector,
    pub ttl: f64,
}

impl Particle {
    /// Create a particle with the given vector and time to live in seconds
    pub fn new(vector: Vector, ttl: f64) -> Particle {
        Particle { vector: vector, ttl: ttl }
    }
}

/// A model that contains the other models and renders them
pub struct World {
    pub bullets: Vec<Bullet>,
    pub current_time: f64,
    pub enemies: Vec<Enemy>,
    pub last_spawned_enemy: f64,
    pub particles: Vec<Particle>,
    pub player: Player,
    pub rng: ThreadRng,
    pub size: Size,
}

impl World {
    /// Returns a new `World` of the given `Size`
    pub fn new(size: Size) -> World {
        let mut rng = thread_rng();

        World {
            bullets: vec![],
            current_time: 0.0,
            enemies: vec![],
            last_spawned_enemy: 0.0,
            particles: vec![],
            player: Player::random(&mut rng, &size),
            rng: rng,
            size: size,
        }
    }
}

/// Active actions (toggled by user input)
#[derive(Default)]
pub struct Actions {
    pub boost: bool,
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub shoot: bool,
}

/// The game model keeps track of the world state and the actions state
pub struct Game {
    pub actions: Actions,
    pub world: World,
}

impl Game {
    /// Returns a new `Game` containing a `World` of the given `Size` and a default action state
    pub fn new(size: Size) -> Game {
        Game {
            actions: Actions::default(),
            world: World::new(size),
        }
    }
}

/// Additional resources needed for the game
pub struct Resources {
    pub font: GlyphCache<'static>
}
