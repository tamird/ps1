//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 1+
//
// Note that this code has serious security risks! You should not run it
// on any system with access to sensitive files.
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;
use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    let addr = "127.0.0.1:4414";

    let listener = TcpListener::bind(addr).unwrap();

    println!("Listening on [{}] ...", addr);

    let visitor_count_arc: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    for stream in listener.incoming() {
        match stream {
            Err(_) => (),
            Ok(mut stream) => {
                let visitor_count_mutex = visitor_count_arc.clone();

                // Spawn a thread to handle the connection
                thread::spawn(move|| {
                    let visitor_count = {
                        let mut visitor_count_guard = visitor_count_mutex.lock().unwrap();
                        *visitor_count_guard += 1;
                        *visitor_count_guard
                    };

                    match stream.peer_addr() {
                        Err(_) => (),
                        Ok(pn) => println!("Received connection from: [{}]", pn),
                    }

                    let mut buf = [0 ;500];
                    stream.read(&mut buf).unwrap();
                    match str::from_utf8(&buf) {
                        Err(error) => println!("Received request error:\n{}", error),
                        Ok(body) => println!("Recieved request body:\n{}", body),
                    }

                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                         <doctype !html><html><head><title>Hello, Rust!</title>
                         <style>body {{ background-color: #111; color: #FFEEAA }}
                                  h1 {{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}}
                                  h2 {{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}}
                         </style></head>
                         <body>
                         <h1>Greetings, Krusty!</h1>
                         <h2>Visitor count: {}</h2>
                         </body></html>\r\n", visitor_count);

                    stream.write(response.as_bytes()).unwrap();
                    println!("Connection terminates.");
                });
            },
        }
    }

    drop(listener);
}
