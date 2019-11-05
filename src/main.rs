mod dispatch;
mod threadpool;
#[macro_use]
extern crate serde_derive;
mod config;

use std::net::TcpListener;
use std::fs::File;


fn main() {
    let mut f = File::open("config.toml")
        .expect(&format!("Can't open configuration file."));
    let settings = config::read_config(&mut f).expect("Can't read configuration file.");
    println!("IOT hub address: {}", settings.iothub.broker_address);

    let listener = TcpListener::bind(settings.iothub.broker_address).unwrap();

    let pool = threadpool::ThreadPool::new(settings.iothub.thread_num);

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
