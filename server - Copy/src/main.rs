use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{IpAddr, Ipv4Addr,SocketAddr,  TcpListener, TcpStream},
    process,
};

use rand::Rng;

//static mut counter: i32 = 0;

const LAN: bool = false;
const PRINTING: bool = true;

const LOCALHOST_IP_V4: &str = "127.0.0.1";
const SERVER_IP_V4: &str = "192.168.1.178";

const RUST_PORT: u16 = 7878;

fn main() {
    println!("\nInitializing server...");

    let listener = TcpListener::from(if LAN {
        init_server_on_lan().unwrap_or_else(|err| {
            println!("Problem initializing server: {err}");
            process::exit(1);
        })
    } else {
        init_server_on_localhost().unwrap_or_else(|err| {
            println!("Problem initializing server: {err}");
            process::exit(1);
        })
    });

    //println!("Initializing database...");
    //init_database();

    //test_bufReader();

    println!("Running server on: {}", listener.local_addr().unwrap());
    run_server(listener);
    println!("\nExiting server...");
}

fn init_server_on_localhost() -> Result<TcpListener, &'static str> {
    let ip_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = RUST_PORT;
    let socket_address = SocketAddr::new(ip_v4, port);

    assert_eq!(socket_address.port(), port);
    assert_eq!(socket_address.is_ipv4(), true);
    assert_eq!(socket_address.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    let listener = TcpListener::bind(socket_address).unwrap();
    if listener.local_addr().unwrap() != socket_address {
        return Err("Fail")
    }

    Ok(listener)
}

fn init_server_on_lan() -> Result<TcpListener, &'static str> {
    let ip_v4 = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 178));
    let port = RUST_PORT;
    let socket_address = SocketAddr::new(ip_v4, port);

    assert_eq!(socket_address.port(), port);
    assert_eq!(socket_address.is_ipv4(), true);
    assert_eq!(socket_address.ip(), ip_v4);

    let listener = TcpListener::bind(socket_address).unwrap();
    if listener.local_addr().unwrap() != socket_address {
        return Err("Fail")
    }

    Ok(listener)
}

fn run_server(listener: TcpListener) {
    let mut counter = 0;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        counter += 1;
        if PRINTING {println!("Connection established! Counter {}", counter)};
        //handle_connection(stream, counter);
        handle_connection(stream, counter);
    }
}

fn init_database() {
    /*
    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

    client.simple_query("
    CREATE TABLE person (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        data    BYTEA
    )
    ").unwrap();
    */
}

fn handle_connection_test(mut stream: TcpStream, mut counter: i32) {
    let mut buffer = [0; 2048]; // store stream bytes

    stream.read(&mut buffer).unwrap(); // store stream bytes in buffer

    let request = String::from_utf8_lossy(&buffer[..]); /* convert buffer to string
                                                                 *  use to_string() to convert to
                                                                 *  String*/

    // determine the type of http request
    if request.starts_with("GET") {
        println!("Incoming GET-request from client!");
    } else if request.starts_with("POST") {
        println!("Incoming POST-request from client!");
    } else {
        println!("Unknown request from client!");
    }

    println!("Request: {}", request);

    let (status_line, filename) = ("HTTP/1.1 200 OK", "index.html");
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection_test2(mut stream: TcpStream, mut counter: i32) {
    let mut buf_reader = BufReader::new(&mut stream);

    //let http_request = String::new();
/*
    let http_request: Vec<_> = buf_reader
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line|!line.is_empty())
        .collect();*/

    //if PRINTING {println!("Request:\n{}", http_request);} // need to examine borrow errors with buf_reader

    //if PRINTING {println!("Request:\n{:#?}", http_request);} // need to examine borrow errors with buf_reader

    let (status_line, filename) = ("HTTP/1.1 200 OK", "index.html");
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write(response.as_bytes()).unwrap();
    //stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream, mut counter: i32) {
    // 1. Get request and convert to string (create function for this)
    //let mut buf_reader = BufReader::new(&mut stream);
    let http_request: String = get_request_string(&mut stream);

    //let request_line: &String = &buf_reader.by_ref().lines().next().unwrap().unwrap();
    //let request_line: &String = &String::new();
    let request_line: String = get_request_line(&http_request);
    let request_line_print = http_request.lines().next().unwrap();
    println!("================================================");
    println!("HTTP request:\n{}", http_request);
    println!("================================================");
    println!("Request line print: {}", request_line_print);
    println!("Request line: {}", request_line);
    println!("================================================");

    /*
    let http_request: Vec<_> = buf_reader
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    */

    //if PRINTING {println!("HTTP request: {request_line}\n{:#?}", http_request);} // need to examine borrow errors with buf_reader

    // 2. Response creation
    let mut response = String::new();

    // determine the type of http request and handle it to get a response
    if request_line.starts_with("GET") {
        println!("Incoming GET-request from client!");
        response = handle_get_request(&request_line);
    } else if request_line.starts_with("POST") {
        println!("Incoming POST-request from client!");
        println!("a");
        //let body = &buf_reader.by_ref().lines().next().unwrap().unwrap();
        let body = "";
        println!("b");
        println!("Request body: {}", body);
        println!("c");

        response = handle_post_request(&request_line);
    } else {
        println!("Unknown request from client!");
        response = handle_get_request(&String::from("GET / HTTP/1.1"));
    }

    stream.write_all(response.as_bytes()).unwrap(); // send http response to client
}

fn handle_connection_new(mut stream: TcpStream, mut counter: i32) {
    let mut buffer = [0; 2048]; // store stream bytes
    stream.read(&mut buffer).unwrap(); // store stream bytes in buffer
    let request = String::from_utf8_lossy(&buffer[..]); // convert buffer to string
    //let mut response: String = String::new();

    // determine the type of http request
    if request.starts_with("GET") {
        println!("Incoming GET-request from client!");
        //let response = handle_get_request(request);
    } else if request.starts_with("POST") {
        println!("Incoming POST-request from client!");
    } else {
        println!("Unknown request from client!");
    }

    println!("Request: {}", request);
    let (status_line, filename) = ("HTTP/1.1 200 OK", "index.html");
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");





    //stream.write(response.as_bytes()).unwrap();
    //stream.flush().unwrap();
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_get_request(request_line: &String) -> String {
    println!("---------------------------------------------------------");
    println!("Hello from handle_post_request!");
    println!("Request line received: {}", request_line);

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" || request_line == "GET /index.html HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else if request_line == "GET /modulo.html HTTP/1.1" {
        ("HTTP/1.1 200 OK", "modulo.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "error.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //println!("Response from helper:\n {}", response);
    println!("---------------------------------------------------------");
    response
}

fn handle_post_request(request_line: &String) -> String {
    println!("---------------------------------------------------------");
    println!("Hello from handle_post_request!");
    println!("Request line received: {}", request_line);

    let (status_line, filename) = ("HTTP/1.1 204 No Content", "index.html");

    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //println!("response from helper:\n {}", response);
    println!("---------------------------------------------------------");
    response
}

fn test_bufReader() {
    // 1. Printing text file contents
    let file = fs::File::open("test_file.txt").unwrap();
    let mut buf_reader = BufReader::new(file);

    println!("\n1. Reading file_contents:\n");
    for line in buf_reader.lines() {
        println!("{}", line.unwrap());
    }

    // 2. Storing text file contents in String using io::read_to_string
    let file = fs::File::open("test_file.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let file_contents = io::read_to_string(buf_reader).unwrap();

    println!("\n2.1 Reading file_contents:\n{:?}", file_contents);
    println!("\n2.2 Reading file_contents:\n{}", file_contents);

    // 3. Storing text file contents in String using io::read_to_string
    let file = fs::File::open("test_file.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut file_contents: String = String::new();

    for line in buf_reader.by_ref().lines() {
        //buf_reader.read_line(&mut file_contents);
        file_contents.push_str(line.unwrap().as_str());
        file_contents.push_str("\n");
    }

    println!("\n3.1 Reading file_contents:\n{:?}", file_contents); // printing raw string
    println!("\n3.2 Reading file_contents:\n{}", file_contents); // pretty printing with newlines
}

fn test_array_buffer() {

}

fn get_request_string(mut stream: & TcpStream) -> String {
    let mut buffer = [0; 2048]; // store stream bytes

    stream.read(&mut buffer).unwrap(); // store stream bytes in buffer

    let request = String::from_utf8_lossy(&buffer[..]).to_string(); /* convert buffer to string
                                                                     *  use to_string() to convert
                                                                     * to  String */
    request
}

fn get_request_line(http_request: &String) -> String {
    http_request.lines().next().unwrap().to_string()
}
