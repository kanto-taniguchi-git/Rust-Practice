# Rust忘備録
## Rustとは
安全で高速かつ並行処理に強いプログラミング言語。  
- メモリ管理が非常に厳格
- OSのコア部分の開発ができる
- 複数のタスクを同時に実行する並行処理に強い  

など様々な特徴がある。

### コンパイルと実行
```bash
rustc ファイル名.rs
exeファイルパス
```

- [Rustのコンパイル](https://doc.rust-jp.rs/book-ja/ch01-02-hello-world.html)


### ビルドと実行
[Cargo](https://doc.rust-jp.rs/book-ja/ch01-03-hello-cargo.html)を使う。Cargoはビルドシステム兼パッケージマネージャー。
```bash
cargo --version
cargo new --bin プロジェクト名
```
バージョンナンバーが表示されていたらインストールされている。
```bash
cargo build
exeファイルパス
```
一つのコマンドでビルドから実行まで行うことが出来る。
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

### 値の取り扱い
[変数・定数について](https://doc.rust-jp.rs/book-ja/ch03-01-variables-and-mutability.html)  
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
※定数は大文字とアンダースコアで表記し、型は必ず注釈する。  

### 資料
- [Rustのドキュメント(日本語訳)](https://doc.rust-jp.rs/book-ja/title-page.html)

- [基礎からしっかり学ぶRust入門](https://atmarkit.itmedia.co.jp/ait/series/24844/)