//A sample webserver with 2 html files.
//to run this program, on the terminal type - 
//cargo run
//open browser to localhost_8080
use std::{
    fs,
    io::{prelude::*, BufReader},//for the traits and types to read / write into the stream.
    net::{TcpListener, TcpStream},//tcp connection lib
};

fn main() {
    //create a listener on port 8080
    //assume an error-free conn, so not handling ::bind() errors, but using unwrap() to stop the program
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    

    for stream in listener.incoming() {
        //got an connection from client
        let stream = stream.unwrap();
        handle_connection(stream);//client request handler logic
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap(); //by lines().next() ge the first item from stream

    if request_line == "GET / HTTP/1.1" { //accept if we recieved a GET
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else { //not a GET request
        let status_line = "HTTP/1.1 404 NOT FOUND"; //handling just 404 error only.
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap(); //sends data (html) to the connection end.
    }
}