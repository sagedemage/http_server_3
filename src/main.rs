use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};
use std::{fs, io};
use image::codecs::png::{FilterType, CompressionType, PngEncoder};
use image::{EncodableLayout, ImageReader};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server running at http://127.0.0.1:8080");
    for stream in listener.incoming() {
        let stream = stream?;
        
        handle_connection(stream)?;
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()>  {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = &http_request[0];
    let request_items_split = request_line.split(" ");
    let request_items = request_items_split.collect::<Vec<&str>>();
    let route = request_items[1];

    let status_line = "HTTP/1.1 200 OK";
    if route == "/" {
        let contents = fs::read_to_string("server_files/index.html")?;
        let length = contents.len();
        let content_type = "document";
        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes())?;
    } else if route == "/about" {
        let contents = fs::read_to_string("server_files/about.html")?;
        let length = contents.len();
        let content_type = "document";
        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes())?;
    } else if route == "/styles.css" {
        let contents = fs::read_to_string("server_files/styles.css")?;
        let length = contents.len();
        let content_type = "text/css";
        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes())?;
    } else if route == "/main.js" {
        let contents = fs::read_to_string("server_files/main.js")?;
        let length = contents.len();
        let content_type = "script";
        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes())?;
    } else if route == "/favicon.png" {
        let mut img_bytes: Vec<u8> = Vec::new();

        img_bytes = get_encoded_image_data(img_bytes, "server_files/favicon.png");

        let length = img_bytes.len();
        let response_line = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: image/png\r\n\r\n");
        let vector_response: Vec<u8> = [response_line.as_bytes(), img_bytes.as_bytes()].concat();
        let response = vector_response.as_slice();
        stream.write_all(response)?;
    }
    
    Ok(())
}

fn get_encoded_image_data(mut img_bytes: Vec<u8>, img_file_path: &str) -> Vec<u8> {
    let encoder = PngEncoder::new_with_quality(&mut img_bytes, CompressionType::Default, FilterType::NoFilter);
    let img_reader_result = ImageReader::open(img_file_path); //.unwrap().decode().unwrap();
    if img_reader_result.is_ok() {
        let img_reader = img_reader_result.unwrap();
        let img = img_reader.decode().unwrap();
        img.write_with_encoder(encoder).unwrap();
    }

    return img_bytes;
}