//mod block;
mod board;
use board::*;
use board::block::*;
use board::block::cell::*;
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
    let mut n = 0;
    for _ in 0..20 {
        //let mut p=Piece::new(5,5,Shape::rand(),Color::rand());
        let mut p = Piece::new(
            rand::random::<usize>() % 20,
            rand::random::<usize>() % 20,
            Shape::rand(),
            Color::rand());
        let pts = p.rotate_ccw();
        if b.compatible2(&pts) {
            p.set_coords(&pts);
            b.incorporate(&p);
            n += 1;
        }
    }
    println!("{} / 100", n);
    println!("{}", b);

}
