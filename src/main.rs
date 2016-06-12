extern crate pancurses;
use pancurses::*;
mod board;
use board::board::*;
extern crate rand;

//extern crate timer;
mod timer;
use timer::timer::*;

fn main() {
    let window = initscr();
    start_color();
    use_default_colors();
    window.nodelay(true);
    noecho();

    let (height, width) = window.get_max_yx();

    //let mut b = Board::random();
    let mut b = Board::new();
    b.new_piece();
    //b.print(&window);

    //let timer = time::init();
    //let timer = time::Timer::new();

    //window.erase();

    //let w = Board::WIDTH;
    let (w,h) = (WIDTH as i32, HEIGHT as i32);
    assert!(window.get_max_x() > w);
    assert!(window.get_max_y() > h);
//    /*
    let subwin = window.subwin(
        h,
        w,
        (window.get_max_y() - h)/2,
        (window.get_max_x() - w)/2);
    window.erase();
    //window.printw("hello world?");
    //window.refresh();
    //b.print(&subwin.unwrap());
    if let Err(e) = subwin {
        println!("Subwindow Error: {}", e);
        endwin();
        return;
    }
    let subwin = subwin.unwrap();
    b.print(&subwin);

    //use timer::*;
    //let timer = time::Timer::init();
    //let d = time::DELAY;
    //let d = timer::DELAY;
    let mut timer = Timer::init();
    //let t = timer::Timer::init();

    //TODO: cancel `getch()` call if no result returned within X milliseconds
    //Can spawn a thread to call getch, then in the main thread wait, 
    // then check on the data of the child thread; if none, board.update().
    //Will this screw with ownership on the window/subwin? 
    // Will I have to wrap it in an ARC?

    loop {
        //main program loop
        match subwin.getch().unwrap() {
            pancurses::Input::Character('q') | Input::KeyEnd | Input::KeyBackspace => break,
            c   => b.command(c),
        };

        if timer.is_expired() {
            b.update();
        }

        subwin.erase();
        subwin.refresh();

        b.print(&subwin);
        napms(50);
    }
//    */



    //endwin();
    //return;



     /*
    for i in 0..std::cmp::min(height, width){
        napms(200);
        //window.mv(i,i);
        //init_pair(1, COLOR_WHITE, COLOR_BLUE);
        //window.bkgd(COLOR_PAIR(1));
        //window.erase();
        init_pair(2, COLOR_RED, COLOR_RED);
        init_pair(2, COLOR_WHITE, 2);

        window.attrset(COLOR_PAIR(2));
        //init_pair(2, COLOR_RED, COLOR_RED);
        //window.attrset(COLOR_PAIR(2));
        window.mvaddch(i,i,'X');
        window.printw("X");
        //window.mvaddstr(2, 2, "/");
        window.refresh();
        //window.erase();

    }
    */
    endwin();
}
