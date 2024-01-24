// use core::num;
// use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
// use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
// use std::thread;
// use std::time::Duration;

pub fn atomic() {
    // f1();
    // f2();
    // f3();
    // f4();
    let p = allocate_new_id();
    println!("{p}");
}

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    assert!(id < 1000, "too many IDs");
    id
}

// fn f4() {
//     let num_done = &AtomicUsize::new(0);

//     thread::scope(|s| {
//         for _ in 0..4 {
//             s.spawn(move || {
//                 for i in 0..25 {
//                     process_item(i);
//                     num_done.fetch_add(1, Relaxed);
//                 }
//             });
//         }

//         loop {
//             let n = num_done.load(Relaxed);
//             if n == 100 {
//                 break;
//             }
//             println!("Working.. {n}/100 done");
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     println!("Done");
// }

// fn f3() {
//     let num_done = AtomicUsize::new(0);
//     let main_thread = thread::current();

//     thread::scope(|s| {
//         s.spawn(|| {
//             for i in 0..100 {
//                 process_item(i);
//                 num_done.store(i + 1, Relaxed);
//                 main_thread.unpark();
//             }
//         });

//         loop {
//             let n = num_done.load(Relaxed);
//             if n == 100 {
//                 break;
//             }
//             println!("Working.. {n}/100 done");
//             thread::park_timeout(Duration::from_secs(1));
//         }
//     });

//     println!("Done");
// }

// fn f2() {
//     let num_done = AtomicUsize::new(0);

//     thread::scope(|s| {
//         s.spawn(|| {
//             for i in 0..100  {
//                 process_item(&i);
//                 num_done.store(i+1, Relaxed);
//             }
//         });

//         loop {
//             let n = num_done.load(Relaxed);
//             if n==100 {break;}
//             println!("Working.. {n}/100 done");
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     println!("Done");
// }

// fn process_item(_i: usize) {
//     println!("hello");
// }

// ストップフラグを実装すると、他のスレッドに停止するように伝えることができる
// fn f1() {
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
//     println!("This is from some work");
// }
