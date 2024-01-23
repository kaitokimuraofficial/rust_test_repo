// use std::borrow::BorrowMut;
// use std::rc::Rc;
// use std::sync::Arc;
// use std::cell::Cell;
// use std::cell::RefCell;
// use std::sync::Mutex;
// use std::time::Duration;
// use std::collections::VecDeque;
// use std::thread;
// use std::sync::Condvar;

// threadには引数として分岐するスレッドに実行させたい関数を与える。
// 普通はクロージャを与えることが一般的

// Send: 安全に別のスレッドに送ることができる
// Sync: 安全に別のスレッドと共有できる

pub fn thread_use() {
    // let t1 = thread::spawn(f);
    // let t2 = thread::spawn(f);
    // println!("Hello from the main thread");
    // let v1_1:Cell<i32> = Cell::new(1);
    // let v1_2:Cell<i32> = Cell::new(2);
    // let v1_3:Cell<i32> = Cell::new(3);
    // let v2 = RefCell::new(vec![5]);

    // t1.join().unwrap();
    // t2.join().unwrap();
    // f2();
    // f3();
    // f4();
    // f5();
    // f6(&v1_1, &v1_2); // Cell { value: 1 }, Cell { value: 3 }
    // f6(&v1_1, &v1_1); // Cell { value: 2 }, Cell { value: 2 }
    // f6_2(&v1_3);
    // f7(&v2); // RefCell { value: [1] }
    // f8();
    // f9();
    // f10();
    // f11();
}

// fn f11() {
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }


// fn f10() {
//     let queue = Mutex::new(VecDeque::new());
//     let not_empty = Condvar::new();

//     thread::scope(|s| {
//         s.spawn(|| {
//             loop {
//                 let mut q = queue.lock().unwrap();
//                 let item = loop {
//                     if let Some(item) = q.pop_front() {
//                         break item;
//                     } else {
//                         q = not_empty.wait(q).unwrap();
//                     }
//                 };
//                 drop(q);
//                 dbg!(item);
//             }
//         });

//         for i in 0.. {
//             queue.lock().unwrap().push_back(i);
//             not_empty.notify_one();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
// }

// fn f9() {
//     let queue = Mutex::new(VecDeque::new());
//     thread::scope(|s| {
//         let t = s.spawn(|| loop {
//             let item = queue.lock().unwrap().pop_front();
//             if let Some(item)=item {
//                 dbg!(item);
//             }else {
//                 thread::park();
//             }
//         });
        
//         for i in 0.. {
//             queue.lock().unwrap().push_back(i);
//             t.thread().unpark();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
// }

// fn f8() {
//     let n = Mutex::new(0);
//     thread::scope(|s| {
//         for _ in 0..10 {
//             s.spawn(|| {
//                 let mut guard = n.lock().unwrap();
//                 for _ in 0..100 {
//                     *guard += 1;
//                 }
//                 drop(guard);
//                 thread::sleep(Duration::from_secs(1));
//             });
//         }
//     });
//     assert_eq!(n.into_inner().unwrap(), 1000)
// }

// // RefCellは借用を許す
// fn f7(v: &RefCell<Vec<i32>>) {
//     v.borrow_mut().push(1);
//     println!("{:?}", v);
// }

// // Cellは共有参照を通じた変更を許す
// fn f6(a: &Cell<i32>, b:&Cell<i32>) {
//     let before = a.get();
//     b.set(b.get() + 1); // 直接編集
//     let after = a.get();
//     println!("{:?}, {:?}", a, b);
//     if before != after {
//         println!("might be reached!");
//     }
// }

// // Cellの内部の値を直接借用できないので、一旦、代わりを作ってそれを代入する形
// fn f6_2(v: &Cell<i32>) {
//     // v = &Cell::new(v.take()+1); // cannot mutate immutable variable `v'
//     println!("First 6_2:  {:?}", v);
//     v.set(v.get() + 23);
//     println!("Middle 6_2:  {:?}", v);
//     let mut v2 = v.take();
//     v2 += 1;
//     println!("Last 6_2:   {:?}", v.set(v2));
//     // v.borrow_mut();
// }

// Arcはスレッド間で共有データを内部可変性として持てる
// fn f5() {
//     let a = Arc::new([1, 2, 3]);
//     let b = a.clone();

//     thread::spawn(move || dbg!(a));
//     thread::spawn(move || dbg!(b));
// }

// Rcはスレッド安全ではない
// fn f4() {
//     let a = Rc::new([1, 2, 3]);
//     let b = a.clone();

//     assert_eq!(a.as_ptr(), b.as_ptr());
// }

// fn f3() {
//     let numbers = vec![1,2,3,4,5,6];

//     thread::scope(|s| {
//         s.spawn(|| {
//             println!("length: {}", numbers.len());
//         });
//         s.spawn(|| {
//             for n in& numbers {
//                 println!("{n}");
//             }
//         });
//     });
// }

// fn f2() {
//     let numbers = vec![1,2,3,4,5,6];

//     thread::spawn(move || {
//         for n in numbers {
//             println!("{n}");
//         }
//     }).join().unwrap();
// }


// fn f() {
//     println!("Hello from another thread");

//     let id = thread::current().id();
//     println!("This is my thread id: {id:?}");
// }


