#![allow(unused)]
use super::piece::{Gamepiece, Held, Piece, PieceResult, PieceType};
use crate::game::board;
use crate::game::board::Point;
use crate::game::board::Tile;
use crate::game::piece;
use opengl_graphics::GlGraphics;
use piston::{Button, Key, RenderArgs, UpdateArgs};
use rand::Rng;

pub struct Game {
    pub gl: GlGraphics, // OpenGL drawing backend.
    grid: board::Board,
    pub level: u8,
    actions: Actions,
}

impl Game {
    pub fn new(gl: GlGraphics) -> Game {
        let mut p = Piece {
            rotation: 0,
            piece_type: piece::PieceType::OPiece,
        };
        let mut g = Game {
            gl: gl,
            grid: board::Board::new(),
            level: 1,
            actions: Actions::new(),
        };
        g.grid.generate_new_piece();

        return g;
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

            for row in (0..20).rev() {
                graphics::line(
                    RED,
                    1.0,
                    [
                        0.0,
                        args.window_size[0] / 10.0 * row as f64,
                        args.window_size[0],
                        args.window_size[0] / 10.0 * row as f64,
                    ],
                    c.transform.trans(0.0, 0.0),
                    gl,
                );
                for col in (0..10) {
                    graphics::line(
                        RED,
                        1.0,
                        [
                            args.window_size[1] / 20.0 * col as f64,
                            0.0,
                            args.window_size[1] / 20.0 * col as f64,
                            args.window_size[1],
                        ],
                        c.transform.trans(0.0, 0.0),
                        gl,
                    );
                    match self.grid.get_value(Point { row: row, col: col }) {
                        Tile::Alive => graphics::rectangle(
                            GREEN,
                            rectangle::square(
                                args.window_size[0] / 10.0 * col as f64,
                                args.window_size[1] / 20.0 * row as f64,
                                args.window_size[0] / 10.0,
                            ),
                            c.transform.trans(0.0, 0.0),
                            gl,
                        ),
                        Tile::Dead => graphics::rectangle(
                            WHITE,
                            rectangle::square(
                                args.window_size[0] / 10.0 * col as f64,
                                args.window_size[1] / 20.0 * row as f64,
                                args.window_size[0] / 10.0,
                            ),
                            c.transform.trans(0.0, 0.0),
                            gl,
                        ),
                        Tile::Empty => graphics::rectangle(
                            BLACK,
                            rectangle::square(
                                args.window_size[0] / 10.0 * col as f64,
                                args.window_size[1] / 20.0 * row as f64,
                                args.window_size[0] / 10.0,
                            ),
                            c.transform.trans(0.0, 0.0),
                            gl,
                        ),
                    }
                }
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        if self.actions.piece_can_lock_timer == 0{
            self.actions.piece_can_lock = true;
        }
        if self.actions.piece_can_lock_timer > 0 {
            self.actions.piece_can_lock_timer -= 1;
        }
        
        match self.actions.down_held {
            true => {
                if self.actions.down_held_timer <= 0 {
                    self.actions.piece_can_lock = true;
                    self.grid.downshift_all(&mut self.actions);
                    self.actions.down_held_timer = self.actions.down_held_delay;
                } else {
                    self.actions.down_held_timer -= 1;
                }
            }
            false => {
                //*Downshift to gravity */
                self.actions.gravity_timer -= 1;
                if self.actions.gravity_timer == 0 {
                    self.grid.downshift_all(&mut self.actions);
                    self.actions.gravity_timer = self.actions.gravity_delay;
                }
            }
        }

        

        if !matches!(self.actions.direction, Direction::None) {
            match self.actions.frames_until_das_activates {
                0 => match self.actions.frames_until_next_das {
                    0 => match self.actions.direction {
                        Direction::Left => {
                            self.grid.leftshift_all(&mut self.actions);
                            self.actions.frames_until_next_das = self.actions.das_speed
                        }
                        Direction::Right => {
                            self.grid.rightshift_all(&mut self.actions);
                            self.actions.frames_until_next_das = self.actions.das_speed
                        }
                        Direction::None => (),
                    },
                    _ => self.actions.frames_until_next_das -= 1,
                },
                _ => self.actions.frames_until_das_activates -= 1,
            }
        }
    }

    pub fn on_pressed(&mut self, button: &Button) {
        match button {
            Button::Keyboard(Key::Left) => {
                self.grid.leftshift_all(&mut self.actions);
                self.actions.direction = Direction::Left;
            }
            Button::Keyboard(Key::Right) => {
                self.grid.rightshift_all(&mut self.actions);
                self.actions.direction = Direction::Right;
            }
            Button::Keyboard(Key::Up) => self.grid.upshift_all(),
            Button::Keyboard(Key::Down) => {
                self.grid.downshift_all(&mut self.actions);
                self.actions.down_held = true;
            }
            Button::Keyboard(Key::Space) => self.grid.hard_drop(&mut self.actions),
            Button::Keyboard(Key::X) => self.rotate_clockwise(),

            Button::Keyboard(Key::Z) => self.rotate_counterclockwise(),

            Button::Keyboard(Key::C) => self.hold_piece(),
            _ => (),
        }
    }

    pub fn on_released(&mut self, button: &Button) {
        match button {
            Button::Keyboard(Key::Left) => {
                if matches!(self.actions.direction, Direction::Left) {
                    self.actions.direction = Direction::None;
                    self.actions.frames_until_das_activates = self.actions.das_delay;
                    self.actions.frames_until_next_das = self.actions.das_speed
                }
            }
            Button::Keyboard(Key::Right) => {
                if matches!(self.actions.direction, Direction::Right) {
                    self.actions.direction = Direction::None;
                    self.actions.frames_until_das_activates = self.actions.das_delay;
                    self.actions.frames_until_next_das = self.actions.das_speed
                }
            }
            Button::Keyboard(Key::Down) => self.actions.down_held = false,

            _ => (),
        }
    }

    pub fn rotate_clockwise(&mut self) {
        let piece_coordinates = self.grid.get_alive_tiles();
        let mut piece_copy = Piece {
            rotation: self.grid.piece.rotation,
            piece_type: self.grid.piece.piece_type,
        };
        let result = piece_copy.rotate_clockwise(&mut self.grid);
        match result {
            PieceResult::Success => {
                self.grid.recalculate_stall(&mut self.actions);
            }
            PieceResult::Failure => (),
        }
    }

    pub fn rotate_counterclockwise(&mut self) {
        let piece_coordinates = self.grid.get_alive_tiles();
        let mut piece_copy = Piece {
            rotation: self.grid.piece.rotation,
            piece_type: self.grid.piece.piece_type,
        };
        let result = piece_copy.rotate_counterclockwise(&mut self.grid);
        match result {
            PieceResult::Success => {
                self.grid.recalculate_stall(&mut self.actions);
            }
            PieceResult::Failure => (),
        }
    }

    pub fn hold_piece(&mut self) {
        if !self.grid.held.available {
            return;
        }
        match self.grid.held.held_type {
            Some(PieceType) => {
                let current_held = self.grid.held.held_type.clone();
                self.grid.held = Held {
                    held_type: Some(self.grid.piece.piece_type.clone()),
                    available: true,
                };
                self.grid.remove_all_alive_tiles();
                self.grid.generate_specific_piece(current_held);
            }
            None => {
                self.grid.held = Held {
                    held_type: Some(self.grid.piece.piece_type.clone()),
                    available: true,
                };
                self.grid.remove_all_alive_tiles();
                self.grid.generate_new_piece();
            }
        }
        self.grid.held.available = false;
        self.actions.gravity_timer = self.actions.gravity_delay;
        self.actions.stall_lock_count = self.actions.stall_lock_max;
        self.actions.piece_can_lock_timer = 30;
    }
}

pub struct Actions {
    direction: Direction,

    down_held: bool,
    ///How many frames until the piece downshifts to down held
    down_held_timer: u8,
    ///How many frames it will take for piece to downshift due to down held
    down_held_delay: u8,
    ///How many frames it will take for piece to downshift due to gravity
    gravity_delay: u8,
    ///How many frames until the piece downshifts to gravity
    gravity_timer: u8,

    ///How many frames it takes for DAS to activate
    das_delay: u8,
    ///How many remaining frames until DAS activates
    frames_until_das_activates: u8,
    ///Once DAS activates, it does so every x frames
    das_speed: u8,
    ///The number of frames since DAS last activated
    frames_until_next_das: u8,

    ///Number of actions that can stall the downshift
    pub stall_lock_max: u8,
    ///Number of stalls currently used
    pub stall_lock_count: u8,
    ///Whether or not to lock the downshift when downshift_all is called
    pub piece_can_lock: bool,
    pub piece_can_lock_timer: u8
}

impl Actions {
    pub fn new() -> Actions {
        Actions {
            direction: Direction::None,
            down_held: false,
            down_held_delay: 3,
            down_held_timer: 3,
            gravity_delay: 5,
            gravity_timer: 5,
            das_delay: 10,
            frames_until_das_activates: 10,
            das_speed: 3,
            frames_until_next_das: 3,
            stall_lock_count: 0,
            stall_lock_max: 15,
            piece_can_lock_timer: 30,
            piece_can_lock: false,
        }
    }
}
pub enum Direction {
    Left,
    Right,
    None,
}
