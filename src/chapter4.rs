use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::thread;

pub fn do_chapter() {
    try_spin_lock();
}

// pub struct SpinLock {
//     locked: AtomicBool,
// }

// // Acquire/Releaseによって共有データへのアクセスを保護する
// impl SpinLock {
//     pub const fn new() -> Self {
//         Self {
//             locked: AtomicBool::new(false),
//         }
//     }

//     pub fn lock<'a>(&'a self) {
//         while self.locked.swap(true, Acquire) {
//             // spin_loopは何かが変わるのをスピンして待っているということを伝える
//             // プロセッサコアに向けて情報を伝える
//             std::hint::spin_loop();
//         }
//     }

//     pub fn unlock(&self) {
//         self.locked.store(false, Release);
//     }
// }

// ------------------------------------------------------------------------

// 使いやすいようにlockメソッドを変更してロックによって保護された参照を返すことにする

// pub struct SpinLock<T> {
//     locked: AtomicBool,
//     value: UnsafeCell<T>
// }

// unsafe impl<T> Sync for SpinLock<T> where T: Send {}

// impl<T> SpinLock<T> {
//     pub const fn new(value: T) -> Self {
//         Self {
//             locked: AtomicBool::new(false),
//             value: UnsafeCell::new(value)
//         }
//     }

//     pub fn lock<'a>(&'a self) -> &'a mut T {
//         while self.locked.swap(true, Acquire) {
//             std::hint::spin_loop();
//         }
//         unsafe { &mut *self.value.get() }
//     }

//     pub fn unlock(&self) {
//         self.locked.store(false, Release);
//     }
// }

// ------------------------------------------------------------------------

// ロックガードを用いた安全なインターフェイス

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}
unsafe impl<T> Send for Guard<'_, T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }
}

fn try_spin_lock() {
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| x.lock().push(1));
        s.spawn(|| {
            let mut g = x.lock();
            // drop(g);
            g.push(2);
            g.push(2);
        });
    });

    let g = x.lock();
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
}
