extern crate pancurses;
use pancurses::*;
mod board;

fn main() {
    let window = initscr();
    start_color();
    use_default_colors();
    window.nodelay(true);
    noecho();

    let (height, width) = window.get_max_yx();

    let a = window.getch();

    //pancurses::ToChtype
    /*
    while window.getch().is_none() {
        //window.erase();
        window.mv(1,1);
        napms(200);
    }*/
    for i in 0..std::cmp::min(height, width) {
        napms(200);
        //window.mv(i,i);
        init_pair(1, COLOR_WHITE, COLOR_BLUE);
        //window.bkgd(COLOR_PAIR(1));
        //window.erase();
        init_pair(2, COLOR_RED, COLOR_RED);
        window.attrset(COLOR_PAIR(2));
        //init_pair(2, COLOR_RED, COLOR_RED);
        //window.attrset(COLOR_PAIR(2));
        window.mvaddch(i,i,'X');
        window.printw("~");
        //window.mvaddstr(2, 2, "/");
        window.refresh();
        //window.erase();

    }
    endwin();
}
