use std::{
    sync::{Arc, RwLock},
    thread,
};

fn main() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        let _a = val.read().unwrap();
        *val.write().unwrap() = false;
        println!("deadlock");
    });

    t.join().unwrap();
}
