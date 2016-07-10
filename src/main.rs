extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;

mod models;
mod update;
mod view;

use piston_window::{PistonWindow, WindowSettings, OpenGL, Event, Events, EventLoop};
use opengl_graphics::GlGraphics;
use sdl2_window::Sdl2Window;

use models::{Game, Size};
use update::{update_world, update_actions};
use view::{render_world};

// Use this typedef to make type of window prettier.
// Need to use Sdl2Window as backend in order to get controller/joystick events currently.
pub type SDL2GameWindow = PistonWindow<Sdl2Window>;

fn main() {
    let opengl = OpenGL::V3_2;

    let game_size = Size { width: 1024.0, height: 600.0 };

    let mut window: SDL2GameWindow = WindowSettings::new("Elmish Rocket!", [game_size.width as u32, game_size.height as u32])
        .opengl(opengl)
        .samples(8)
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);
    let mut events = window.events();

    // Game state
    let mut game = Game::new(game_size);

    while let Some(ev) = events.next(&mut window) {
        // Event handling
        match ev {
            // Core render and update
            Event::Render(args) => {
                gl.draw(args.viewport(), |c, g| render_world(&game.world, &c, g));
            }

            Event::Update(args) => {
                update_world(&mut game.world, &game.actions, args.dt);
            }

            // Handle user input
            Event::Input(args) => {
                update_actions(&mut game.actions, args);
            }

            _ => {}
        }
    }
}
