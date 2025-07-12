use std::{
    net::{TcpListener, TcpStream},
    io::{BufReader, prelude::*}
};
use simplelog::*;
use log::{info, error, debug};

mod response;

// Selectively enable log levels based on debug enabled
#[cfg(debug_assertions)]
fn configure_logging() {
    let _ = TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
}

#[cfg(not(debug_assertions))]
fn configure_logging() {
    let _ = TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
}


fn handle_connection(mut stream: TcpStream) {
    // Handle the connection here
    debug!("Handling connection from: {:?}", stream.peer_addr().unwrap());
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    debug!("Parsed HTTP Data: {:#?}", http_request);
    info!("{}", http_request[0]);

    let mut path = http_request[0]
        .split_whitespace()
        .nth(1)
        .unwrap_or("/");

    // Handle File Extensions
    let ext = path.split('.').last();

    if !path.starts_with("/api") && !ext.is_some() {
        let str_path = format!("{}index.html", path);
        path = str_path.as_str();
    }
    
    let mut response;

    match path {
        "/" => {
            response = response::Response::new(
                200,
                vec!["Content-Type: text/html; charset=UTF-8"],
                "<html><body><h1>Welcome to SunfireSite.rs!</h1></body></html>",
            );
        }
        _ => {
            error!("404 Not Found for path: {}", path);
            response = response::Response::new(
                404,
                vec!["Content-Type: text/html; charset=UTF-8"],
                "<html><body><h1>404 Not Found</h1></body></html>",
            );
        }
    }

    let response_string = response.to_string();
    stream.write_all(response_string.as_bytes()).unwrap_or_else(|e| {
        error!("Failed to write response: {}", e);
    });
    debug!("Response sent to client: {:#?}", response_string);
    debug!("Connection handled successfully.");
}

fn main() {
    configure_logging();
    info!("Starting the TCP server...");
    debug!("Debug mode is enabled, additional debug information will be logged.");


    let listener = TcpListener::bind("127.0.0.1:3000").unwrap_or_else(|e| {
        error!("Failed to bind to address: {}", e);
        std::process::exit(1);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        debug!("New connection established: {:?}", stream.peer_addr().unwrap());
        handle_connection(stream);
    }
}
