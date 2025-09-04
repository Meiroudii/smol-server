use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("{:#?}", stream);
        invoke_link(stream);
    }

}

fn invoke_link(mut stream: TcpStream) {
    let buffr_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buffr_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("\tRequest: {http_request:#?}");
}
