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
    //
    b.new_piece();
    b.shift_left();
    b.shift_left();
    b.shift_left();
    b.shift_left();
    loop {
        //println!("{:?}", b.block);
        //b.rotate3();
        //b.shift2(1,2);
        //b.shift2(board::cell::Direction::Right);
        //b.shift_left();
        b.shift_right();
        b.shift_down();

        //println!("{:?}", b.block);
        println!("{}", b);
        //std::thread::sleep_ms(500);
        let t = std::time::Duration::from_millis(200);
        std::thread::sleep(t);
    }
}
