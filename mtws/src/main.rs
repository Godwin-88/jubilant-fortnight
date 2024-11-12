use std::net::TcpListener;
use std::io::Read;
use std::io::Write;
mod thread_pool;

use thread_pool::ThreadPool;

fn main() -> std::io::Result<()> {
    // Bind to a local address and port
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(4); // Initialize a thread pool with 4 threads

    println!("Server running on 127.0.0.1:7878");

    // Accept incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // Submit the stream handling to the thread pool
        pool.execute(|| {
            handle_client(stream);
        });
    }

    Ok(())
}

// Function to handle a client connection
fn handle_client(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 512];  // Buffer to store the incoming data

    // Read the request from the stream
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Simple HTTP response
            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
                            <html><body><h1>Hello, World!</h1></body></html>";

            // Send the response to the client
            if let Err(e) = stream.write(response.as_bytes()) {
                eprintln!("Failed to send response: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}

