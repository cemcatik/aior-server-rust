mod errors;
mod messages;
mod server;

use errors::*;
use server::*;

#[macro_use]
extern crate lazy_static;

const PORT: u16 = 19876;

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::new(PORT);
    server.start().await
}
