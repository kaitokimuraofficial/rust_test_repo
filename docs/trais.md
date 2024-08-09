# Traits

トレイトとは任意の型となりうる`Self`に対して定義されたメソッドの集合のことです。同じトレイト内で宣言されたメソッド同士はお互いにアクセスすることができます。
トレイトはあらゆるデータ型に実装することができます。

## トレイトの基本

トレイトを定義するときは以下のようにします。
```rs
trait <TraitName> {
    fn <method_name>(<parameters>) -> <return_type>;
}
```

トレイトをある方について実装するときは以下のようにします。
```rs
impl <TraitName> for <TypeName> {
    fn <method_name>(<parameters>) -> <return_type> {
    }
}
```


例を見てみましょう。以下の例ではまず`Animal`というメソッドの集合を定義し、その後`Animal`トレイトを`Sheep`というデータ型に対して実装します。
これによりAnimalのメソッドをSheepが使用することが可能になります。
```rs
struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // 関連関数のシグネチャ。
    // `Self` はこのトレイトを実装している型になる。
    fn new(name: &'static str) -> Self;

    // メソッドのシグネチャ。
    // これらの関数は文字列を返す。
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // メソッドのデフォルトの挙動を定義することもできる。
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // メソッドをある型に実装する際に、その型のトレイトメソッドを使用することができる。
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// `Animal`というトレイトを`Sheep`に実装する。
impl Animal for Sheep {
    // `Self`は実装対象の型: ここでは`Sheep`
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    
    // デフォルトのトレイトメソッドはオーバーライドすることができる。
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // この場合、型アノテーションが必須。
    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
}
```


## [`孤児ルール(Orphan rule)`](https://rust-exercises.com/100-exercises/04_traits/02_orphan_rule)

他のクレートで型が定義されているとき、新しいメソッドをそのまま定義することはできません。
`u32`は標準ライブラリによって定義されているので、以下のようなことはできません。
```rs
impl u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
```

トレイトに関して以下の2つのうち少なくともどちらかは成り立っていなければいけません。
- トレイトは現在のクレイトで定義されている
- トレイトを実装する型は現在のクレイトで定義されている

このルールを`Orphan rule`と呼ぶ。

以下のような状況を考えてみよう。

1. クレイト`A`が`IsEven`トレイトを定義している
2. クレイト`B`が`u32`に対して`IsEven`トレイトを実装している
3. クレイト`C`が異なる`IsEven`トレイトを`u32`に対して実装している
4. クレイト`D`は`B`と`C`に依存している。今`1.is_even()`を呼び出した。

`A`と`C`のどちらの実装を呼び出せばいいのか？

