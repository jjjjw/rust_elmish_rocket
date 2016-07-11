use models::{World, Actions, Player, Vector, Position, Size, Particle};
use piston::input::{Button, Key, Input};
use std::f64;

const UPS: u16 = 120;

fn wrap(k: f64, bound: f64) -> f64 {
    if k < 0.0 {
        k + bound
    } else if k >= bound {
        k - bound
    } else {
        k
    }
}

fn invert(vector: &mut Vector) {
    vector.direction -= f64::consts::PI;
}

fn advance(vector: &mut Vector, units: f64) {
    vector.position.x = vector.position.x + vector.direction.cos() * units;
    vector.position.y = vector.position.y + vector.direction.sin() * units;
}

/// Advances the object in the given amount of units, according to its direction
///, but the final possition will be wrapped
/// around the given bounds
fn advance_wrapping(vector: &mut Vector, units: f64, bounds: &Size) {
    vector.position.x = wrap(vector.position.x + vector.direction.cos() * units, bounds.width);
    vector.position.y = wrap(vector.position.y + vector.direction.sin() * units, bounds.height);
}

/// Update the particle
fn update_particle(particle: &mut Particle, elapsed_time: f64) {
    particle.ttl -= elapsed_time;
    let speed = 500.0 * particle.ttl * particle.ttl;
    advance(&mut particle.vector, elapsed_time * speed);
}

pub fn update_world (world: &mut World, actions: &Actions, dt: f64) {
  let ref mut playerVector = world.player.vector;

  world.timers.current_time += dt;

  // Update rocket rotation
  if actions.rotate_left {
      playerVector.direction += (-0.06 * UPS as f64) * dt;
  }
  if actions.rotate_right {
      playerVector.direction += (0.06 * UPS as f64) * dt;
  };

  // Set speed and advance the player with wrap around
  let speed = if actions.boost { 400.0  } else { 200.0 };
  advance_wrapping(playerVector, dt * speed, &world.size);

  // Update particles
  for mut particle in &mut world.particles {
      update_particle(&mut particle, dt);
  }

  // Remove old particles
  world.particles.retain(|p| p.ttl > 0.0);

  // Add new particles at the player's position, to leave a trail
  if world.timers.current_time - world.timers.last_tail_particle > 0.05 {
      world.timers.last_tail_particle = world.timers.current_time;
      let mut particleVector = playerVector.clone();
      invert(&mut particleVector);
      world.particles.push(Particle::new(particleVector, 0.5));
  }
}

/// Handles a key press or release
fn handle_key(actions: &mut Actions, key: Key, pressed: bool) {
    match key {
        Key::Left => actions.rotate_left = pressed,
        Key::Right => actions.rotate_right = pressed,
        Key::Up => actions.boost = pressed,
        Key::Space => actions.shoot = pressed,
        _ => ()
    }
}

/// Handles use input and updates the actions state for the next world update
pub fn update_actions (actions: &mut Actions, input: Input) {
    match input {
        Input::Press(Button::Keyboard(key)) => {
            handle_key(actions, key, true);
        }

        Input::Release(Button::Keyboard(key)) => {
            handle_key(actions, key, false);
        }

        _ => ()
    }
}
