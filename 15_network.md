# Network

## TCP Server
```text
use std::io::{Read, Result, Write};

fn main() -> Result<()> {
    let listener = std::net::TcpListener::bind("0.0.0.0:12345")?;

    // This single threaded server can handle only one incoming connection at a
    // time.
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0u8; 4096];
        let count = stream.read(&mut buffer)?;
        stream.write_all(&buffer[0..count])?;
    }
    Ok(())
}
```
## TCP Client
```text
use std::io::{Read, Result, Write};

fn main() -> Result<()> {
    let mut stream = std::net::TcpStream::connect("127.0.0.1:12345")?;

    stream.write_all(&[0, 1, 2, 3])?;

    let mut buffer = [0u8; 4];
    stream.read_exact(&mut buffer)?;
    println!("Received {buffer:?}");

    Ok(())
}
```