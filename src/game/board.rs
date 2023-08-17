#![allow(dead_code)]

pub struct Board{
    gamefield: Vec<Vec<String>>,
}

impl Board{
    pub fn empty()->Board{
        Board{
            gamefield: vec![vec![String::from(""); 10]; 20]
        }
        
    }

    pub fn get_value(&self, row: usize, col: usize)->&String{
        &self.gamefield[row][col]
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: String){
        self.gamefield[row][col] = value;
    }

    pub fn downshift(&mut self, row: usize, col: usize){
        self.gamefield[row-1][col] = String::from("piece");
        self.gamefield[row][col] = String::from("");
    }

}


pub enum Piece{
    OPiece,
    TPiece,
    ZPiece,
    SPiece,
    LPiece,
    JPiece,

}

pub struct OPiece{

}
pub struct ZPiece{

}

pub struct SPiece{

}

pub struct LPiece{
    x: u16
}


pub struct JPiece{

}

pub struct TPiece{

}