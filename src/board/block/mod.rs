//all the block stuff

extern crate std;

pub mod cell;
use board::block::cell::*;

#[derive(Debug)]
pub struct Piece {
    pub x: usize,
    pub y: usize,
    pub cells: [Cell; 4],
}

impl Piece {
    pub fn rotate_ccw(&self) -> [(usize,usize); 4] {
        //you live on the stack and you live on the stack
        //everyone gets to live on the stack!
        // (x',y') = (x*cos(t)-y*sin(t),x*sin(t)+y*cos(t)
        //         = (-y,x)
        //The piece must rotate in place, so the y(x') values
        // must be incremented by their max (so the smallest = 0)
        //Returns the points it would occupy if rotated.
        //If these points are acceptable, then call set(points)
        //Probably a better way with references
        let mut a: [(usize,usize); 4] = [(0,0); 4]; 
        let max_y = &self.cells.iter().fold(
            std::usize::MIN, 
            |max, c| std::cmp::max(max, c.y));
        for (i,&Cell{x,y,..}) in self.cells.iter().enumerate() {
            a[i] = (self.x+max_y-y, self.y+x)
        }
        a
    }
    pub fn set_coords(&mut self, points: &[(usize,usize); 4]) {
        //assumes these points are valid and changes cell coords
        for (i, c) in &mut self.cells.iter_mut().enumerate() {
            (*c).x = points[i].0 - self.x;
            (*c).y = points[i].1 - self.y;
        }
    }
    pub fn rotate_counterclockwise(&self) {
        //45deg: will be {-1,0,1}
        //should return the coordinates it would occupy,
        // and not actually change `self`
        let cos = (std::f64::consts::PI/2.0).cos() as i32;
        let sin = (std::f64::consts::PI/2.0).sin() as i32;

        let points : Vec<(i32,i32)> = {
            let points = self.cells.iter()
                .map(|  p  | (p.x as i32, p.y as i32))
                .map(|(x,y)| (x*cos-y*sin, x*sin+y*cos));
                //.map(|(x,y)| ((x-y)*trig, (x+y)*trig));
            //let mins = points.into_iter()
            let points: Vec<(i32,i32)> = points.collect();
            let (min_x, min_y) = points.iter()
                .fold(
                    (std::i32::MAX, std::i32::MAX),
                    |(xm,ym), &(x,y)| 
                    (std::cmp::min(xm,x), std::cmp::min(ym,y)));
            points.into_iter()
                .map(|(x,y)| (x-min_x, y-min_y))
                .collect()
                //.map(|(x,y)| (x-min_x, y-min_y))
        };
        //for (i,(x,y)) in points.into_iter().enumerate() {
        //    self.cells[i].x = x as usize;
        //    self.cells[i].y = y as usize;
        //}
    }
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
