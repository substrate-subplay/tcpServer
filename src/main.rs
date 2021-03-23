//导入标准库
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0; 512]; 
    
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;//读取字节流到bytes_read
        println!("message coming");
        if bytes_read == 0 {
            return Ok(());
        }

        stream.write(&buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1 as u64));
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    
    let listener = TcpListener::bind("127.0.0.1:8080")?;//监听tcp端口
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    for stream in listener.incoming() {//执行监听的结果
        let stream = stream.expect("failed!");
        let handle = thread::spawn(move || {
            handle_client(stream)
		.unwrap_or_else(|error| eprintln!("{:?}", error));
        });//监听

        thread_vec.push(handle);//push变量handle到thread_vec
        println!("we got connect!")
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}