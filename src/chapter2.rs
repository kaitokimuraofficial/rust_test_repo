use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

pub fn atomic() {
    f1();
}

// ストップフラグを実装すると、他のスレッドに停止するように伝えることができる
fn f1() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            some_work();
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }

    STOP.store(true, Relaxed);

    background_thread.join().unwrap();
}

fn some_work() {
    println!("This is from some work");
}
