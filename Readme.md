# Rust忘備録
## Rustとは
安全で高速かつ並行処理に強いプログラミング言語。  
- メモリ管理が非常に厳格
- OSのコア部分の開発ができる
- 複数のタスクを同時に実行する並行処理に強い  

など様々な特徴がある。

### コンパイル
```bash
rustc ファイル名.rs
exeファイルパス
```

- [Rustのコンパイル](https://doc.rust-jp.rs/book-ja/ch01-02-hello-world.html)


### ビルド
[Cargo](https://doc.rust-jp.rs/book-ja/ch01-03-hello-cargo.html)を使う。Cargoはビルドシステム兼パッケージマネージャー。
```bash
cargo --version
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
### 資料
- [Rustのドキュメント(日本語訳)](https://doc.rust-jp.rs/book-ja/title-page.html)

- [基礎からしっかり学ぶRust入門](https://atmarkit.itmedia.co.jp/ait/series/24844/)