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
        pub block: Option<Piece>,
        //block: Piece,
    }


    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut copy = self.clone();
            //println!("{:?}", copy.block);
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
                    2,
                    Shape::rand(),
                    Color::rand()));
        }
        pub fn rotate2(&mut self) {
            if self.block.is_some() {
                let rotated = self.block.unwrap().rotate4();
                if self.compatible2(&rotated) {
                    self.block = Some(rotated);
                }
            }
        }
        pub fn rotate3(&mut self) {
            //rotate without calling something on Piece
            if let Some(mut clone) = self.block {
                let max_y = clone.cells.iter().fold(
                    std::usize::MIN,
                    |max, c| std::cmp::max(max, c.y));
                if clone.cells.iter()
                    .all(|&Cell{x,y,..}| 
                         self.get(
                                 max_y-y+clone.x, 
                                 x+clone.y)
                            .is_none()) {
                    if let Some(ref mut block) = self.block {
                        for cell in block.cells.iter_mut() {
                            let tmp = cell.x;
                            cell.x = max_y - cell.y;
                            cell.y = tmp;
                        }
                    }
                }
            }
        }
        pub fn shift(&mut self, dx: i32, dy: i32) -> bool {
            //attempts to shift self.
            //self.block.unwrap().x = 0;
            //let = self.block.unwrap().cells.into_iter();
            //if let Some(block) =
            /*should this functionality be here (it's simple) 
             * or in Piece (that's where rotate is.
             * Should I move rotate?
             */
            if let Some(mut clone) = self.block {
                let points = [(0i32,0i32); 4];
                //clone.cells.iter().map(|_| 0);
                if clone.cells.iter().all(
                    |&Cell{x,y,..}|
                    {
                        let x_ = (x as i32) + dx;
                        let y_ = (y as i32) + dy;
                        if x_ < 0 || y_ < 0 {
                            false
                        } else {
                            let x_ = x_ as usize;
                            let y_ = y_ as usize;
                            clone.x + x_ < WIDTH  && 
                            clone.y + y_ < HEIGHT &&
                                self.get(x_,y_).is_none()
                        } 
                    }) {
                    //a valid move
                    if let Some(ref mut block) = self.block {
                        for cell in block.cells.iter_mut() {
                            cell.x = ((cell.x as i32) + dx) as usize;
                            cell.y = ((cell.y as i32) + dy) as usize;

                        }
                    }

                }
                //self.compatible4(a);
                   
                //if clone.cells.iter()
                //    .all(|&Cell{x,y,..}|
                //         self.get(
                //             clone.x+x+dx 
                true     
            } else {
                false
            }
        }
        fn shift2(&mut self, dir: Direction) -> bool {
            if let Some(mut clone) = self.block {
                clone.shift(dir);
                if self.compatible2(&clone) {
                    self.block = Some(clone);
                }
            }
            true
        }
        fn compatible4(&self, cells: &[(i32,i32);4]) -> bool {
            //wtf is the best way to do this
            cells.iter().all(|&(x,y)| 
                             0<=x && (x as usize) < WIDTH &&
                             0<=y && (y as usize) < HEIGHT &&
                             //x < WIDTH && y < HEIGHT && 
                             //self.get(x,y).is_none())
                             self.get(
                                     x as usize,
                                     y as usize)
                                 .is_none())
        }
        fn compatible3(&self, cells: &[Cell; 4]) -> bool {
            cells.iter().all(|&Cell{x,y,..}| 
                             x < WIDTH && y < HEIGHT && 
                             self.get(x,y).is_none())
        }
		fn compatible2(&self, p: &Piece) -> bool {
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
            self.shift2(Direction::Left);
        }
        pub fn shift_right(&mut self) {
            self.shift2(Direction::Right);
        }
        pub fn shift_down(&mut self) {
            self.shift2(Direction::Down);
        }

    }
}
