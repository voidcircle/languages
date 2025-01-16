use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
    time::Duration,
};

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    // the first unwrap method covers the Option stuff for lines method
    // the second unwrap method covers the Result stuff for the next method
    let request_line: String = BufReader::new(&stream).lines().next().unwrap().unwrap();
    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
    let route: &str = parts.get(1).unwrap();

    let mut file_path: String = get_file_route_path(&route);
    let mut status_line: &str = "HTTP/1.1 200 OK";

    match Path::new(&file_path).exists() {
        false => {
            file_path = get_file_route_path("404");
            status_line = "HTTP/1.1 404 NOT FOUND";
        }
        _ => {}
    };

    let content: String = fs::read_to_string(&file_path).unwrap();
    let content_length: usize = content.len();
    let response: String = get_response(status_line, content_length, &content);

    if route == "/sleep" {
        thread::sleep(Duration::from_secs(4));
    }

    stream.write_all(response.as_bytes()).unwrap();
}

fn get_file_route_path(route: &str) -> String {
    if route.starts_with("/") {
        format!("pages{route}/index.html")
    } else {
        String::from("pages/404.html")
    }
}

fn get_response(status_line: &str, content_length: usize, content: &str) -> String {
    format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n{content}",
        content_length.to_string()
    )
}
