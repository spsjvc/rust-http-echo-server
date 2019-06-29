use std::io::Read;
use std::io::Write;
use std::net::{Shutdown, TcpListener};
use std::str;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8888")?;

    println!("Listening on port 8888...\n");

    loop {
        let (mut stream, _) = listener.accept()?;
        let mut buffer = [0; 1024];

        stream.read(&mut buffer)?;

        let content = match str::from_utf8(&mut buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        stream.write_all("HTTP/1.1 200 OK\n\n".as_bytes())?;
        stream.write_all(content.as_bytes())?;

        stream.shutdown(Shutdown::Both)?;
    }
}
