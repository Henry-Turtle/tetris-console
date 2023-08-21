#![allow(unused)]
use opengl_graphics::GlGraphics;
use piston::{UpdateArgs, Button, RenderArgs, Key};
use rand::Rng;
use crate::game::board;
use crate::game::board::Point;
use crate::game::piece;
use crate::game::board::Tile;
use super::piece::{Piece, PieceType, Gamepiece, PieceResult};

pub struct Game {
    pub gl: GlGraphics, // OpenGL drawing backend.
    grid: board::Board,
    pub level: u8,
    pub frame: u8,
    actions: Actions
}

impl Game {
    pub fn new(gl: GlGraphics) -> Game{
        let mut p = Piece { rotation: 0, piece_type: piece::PieceType::OPiece };
        let mut g = Game{
            gl: gl,
            grid: board::Board::new(),
            level: 1,
            frame: 0,
            actions: Actions::new()
        };
        g.grid.generate_new_piece();
        
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
            graphics::clear(BLACK, gl);
            let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

            
            for row in (0..20).rev(){
                graphics::line(RED, 1.0, [0.0, args.window_size[0]/10.0 * row as f64, args.window_size[0], args.window_size[0]/10.0 * row as f64], c.transform.trans(0.0, 0.0), gl);
                for col in (0..10){
                    graphics::line(RED, 1.0, [args.window_size[1]/20.0 * col as f64, 0.0, args.window_size[1]/20.0 * col as f64, args.window_size[1]], c.transform.trans(0.0, 0.0), gl);
                    match self.grid.get_value(Point{row: row, col: col}){
                        Tile::Alive =>
                        graphics::rectangle(
                            GREEN, 
                            rectangle::square(args.window_size[0]/10.0 * col as f64, args.window_size[1]/20.0* row as f64, args.window_size[0]/10.0),
                            c.transform.trans(0.0, 0.0),
                            gl
                        ),
                        Tile::Dead =>
                        graphics::rectangle(
                            WHITE,
                            rectangle::square(args.window_size[0]/10.0 * col as f64, args.window_size[1]/20.0* row as f64, args.window_size[0]/10.0),
                            c.transform.trans(0.0, 0.0),
                            gl
                        ),
                        Tile::Empty =>
                        graphics::rectangle(
                            BLACK,
                            rectangle::square(args.window_size[0]/10.0 * col as f64, args.window_size[1]/20.0* row as f64, args.window_size[0]/10.0),
                            c.transform.trans(0.0, 0.0),
                            gl
                        ),
                    }
                }
            }
        });
    }

    

    pub fn update(&mut self, _args: &UpdateArgs){
        self.frame += 1;


        //*Automatic gravity */
        if self.frame % ((60/self.level) as u8) == 0{
            self.grid.downshift_all();
        }

        if ! matches!(self.actions.direction, Direction::None){
            match self.actions.frames_until_das_activates{
                0 => {
                    match self.actions.frames_until_next_das{
                        0 => {
                            match self.actions.direction{
                                Direction::Left => {self.grid.leftshift_all(); self.actions.frames_until_next_das = self.actions.das_speed},
                                Direction::Right => {self.grid.rightshift_all(); self.actions.frames_until_next_das = self.actions.das_speed},
                                Direction::None => ()
                            }
                        },
                        _ => self.actions.frames_until_next_das -= 1
                    }
                }
                _ => self.actions.frames_until_das_activates -= 1
            }
        }
        

        if self.frame == 60{
            self.frame = 1;
        }
        
        
    }

    pub fn on_pressed(&mut self, button: &Button){
        match button{
            Button::Keyboard(Key::Left) => 
                {self.grid.leftshift_all();
                self.actions.direction = Direction::Left;},
            Button::Keyboard(Key::Right)=>
                {self.grid.rightshift_all();
                self.actions.direction = Direction::Right;},
            Button::Keyboard(Key::Up)=>
                self.grid.upshift_all(),
            Button::Keyboard(Key::Down) =>
                {self.grid.downshift_all();
                self.actions.down_held = true;},
            Button::Keyboard(Key::X) =>
                self.rotate_clockwise(),

            Button::Keyboard(Key::Z) =>
                self.rotate_counterclockwise(),

            _ => () 
        }
        
    }

    pub fn on_released(&mut self, button: &Button){
        match button{
            Button::Keyboard(Key::Space)=>            
                self.grid.hard_drop(),
            Button::Keyboard(Key::Left)=>
                if matches!(self.actions.direction, Direction::Left)
                {
                    self.actions.direction = Direction::None; 
                    self.actions.frames_until_das_activates = self.actions.das_delay; 
                    self.actions.frames_until_next_das = self.actions.das_speed
                }
            Button::Keyboard(Key::Right)=>
                if matches!(self.actions.direction, Direction::Right)
                {
                    self.actions.direction = Direction::None; 
                    self.actions.frames_until_das_activates = self.actions.das_delay; 
                    self.actions.frames_until_next_das = self.actions.das_speed
                },

            _ => ()
            }
        }


        pub fn rotate_clockwise(&mut self){
            
            let piece_coordinates = self.grid.get_alive_tiles();
            let mut piece_copy = Piece{rotation: self.grid.piece.rotation, piece_type: self.grid.piece.piece_type};
            let result = piece_copy.rotate_clockwise(&mut self.grid);
            match result{
                PieceResult::Success => {
                    match self.grid.piece.rotation{
                        0 | 1 | 2 => self.grid.piece.rotation += 1,
                        3 => self.grid.piece.rotation = 0,
                        _ => panic!("Rotation OOB")
                    }
                },
                PieceResult::Failure => ()
            }
        }

        pub fn rotate_counterclockwise(&mut self){
            let piece_coordinates = self.grid.get_alive_tiles();
            let mut piece_copy = Piece{rotation: self.grid.piece.rotation, piece_type: self.grid.piece.piece_type};
            let result = piece_copy.rotate_counterclockwise(&mut self.grid);
            match result{
                PieceResult::Success => {
                    match self.grid.piece.rotation{
                        1 | 2 | 3 => self.grid.piece.rotation -= 1,
                        0 => self.grid.piece.rotation = 3,
                        _ => panic!("Rotation OOB")
                    }
                },
                PieceResult::Failure => ()
            }
        }
        
    }



pub struct Actions{
    direction: Direction,
    down_held: bool,
    
    ///How many frames it takes for DAS to activate
    das_delay: u8, 
    ///How many remaining frames until DAS activates
    frames_until_das_activates: u8, 
    ///Once DAS activates, it does so every x frames
    das_speed: u8,
    ///The number of frames since DAS last activated
    frames_until_next_das: u8 
}

impl Actions{
    pub fn new() -> Actions{
        Actions { direction: Direction::None, down_held: false, das_delay: 10, frames_until_das_activates: 10, das_speed: 3, frames_until_next_das: 3}
    }
}
pub enum Direction{
    Left,
    Right,
    None
}