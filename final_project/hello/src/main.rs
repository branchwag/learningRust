use std::{
    fs,
    io::{BufReader, prelude::*}, //read-from write-to stream
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream); //ref to stream
    let http_request: Vec<_> = buf_reader //collect what browser sends to out server
        .lines() //returns iterator of Result<String, std::io::Error> by splitting the stream of data whenever it sees a newline byte
        .map(|result| result.unwrap()) //getting each String
        .take_while(|line| !line.is_empty()) //end of hhtp = 2 newline chars
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
