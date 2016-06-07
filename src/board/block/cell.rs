//block helper stuff

extern crate rand;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Color { Red, Orange, Yellow, Green, Blue, Indigo, Violet, }

impl Color {
    pub fn rand() -> Color {
        let len = 7;    //ROYGBIV
        //use Color::*;
        match rand::random::<u8>() % len {
            0 => Color::Red,
            1 => Color::Orange,
            2 => Color::Yellow,
            3 => Color::Green,
            4 => Color::Blue,
            5 => Color::Indigo,
            _ => Color::Violet,
        }
    }
}


pub enum Shape { I, O, T, Z, S, L, J, }
impl Shape {
    pub fn rand() -> Shape {
        let len = 7;
        match rand::random::<u8>() % len {
            0 => Shape::I,
            1 => Shape::O,
            2 => Shape::T,
            3 => Shape::Z,
            4 => Shape::S,
            5 => Shape::L,
            _ => Shape::J,
        }
    }
}


#[derive(Debug, Clone, Copy)]
//Either absolute coordinates if it's part of the board
//or relative coordinates if it's in a Piece
pub struct Cell {
    pub x:  usize,
    pub y:  usize,
    pub col:Color,
}

impl Cell {
    pub fn blank() -> Cell {
        Cell {
            x:  0,
            y:  0,
            col:Color::rand(),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = match self.col {
            Color::Red      => "\x1B[31m",
            Color::Orange   => "\x1B[33m",
            Color::Yellow   => "\x1B[93m",  //Bright Orange
            Color::Green    => "\x1B[32m",
            Color::Blue     => "\x1B[34m",
            Color::Indigo   => "\x1B[36m",  //Cyan
            Color::Violet   => "\x1B[35m",  //Magenta
        }.to_string();
        //s.push_str("■");
        s.push_str("⠀");
        s.push_str("\x1B[0m");
        write!(f, "{}", s)
    }
}
