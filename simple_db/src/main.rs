use std::collections::HashMap;
use std::io::{Read,Write};
use std::process::exit;
use std::net::TcpListener;


fn main() {
    let mut key_value_store: HashMap<String, String> = HashMap::new();
    run_server(&"localhost:5123".to_string());

}

fn run_server(host: &String){
    let listener = TcpListener::bind(host);
    if let Err(e) = listener{
        eprintln!("failed to bind on {}",host);
        exit(1);
    }

    let listener = listener.unwrap();

    loop{
        println!("accept");
        let stream: Result<(std::net::TcpStream, std::net::SocketAddr), std::io::Error> = listener.accept();
        if let Err(e) = stream{
            eprintln!("failed to accept {}",e);
            exit(1);
        }

        let stream = stream.unwrap();
        println!("connected to {}",stream.1);
        println!("stream = {:?}",stream);
        let mut stream = stream.0;

    
        let mut buf: [u8;512] = [0;512];
        let size = stream.read(&mut buf);
        if let Err(e) = size{
            eprintln!("failed to read {}",e);
            exit(1);
        }
    
        let size = size.unwrap();
        if size == 0{
            continue;
        }else{
            println!("{}",size);
        }

        let s = String::from_utf8(buf.to_vec()).unwrap();
        println!("{}",s);

        if let Err(e) = stream.write(b"ok"){
            eprintln!("failed to write {}", e);
        }else{
            println!("{:?}",stream);
        }
    }



}
fn insert(key: String,value: String,mut hashmap: HashMap<String,String>){
    hashmap.insert(key,value);
}