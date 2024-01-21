use std::thread;

// threadには引数として分岐するスレッドに実行させたい関数を与える。
// 普通はクロージャを与えることが一般的


pub fn thread_use() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello from the main thread");

    t1.join().unwrap();
    t2.join().unwrap();
    f2();
}

fn f() {
    println!("Hello from another thread");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}

fn f2() {
    let numbers = vec![1,2,3,4,5,6];

    thread::spawn( || {
        for n in numbers {
            println!("{n}");
        }
    }).join().unwrap();
}