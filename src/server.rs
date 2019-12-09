use crate::errors::*;
use crate::messages::*;
use crate::robot::*;
use futures::prelude::*;
use lazy_static::lazy_static;
use std::net::SocketAddr;
use tokio;
use tokio::net::UdpSocket;

lazy_static! {
    static ref ACCEPT_CONNECTION_RESP: String = {
        let os = String::from("Mac OS X");
        let version = String::from("10.15.1");
        let arch = String::from("x86_64");
        let msg = Message::ConnStatus {
            sender: String::from("server"),
            status: String::from("acceptUdpConnection"),
            message: format!("{}-{}-{}", os, version, arch),
        };
        Message::to_string(&msg).unwrap() + "\n"
    };
}

pub struct Server {
    port: u16,
    robot: Robot,
}

impl Server {
    pub fn new(port: u16, robot: Robot) -> Server {
        Server { port, robot }
    }

    fn address(&self) -> String {
        format!("0:{}", self.port)
    }

    pub async fn start(&mut self) -> Result<()> {
        let mut socket = self.bind().await?;
        self.serve(&mut socket).await
    }

    async fn bind(&self) -> Result<UdpSocket> {
        let addr = self.address();
        println!("starting server on {}", addr);
        UdpSocket::bind(&addr)
            .inspect(|socket| match socket {
                Ok(_) => println!("server started on {}", addr),
                Err(e) => eprintln!("failed to bind: {}", e),
            })
            .map_err(Error::from)
            .await
    }

    async fn serve(&mut self, socket: &mut UdpSocket) -> Result<()> {
        loop {
            let mut buf = vec![0; 1024];
            let (size, dest) = socket.recv_from(&mut buf).await?;
            let parsed = std::str::from_utf8(&buf[0..size])
                .map_err(Error::from)
                .and_then(|m| Message::from_str(m).map_err(Error::from));
            match parsed {
                Ok(msg) => self.handle_msg(socket, dest, msg).await?,
                Err(e) => eprintln!("error parsing message: {}", e),
            }
        }
    }

    async fn handle_msg(
        &mut self,
        socket: &mut UdpSocket,
        dest: SocketAddr,
        msg: Message,
    ) -> Result<()> {
        match msg {
            Message::Aioc {
                id: AiocId::ConnectionReceived,
            } => {
                println!("connection attempt from {}", dest);

                let dest = {
                    let mut d = SocketAddr::from(dest);
                    d.set_port(self.port);
                    d
                };

                return socket
                    .send_to(ACCEPT_CONNECTION_RESP.as_bytes(), dest)
                    .map_err(Error::from)
                    .map(|_| Ok(()))
                    .await;
            }
            m => self.robot.handle(m),
        };
        Ok(())
    }
}
