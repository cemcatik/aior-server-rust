mod errors;
mod messages;
mod robot;
mod server;

use errors::*;
use robot::*;
use server::*;

const PORT: u16 = 19876;
const MOUSE_SPEED: f32 = 1.0;
const WHEEL_SPEED: f32 = 1.0;

#[tokio::main]
async fn main() -> Result<()> {
    let robot = Robot::new(MOUSE_SPEED, WHEEL_SPEED);
    let mut server = Server::new(PORT, robot);
    server.start().await
}
