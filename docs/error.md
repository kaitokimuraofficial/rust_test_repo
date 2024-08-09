# Error

Rustにおけるエラーの扱い方についてまとめます。

まずRustのエラーは大きく2つに分けることができます。
| `panic` |	`Result` |
| -- | -- |
| 復帰不可能なエラー | 復帰可能なエラー |
| プログラムの実行を中断し、スタックを巻き戻すことでエラーを報告する。 | 問題が発生した際にプログラムの実行を継続することが可能であり、エラーに応じて適切な処理を行うことができる。 |

## [`Result`](https://doc.rust-lang.org/std/result/index.html)

`Result`型は、標準ライブラリの`std::result`モジュールで定義されており、成功時の戻り値と失敗時の戻り値の両方を表現することができます。
以下は`Result`の定義です。
```rs
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```

実際に例を見てみましょう。
```rs
fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        Err("Divide by 0".to_string())
    } else {
        Ok(numerator / denominator)
    }
}
```

ただこれではエラーの型だけを見て、エラーの原因が分かりにくいです。そこでRustはエラーに自作の型を入れることができます。

```rs
struct DivideByZero;

fn divide(numerator: i32, denominator: i32) -> Result<i32, DivideByZero> {
    if denominator == 0 {
        Err(DivideByZero)
    } else {
        Ok(numerator / denominator)
    }
}
```

エラーを`Enum`にして、エラーの原因によって分けることもできます。

```rs
enum U32ParseError {
    NotANumber,
    TooLarge,
    Negative,
}

// Result型を返すparse_u32を実行した時の結果をパターンマッチングする
match s.parse_u32() {
    Ok(n) => n,
    Err(U32ParseError::Negative) => 0,
    Err(U32ParseError::TooLarge) => u32::MAX,
    Err(U32ParseError::NotANumber) => {
        panic!("Not a number: {}", s);
    }
}
```

## [`Error trait`](https://rust-exercises.com/100-exercises/05_ticket_v2/09_error_trait)
Rustでは上のように自作のエラー型を使うのではなく、`std::error::Error`を使用するのが慣習である。
```rs
pub trait Error: Debug + Display {}
```

このようにエラートレイトは`Debug`と`Display`トレイトを持つことを条件に課している。

`Display`はエンドユーザー向けであり、`Debug`は開発者に向けている。
なので、`debug`は`#[derive(Debug)]`をつけるだけで自動で実装されるが、`Display`は手動で実装しなければいけない。

以下が例である
```rs
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

// 自前のエラー型の定義。エラーハンドリングのケースの応じてカスタマイズされる。
// ここで新たなエラーを書くことができ、元のエラーの実装に処理を委譲したり、
// その手前で何らかの処理を挟むことができます。
#[derive(Debug, Clone)]
struct DoubleError;

// エラーの生成は、それがどのように表示されるかとは別物です。
// そのため、エラーの表示スタイルに関する複雑なロジックを煩雑になるなどと気にする必要はありません。

// エラーに関する余分な情報を持たせていないことに注意してください。
// どの文字列がパースに失敗したかなどを出力することは、
// その情報を保持させるようにエラーの定義を修正しない限りできません。
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // 基本となるエラー、原因は記録されていない。
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // エラーを新たな型に変更する。
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // ここでも新たなエラー型に更新する。
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```

