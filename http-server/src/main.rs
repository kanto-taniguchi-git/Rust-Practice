// std:net::TcpListener を使用してTCP接続を受け入れるサンプルコード。

// main関数でエラーが発生したらErr、正常の場合はOKを返すResult型
use std::io::Result;
// Tcp接続を受け入れる為の構造体。0.0.0.0:8080でリッスンする。
use std::net::TcpListener;
// 接続ごとに新しいスレッドを作成する。
use std::thread;
// subモジュールをインポートする。
mod sub;

fn main() -> Result<()> {
    // 外部からのアクセスを許可するための設定。0.0.0.0:8080でリッスンする。
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    // OK((stream, addr))が返却されるが「_」でaddrを無視している。
    while let Ok((stream, _)) = listener.accept() {
        // 接続ごとに新しいスレッドで処理を並列化する。
        thread::spawn(|| sub::tcp_handler(stream));
    }
    // 正常終了
    Ok(())
}


