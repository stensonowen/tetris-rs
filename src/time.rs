
mod time {
    //helper module that spawns a thread to keep track of time
    //every `DELAY` milliseconds, Timer.0 is set to true
    //and is not reset to false until it is checked on
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    const DELAY: u64 = 1000; //ms

	pub struct Timer (
        Arc<Mutex<bool>>
	); 	
	
    impl Timer {
        pub fn init() -> Timer {
            let mut t = Timer(Arc::new(Mutex::new(false)));
            let mut t_ = t.0.clone();
            thread::spawn( move ||
                           {
                               let dur = Duration::from_millis(DELAY);
                               loop {
                                   thread::sleep(dur);
                                   //TODO: handle unwrap??
                                   //it should panic though, right?
                                   //what would cause .lock to return err?
                                   let mut state = t_.lock().unwrap();
                                   *state = true;
                               }});
            t
        }
        fn is_expired(&mut self) -> bool {
            //checks if timer has been tripped
            //if so, return true and reset it
            //otherwise return false
            let t_ = self.0.clone();
            let a = t_.try_lock();
            if let Ok(mut state) = a {
                let tmp: bool = *state;
                *state = false;
                tmp
            } else {
                assert!(false); //how often will this be tripped?
                false
            }
        }
    }

}
