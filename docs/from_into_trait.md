# TRAIT

## [`From`と`Into`](https://doc.rust-jp.rs/rust-by-example-ja/conversion/from_into.html)

`From`トレイトと`Into`トレイトは本質的に結びついており、型変換を行うことができることを表すトレイトです。

AからBに型変換ができることやその逆方向であるBからAに型変換が可能であることを表すのがこれらのトレイトです。標準ライブラリでは基本データ型やよく使われる型に対して、このトレイトが多数実装されています。


### `From`
ある型に対し、別の型からその型を作る方法を定義できるようにするものです。
```rs
// strからStringへの型変換
let my_str = "hello";
let my_string = String::from(my_str);
```

自作の型に対しても、型変換を定義すれば同じように行えます。
```rs
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}
```

上のコードでは、、`i32`を`Number`に`From`によって変換できることが実装されているので、`i32`の`30`を`Number`型の`{ value: 30 }`に変換しています。


### `Into`
単にFromトレイトの逆の働きをします。もし自作の型にFromトレイトが実装されていたら、Intoは必要に応じてそれを呼び出します。

Intoトレイトを使用すると、ほとんどの場合、コンパイラが型を決定することができないため、変換する型を指定する必要があります。しかし、この機能を無料で得られることを考えれば、これは小さなトレードオフです。


```rs
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // この型付けを消すとコンパイルエラー
    let num: Number = int.into();
    println!("My number is {:?}", num); 
}
```

もしくは、
```rs
struct A {}
struct B {}

// BからAに変換する
impl From<B> for A {
    fn from(from: B) -> A {
        A {}
    }
}

fn main() {
    // fromを用いてBからAを生成
    let b = B {};
    let a = A::from(b);

    // intoを用いてBからAを生成
    let b = B {};
    let a: A = b.into();
}
```

上のコードでは、`i32`を`Number`に`From`によって変換できるという実装に対して、

上のコードでは、、`i32`を`Number`に`From`によって変換できることが実装されているので、`i32`の`30`を`Number`型の`{ value: 30 }`に変換しています。


### `From`と`Into`の使う場面
`From`トレイトを実装すると`Into`トレイトは自動的に実装されます。しかし、`Into`トレイトを実装したからといって、`From`トレイトが実装されることはありません。

何かを別の型を持つ異なるものに変換したい時などに`From`トレイトを実装すればいいでしょう。

では、`Into`トレイトはいつ使えばいいのでしょうか。

```md
Only implement Into when targeting a version prior to Rust 1.41 and converting to a type outside the current crate. From was not able to do these types of conversions in earlier versions because of Rust’s orphaning rules. See Into for more details.
[https://doc.rust-lang.org/std/convert/trait.From.html]
```



## [`TryFrom`と`TryInto`](https://doc.rust-jp.rs/rust-by-example-ja/conversion/try_from_try_into.html)

`From`および`Into`と同様に、`TryFrom`および`TryInto`も型変換を行うジェネリックなトレイトです。`From/Into`と異なり、`TryFrom/TryInto`トレイトは失敗する可能性のある型変換に用いられるので、`Result`を返します。


```rs
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
```
