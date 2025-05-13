use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
println!("hello");
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}
