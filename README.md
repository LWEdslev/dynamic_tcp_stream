# DynamicTcpStream
Trait that enables TcpStream to read and write any length byte arrays easily.

This is acheived through prefixing the message with 4 bytes describing the message length.

## Example:
```rust
// simply import the `DynamicTcpStream` trait
use dynamic_tcp_stream::DynamicTcpStream;

// for a `std::net::TcpStream`
let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

// you can now send any message 
stream.write_entire_message(b"Hello world").unwrap();

// and the exact message will be received regarding length
let received = stream.read_entire_message().unwrap();
```