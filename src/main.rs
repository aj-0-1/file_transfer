use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

// Listens for incoming connections, accepts connections from clients, receives file data and
// writes it to a new file

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let mut file = File::create("received_file.txt")?;

    loop {
        // bytes_read represents the number of bytes that were actually read into the buffer
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            // connection was closed
            break;
        }
        // we only write all the bytes received from the stream
        file.write_all(&buffer[..bytes_read])?;
    }

    println!("File transfer completed");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                if let Err(e) = handle_client(stream) {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    Ok(())
}
