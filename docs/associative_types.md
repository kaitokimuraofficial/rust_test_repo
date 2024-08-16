# Associative Types

## [`関連型`](https://doc.rust-jp.rs/book-ja/ch19-03-advanced-traits.html)
関連型は、トレイトのメソッド定義がシグニチャでプレースホルダーの型を使用できるように、トレイトと型のプレースホルダーを結び付けます。
トレイトを実装するものがこの特定の実装で型の位置に使用される具体的な型を指定します。
そうすることで、 なんらかの型を使用するトレイトをトレイトを実装するまでその型が一体なんであるかを知る必要なく定義できます。

関連型があるトレイトの一例は、標準ライブラリが提供する`Iterator`トレイトです。その関連型は`Item`と名付けられ、 `Iterator`トレイトを実装している型が走査している値の型の代役を務めます。以下がコード例です。

```rs
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

`Iterator`トレイトを実装するときは`Item`の具体的な型を指定し、`next`メソッドは、 その具体的な型の値を含むOptionを返すようにします。


## ジェネリクスとの違い

ジェネリクスとの違いを確認します。

```rs
// 関連型
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
    }
}
```

```rs
// ジェネリクス
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

**ジェネリクスを使用すると、各実装で型を注釈しなければいけません。**
`Iterator<String> for Counter`や他のどんな型にも実装することができるので、 CounterのIteratorの実装が複数できるでしょう。
つまり、トレイトにジェネリックな引数があると、 毎回ジェネリックな型引数の具体的な型を変更してある型に対して複数回実装できるということです。
`Counter`に対して`next`メソッドを使用する際に、どの`Iterator`の実装を使用したいか型注釈をつけなければならないでしょう。

関連型なら、同じ型に対してトレイトを複数回実装できないので、型を注釈する必要はありません。 
関連型を使用する定義があるリスト19-20では、Itemの型は1回しか選択できませんでした。 
1つしか`impl Iterator for Counter`がないからです。
`Counter`に`next`を呼び出す度に、 `u32`のイテレータが欲しいと指定しなくても良いのです。
