//block helper stuff

extern crate rand;
extern crate pancurses;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn init_color_pairs() {
        //where should this be??
        //Color as u32 = valid conversion
        use pancurses::*;
        //apparently 0 not a valid option?!
        //init_pair(1, COLOR_RED,     COLOR_RED);
        //init_pair(2, COLOR_YELLOW,  COLOR_YELLOW);//orange
        //init_pair(3, COLOR_WHITE, COLOR_WHITE);//no yllw :(
        //init_pair(4, COLOR_GREEN,   COLOR_GREEN);
        //init_pair(5, COLOR_BLUE,    COLOR_BLUE);
        //init_pair(6, COLOR_CYAN,    COLOR_CYAN);
        //init_pair(7, COLOR_MAGENTA, COLOR_MAGENTA);
        init_pair(1, COLOR_BLACK,  COLOR_RED);
        init_pair(2, COLOR_BLACK,  COLOR_YELLOW);//orange
        init_pair(3, COLOR_BLACK,   COLOR_WHITE);//no yllw :(
        init_pair(4, COLOR_BLACK,   COLOR_GREEN);
        init_pair(5, COLOR_BLACK,    COLOR_BLUE);
        init_pair(6, COLOR_BLACK,    COLOR_CYAN);
        init_pair(7, COLOR_BLACK, COLOR_MAGENTA);
    }
    pub fn to_pancurses3(&self) -> u64 {
        use self::Color::*;
        let mut cp = pancurses::COLOR_PAIR(*self as u64 + 1);
        //cp |= pancurses::A_COLOR;
        //if *self == Yellow {
        //    cp |= pancurses::A_COLOR;
        //}
        cp
    }
    pub fn to_pancurses2(&self) -> u64 {
        *self as u64 + 1
    }
    pub fn to_pancurses(&self) -> i16 {
        //define a custom color for orange/others?
        match self {
            &Color::Red     => pancurses::COLOR_RED,
            &Color::Orange  => pancurses::COLOR_WHITE,    //?
            &Color::Yellow  => pancurses::COLOR_YELLOW,
            &Color::Green   => pancurses::COLOR_GREEN,
            &Color::Blue    => pancurses::COLOR_BLUE,
            &Color::Indigo  => pancurses::COLOR_CYAN,
            &Color::Violet  => pancurses::COLOR_MAGENTA,
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
    #[allow(dead_code)]
    pub fn blank() -> Cell {
        //mostly used for debugging
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

#[derive(PartialEq)]
pub enum Direction { Down, Left, Right, Counterclockwise }
