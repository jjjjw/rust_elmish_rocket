use graphics::{Context, Polygon, Ellipse, Transformed, clear};
use opengl_graphics::GlGraphics;
use models::{Player, World, Particle};

/// The player is drawn as the triangle below
const POLYGON: &'static [[f64; 2]] = &[
    [0.0, -8.0],
    [20.0, 0.0],
    [0.0, 8.0]
];

pub mod color {
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const VIOLET: [f32; 4] = [0.6, 0.0, 1.0, 1.0];
}

fn render_particle (particle: &Particle, c: &Context, gl: &mut GlGraphics) {
    let radius = 5.0 * particle.ttl;
    Ellipse::new(color::VIOLET).resolution(8).draw(
        [particle.vector.position.x - radius, particle.vector.position.y - radius, radius * 2.0, radius * 2.0],
        &c.draw_state, c.transform, gl);
}

fn render_player (player: &Player, ctx: &Context, gl: &mut GlGraphics) {
    // Set the center of the player as the origin and rotate it
    let transform = ctx.transform.trans(player.vector.position.x, player.vector.position.y)
                               .rot_rad(player.vector.direction);

    // Draw a rectangle on the position of the player
    Polygon::new(color::RED).draw(POLYGON, &ctx.draw_state, transform, gl);
}

pub fn render_world (world: &World, ctx: &Context, gl: &mut GlGraphics) {
  // Clear everything
  clear(color::BLACK, gl);

  // Render the player
  render_player(&world.player, ctx, gl);

    // Render all the particles
  for particle in &world.particles {
    render_particle(&particle, ctx, gl);
  }
}
