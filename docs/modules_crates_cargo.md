# Modules Crates Cargo

モジュール、クレート、Cargoについてまとめたドキュメント。

## [`Modules`](https://doc.rust-jp.rs/rust-by-example-ja/mod.html)

大きなコードをいくつかのまとまりに分解することができます。このような構造を`Module`といいます。
`Module`の中にさらに`Module`を持つことができます。

### 可視性について(`public`と`private`)

デフォルトでは、モジュール内の要素はプライベートです。
プライベートなものは以下の2つからのみアクセスすることができます。

- プライベートなものが宣言された同じモジュール内部
- そのプライベートなモジュールのサブモジュール

また、修飾子をつけることで可視性を変更することができます。

- `pub`: どこからでもアクセス可能になります
- `pub(crate)`: 同じクレートに対してアクセス可能になります
- `pub(super)`: 親モジュールに対してアクセス可能になります
- `pub(in path::to::module)`: ある指定したモジュールに対してアクセス可能になります

下のコードを見ながら理解してください。

```rs
// `my_mod`という名称のモジュール
mod my_mod {
    // モジュール内の要素はデフォルトでプライベート
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // `pub`を用いてパブリックに変更
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // モジュール内からならば、プライベートな属性にアクセスすることに支障はない。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // モジュールもネストできる
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // `pub(in path)`形式で宣言された関数は該当のパス内でのみアクセスできる。
        // `path`は親や先祖のモジュールでなくてはならない。
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // `pub(self)`形式で宣言された関数は現在のモジュール内でのみアクセスできる。
        // つまり、プライベートにするのと同じである。
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // `pub(super)`形式で宣言された関数は親モジュール内でのみアクセスできる。
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate)により関数は現在のクレート内でのみアクセスできる。
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // ネストしたモジュールも、同様の性質を示す。
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // 親がプライベートな場合、子要素がより大きなスコープでアクセスできるように宣言されていても、
        // 子要素にアクセス可能な範囲は制限されます。
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // モジュールによって、同名の関数を区別することができる。
    function();
    my_mod::function();

    // パブリックな要素ならば、たとえネストしたものでも、
    // モジュールの外からアクセスすることができる。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    // pub(crate)の要素は同じクレートのどこからでも呼び出すことができる。
    my_mod::public_function_in_crate();

    // pub(in path)の要素は指定されたモジュールからのみ呼び出すことができる。
    // my_mod::nested::public_function_in_my_mod();
}
```

### `use`
use宣言をすることで、要素の絶対パスを新しい名前にバインドすることができ、より簡潔な記述が可能になります。
```rs
use crate::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType
};

fn main() {
    // crate::deeply::nested::my_first_functionではない！
    my_first_function();
}
```


## [`Crates`](https://doc.rust-jp.rs/rust-by-example-ja/crates.html)

クレートはRustにおけるコンパイルの単位です。`rustc some_file.rs`が呼ばれると、`some_file.rs`は必ずクレートファイルとして扱われます。
もし`some_file.rs`が`mod`宣言を含んでいるのならば、コンパイルの前に、そのモジュールファイルの中身が`mod`の位置に挿入されます。
言い換えると、それぞれのモジュールが独立にコンパイルされるということはありませんが、それぞれのクレートは互いに独立にコンパイルされるということです。

クレートを新しいライブラリにリンクするためには`rustc`の`--extern`フラグを利用します。
クレートの要素を全てライブラリと同じ名前のモジュールにインポートします。
一般に、このモジュールは他のモジュールと同じように振る舞います。例を見てみましょう。

```rs
// in rary.rs
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```
上の`rary.rs`を用いて、、、
```rs
// extern crate rary; // Rust 2015以前で必要

fn main() {
    rary::public_function();

    // エラー!`private_function`はプライベート
    //rary::private_function();

    rary::indirect_access();
}
```

```md
# library.rlibがコンパイルされたライブラリのパスで、同じディレクトリにあるものとする:
$ rustc executable.rs --extern rary=library.rlib && ./executable 
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`
```


## [`Cargo`]()

`Cargo`はRustの公式パッケージ管理ツールです。とても便利な機能が多くあり、コードの品質や開発速度の向上に役立ちます。

```txt
# バイナリ
cargo new foo

# ライブラリ
cargo new --lib bar
```

下のような構造になる
```txt
.
├── bar
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── foo
    ├── Cargo.toml
    └── src
        └── main.rs
```
