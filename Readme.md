# Rust忘備録
## Rustとは
安全で高速かつ並行処理に強いプログラミング言語。  
- メモリ管理が非常に厳格
- OSのコア部分の開発ができる
- 複数のタスクを同時に実行する並行処理に強い  

など様々な特徴がある。

### 1.コンパイルと実行
```bash
rustc ファイル名.rs
exeファイルパス
```
- [Rustのコンパイル](https://doc.rust-jp.rs/book-ja/ch01-02-hello-world.html)

### 2.ビルドと実行
[Cargo](https://doc.rust-jp.rs/book-ja/ch01-03-hello-cargo.html)を使う。Cargoはビルドシステム兼パッケージマネージャー。
```bash
cargo --version // インストールの確認
cargo new --bin project // プロジェクトの作成
```
プロジェクトのビルドと実行。
```bash
cargo build
exeファイルパス
```
一つのコマンドでコードのコンパイルから実行まで行うことが出来る。
```bash
cargo run
```
継続的にコンパイルができるか確認する。
```bash
cargo check
```
リリースに向けたビルド。
```bash
cargo build --release
```

### 3.値の取り扱い 
[Link](https://github.com/kanto-taniguchi-git/Rust-Practice/blob/main/variables/src/main.rs)  
変数は不変である。
```rust
let x = 5;
x = 6; // エラー
```
可変性にも出来る。
```rust
let mut x = 5;
x = 6; // OK
```
しかし、変数の型は不変である。
```rust
let mut = spaces = "   ";
spaces = spaces.len(); // エラー
```
シャドーイングにより値の型を変えつつ、同じ変数名を使える。
```rust
let spaces = "   ";
let spaces = spaces.len();
```
定数は常に不変である。
```rust
const MAX_POINTS: u32 = 100_000;
println("{}", MAX_POINTS);
```
タプルの添え字は0から始まる。型も自由。 
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
println!("{}", tup.0); // 500
```
配列は全て同じ型でなければならない。**固定長であり、要素数を変更することは出来ない**。  
ベクタ型なら変更できるので基本的にはベクタ型を使う。
```rust
let a = [1, 2, 3, 4, 5];
let b: [f32; 5] = [6, 7, 8, 9, 10];
let c = [3; 5]; // 初期値3の要素が5つ
println!("{}", a[0]); // 1
```
※定数は大文字とアンダースコアで表記し、型は必ず指定する。  
※未使用変数には**アンダースコア**をつけることで警告が消える。  
[変数・定数について](https://doc.rust-jp.rs/book-ja/ch03-01-variables-and-mutability.html) 

### 4.データ型
[Link](https://github.com/kanto-taniguchi-git/Rust-Practice/blob/main/variables/src/main.rs)  
**let 変数名: データ型 = 値;**
- 整数型(上段：符号付き、下段：符号なし)
    - i8, i16, i32, i64, i128, isize
    - u8, u16, u32, u64, u128, usize  

    整数はi32が基準型。浮動小数点数はf64が基準型。

可読性を上げるために、100000を100_000のようにアンダースコアで区切ることが出来る。 他にも先頭に0xで16進数、0oで8進数、0bで2進数、b'A'でバイト(u8のみ)が使える。 

論理値型は型の指定が必要。
```rust
let t: bool = true;
let f: bool = false;
```

char型はシングルクォートで囲む。
```rust
let c = 'z';
```
[データ型について](https://doc.rust-jp.rs/book-ja/ch03-02-data-types.html)

### 関数
- 文は値を返さない。式は値を返す。
```rust
let y = 5; // 文
y + 1 //式：終端にセミコロンがない
```
変数は束縛される。
```rust
let z = {
    let zz = 3;
    zz + 1
};
println!("The value of z is: {}", z); // z = 4, zz = error
```
関数には戻り値を返す関数と返さない関数がある。
```rust
// 戻り値を返さない関数
fn print(num: i32) { // 仮引数の型の指定は必須
    println!("The value of num is: {}", num);
}

// 戻り値を返す関数
fn five() -> i32 { // 戻り値の型の指定は必須
    5
}
```
[関数について](https://doc.rust-jp.rs/book-ja/ch03-03-how-functions-work.html)

### 5.資料
- [Rustのドキュメント(日本語訳)](https://doc.rust-jp.rs/book-ja/title-page.html)

- [基礎からしっかり学ぶRust入門](https://atmarkit.itmedia.co.jp/ait/series/24844/)