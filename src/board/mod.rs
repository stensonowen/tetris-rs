//pub mod block;

pub mod block;

pub mod board {
    extern crate rand;
    use board::block::cell::*;
    use board::block::*;
    use std::fmt;

    pub const WIDTH:  usize = 20;
    pub const HEIGHT: usize = 20;

    //Board is an array of rows
    #[derive(Debug)]
    pub struct Board {
        //stores the actual table itself (2d array)
        //and the piece that is currently moving
        table: [[Option<Cell>; WIDTH]; HEIGHT],
        block: Option<Piece>,
    }


    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            try!(write!(f, "\n\t"));
            for i in self.table.iter() {
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
        pub fn random() -> Board {
            let mut b = Board::new();
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let opts = 8;   // ROYGBIV + 1
                    use board::block::cell::Color::*;
                    b.table[y][x] = match rand::random::<u8>()%opts {
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
        pub fn get(&self, x:usize, y:usize) -> Option<Cell> {
            self.table[y][x]
        }
        pub fn compatible2(&self, points: &[(usize,usize); 4])->bool{
            points.iter().all( | &(x,y) | 
                x<WIDTH && y<HEIGHT &&
                    self.get(x,y).is_none())
        }
        pub fn compatible(&self, p: &Piece) -> bool {
            p.cells.iter().all(
                |&Cell{x,y,..}| 
                0 <= p.x+x && p.x+x < WIDTH  &&
                0 <= p.y+y && p.y+y < HEIGHT &&
                self.get(x,y).is_none())
        }
        pub fn incorporate(&mut self, p: &Piece) {
            for &c in p.cells.iter() {
                self.table[p.y+c.y][p.x+c.x] = Some(c);
            }
        }
    }
}
