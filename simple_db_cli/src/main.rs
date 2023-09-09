use std::net::TcpStream;
use std::io::{Read,Write,stdin};
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
    loop{
        let command = read_buffer();
        if command == "exit"{
            println!("bye");
            exit(0)
        }
        let mut stream = stream.as_ref().unwrap();

        if let Err(e) = stream.write_all(command.as_bytes()){
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

}

fn read_buffer() -> String{
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Faild to read line");
    buf.trim().to_string()
}