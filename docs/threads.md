# Threads

## Threadsとは
シングルプロセスは複数のスレッドを持つことができる。同じプロセスの異なるスレッドは同じデータを共有する。
スレッドは論理的な構成要素です。物理的な実行ユニットであるCPUコアは、一度に１セットの命令しか実行できないです。
スレッドの数はCPUコアよりもはるかに多いので、OSのスケジューラがどのスレッドを実行するかを決めます。


Rustのプログラムが最初に実行されるのは、`main`スレッドです。このスレッドはOSによって作られ、`main`関数を実行します。

```ts
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

## `std::thread`

Rustの標準ライブラリである`std::thread`は、スレッドを作り管理します。

Rust's standard library provides a module, std::thread, that allows you to create and manage threads.

### spawn

`std::spawn`を使えば新しいスレッドを作成し、その上でコードを実行させることができます。

```rs
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });
    
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

### join

`std::join`によって`spawn`したスレッドが終了するのを待つことができます。
`spawn`すると`JoinHandle`が返されますが、その`JoinHandle`上で`join`を実行します。

```rs
use std::thread;
fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}
```

## `'static lifetime`

Rustでは、子供のスレッドが親のスレッドよりも長生きするようなことが起こり得る。

```rs
use std::thread;

fn f() {
    thread::spawn(|| {
        thread::spawn(|| {
            loop {
                thread::sleep(std::time::Duration::from_secs(1));
                println!("Hello from the detached thread!");
            }
        });
    });
}
```

親のスレッドが終了しても子供は生き続けることができる。このような状態をRustでは`the child thread has outlived its parent.`と表現する。

`spawn`された親のスレッドよりも長く生きることや、プログラム終了まで生きている可能性がある。
そのようなスレッドはプログラムが終了するまでに`drop`してしまうかもしれない`value`を借用することはできない。
なので`std::thread::spawn`は、クロージャがスレッドに`'static lifetime`を与えることが必要である。


```rs
pub fn sum(v: Vec<i32>) -> i32 {
    let (v1, v2) = v.split_at(v.len() / 2);

    let t1 = thread::spawn(|| v1.iter().sum::<i32>());
    let t2 = thread::spawn(|| v2.iter().sum::<i32>());

    let res1 = t1.join().unwrap();
    let res2 = t2.join().unwrap();

    res1 + res2
}
```

上のコードについて考えてみたい。
v1やv2はspawnされたスレッドよりも短いlifetimeを持つかもしれない。なので

```rs
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;

    let v1 = v[..mid].to_vec();
    let v2 = v[mid..].to_vec();

    let t1 = thread::spawn(move || v1.iter().sum::<i32>());
    let t2 = thread::spawn(move || v2.iter().sum::<i32>());

    let res1 = t1.join().unwrap();
    let res2 = t2.join().unwrap();

    res1 + res2
}
```
このように`move`してそもそも所有権をthreadに移してしまうのが良い。


プログラム実行中はずっと生き続けるリファレンスを`Static data`と呼ぶ。
その最たる例はstring literalsだが、
```rs
let s: &'static str = "Hello world!";
```
このように表せる。
