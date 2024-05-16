use std::io::prelude::*;
use wasmedge_wasi_socket::{TcpListener, TcpStream};

const HTML: &str = r#"
<!DOCTYPE html>
<html lang="ja">
<head>
  <meta charset="UTF-8">
  <title>ハロー</title>
</head>
<body>
  こんにちは、世界！
</body>
</html>
"#;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878", false).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", HTML);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
