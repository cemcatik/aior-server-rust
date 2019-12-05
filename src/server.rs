use crate::errors::*;
use crate::messages::*;
use crate::robot::*;
use futures::prelude::*;
use std::net::SocketAddr;
use tokio;
use tokio::net::UdpSocket;

lazy_static! {
    static ref ACCEPT_CONNECTION_RESP: Message = {
        let info = os_info::get();
        let os = info.os_type().to_string();
        let version = info.version().to_string();
        let arch = String::from("???");
        Message::ConnStatus {
            sender: String::from("server"),
            status: String::from("acceptUdpConnection"),
            message: format!("{}-{}-{}", os, version, arch),
        }
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
        format!("localhost:{}", self.port)
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
            .map_err(|e| Error::from(e))
            .await
    }

    async fn serve(&mut self, socket: &mut UdpSocket) -> Result<()> {
        loop {
            let mut buf = vec![0; 1024];
            let (size, dest) = socket.recv_from(&mut buf).await?;
            let parsed = std::str::from_utf8(&buf[0..size])
                .map_err(|e| Error::from(e))
                .and_then(|m| Message::from_str(m).map_err(|e| Error::from(e)));
            match parsed {
                Ok(msg) => self.serve_msg(socket, dest, msg).await?,
                Err(e) => eprintln!("error parsing message: {}", e),
            }
        }
    }

    async fn serve_msg(
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
                let conn_resp = Message::to_string(&ACCEPT_CONNECTION_RESP)? + "\n";
                socket
                    .send_to(conn_resp.as_bytes(), dest)
                    .map_err(|e| Error::from(e))
                    .map(|_| Ok(()))
                    .await
            }
            Message::MouseMove { x, y } => self.robot.mouse_move(x, y).await,
            Message::Aioc {
                id: AiocId::MouseLeftPress,
            } => self.robot.mouse_press(MouseButton::Left).await,
            Message::Aioc {
                id: AiocId::MouseLeftRelease,
            } => self.robot.mouse_release(MouseButton::Left).await,
            Message::Aioc {
                id: AiocId::MouseRightPress,
            } => self.robot.mouse_press(MouseButton::Right).await,
            Message::Aioc {
                id: AiocId::MouseRightRelease,
            } => self.robot.mouse_release(MouseButton::Right).await,
            Message::Aioc {
                id: AiocId::MouseWheelUp,
            } => self.robot.mouse_wheel(WheelDirection::Up).await,
            Message::Aioc {
                id: AiocId::MouseWheelDown,
            } => self.robot.mouse_wheel(WheelDirection::Down).await,
            Message::KeyboardStr { state: _, letter } => self.robot.keyboard_type_str(letter).await,
            Message::KeyboardInt { state: _, letter } => self.robot.keyboard_type_int(letter).await,
            _ => {
                println!("maybe next time");
                Ok(())
            }
        }
    }
}
