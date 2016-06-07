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
    #[derive(Debug, Clone, Copy)]
    pub struct Board {
        //stores the actual table itself (2d array)
        //and the piece that is currently moving
        table: [[Option<Cell>; WIDTH]; HEIGHT],
        block: Option<Piece>,
        //block: Piece,
    }


    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut copy = self.clone();
            if let Some(piece) = copy.block {
                copy.incorporate(&piece);
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
                /*block: Piece::new(
                    rand::random::<usize>() % WIDTH,
                    //rand::random::<usize>() % HEIGHT,
                    2,
                    Shape::rand(),
                    Color::rand()
                    )*/
            }
        }
        fn get(&self, x:usize, y:usize) -> Option<Cell> {
            self.table[y][x]
        }
        pub fn new_piece(&mut self) {
            assert!(self.block.is_none());
            self.block = Some(Piece::new(
                    //(rand::random::<usize>() % WIDTH)-3,
                    //(rand::random::<usize>() % HEIGHT)-3,
                    WIDTH/2-1,
                    0,
                    Shape::rand(),
                    Color::rand()));
        }
        pub fn rotate(&mut self) {
            //rotates the active piece if it can be rotated
            //self.block.unwrap();
            if let Some(mut block) = self.block {
                let points = block.rotate();
                if self.compatible(&points) {
                    block.set_coords(&points);
                    self.block = Some(block);
                }
            }
            //if let Some(mut block) = self.block.clone() {
            //    let points = block.rotate();
            //    if self.compatible(&points) {
            //        block.set_coords(&points);
            //        self.block = Some(block);
            //    }
            //}
        }
        pub fn shift(&mut self, dx: i32, dy: i32) -> bool {
            //attempts to shift self.
            //self.block.unwrap().x = 0;
            //let = self.block.unwrap().cells.into_iter();
            //if let Some(block) =
            true
        }
        fn compatible(&self, 
                      points: &[(usize,usize); 4]) -> bool {
            points.iter().all( | &(x,y) | 
                x<WIDTH && y<HEIGHT &&
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
    }
}
