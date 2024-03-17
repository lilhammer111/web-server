mod server;
mod router;
mod handler;

use core::panicking::panic_nounwind_nobacktrace;
use std::alloc::handle_alloc_error;

use server::Server;

fn main() {
    let server = Server::new("localhost:3000");
    server.run();
}
