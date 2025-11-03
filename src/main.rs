use std::net::TcpListener;
use zero2prod::startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind("127.0.0.1:0").expect("unable to provide port for tcp listener");
    println!("port is: {}", listener.local_addr().unwrap().port());
    startup::run(listener)?.await
}
