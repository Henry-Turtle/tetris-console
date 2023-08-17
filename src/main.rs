extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod game;


use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventLoop, ButtonState};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent, ButtonEvent};
use piston::window::WindowSettings;
use game::game::Game;




fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

   
    // Create a Glutin window. Tetris field is 10 wide, 20 high
    let mut window: Window = WindowSettings::new("Tetris", [250, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(GlGraphics::new(opengl));
    // Create a new game and run it.
    let mut events = Events::new(EventSettings::new()).ups(*&game.level as u64);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(press) = e.button_args(){
            if press.state == ButtonState::Press{
                game.on_pressed(&press.button);
                //game.draw_board();
            }
        }
    }
}
