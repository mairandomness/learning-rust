extern crate simple_server;

use simple_server::Server;

fn main() {
    let host = "127.0.0.1";
    let port = "7878";

    let server = Server::new(|request, mut response| {
        Ok(response.body("Hello RustBridge Portland!".as_bytes().to_vec())?)
    });

    server.listen(host, port);
}
