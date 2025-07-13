use std::{
    net::{TcpListener, TcpStream},
    io::{BufReader, prelude::*},
    fs::{exists, read_to_string}
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

    let path = http_request[0]
        .split_whitespace()
        .nth(1)
        .unwrap_or("/");

    let normalized_path = sunfire_site::normalize_http_path(path);
    debug!("Normalized Path: {} -> {}", path, normalized_path);

    let mut response;

    match path {
        _ => {
            let asset_path = sunfire_site::build_asset_path(normalized_path.as_str());
            if exists(&asset_path).expect("Unable to read files") {
                debug!("Asset at {} Exists!", asset_path);
                let content = std::fs::read_to_string(asset_path);
                response = response::Response::new(
                    200,
                    vec![],
                    content.expect("Unable to read content")
                );
            } else {
                error!("404 Not Found for path: {}", path);
                response = response::Response::new(
                    404,
                    vec!["Content-Type: text/html; charset=UTF-8"],
                    "<html><body><h1>404 Not Found</h1></body></html>".to_string(),
                );
            }
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
