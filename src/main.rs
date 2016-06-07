//mod block;
mod board;
use board::board::*;
extern crate rand; 

fn main() {
    //let b = board::board::new();
    //board::new();
    //let mut b = board::Board::new();
    //println!("{}", b);
    //b = board::Board::new();
    //let mut p = board::block::Piece::new(5,5,);

    let mut b = Board::new();
    //println!("{:?}", p);
    //p.rotate_counterclockwise();
    //b.incorporate(&p);
    //println!("{}", b);
    b.new_piece();
    b.rotate2();
    println!("{}", b);

}
