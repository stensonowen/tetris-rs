//pub mod block;

pub mod block;

pub mod board {
    extern crate rand;
    extern crate std;
    extern crate ncurses;
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
            }
            let horiz: String = std::iter::repeat("-")
                .take(WIDTH+1).collect();
            //try!(write!(f, "\n\t"));
            try!(write!(f, "\n\t{}|\n\t|", horiz.clone()));
            for i in copy.table.iter() {
                for j in i {
                    match *j {
                        Some(c) => try!(write!(f, "{}", c)),
                        None    => try!(write!(f, " ")),
                    };
                }
                try!(write!(f, "|\n\t|"));
            }
            writeln!(f, "{}", horiz)
        }
    }


    impl Board{
        pub fn new() -> Board {
            Board {
                table: [[None; WIDTH]; HEIGHT],
                block: None,
            }
        }
        pub fn get(&self, x:usize, y:usize) -> Option<Cell> {
            self.table[y][x]
        }
        pub fn new_piece(&mut self) {
            assert!(self.block.is_none());
            self.block = Some(Piece::new(
                    WIDTH/2-1,
                    0,
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
                    true
                } else {
                    false
                }
            } else {
                false 
            }
        }
		fn compatible(&self, p: &Piece) -> bool {
			p.cells.iter().all(
				|&Cell{x,y,..}|
				p.x+x < WIDTH  &&
				p.y+y < HEIGHT &&
				self.get(p.x+x,p.y+y).is_none())
		}
        fn incorporate(&mut self, p: &Piece) {
            for &c in p.cells.iter() {
                self.table[p.y+c.y][p.x+c.x] = Some(c);
            }
        }
        fn incorporate2(&mut self) {
            if let Some(ref b) = self.block {
                for &c in b.cells.iter() {
                    self.table[b.y+c.y][b.x+c.x] = Some(c);
                }
            }
            self.block = None;
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
        pub fn command(&mut self, command: i32) {
            //ncurses::getch() returns i32
            use board::block::cell::Direction::*;
            use ncurses::*;
            let _success = match command {
                //arros keys OR wasd OR hjkl
                 97|104|KEY_LEFT    => self.shift(Left), 
                100|108|KEY_RIGHT   => self.shift(Right),
                115|106|KEY_DOWN    => self.shift(Down),
                119|107|KEY_UP      => self.shift(Counterclockwise),
                _                   => false,
            };
        }
        pub fn update(&mut self) {
            //shift down and check for complete lines 
            if self.shift(Direction::Down) == false {
                //block couldn't shift down
                //add it to the board and create a new block
                self.incorporate2();
                self.new_piece();
                //start checking for full lines (from bottom?)
                for line in self.table.iter().rev() {
                    if line.iter().all(|x| x.is_some()) {
                        //full line
                        println!("full line");
                    }

                }
                
            }
                
        }
        //for testing:
        /*
        pub fn shift_left(&mut self) {
            self.shift(Direction::Left);
        }
        pub fn shift_right(&mut self) {
            self.shift(Direction::Right);
        }
        pub fn shift_down(&mut self) {
            self.shift(Direction::Down);
        }*/

    }
}
