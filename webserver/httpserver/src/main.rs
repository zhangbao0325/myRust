use server::Server;

mod handler;
mod router;
mod server;
fn main() {
    let addr = "127.0.0.1:3000";
    let server = Server::new(addr);
    server.run();

    println!("Hello, world!");
}
