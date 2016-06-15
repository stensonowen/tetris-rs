
extern crate termios;
use std::io;
use std::io::Read;
use std::io::Write;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


fn main() {
    let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work 
                   // on /dev/stdin or /dev/tty
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();  // make a mutable copy of termios 
                                            // that we will modify
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];  // read exactly one byte

    println!("Hit a key! ");
    
    let buff = Arc::new(Mutex::new([0;1]));
    loop {
        let buf_ = buff.clone();
        thread::spawn( move || {
            let stdout = io::stdout();
            let mut stdin  = io::stdin();
            stdout.lock().flush().unwrap();
            stdin.read_exact(&mut buffer).unwrap();
            let mut tmp = buf_.lock().unwrap();
            *tmp = buffer;
        });
        thread::sleep(Duration::from_secs(1));
        println!("{:?}", buff);
    }

        //let buf2 = buff.clone();
    //println!("{:?}", buf2);

	//loop {
	//	stdout.lock().flush().unwrap();
	//	reader.read_exact(&mut buffer).unwrap();
	//	//println!("You have hit: {:?}", buffer);
    //    //buffer: [u8; 1]
	//	println!("{:?}", buffer);
	//}

    tcsetattr(stdin, TCSANOW, & termios).unwrap();  // reset the stdin to 
                                                    // original termios data
}
