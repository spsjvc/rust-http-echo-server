# Rust HTTP Echo Server

A tiny server that echoes back an HTTP response with the content recieved over the TCP connection. 

Written for the sole purpose of trying out the [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html#installation) programming language.

## Usage

To start the server run:

```
cargo run
```

Try it out by sending an HTTP request using cURL:

```
curl http://localhost:8888/
```
