use graphics::{Context, Polygon, Ellipse, Transformed, clear, Text};
use opengl_graphics::GlGraphics;
use models::*;

pub mod color {
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const VIOLET: [f32; 4] = [0.6, 0.0, 1.0, 1.0];
}

fn diameter (radius: Radius) -> f64 {
    radius * 2.0
}

fn render_particle (particle: &Particle, ctx: &Context, gl: &mut GlGraphics) {
    let radius: Radius = 5.0 * particle.ttl;
    Ellipse::new(color::VIOLET).resolution(8).draw(
        [particle.vector.x - radius, particle.vector.y - radius, diameter(radius), diameter(radius)],
        &ctx.draw_state, ctx.transform, gl);
}

fn render_bullet (bullet: &Bullet, ctx: &Context, gl: &mut GlGraphics) {
    Ellipse::new(color::BLUE).resolution(8).draw(
        [bullet.vector.x - bullet.radius, bullet.vector.y - bullet.radius, diameter(bullet.radius), diameter(bullet.radius)],
        &ctx.draw_state, ctx.transform, gl);
}

fn render_enemy (enemy: &Enemy, ctx: &Context, gl: &mut GlGraphics) {
    Ellipse::new([1.0, 1.0, 0.0, 1.0]).resolution(16).draw(
        [enemy.vector.x - enemy.radius, enemy.vector.y - enemy.radius, diameter(enemy.radius), diameter(enemy.radius)],
        &ctx.draw_state, ctx.transform, gl);
}

fn render_player (player: &Player, ctx: &Context, gl: &mut GlGraphics) {
    // Set the center of the player as the origin and rotate it
    let transform = ctx.transform.trans(player.vector.x, player.vector.y)
                               .rot_rad(player.vector.direction);

    // Draw a rectangle on the position of the player
    Polygon::new(color::RED).draw(PLAYER_POLYGON, &ctx.draw_state, transform, gl);
}

fn render_score (player: &Player, resources: &mut Resources, ctx: &Context, gl: &mut GlGraphics) {
  // Render the score
  let mut text = Text::new(22);
  text.color = color::ORANGE;
  text.draw(&format!("Score: {}", player.score),
            &mut resources.font,
            &ctx.draw_state,
            ctx.trans(10.0, 20.0).transform,
            gl);
}

pub fn render_world (world: &World, resources: &mut Resources, ctx: &Context, gl: &mut GlGraphics) {
  // Clear everything
  clear(color::BLACK, gl);

  // Render the player
  render_player(&world.player, ctx, gl);

  // Render the score
  render_score(&world.player, resources, ctx, gl);

    // Render all the particles
  for particle in &world.particles {
    render_particle(&particle, ctx, gl);
  }

    // Render all the bullets
  for bullet in &world.bullets {
    render_bullet(&bullet, ctx, gl);
  }

    // Render all the enemies
  for enemy in &world.enemies {
    render_enemy(&enemy, ctx, gl);
  }
}
