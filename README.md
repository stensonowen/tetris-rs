# tetris-rs
Practice implementation of Tetris in Rust

Pretty basic Tetris implementation. Uses [pancurses](https://crates.io/crates/pancurses) for I/O.

Forked from [here](https://github.com/stensonowen/less_Rusty). This is a slightly larger project, so I'm putting it in its own repository. 

Edit: I think I'm going to take a break from this project because I've run into an issue (which I'll elaborate on for my own future reference) that is both nontrivial and not enjoyable to solve. I'll probably resume eventually when I have some epiphany in the shower in a month or run into a harder/worse problem in a future project.

The issue is that of reading input and updating the board independently; the window must be responsive to user input (i.e. shifting/rotating the active block), but must also update in the absense of user input (e.g. shifting the active block down). 
The command to read input is `pancurses::Window::getch()`, which requires a handle to the current window and does nothing until a character is entered. 

The way to implement this board updating behavior seems to be with threads (similarly to [timer.rs](src/timer.rs), which uses an independent thread to keep track of an independent process, here the down-shifting of the active block).
The simplest way to implement the desired behavior seems to be spawning a thread which calls `getch()`, and then waiting in the parent thread before checking on the child and either processing its results or ignoring its lack thereof. 
I think the problem of two threads both needing a handle to the pancurses window can be solved by creating a subwindow and calling `getch()` on that, which has effectively the same behavior as far as I can tell; however, `pancurses::Window` does not implement `Send` or `Sync`, which, in my limited understanding or Rust's concurrency model, means a window cannot be referenced a thread that did not create it.
A solution to this problem that still uses pancurses eludes me; perhaps there is a better solution to this timing problem that does not require threads, but I haven't taken enough showers to have thought of it (the curse of the CS major).

One option is to ditch the pancurses crate and use the vanilla terminal; I implemented the [pretty printing](src/board/mod.rs#L28) (with colors and all) in the debugging phase of the initial data structure, and multithreaded capturing of individual characters is possible with the [termios](https://crates.io/crates/termios) crate (shoutout [stackoverflow](http://stackoverflow.com/a/37416107/6463216).  
This would mean redrawing the whole terminal one character at a time every time a frame is rendered (the framerate is only like 5 fps though). Less importantly, it would mean my I/O implementation had progressed from vanilla stdout to ncurses to pancurses to stdout again. 

This has led me to the conclusion that, though the problem is interesting, the best solution is shitty and boring. 
I don't particularly like writing shitty, boring code, so I'm going to drop the project until I change my mind about the quality of my current solution or come up with a better one (which may come with more practice with concurrency in Rust, or perhaps by taking a really long shower).

