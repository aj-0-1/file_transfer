use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::path::Path;

// Initiates connection to server, reads data from local file, sends file data to the server

fn send_file(mut stream: TcpStream, file_path: &str) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            // reached end of file
            break;
        }
        stream.write_all(&buffer[..bytes_read])?;
    }

    println!("File transfer completed");
    Ok(())
}

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to server");

    let file_path = "file_to_send.txt";
    if Path::new(file_path).exists() {
        send_file(stream, file_path)?;
    } else {
        eprintln!("File '{}' not found", file_path);
    }

    Ok(())
}
