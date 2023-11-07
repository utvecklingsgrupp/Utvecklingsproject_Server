use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{IpAddr, Ipv4Addr,SocketAddr,  TcpListener, TcpStream},
    process,
};

use rand::Rng;

//static mut counter: i32 = 0;

const LAN: bool = false;
const PRINTING: bool = true;


const LOCALHOST_IP_V4: &str = "127.0.0.git 1";
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
        handle_connection(stream, counter);
    }
}

fn handle_connection(mut stream: TcpStream, mut counter: i32) {
    let mut buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.by_ref().lines().next().unwrap().unwrap();

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

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if PRINTING {println!("Request: {request_line}\n{:#?}", http_request);} // need to examine borrow errors with buf_reader

    stream.write_all(response.as_bytes()).unwrap();
}
