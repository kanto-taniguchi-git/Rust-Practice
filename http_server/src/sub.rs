use std::{
    // リクエストされたファイルが存在するか確認。
    path::Path,
    // 指定したファイルの内容を文字列として読み込む。
    fs::read_to_string,
    // Readは読み込み、Writeは書き込み。
    io::{Result, Read, Write},
    net::TcpStream,
};
// 正規表現を使う為のライブラリ。HTTPリクエストを解析する。
use regex::Regex;

// TcpStreamを受け取り、リクエストを処理してレスポンスを返す。
pub fn tcp_handler(mut stream: TcpStream) -> Result<()> {
    // 1024バイトの固定サイズのバッファを用意。
    let mut buffer = [0; 1024];
    // クライアントから送られてきたデータを読み込む。
    stream.read(&mut buffer)?;
    // GETリクエストのpathを抽出。
    let re = Regex::new(r"^GET /([a-zA-Z\d\.\-/_]+) HTTP/1\.1").unwrap();
    // 正規表現にマッチする部分を抽出し、capsに格納。
    let response = match re.captures(&String::from_utf8(buffer.to_vec()).unwrap()) {
        Some(caps) => {
            // 抽出したURLのパス部分(/から始まるファイルパス)をPathに変換し、ファイルが存在するかどうか確認する。
            let path = Path::new(&caps[1]);
            // 指定されたパスがファイルであるかどうかを確認。
            if path.is_file() {
                let status_line = "HTTP/1.1 200 OK";
                // ファイルの内容を読み込み、文字列として保持する。
                let html = read_to_string(path)?;
                let http_header = format!(
                    "Content-Length: {}\r\nContent-Type: text/html;charset=UTF-8",
                    html.len()
                );
                format!("{}\r\n{}\r\n\r\n{}", status_line, http_header, html)
            } else {
                // ファイルが存在しない場合、404を返す。
                "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
            }
        },
        None => {
            // リクエストが不正の場合。
            "HTTP/1.1 400 Bad Request\r\n\r\n".to_string()
        },
    };
    // レスポンスをバイト列に変換し、クライアントに送信。
    stream.write_all(response.as_bytes())?;
    // ストリームに書き込んだデータを即時に送信する為にバッファをフラッシュ。
    stream.flush()
}
