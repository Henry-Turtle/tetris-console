#![allow(unused)]
use opengl_graphics::GlGraphics;
use piston::{UpdateArgs, Button, RenderArgs, Key};
use crate::game::board;

pub struct Game {
    pub gl: GlGraphics, // OpenGL drawing backend.
    grid: board::Board,
    piece: Option<board::Piece>,
    pub level: u8
    
}

impl Game {
    pub fn new(gl: GlGraphics) -> Game{
        let mut g = Game{
            gl: gl,
            grid: board::Board::empty(),
            piece: Some(board::Piece::LPiece),
            level: 1
        };
        g.grid.set_value(2, 2, String::from("piece"));
        
        return g
    }
    
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

            // Draw a box rotating around the middle of the screen.
            for row in (0..20).rev(){
                for col in (0..10){
                    if self.grid.get_value(row, col) == "piece"{
                        rectangle(
                            WHITE, 
                            rectangle::square(args.window_size[0]/10.0 * col as f64, args.window_size[1]/20.0* row as f64, args.window_size[0]/10.0),
                            c.transform.trans(0.0, 0.0),
                            gl
                        )
                    }
                }
            }
        });
    }

    pub fn draw_board(&self){
        for row in (0..20){
            print!("Row {}: ", row);
            for col in (0..10){
                print!("{}", self.grid.get_value(row, col));
            }
            println!("");
        }
    }

    pub fn update(&mut self, _args: &UpdateArgs){
        
        for row in (0..20){
            for col in (0..10){
                if self.grid.get_value(row, col) == &String::from("piece"){
                    self.grid.downshift(row, col);
                }
            }
        }
    }

    pub fn on_pressed(&mut self, button: &Button){
        /* 
        match button{
            Button::Keyboard(Key::Left) => 
                self.piece.x -= 50,
            Button::Keyboard(Key::Right)=>
                self.piece.x += 50,
            Button::Keyboard(Key::Up)=>
                self.piece.y -= 50,
                

            _ => () 
        }
        */
    
    }
}