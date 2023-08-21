use super::board::{Point, Tile, Board};
pub struct Piece{
    //*Number of clockwise rotations. 0 is base, goes to 3 */
    pub rotation: usize,
    pub piece_type: PieceType
}

pub enum PieceType{
    OPiece,
    TPiece,
    IPiece,
    ZPiece,
    SPiece,
    LPiece,
    JPiece, 
} 

pub trait Gamepiece{
    fn spawn_coordinates(&self)->Vec<Point>;
    fn rotate_clockwise(&mut self, gamefield: &mut Board) -> PieceResult;
    fn rotate_counterclockwise(&mut self, gamefield: &mut Board) -> PieceResult;
    fn get_rotation_transformation(&self, direction: RotationDirection) -> &[(i8, i8); 4];
    fn get_wallkicks(&self, direction: RotationDirection)->[(i8, i8); 5];
    const O_ROTATIONS: [[(i8, i8); 4]; 4];
    const T_ROTATIONS: [[(i8, i8); 4]; 4];
    const I_ROTATIONS: [[(i8, i8); 4]; 4];
    const Z_ROTATIONS: [[(i8, i8); 4]; 4];
    const S_ROTATIONS: [[(i8, i8); 4]; 4];
    const L_ROTATIONS: [[(i8, i8); 4]; 4];
    const J_ROTATIONS: [[(i8, i8); 4]; 4];
    const O_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4];
    const T_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4];
    const I_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4];
    const Z_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4];
    const S_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4];
    const L_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4];
    const J_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4];
    
}

pub enum PieceResult{
    Success,
    Failure
}

//*Checks if a rotation is valid. If valid, preforms rotation and returns True. Otherwise, returns False */
pub fn rotation_is_valid(board: &mut Board, rotation: &[(i8, i8); 4], wallkick: (i8, i8)) -> bool{
    let alive_tiles = board.get_alive_tiles();
    for tile in 0..4{
        if alive_tiles[tile].row + rotation[tile].0 + wallkick.0 >= 20 || alive_tiles[tile].row + rotation[tile].0 + wallkick.0 < 0 || alive_tiles[tile].col + rotation[tile].1 + wallkick.1 >= 10 || alive_tiles[tile].col + rotation[tile].1 + wallkick.1 < 0 {
            return false
        }
        match board.get_value_by_coords(alive_tiles[tile].row + rotation[tile].0 + wallkick.0, alive_tiles[tile].col + rotation[tile].1 + wallkick.1){
            Tile::Dead => return false,
            _ => ()
        }
    }

    board.remove_all_alive_tiles();
    for tile in 0..4{
        board.set_value_by_coords(alive_tiles[tile].row + rotation[tile].0 + wallkick.0, alive_tiles[tile].col + rotation[tile].1 + wallkick.1, Tile::Alive);
        
    }
    println!("");
    return true;
}


impl Gamepiece for Piece{
    const O_ROTATIONS: [[(i8, i8); 4]; 4] = [[(0,0),(0,0),(0,0),(0,0)], [(0,0),(0,0),(0,0),(0,0)], [(0,0),(0,0),(0,0),(0,0)], [(0,0),(0,0),(0,0),(0,0)]];
    const T_ROTATIONS: [[(i8, i8); 4]; 4] = [[(0,0),(1,1),(0,0),(0,0)], [(1, -1), (0,0), (0,0), (0,0)], [(0,0), (0,0), (-1, -1), (0,0)], [(0,0),(0,0),(0,0),(-1,1)]];
    const I_ROTATIONS: [[(i8, i8); 4]; 4] = [[(-1, 2), (0, 1), (1, 0), (2, -1)], [(2, 1), (1, 0), (0, -1), (-1, -2)], [(-2, 1), (-1, 0), (0, -1), (1, -2)], [(1, 2), (0, 1), (-1, 0), (-2, -1)]];
    const Z_ROTATIONS: [[(i8, i8); 4]; 4] = [[(0, 2), (1, 1), (0,0), (1, -1)], [(1, -2), (1, 0), (0, -1), (0, 1)], [(-1, 1), (0, 0), (-1, -1), (0, -2)], [(0, -1), (0, 1), (-1, 0), (-1, 2)]];
    const S_ROTATIONS: [[(i8, i8); 4]; 4] = [[(1, 0), (0, -1), (1, 2), (0, 1)], [(1, 1), (0,0), (1, -1), (0, -2)], [(0,-1), (-1, -2), (0, 1), (-1, 0)], [(0, 2), (-1, 1,), (0,0), (-1, -1)]];
    const L_ROTATIONS: [[(i8, i8); 4]; 4] = [[(2, 0), (-1, 1), (0,0), (1, -1)], [(1, 1), (0,0), (-1, -1), (0, -2)], [(-1, 1), (0,0), (1, -1), (-2, 0)], [(0, 2), (1, 1), (0,0), (-1, -1)]];
    const J_ROTATIONS: [[(i8, i8); 4]; 4] = [[(0,2), (-1, 1), (0,0), (1, -1)], [(1, 1), (2, 0), (0,0), (-1, -1)], [(-1, 1), (0,0), (1, -1), (0, -2)], [(1, 1), (0,0), (-2, -0), (-1, -1)]];

    const O_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4] = [[(0,0),(0,0),(0,0),(0,0)], [(0,0),(0,0),(0,0),(0,0)], [(0,0),(0,0),(0,0),(0,0)], [(0,0),(0,0),(0,0),(0,0)]];
    const T_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4]= [[(0,0),(0,0),(0,0),(1,-1)], [(0,0), (0,0), (0,0), (-1,-1)], [(-1,1), (0,0), (0,0), (0,0)], [(1,1), (0,0), (0,0), (0,0)]];
    const I_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4]= [[(2, 1), (1, 0), (0, -1), (-1, -2)], [(1, -2), (0, -1), (-1, 0), (-2, 1)], [(1, 2), (0, 1), (-1, 0), (-2, -1)], [(2, -1), (1, 0), (0, 1), (-1, 2)]];
    const Z_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4] = [[(0, 1), (1, 0), (0, -1), (1, -2)], [(0, -2), (0,0), (-1, -1), (-1, 1)], [(-1, 2), (0, 1), (-1, 0), (0, -1)], [(1, -1), (1, 1), (0,0), (0, 2)]];
    const S_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4] = [[(1, -1), (0, -2), (1, 1), (0,0)], [(0, 1), (-1, 0), (0, -1), (-1, -2)], [(0,0), (-1, -1), (0, 2), (-1, 1)], [(1, 2), (0, 1), (1, 0), (0, -1)]];
    const L_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4] = [[(0, -2), (1, 1), (0,0), (-1, -1)], [(1, -1), (0,0), (-1, 1), (-2, 0)], [(1, 1), (0,0), (-1, -1), (0, 2)], [(2, 0), (1, -1), (0,0), (-1, 1)]];
    const J_COUNTER_ROTATIONS: [[(i8, i8); 4]; 4] = [[(2, 0), (1, 1), (0,0), (-1, -1)], [(1, -1), (0, -2), (0,0), (-1, 1)], [(1, 1), (0,0), (-1, -1), (-2, 0)], [(1, -1), (0,0), (0, 2), (-1, 1)]];

    
    
    fn rotate_clockwise(&mut self, board: &mut Board) -> PieceResult{
        let rotation = self.get_rotation_transformation(RotationDirection::Clockwise);
        let wallkicks = self.get_wallkicks(RotationDirection::Clockwise);
        for kick in wallkicks{
            if rotation_is_valid(board, rotation, kick){
                return PieceResult::Success
            }
        }
        return PieceResult::Failure
    }

    fn rotate_counterclockwise(&mut self, board: &mut Board) -> PieceResult{
        let rotation = self.get_rotation_transformation(RotationDirection::Counterclockwise);
        let wallkicks = self.get_wallkicks(RotationDirection::Clockwise);
        for kick in wallkicks{
            if rotation_is_valid(board, rotation, kick){
                return PieceResult::Success
            }
        }
        return PieceResult::Failure
    }

    fn spawn_coordinates(&self)->Vec<Point> {
        match self.piece_type{
            PieceType::OPiece => vec![Point{row: 0, col: 4}, Point{row: 0, col: 5}, Point{row: 1, col: 4}, Point{row: 1, col: 5}],
            PieceType::TPiece => vec![Point{row: 1, col: 3}, Point{row: 1, col: 4}, Point{row: 1, col: 5}, Point{row: 0, col:4}],
            PieceType::IPiece => vec![Point{row: 0, col: 3}, Point{row: 0, col: 4}, Point{row: 0, col: 5}, Point{row: 0, col: 6}],
            PieceType::ZPiece => vec![Point{row: 0, col: 3}, Point{row: 0, col: 4}, Point{row: 1, col: 4}, Point{row: 1, col: 5}],
            PieceType::SPiece => vec![Point{row: 0, col: 4}, Point{row: 0, col: 5}, Point{row: 1, col: 3}, Point{row: 1, col: 4}],
            PieceType::JPiece => vec![Point{row: 1, col: 3}, Point{row: 1, col: 4}, Point{row: 1, col: 5}, Point{row: 0, col:3}],
            PieceType::LPiece => vec![Point{row: 1, col: 3}, Point{row: 1, col: 4}, Point{row: 1, col: 5}, Point{row: 0, col:5}], 
        }
    }
    fn get_rotation_transformation(&self, direction: RotationDirection) -> &[(i8, i8); 4] {
        match direction{
            RotationDirection::Clockwise => {
                match self.piece_type{
                    PieceType::IPiece => return &Piece::I_ROTATIONS[self.rotation],
                    PieceType::JPiece => return &Piece::J_ROTATIONS[self.rotation],
                    PieceType::LPiece => return &Piece::L_ROTATIONS[self.rotation],
                    PieceType::OPiece => return &Piece::O_ROTATIONS[self.rotation],
                    PieceType::SPiece => return &Piece::S_ROTATIONS[self.rotation],
                    PieceType::TPiece => return &Piece::T_ROTATIONS[self.rotation],
                    PieceType::ZPiece => return &Piece::Z_ROTATIONS[self.rotation]
                }
            },
            RotationDirection::Counterclockwise => {
                match self.piece_type{
                    PieceType::IPiece => return &Piece::I_COUNTER_ROTATIONS[self.rotation],
                    PieceType::JPiece => return &Piece::J_COUNTER_ROTATIONS[self.rotation],
                    PieceType::LPiece => return &Piece::L_COUNTER_ROTATIONS[self.rotation],
                    PieceType::OPiece => return &Piece::O_COUNTER_ROTATIONS[self.rotation],
                    PieceType::SPiece => return &Piece::S_COUNTER_ROTATIONS[self.rotation],
                    PieceType::TPiece => return &Piece::T_COUNTER_ROTATIONS[self.rotation],
                    PieceType::ZPiece => return &Piece::Z_COUNTER_ROTATIONS[self.rotation]
                }

            }
    }
        
    }

    fn get_wallkicks(&self, direction: RotationDirection)->[(i8, i8); 5] {
        match self.piece_type{
            PieceType::IPiece => {
                match self.rotation{
                    0 => {
                        match direction{
                            RotationDirection::Clockwise => return [(0,0), (0, -2), (0, 1), (1, -2), (2, 1)],
                            RotationDirection::Counterclockwise => return [(0,0), (0, -1), (0, 2), (2, 1), (1, 2)]
                        }
                    },
                    1 => {
                        match direction{
                            RotationDirection::Clockwise => return [(0,0), (0, -1), (0, 2), (-2, -1), (1, 2)],
                            RotationDirection::Counterclockwise => return [(0,0), (0, 2), (0, -2), (-1, 2), (2, -1)]
                        }
                    },
                    2 => {
                        match direction{
                            RotationDirection::Clockwise => return [(0,0), (0, 2), (0, -1), (-1, 2), (2, -1)],
                            RotationDirection::Counterclockwise => return [(0,0), (0, 1), (0, -2), (2, -1), (-1, -2)]
                        }
                    }
                    3 => {
                        match direction{
                            RotationDirection::Clockwise => return [(0,0), (0, 1), (0, -2), (2, 1), (-1, -2)],
                            RotationDirection::Counterclockwise => return [(0,0), (0, -2), (0, 1), (1, -2), (-2, 1)]
                        }
                    }
                    _ => panic!("ROTATION ERROR")
                }
            }
            _ => {
                match self.rotation{
                    0 => {
                        match direction{
                            RotationDirection::Clockwise => return [(0,0), (0, -1), (-1, -1), (2, 0), (2, -1)],
                            RotationDirection::Counterclockwise => return [(0,0), (0, 1), (-1,1), (2, 0), (2, 1)]
                        }
                    },
                    1 => {
                        match direction{
                            RotationDirection::Clockwise => return [(0,0), (0, 1), (1, 1), (-2, 0), (-2, 1)],
                            RotationDirection::Counterclockwise => return [(0,0), (0, 1), (1, 1), (-2, 0), (-2, 1)]
                        }
                    }

                    2 => {
                        match direction{
                            RotationDirection::Clockwise => return [(0,0), (0, 1), (-1, 1), (2, 0), (2, 1)],
                            RotationDirection::Counterclockwise => return [(0,0), (0, -1), (-1, -1), (2, 0), (2, -1)]
                        }
                    },

                    3 => {
                        match direction{
                            RotationDirection::Clockwise => return [(0,0), (0, -1), (1, -1), (-2, 0), (-2, -1)],
                            RotationDirection::Counterclockwise => return [(0,0), (0, -1), (1, -1), (-2, 0), (-2, -1)]
                        }
                    },
                    _ => panic!("INVALID ROTATION")
                }
            }
        }
    }

    

    
}

impl Copy for PieceType {}

impl Clone for PieceType{
    fn clone(&self) -> PieceType {
        match self{
            PieceType::IPiece => PieceType::IPiece,
            PieceType::JPiece => PieceType::JPiece,
            PieceType::LPiece => PieceType::LPiece,
            PieceType::SPiece => PieceType::SPiece,
            PieceType::ZPiece => PieceType::ZPiece,
            PieceType::OPiece => PieceType::OPiece,
            PieceType::TPiece => PieceType::TPiece

        }
    }
}
pub enum RotationDirection{
    Clockwise,
    Counterclockwise
}

pub struct Held{
    pub held_type: Option<PieceType>,
    pub available: bool
}