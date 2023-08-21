use rand::Rng;
use core::clone::Clone;
use core::cmp::PartialEq;
use core::fmt;
use super::{piece::{Piece, PieceType, Gamepiece, Held}, game::Actions};




pub struct Board{
    pub field: Vec<Vec<Tile>>,
    pub piece: Piece,
    pub held: Held
}
impl Board{

    pub fn new() -> Board{
        Board { field: vec![vec![Tile::Empty; 10]; 20], piece: Piece { rotation: 0, piece_type: PieceType::OPiece }, held: Held{held_type: None, available: true}}
    }

    pub fn get_value(&self, point: Point)->Tile{
        self.field[point.row as usize][point.col as usize].clone()
    }

    pub fn get_value_by_coords(&self, row: i8, col: i8)->Tile{
        self.field[row as usize][col as usize].clone()
    }

    pub fn set_value(&mut self, point: Point, value: Tile){
        self.field[point.row as usize][point.col as usize] = value;
    }
    pub fn set_value_by_coords(&mut self, row:i8, col: i8, value: Tile){
        self.field[row as usize][col as usize] = value;
    }

    //*Scans all tiles, from left to right, top to bottom, and returns the alive ones in a vec in the order found */
    pub fn get_alive_tiles(&mut self) -> Vec<Point>{
        let mut alive_tiles: Vec<Point> = vec![];
        for row in 0..20{
            for col in 0..10{
                match self.field[row][col]{
                    Tile::Alive => alive_tiles.push(Point{row: row as i8, col: col as i8}),
                    _ => ()
                }
            }
        }
        return alive_tiles

    }

    pub fn remove_all_alive_tiles(&mut self){
        for row in 0..20{
            for col in 0..10{
                match self.field[row][col]{
                    Tile::Alive => self.field[row][col] = Tile::Empty,
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

            match self.get_value_by_coords(tile.row, tile.col+1){
                Tile::Dead => return,
                _ => ()
            }
        }

        //*Do it in two parts to avoid changes overriding each other
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col, Tile::Empty);
        }
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col+1, Tile::Alive);
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

            match self.get_value_by_coords(tile.row, tile.col-1){
                Tile::Dead => return,
                _ => ()
            }
        }

        //*Do it in two parts to avoid changes overriding each other
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col, Tile::Empty);
        }
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col-1, Tile::Alive);
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

            match self.get_value_by_coords(tile.row-1, tile.col){
                Tile::Dead => return,
                _ => ()
            }
        }

        //*Do it in two parts to avoid changes overriding each other
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col, Tile::Empty);
        }
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row-1, tile.col, Tile::Alive);
        }

    }

    pub fn downshift_all(&mut self, actions: &mut Actions){
        let alive_tiles = self.get_alive_tiles();
        

        for tile in &alive_tiles{
            match tile.row{
                19 => 
                {
                    if actions.piece_can_lock{
                        self.lock_piece(actions);
                    }
                    return
                    
                },
                _ => ()
            }

            match self.get_value_by_coords(tile.row+1, tile.col){
                Tile::Dead => {
                    if actions.piece_can_lock{
                        self.lock_piece(actions);
                    }
                    return;
                    
                },
                _ => ()
            }
        }

        //*Do it in two parts to avoid changes overriding each other
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col, Tile::Empty);
        }
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row+1, tile.col, Tile::Alive);
        }

        actions.piece_can_lock = false;
        actions.piece_can_lock_timer = 30;


    }


    fn lock_piece(&mut self, actions: &mut Actions){
        for row in 0..20{
            for col in 0..10{
                match self.field[row][col]{
                    Tile::Alive => self.field[row][col] = Tile::Dead,
                    _ => ()
                }
            }
        }
        self.check_for_line_clears();
        self.generate_new_piece();
        self.held.available = true;
        actions.stall_lock_count = actions.stall_lock_max;
        actions.piece_can_lock = false;
        
    }
    

    //*A modified version of the downshift_all function */
    pub fn hard_drop(&mut self, actions: &mut Actions){
        loop {
            let alive_tiles = self.get_alive_tiles();
        

        for tile in &alive_tiles{
            match tile.row{
                19 => 
                {
                    self.lock_piece(actions);
                    return;
                },
                0..=18 => (),
                _ => panic!("downshift OOB")
            }

            match self.get_value_by_coords(tile.row+1, tile.col){
                Tile::Dead => 
                {
                    self.lock_piece(actions);
                    return;
                },
                _ => ()
            }
        }

        //*Do it in two parts to avoid changes overriding each other
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row, tile.col, Tile::Empty);
        }
        for tile in &alive_tiles{
            self.set_value_by_coords(tile.row+1, tile.col, Tile::Alive);
        }
        }
    }

    fn check_for_line_clears(&mut self){
        let mut lines_cleared: Vec<usize> = vec![];
        'row: for row in (0..20).rev(){
            for col in 0..10{
                match self.field[row][col]{
                    Tile::Alive | Tile::Empty => continue 'row,
                    Tile::Dead => ()
                }
            }
            //*This line should be cleared */
            self.field[row] = vec![Tile::Empty; 10];
            lines_cleared.insert(0, row);
        }

        for line in lines_cleared{  
            for row in (0..line).rev(){
                for col in 0..10{
                    self.field[row+1][col] = self.field[row][col];
                    self.field[row][col] = Tile::Empty;
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
            match self.get_value(point){
                Tile::Alive | Tile::Dead => return,
                Tile::Empty => ()
            }
        }
        for point in self.piece.spawn_coordinates(){
            self.set_value(point, Tile::Alive);
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
            match self.get_value(point){
                Tile::Alive | Tile::Dead => return,
                Tile::Empty => ()
            }
        }
        for point in self.piece.spawn_coordinates(){
            self.set_value(point, Tile::Alive);
        }
    }
    
}

pub enum Tile{
    Dead,
    Alive,
    Empty
}

impl Copy for Tile {}

impl Clone for Tile{
    fn clone(&self) -> Self{
        *self
    }
}
impl PartialEq for Tile{
    fn eq(&self, other:&Self) -> bool{
        match (self, other){
            (Tile::Dead, Tile::Dead) => true,
            (Tile::Alive, Tile::Alive) => true,
            (Tile::Empty, Tile::Empty) => true,
            _ => false
        }
    }
}

impl fmt::Debug for Tile{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instance of Tile")
    }
}

pub struct Point{
    pub row: i8,
    pub col: i8
}