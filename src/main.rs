use std::io::{self, Read, Write};
use std::net::{TcpStream, TcpListener};
use std::thread;

fn main() -> io::Result<()> {
    // ta in cli argument
    let args: Vec<String> = std::env::args().collect();

    // printa felmeddelanden om det saknas argument
    if args.len() != 4 {
        println!("Not enough arguments: {}", args[0]);
        return Ok(());
    }

    let mode = &args[1];
    let ip_addr = &args[2];
    let port = &args[3];
    // om första argumentet är listen sätt up lystnare, om connect försök ansluta till server
    match mode.as_str() {
        "listen" => {
            let listener = TcpListener::bind(format!("{}:{}", ip_addr, port))?;
            println!("Listening on {}:{}", ip_addr, port);

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => { 
                      thread::spawn(move || handle_client(stream));  
                    }
                    Err(e) => {eprintln!("Connection failed: {}", e)}
                }
            }
        }

        "connect" => {
            match TcpStream::connect(format!("{}:{}", ip_addr, port)) {
                Ok(mut stream) => {
                    println!("Connected to {}:{}", ip_addr, port);
                    handle_connection(&mut stream)?;
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e)
                }
            }
        }

        &_ => {eprintln!("Not a valid argument")}
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    if let Ok(addr) = stream.peer_addr() {
        println!("New client connected: {}", addr);
    } else {
        println!("New client connected");
        }
        handle_connection(&mut stream).unwrap_or_else(|err| {
            eprintln!("Error handling connection: {}", err);
        });
    println!("Client disconnected");
}

fn handle_connection(stream: &mut TcpStream) -> io::Result<()> {
    let mut client_reader = stream.try_clone()?;
    
    thread::spawn(move || {
        
        let mut buffer = [0; 1024];
        loop {
            match client_reader.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        // om buffern är tom
                        // så dödar vi uppkopplingen
                        break;
                    }
                    // om det finns data i buffern så printar den det
                    if let Ok(data) = std::str::from_utf8(&buffer[..n]) {
                        print!("{}", data);
                    }
                }
                Err(e) => {
                    eprintln!("error reading from client: {}", e);
                    break;
                }
            }
        }
    });
    
    let mut input_buffer = String::new();
    loop {
        match io::stdin().read_line(&mut input_buffer) {
            Ok(0) => {
                // om buffern är tom så break
                break;
            }
            Ok(_) => {
                if let Err(e) = stream.write(input_buffer.as_bytes()) {
                    eprintln!("error writing to client: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("error reading from stdin: {}", e);
                break;
            }
        }
        input_buffer.clear();
    }
    Ok(())
}
