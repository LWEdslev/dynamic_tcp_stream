use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

pub trait DynamicTcpStream {
    type Err;
    
    fn read_entire_message(&mut self) -> Result<Vec<u8>, Self::Err>;

    fn write_entire_message(&mut self, msg: &[u8]) -> Result<(), Self::Err>;
}


impl DynamicTcpStream for TcpStream {
    type Err = std::io::Error;
    
    fn read_entire_message(&mut self) -> io::Result<Vec<u8>> {
        let mut len_buffer = [0u8; 4];
        self.read_exact(&mut len_buffer)?;
        let msg_len = u32::from_be_bytes(len_buffer) as usize;
        let mut buffer = vec![0u8; msg_len];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    fn write_entire_message(&mut self, msg: &[u8]) -> io::Result<()> {
        let msg_len = (msg.len() as u32).to_be_bytes();
        self.write(&msg_len)?; // first 4 bytes are the message length (in bytes) this gives us ability to send 4 GB
        self.write(msg)?; // next msg_len bytes is the message
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{net::TcpListener, thread, time::Duration};

    use super::*;

    #[test]
    fn test_read_write() {
        thread::spawn(|| {
            let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
            stream.write_entire_message(b"Hello world").unwrap();
            thread::sleep(Duration::from_millis(50));
        });

        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        for stream in listener.incoming() {
            assert_eq!(
                "Hello world".to_string(),
                String::from_utf8_lossy(&stream.unwrap().read_entire_message().unwrap())
            );
            thread::sleep(Duration::from_millis(50));
            break;
        }
    }
}
