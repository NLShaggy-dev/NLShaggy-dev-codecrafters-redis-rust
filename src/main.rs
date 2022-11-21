// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::str;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
           Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
               println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        let mut buf = [0,512];
        match stream.read(&mut buf) {
            Ok(_) => {
                match process_input(&buf) {
                    Ok(value) => {
                        if let Err(e) = stream.write(value.as_bytes()) {
                            println!("Failed to write response: {}", e);
                        }
                    },
                    Err(e) => {
                        println!("Error handling command: {}", e);
                    }
                }
            },
            _ => break
        }
    }
    Ok(())
}

fn process_input (value: &[u8]) -> std::io::Result<String> {
    let message = str::from_utf8(value).expect("Failed to parse command");
    let args = message.split("\r\n").collect::<Vec<&str>>();

    match args[0] {
        "*0" => return Ok(String::from("")),
        "*1" => return Ok(String::from("+PONG\r\n")),
        _ => {
            let response = format!("$\r\n{}\r\n", args[4]);
            println!("{}", response);
            return Ok(response.to_owned())
        }
    }
}
