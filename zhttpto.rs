//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};
use std::io::buffered::BufferedReader;

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut VISITS: int = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));

    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        unsafe {
            VISITS += 1;
        }
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
            let mut index = 0;
            for tempstr in request_str.split('\n') {
                if(index == 0) {
                    let first = tempstr.slice_to(3).to_owned();
                    let last = tempstr.slice(tempstr.len() - 9, tempstr.len()-1).to_owned();
                    if( first == ~"GET" &&  last == ~"HTTP/1.1") {
                        let path = tempstr.slice(5, tempstr.len() - 10).to_owned();
                        if(path.len() > 3) {
                            let temp = path.clone();
                            let ext = temp.slice_from(path.len() - 5);
                            if(ext != ".html") {
                                unsafe {
                                    let response = format!(
                                        "HTTP/1.1 403 Forbidden\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                                         <doctype !html><html><head><title>Hello, Rust!</title>
                                         </head>
                                         <body>
                                         <h1>Go away! You don't have access to that file.</h1>
                                         <p> {:d} requests made </p>
                                         </body></html>\r\n", VISITS);
                                    stream.write(response.as_bytes());
                                    println!("Connection terminates.");
                                }
                            } else {
                                let path2 = Path::new(path);
                                if(path2.exists() && path2.is_file()) {
                                    let mut response = ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";
                                    let mut file = BufferedReader::new(File::open(&path2));
                                    let lines: ~[~str] = file.lines().collect();
                                    for s in lines.iter() {
                                        response = response + *s;
                                    }
                                    stream.write(response.as_bytes());
                                    println!("Connection terminates.");
                                } else {
                                    unsafe {
                                        let response = format!(
                                            "HTTP/1.1 404 Not Found\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                                             <doctype !html><html><head><title>Hello, Rust!</title>
                                             </head>
                                             <body>
                                             <h1>Uhoh! Looks like that file wasn't found.</h1>
                                             <p> {:d} requests made </p>
                                             </body></html>\r\n", VISITS);
                                        stream.write(response.as_bytes());
                                        println!("Connection terminates.");
                                    }
                                }
                            }
                        } else {
                            unsafe {
                                let response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                                     <doctype !html><html><head><title>Hello, Rust!</title>
                                     <style>body \\{ background-color: \\#111; color: \\#FFEEAA \\}
                                            h1 \\{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red \\}
                                            h2 \\{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green \\}
                                     </style></head>
                                     <body>
                                     <h1>Greetings, Krusty!</h1>
                                     <p> {:d} requests made </p>
                                     </body></html>\r\n", VISITS);
                                stream.write(response.as_bytes());
                                println!("Connection terminates.");
                            }
                        }
                    }
                }
                index += 1;
            }
        }
    }
}
