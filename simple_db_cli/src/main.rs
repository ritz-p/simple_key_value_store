use std::net::TcpStream;
use std::io::{Read,Write};
use std::process::exit;

fn main() {
    connect(&"localhost:5123".to_owned());
}

fn connect(host: &String){
    let stream = TcpStream::connect(host);
    if let Err(e) = stream{
        eprintln!("failed to connect {}",e);
        exit(1);
    }
    let mut stream = stream.unwrap();

    if let Err(e) = stream.write_all(b"hello"){
        eprintln!("failed to write {}",e);
        exit(1);
    }

    let mut buf:[u8;512] = [0;512];
    if let Err(e) = stream.read(&mut buf){
        eprintln!("failed to read {}",e);
        exit(1);
    }

    let s = String::from_utf8(buf.to_vec()).unwrap();
    println!("{}",s);
}