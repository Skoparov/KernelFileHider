extern crate protobuf;
use std::net::{TcpStream, Shutdown};

pub fn send_message<M: protobuf::Message>(mut stream: &TcpStream, message: &M)
{
    match message.write_to_writer(&mut stream) {
        Ok(_) => println!("Message sent"),
        Err(e) => {
            eprintln!("Failed to send message: {}", e);
            return;
        }
    }

    // Required to make protobuf work without closing the connection
    // as it expects eof after each message
    stream.shutdown(Shutdown::Write).unwrap_or_else(|err|{
        eprintln!("Failed to send message eof: {}", err);
    });
}