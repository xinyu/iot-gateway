mod dispatch;
mod threadpool;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = threadpool::ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    dispatch::handle_message(stream);
                });
            }
            Err(e) => { 
                println!("connection failed! {}", e);
            }
        }
    }
}
