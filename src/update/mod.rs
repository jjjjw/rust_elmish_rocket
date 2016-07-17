use itertools;
use models::*;
use piston::input::{Button, Key, Input};
use std::f64;

const UPS: u16 = 120;
const BULLET_RATE: f64 = 0.10;

fn wrap(k: f64, bound: f64) -> f64 {
    if k < 0.0 {
        k + bound
    } else if k >= bound {
        k - bound
    } else {
        k
    }
}

fn invert(vector: &mut Vector) -> &mut Vector {
    vector.direction -= f64::consts::PI;
    vector
}

/// Get the direction for the vector to point to the given target
fn point_to<'v>(vector: &'v mut Vector, target: &Vector) -> &'v mut Vector {
    let m = (vector.y - target.y) / (vector.x - target.x);

    vector.direction = if target.x > vector.x {
        m.atan()
    } else {
        m.atan() + f64::consts::PI
    };

    vector
}

/// Returns the squared distance from this point to the given one
fn squared_distance_to(from: &Vector, to: &Vector) -> f64 {
    (from.x - to.x) * (from.x - to.x)
    + (from.y - to.y) * (from.y - to.y)
}

/// Rotates a vector through the origin in the given angle (radians)
fn rotate(vector: &mut Vector, radians: f64) -> &mut Vector {
    let radius = (vector.x * vector.x + vector.y * vector.y).sqrt();
    let point_angle = (vector.y / vector.x).atan();
    let final_angle = point_angle + radians;

    vector.x = final_angle.cos() * radius;
    vector.y = final_angle.sin() * radius;

    vector
}

/// Translates the vector by another vector
fn translate<'v>(vector: &'v mut Vector, other: &Vector) -> &'v mut Vector {
    vector.x += other.x;
    vector.y += other.y;
    vector
}

/// Returns true if the two vectors + radii collide and false otherwise
fn collides_with(vector: &Vector, radius: Radius, other: &Vector, other_radius: Radius) -> bool {
    let radii = radius + other_radius;
    squared_distance_to(vector, other) < radii * radii
}

/// Returns true if the `Vector` is contained in this `Size` or false otherwise
fn contains(size: &Size, vector: &Vector) -> bool {
    0.0 <= vector.x && vector.x <= size.width
    && 0.0 <= vector.y && vector.y <= size.height
}


/// Returns the nose of the player rocket
fn nose(player: &Player) -> Vector {
    let mut vec = Vector::new(
      PLAYER_POLYGON[1][0],
      PLAYER_POLYGON[1][1],
      player.vector.direction);

    rotate(&mut vec, player.vector.direction);
    translate(&mut vec, &player.vector);

    vec
}

/// Advances the object in the given amount of units, according to its direction
fn advance(vector: &mut Vector, units: f64) -> &mut Vector {
    vector.x = vector.x + vector.direction.cos() * units;
    vector.y = vector.y + vector.direction.sin() * units;
    vector
}

/// Advances the object in the given amount of units, according to its direction
///, but the final possition will be wrapped around the given bounds
fn advance_wrapping<'v>(vector: &'v mut Vector, units: f64, bounds: &Size) -> &'v mut Vector {
    advance(vector, units);
    vector.x = wrap(vector.x, bounds.width);
    vector.y = wrap(vector.y, bounds.height);
    vector
}

/// Update the particle
fn update_particle(particle: &mut Particle, elapsed_time: f64) -> &mut Particle {
    particle.ttl -= elapsed_time;
    let speed = 500.0 * particle.ttl * particle.ttl;
    advance(&mut particle.vector, elapsed_time * speed);
    particle
}

/// Update the bullet
fn update_bullet(bullet: &mut Bullet, elapsed_time: f64) -> &mut Bullet {
    advance(&mut bullet.vector, elapsed_time * 500.0);
    bullet
}

fn update_enemy<'e>(enemy: &'e mut Enemy, elapsed_time: f64, player_vec: &Vector) -> &'e mut Enemy {
  point_to(&mut enemy.vector, player_vec);
  advance(&mut enemy.vector, elapsed_time * 100.0);
  enemy
}

// Generates a new explosion of the given intensity at the given position. This works best with values between 5 and 25
fn make_explosion(particles: &mut Vec<Particle>, vector: &Vector, intensity: u8) {
    for rotation in itertools::linspace(0.0, 2.0 * f64::consts::PI, 30) {
        for ttl in (1..intensity).map(|x| (x as f64) / 10.0) {
            particles.push(Particle::new(Vector::new(vector.x, vector.y, rotation), ttl));
        }
    }
}

/// reset our game-state
fn reset(world: &mut World) {
    // Reset player
    world.player = Player::random(&mut world.rng, &world.size);

    // Remove all enemies and bullets
    world.bullets.clear();
    world.enemies.clear();
}

pub fn update_world<'w> (world: &'w mut World, actions: &Actions, dt: f64) -> &'w mut World {
  world.current_time += dt;

  // Update rocket rotation
  if actions.rotate_left {
      world.player.vector.direction += (-0.06 * UPS as f64) * dt;
  }
  if actions.rotate_right {
      world.player.vector.direction += (0.06 * UPS as f64) * dt;
  };

  // Set speed and advance the player with wrap around
  let speed = if actions.boost { 400.0  } else { 200.0 };
  advance_wrapping(&mut world.player.vector, dt * speed, &world.size);

  // Update particles
  for mut particle in &mut world.particles {
      update_particle(&mut particle, dt);
  }

  // Remove old particles
  world.particles.retain(|p| p.ttl > 0.0);

  // Add new particles at the player's position, to leave a trail
  if world.current_time - world.player.last_tail_particle > 0.05 {
      world.player.last_tail_particle = world.current_time;

      let mut vec = world.player.vector.clone();
      invert(&mut vec);

      world.particles.push(Particle::new(vec, 0.5));
  }

  // Add bullets
  if actions.shoot && world.current_time - world.player.last_shoot > BULLET_RATE {
      world.player.last_shoot = world.current_time;
      let vec = nose(&world.player);

      world.bullets.push(Bullet::new(vec));
  }

  // Advance bullets
  for mut bullet in &mut world.bullets {
      update_bullet(&mut bullet, dt);
  }

  // Remove bullets outside the viewport
  {
    let size = &world.size;
    world.bullets.retain(|b| contains(&size, &b.vector));
  }

  // Spawn enemies at random locations
  if world.current_time - world.last_spawned_enemy > 2.0 {
      world.last_spawned_enemy = world.current_time;
      let mut new_enemy: Enemy;
      loop {
          new_enemy = Enemy::random(&mut world.rng, &world.size);
          if !collides_with(
                &world.player.vector, world.player.radius,
                &new_enemy.vector, new_enemy.radius) {
              break;
          }
      }
      world.enemies.push(new_enemy);
  }

  // Move enemies in the player's direction
  for mut enemy in &mut world.enemies {
      update_enemy(&mut enemy, dt, &world.player.vector);
  }

  // Handles collisions between the player and the enemies
  let mut collided = false;

  for enemy in &world.enemies {
    if collides_with(&world.player.vector, world.player.radius, &enemy.vector, enemy.radius) {

      // Make an explosion where the player was
      make_explosion(&mut world.particles, &world.player.vector, 8);

      collided = true;

      break;
    }
  }

  if collided { reset(world) };

  // Handles collisions between bullets and the enemies (brute force)
  let old_enemy_count = world.enemies.len();

  { // We introduce a scope to shorten the lifetime of the borrows below
    // The references are to avoid using world in the closure
    // (the borrow checker doesn't like that)
    let bullets = &mut world.bullets;
    let enemies = &mut world.enemies;
    let particles = &mut world.particles;

    enemies.retain(|enemy| {
        let mut collided = false;

        bullets.retain(|bullet| {
          if collides_with(&enemy.vector, enemy.radius, &bullet.vector, bullet.radius) {
            collided = true;

            make_explosion(particles, &enemy.vector, 10);

            false
          } else {
            true
          }
        });

        !collided
     });
  }

  let killed_enemies = (old_enemy_count - world.enemies.len()) as u32;
  world.player.score += 10 * killed_enemies;

  world
}

/// Handles a key press or release
fn handle_key(actions: &mut Actions, key: Key, pressed: bool) -> &mut Actions {
    match key {
        Key::Left => actions.rotate_left = pressed,
        Key::Right => actions.rotate_right = pressed,
        Key::Up => actions.boost = pressed,
        Key::Space => actions.shoot = pressed,
        _ => ()
    }
    actions
}

/// Handles use input and updates the actions state for the next world update
pub fn update_actions (actions: &mut Actions, input: Input) -> &mut Actions {
    match input {
        Input::Press(Button::Keyboard(key)) => {
            handle_key(actions, key, true);
        }

        Input::Release(Button::Keyboard(key)) => {
            handle_key(actions, key, false);
        }

        _ => ()
    }
    actions
}
