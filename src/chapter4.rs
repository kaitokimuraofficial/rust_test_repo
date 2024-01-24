use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};

pub fn do_chapter() {}

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

// // Acquire/Releaseによって共有データへのアクセスを保護する
// impl<T> SpinLock<T> {
//     pub const fn new(value: T) -> Self {
//         Self {
//             locked: AtomicBool::new(false),
//             value: UnsafeCell::new(value)
//         }
//     }

//     pub fn lock<'a>(&'a self) -> &mut T {
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

// pub struct SpinLock<T> {
//     locked: AtomicBool,
//     value: UnsafeCell<T>,
// }

// pub struct Guard<'a, T> {
//     lock: &'a SpinLock<T>,
// }

// unsafe impl<T> Sync for SpinLock<T> where T: Send {}

// impl<T> Deref for Guard<'_, T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         unsafe { &*self.lock.value.get() }
//     }
// }

// impl<T> DerefMut for Guard<'_, T> {
//     fn deref_mut(&mut self) -> &mut T {
//         unsafe { &mut *self.lock.value.get()}
//     }
// }

// impl<T> SpinLock<T> {
//     pub const fn new(value: T) -> Self {
//         Self {
//             locked: AtomicBool::new(false),
//             value: UnsafeCell::new(value),
//         }
//     }

//     pub fn lock<'a>(&self) -> Guard<T> {
//         while self.locked.swap(true, Acquire) {
//             std::hint::spin_loop();
//         }
//         Guard { lock: self }
//     }

//     pub fn unlock(&self) {
//         self.locked.store(false, Release);
//     }
// }
