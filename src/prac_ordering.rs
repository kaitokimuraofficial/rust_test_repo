use std::thread;
// use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

pub fn use_ordering() {
    // prac1()
    // prac2();
    // main2();
    lock();
}

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn lock() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });

    println!("{:?}", unsafe { &DATA });
}

fn f() {
    if LOCKED
        .compare_exchange(false, true, Acquire, Relaxed)
        .is_ok()
    {
        unsafe { DATA.push('!') };
        LOCKED.store(false, Release);
    }
}

// static X: AtomicI32 = AtomicI32::new(0);
// static Y: AtomicI32 = AtomicI32::new(0);

// fn main2() {
//     thread::scope(|s| {
//         s.spawn(|| a());
//         s.spawn(|| b());
//     });
// }

// fn a() {
//     X.store(10, Relaxed);
//     Y.store(20, Relaxed);
// }

// fn b() {
//     let y = Y.load(Relaxed);
//     let x = X.load(Relaxed);

//     println!("{x} {y}");
// }

// fn prac1() {
//     static STOP: AtomicBool = AtomicBool::new(false);

//     let background_thread = thread::spawn(|| {
//         while !STOP.load(Relaxed) {
//             some_work();
//         }
//     });

//     for line in std::io::stdin().lines() {
//         match line.unwrap().as_str() {
//             "help" => println!("commands: help, stop"),
//             "stop" => break,
//             cmd => println!("unknown command: {cmd:?}"),
//         }
//     }

//     STOP.store(true, Relaxed);

//     background_thread.join().unwrap();
// }

// fn some_work() {
//     println!("Hello");
// }
