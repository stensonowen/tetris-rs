//mod block;
mod board;
use board::*;
use board::block::*;
use board::block::cell::*;

fn main() {
    //let b = board::board::new();
    //board::new();
    //let mut b = board::Board::new();
    //println!("{}", b);
    //b = board::Board::new();
    //let mut p = board::block::Piece::new(5,5,);

    let mut b = Board::new();
    let mut p = Piece::new(5,5,Shape::rand(),Color::rand());
    println!("{:?}", p);
    p.rotate_counterclockwise();
    b.incorporate(&p);
    println!("{}", b);

}
