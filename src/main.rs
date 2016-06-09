//mod block;
mod board;
use board::board::*;
extern crate rand; 
extern crate ncurses;

fn main() {
    //let b = board::board::new();
    //board::new();
    //let mut b = board::Board::new();
    //println!("{}", b);
    //b = board::Board::new();
    //let mut p = board::block::Piece::new(5,5,);

    let mut b = Board::new();
    b.new_piece();

    loop {
        b.update();
        println!("{}", b);
        println!("\n\n\n\n\n");
        //std::thread::sleep_ms(500);
        let t = std::time::Duration::from_millis(200);
        std::thread::sleep(t);
    }
}
