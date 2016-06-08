//pub mod block;

pub mod block;

pub mod board {
    extern crate rand;
    extern crate std;
    use board::block::cell::*;
    use board::block::*;
    use std::fmt;

    pub const WIDTH:  usize = 20;
    pub const HEIGHT: usize = 20;

    //Board is an array of rows
    #[derive(Debug, Clone, Copy)]
    pub struct Board {
        //stores the actual table itself (2d array)
        //and the piece that is currently moving
        table: [[Option<Cell>; WIDTH]; HEIGHT],
        block: Option<Piece>,
    }


    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut copy = self.clone();
            if let Some(piece) = copy.block {
                copy.incorporate(&piece);
            } else {
                println!("failed to incorp");
            }
            try!(write!(f, "\n\t"));
            for i in copy.table.iter() {
                for j in i {
                    match *j {
                        Some(c) => try!(write!(f, "{}", c)),
                        None    => try!(write!(f, " ")),
                    };
                }
                try!(write!(f, "\n\t"));
            }
            writeln!(f, "")
        }
    }


    impl Board{
        pub fn new() -> Board {
            Board {
                table: [[None; WIDTH]; HEIGHT],
                block: None,
            }
        }
        fn get(&self, x:usize, y:usize) -> Option<Cell> {
            self.table[y][x]
        }
        pub fn new_piece(&mut self) {
            assert!(self.block.is_none());
            self.block = Some(Piece::new(
                    WIDTH/2-1,
                    2,
                    Shape::rand(),
                    Color::rand()));
        }
        fn shift(&mut self, dir: Direction) -> bool {
            if let Some(mut clone) = self.block {
                if dir == Direction::Counterclockwise {
                    clone.rotate();
                } else {
                    clone.shift(dir);
                }
                if self.compatible(&clone) {
                    self.block = Some(clone);
                }
            }
            true
        }
		fn compatible(&self, p: &Piece) -> bool {
			p.cells.iter().all(
				|&Cell{x,y,..}|
				p.x+x < WIDTH  &&
				p.y+y < HEIGHT &&
				self.get(x,y).is_none())
		}
        fn incorporate(&mut self, p: &Piece) {
            for &c in p.cells.iter() {
                self.table[p.y+c.y][p.x+c.x] = Some(c);
            }
        }
        #[allow(dead_code)]
        pub fn random() -> Board {
            //used for testing display
            let mut b = Board::new();
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let opts = 8;   // ROYGBIV + 1
                    use board::block::cell::Color::*;
                    b.table[y][x] = match 
                        rand::random::<u8>() % opts {
                        0 => Some(Cell{x:x, y:y, col:Red}),
                        1 => Some(Cell{x:x, y:y, col:Orange}),
                        2 => Some(Cell{x:x, y:y, col:Yellow}),
                        3 => Some(Cell{x:x, y:y, col:Green}),
                        4 => Some(Cell{x:x, y:y, col:Blue}),
                        5 => Some(Cell{x:x, y:y, col:Indigo}),
                        6 => Some(Cell{x:x, y:y, col:Violet}),
                        _ => None,
                    };
                }
            }
            b
        }
        //for testing:
        pub fn shift_left(&mut self) {
            self.shift(Direction::Left);
        }
        pub fn shift_right(&mut self) {
            self.shift(Direction::Right);
        }
        pub fn shift_down(&mut self) {
            self.shift(Direction::Down);
        }

    }
}
