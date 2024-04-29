use std::sync::{Arc, Mutex};

fn main() {
    // ミューテックスをArcで作成してクローン
    let lock0 = Arc::new(Mutex::new(0)); // <1>
                                         // Arcのクローンは参照カウンタを増やすだけ
    let lock1 = lock0.clone(); // <2>

    let a = lock0.lock().unwrap();
    println!("{}", a);
    drop(a);
    let b = lock0.lock().unwrap(); // デッドロック <3>
    println!("{}", b);
}
