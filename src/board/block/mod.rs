//all the block stuff

extern crate std;

pub mod cell;
use board::block::cell::*;

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub x: usize,
    pub y: usize,
    pub cells: [Cell; 4],
}

impl Piece {
    /*Though it may be negligibly slower, I think it makes more
     * sense organizationally for shift/rotate to modify self 
     * and for Board to clone and discard them as necessary */
    pub fn shift(&mut self, dir: Direction) {
        //what should happen if it tries to move left of col 0??
        //Should nothing happen? Should it return None/false?
        match dir {
            Direction::Down     =>  self.y += 1,
            Direction::Right    =>  self.x += 1,
            Direction::Left     => {
                //for now, just return a clone I guess
                //the same behavior, just now defined in 2 places
                if self.x > 0 {     self.x -= 1 }
            }
        };
    }
    pub fn rotate(&mut self) {
        // `self` will be altered
        let max_y = self.cells.iter().fold(
            std::usize::MIN,
            |max, c| std::cmp::max(max, c.y));
        for cell in self.cells.iter_mut() {
            // (x', y') = (max_y - y, x)
            std::mem::swap(&mut cell.x, &mut cell.y);
            cell.x = max_y - cell.x;
        }

    }
    /*pub fn rotate(&self) -> [(usize,usize); 4] {
        //you live on the stack and you live on the stack
        //everyone gets to live on the stack!
        // (x',y') = (x*cos(t)-y*sin(t),x*sin(t)+y*cos(t)
        //         = (-y,x)     (t=pi/2: COUNTERCLOCKWISE)
        //The piece must rotate in place, so the y(x') values
        // must be incremented by their max (so the smallest = 0)
        //Returns the points it would occupy if rotated.
        //If these points are acceptable, then call set(points)
        //Probably a better way with references
        let mut points: [(usize,usize); 4] = [(0,0); 4]; 
        let max_y = &self.cells.iter().fold(
            std::usize::MIN, 
            |max, c| std::cmp::max(max, c.y));
        for (i,&Cell{x,y,..}) in self.cells.iter().enumerate() {
            points[i] = (self.x+max_y-y, self.y+x)
        }
        points
    }*/
    pub fn rotate4(&self) -> Piece {
        let mut piece = self.clone();
        let max_y = piece.cells.iter().fold(
            std::usize::MIN, 
            |max, c| std::cmp::max(max, c.y));
        for (i,&Cell{x,y,..}) in self.cells.iter().enumerate() {
            piece.cells[i].x = max_y-y; 
            piece.cells[i].y = x;
        }
        piece
    }
    /*
    pub fn set_coords(&mut self, points: &[(usize,usize); 4]) {
        //assumes these points are valid and changes cell coords
        for (i, c) in &mut self.cells.iter_mut().enumerate() {
            (*c).x = points[i].0 - self.x;
            (*c).y = points[i].1 - self.y;
        }
    }*/
	pub fn new(x:usize, y:usize, s:Shape, c:Color) -> Piece {
        //(x,y) is top-left corner
        //let mut cells = [Cell::blank(); 4];
        let cells = match s {
            Shape::I =>[Cell{x:0,y:0,col:c},
                        Cell{x:0,y:1,col:c},
                        Cell{x:0,y:2,col:c},
                        Cell{x:0,y:3,col:c}],
            Shape::O =>[Cell{x:0,y:0,col:c},
                        Cell{x:1,y:0,col:c},
                        Cell{x:1,y:1,col:c},
                        Cell{x:0,y:1,col:c}],
            Shape::T =>[Cell{x:0,y:0,col:c},
                        Cell{x:1,y:0,col:c},
                        Cell{x:2,y:0,col:c},
                        Cell{x:1,y:1,col:c}],
            Shape::Z =>[Cell{x:0,y:0,col:c},
                        Cell{x:1,y:0,col:c},
                        Cell{x:1,y:1,col:c},
                        Cell{x:2,y:1,col:c}],
            Shape::S =>[Cell{x:0,y:1,col:c},
                        Cell{x:1,y:1,col:c},
                        Cell{x:1,y:0,col:c},
                        Cell{x:2,y:0,col:c}],
            Shape::L =>[Cell{x:0,y:0,col:c},
                        Cell{x:0,y:1,col:c},
                        Cell{x:0,y:2,col:c},
                        Cell{x:1,y:2,col:c}],
            Shape::J =>[Cell{x:0,y:2,col:c},
                        Cell{x:1,y:2,col:c},
                        Cell{x:1,y:1,col:c},
                        Cell{x:1,y:0,col:c}],
        };
        Piece {
            x:      x,
            y:      y,
            cells:  cells,
        }
    }
}
