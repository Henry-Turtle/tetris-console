use rand::Rng;
use core::clone::Clone;
use core::cmp::PartialEq;
use core::fmt;
use std::ops::Add;
use super::{piece::{Piece, PieceType, Gamepiece, Held}, game::Actions};


pub struct Board{
    pub field: Vec<Vec<Tile>>,
    pub piece: Piece,
    pub held: Held
}
impl Board{

    pub fn new() -> Board{
        Board { field: vec![vec![Tile{status: TileStatus::Empty, piece_type: None}; 10]; 20], piece: Piece { rotation: 0, piece_type: PieceType::OPiece }, held: Held{held_type: None, available: true}}
    }

    pub fn get_value(&self, point: Point)->Tile{
        self.field[point.row as usize][point.col as usize].clone()
    }

    pub fn get_value_by_coords(&self, row: i8, col: i8)->Tile{
        self.field[row as usize][col as usize].clone()
    }

    pub fn set_value(&mut self, point: Point, value: TileStatus){
        self.field[point.row as usize][point.col as usize] = Tile{piece_type: Some(self.piece.piece_type), status: value};
    }
    pub fn set_value_by_coords(&mut self, row:i8, col: i8, value: TileStatus){
        self.field[row as usize][col as usize] = Tile{piece_type: Some(self.piece.piece_type), status: value};
    }

    //*Scans all tiles, from left to right, top to bottom, and returns the alive ones in a vec in the order found */
    pub fn get_alive_tiles(&mut self) -> Vec<Point>{
        let mut alive_tiles: Vec<Point> = vec![];
        for row in 0..20{
            for col in 0..10{
                match self.field[row][col].status{
                    TileStatus::Alive => alive_tiles.push(Point{row: row as i8, col: col as i8}),
                    _ => ()
                }
            }
        }
        return alive_tiles

    }

    pub fn remove_all_alive_tiles(&mut self){
        for row in 0..20{
            for col in 0..10{
                match self.field[row][col].status{
                    TileStatus::Alive => self.field[row][col] = Tile{status: TileStatus::Empty, piece_type: None},
                    _ => ()
                }
            }
        }
    }

    pub fn rightshift_all(&mut self, actions: &mut Actions){
        let alive_tiles = self.get_alive_tiles();

        for tile in &alive_tiles{
            match tile.col{
                9 => return,
                0..=8 => (),
                _ => panic!("rightshift OOB")
            }

            match self.get_value_by_coords(tile.row, tile.col+1).status{
                TileStatus::Dead => return,
                _ => ()
            }
        }

        //*Do it in two parts to avoid changes overriding each other
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col, TileStatus::Empty);
        }
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col+1, TileStatus::Alive);
        }
        self.recalculate_stall(actions);
    }

    pub fn leftshift_all(&mut self, actions: &mut Actions){
        let alive_tiles = self.get_alive_tiles();
        for tile in &alive_tiles{
            match tile.col{
                0 => return,
                1..=9=> (),
                _ => panic!("leftshift OOB")
            }

            match self.get_value_by_coords(tile.row, tile.col-1).status{
                TileStatus::Dead => return,
                _ => ()
            }
        }

        //*Do it in two parts to avoid changes overriding each other
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col, TileStatus::Empty);
        }
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col-1, TileStatus::Alive);
        }
        self.recalculate_stall(actions);

    }
    pub fn upshift_all(&mut self){
        let alive_tiles = self.get_alive_tiles();

        for tile in &alive_tiles{
            match tile.row{
                0 => return,
                0..=19 => (),
                _ => panic!("upshift OOB")
            }

            match self.get_value_by_coords(tile.row-1, tile.col).status{
                TileStatus::Dead => return,
                _ => ()
            }
        }

        //*Do it in two parts to avoid changes overriding each other
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col, TileStatus::Empty);
        }
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row-1, tile.col, TileStatus::Alive);
        }

    }

    pub fn downshift_all(&mut self, actions: &mut Actions, score: &mut u128){
        let alive_tiles = self.get_alive_tiles();
        

        for tile in &alive_tiles{
            match tile.row{
                19 => 
                {
                    if actions.piece_can_lock{
                        self.lock_piece(actions, score);
                    }
                    return
                    
                },
                _ => ()
            }

            match self.get_value_by_coords(tile.row+1, tile.col).status{
                TileStatus::Dead => {
                    if actions.piece_can_lock{
                        self.lock_piece(actions, score);
                    }
                    return;
                    
                },
                _ => ()
            }
        }

        //*Do it in two parts to avoid changes overriding each other
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col, TileStatus::Empty);
        }
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row+1, tile.col, TileStatus::Alive);
        }

        actions.piece_can_lock = false;
        actions.piece_can_lock_timer = 30;


    }


    fn lock_piece(&mut self, actions: &mut Actions, score: &mut u128){
        for row in 0..20{
            for col in 0..10{
                match self.field[row][col].status{
                    TileStatus::Alive => self.field[row][col] = Tile{status: TileStatus::Dead, piece_type: Some(self.piece.piece_type)},
                    _ => ()
                }
            }
        }
        self.check_for_line_clears(actions, score);
        self.generate_new_piece();
        self.held.available = true;
        actions.stall_lock_count = actions.stall_lock_max;
        actions.piece_can_lock = false;
        
    }
    

    //*A modified version of the downshift_all function */
    pub fn hard_drop(&mut self, actions: &mut Actions, score: &mut u128){
        loop {
            let alive_tiles = self.get_alive_tiles();
        

        for tile in &alive_tiles{
            match tile.row{
                19 => 
                {
                    self.lock_piece(actions, score);
                    return;
                },
                0..=18 => (),
                _ => panic!("downshift OOB")
            }

            match self.get_value_by_coords(tile.row+1, tile.col).status{
                TileStatus::Dead => 
                {
                    self.lock_piece(actions, score);
                    return;
                },
                _ => ()
            }
        }

        //*Do it in two parts to avoid changes overriding each other
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col, TileStatus::Empty);
        }
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row+1, tile.col, TileStatus::Alive);
        }
        }
    }

    fn check_for_line_clears(&mut self, actions: &mut Actions, score: &mut u128){
        let mut lines_cleared: Vec<usize> = vec![];
        'row: for row in (0..20).rev(){
            for col in 0..10{
                match self.field[row][col].status{
                    TileStatus::Alive | TileStatus::Empty => continue 'row,
                    TileStatus::Dead => ()
                }
            }
            //*This line should be cleared */
            self.field[row] = vec![Tile{status: TileStatus::Empty, piece_type: None}; 10];
            lines_cleared.insert(0, row);
        }
        let mut score_added:u128 = match lines_cleared.len(){
            4 => 800,
            3 => 500,
            2 => 300,
            1 => 100, 
            _ => 0
        };
        score_added = score_added * (31-actions.gravity_delay) as u128;
        *score = *score + score_added;

        self.update_score(actions, score);
        println!("{}", score);
        for line in lines_cleared{  
            for row in (0..line).rev(){
                for col in 0..10{
                    self.field[row+1][col] = self.field[row][col];
                    self.field[row][col] = Tile{status: TileStatus::Empty, piece_type: None}; //might have broken something here
                }
            }
        }
    }
    pub fn recalculate_stall(&mut self, actions: &mut Actions){
        actions.piece_can_lock_timer = 30;
        actions.piece_can_lock = false;
        actions.stall_lock_count += 1;
    }
    pub fn generate_new_piece(&mut self){
        let newtype = match rand::thread_rng().gen_range(0..7){
            0 => PieceType::IPiece,
            1 => PieceType::JPiece,
            2 => PieceType::LPiece,
            3 => PieceType::OPiece,
            4 => PieceType::SPiece,
            5 => PieceType::TPiece,
            6 => PieceType::ZPiece,
            _ => panic!("Invalid pieceRNG")
        };
        println!("SPAWN {:?}", newtype);
        self.piece = Piece{piece_type: newtype, rotation: 0};
        for point in self.piece.spawn_coordinates(){
            match self.get_value(point).status{
                TileStatus::Alive | TileStatus::Dead => return,
                TileStatus::Empty => ()
            }
        }
        for point in self.piece.spawn_coordinates(){
            self.set_value(point, TileStatus::Alive);
        }
    }

    pub fn generate_specific_piece(&mut self, piecetype: Option<PieceType>){
        let newtype: PieceType;
        match piecetype{
            Some(T) => {
                newtype = T;
            },
            None => {
                self.generate_new_piece();
                return;
            }
        }
        self.piece = Piece{piece_type: newtype, rotation: 0};
        for point in self.piece.spawn_coordinates(){
            match self.get_value(point).status{
                TileStatus::Alive | TileStatus::Dead => return,
                TileStatus::Empty => ()
            }
        }
        for point in self.piece.spawn_coordinates(){
            self.set_value(point, TileStatus::Alive);
        }
    }

    fn update_score(&mut self, actions: &mut Actions, score: &mut u128){
        let score_copy = score.clone();
        let level = (score_copy as f64 / 1000.0) as i8;
        if 30-level > 0{
            actions.gravity_delay = 30 - level as u8
        }
        else{
            actions.gravity_delay = 1
        }
        
    }
    
}

#[derive(Copy, Clone)]
pub struct Tile{
    pub status: TileStatus,
    pub piece_type: Option<PieceType>

}
pub enum TileStatus{
    Dead,
    Alive,
    Empty
}

impl Copy for TileStatus {}

impl Clone for TileStatus{
    fn clone(&self) -> Self{
        *self
    }
}
impl PartialEq for TileStatus{
    fn eq(&self, other:&Self) -> bool{
        match (self, other){
            (TileStatus::Dead, TileStatus::Dead) => true,
            (TileStatus::Alive, TileStatus::Alive) => true,
            (TileStatus::Empty, TileStatus::Empty) => true,
            _ => false
        }
    }
}

impl fmt::Debug for TileStatus{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instance of Tile")
    }
}

pub struct Point{
    pub row: i8,
    pub col: i8
}