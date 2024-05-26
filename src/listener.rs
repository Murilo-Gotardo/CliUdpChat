use std::net::{UdpSocket, Ipv4Addr};
use std::io;
use std::io::{stdout, Write};
use crate::message::Message;

pub struct Listener {
    multicast_address: Ipv4Addr,
    multicast_port: u16,
    socket: Option<UdpSocket>
}

impl Listener {
    pub fn new(address: Ipv4Addr, port: u16) -> Self {
        Self {
            multicast_address: address,
            multicast_port: port,
            socket: None,
        }
    }

    pub fn start_multicast(&mut self, local_ip_address: Ipv4Addr) -> io::Result<()> {
        let socket = UdpSocket::bind((local_ip_address, self.multicast_port))?;
        socket.join_multicast_v4(&self.multicast_address, &local_ip_address)?;
        self.socket = Some(socket);
        Ok(())
    }

    pub fn receive_broadcast_messages(&self) -> io::Result<()> {
        if let Some(ref socket) = self.socket {
            //socket.set_read_timeout(Some(Duration::from_secs(30)))?;
            let done = false;
            let mut buf = [0u8; 1000];

            while !done {
                match socket.recv_from(&mut buf) {
                    Ok((size, _src)) => {
                        let json = &*String::from_utf8_lossy(&buf[..size]);
                        let message: Message = serde_json::from_str(json)?;
                        let mut stdout = stdout().lock();
                        
                        writeln!(stdout).unwrap();
                        writeln!(stdout, "{:?}: {}: {}", message.time(), message.user_name(), message.message()).unwrap();
                        writeln!(stdout).unwrap();
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        println!("Erro ao receber mensagem: {}", e);
                        break;
                    }
                }
            }

            socket.leave_multicast_v4(&self.multicast_address, &socket.local_addr()?.ip().to_string().parse::<Ipv4Addr>().unwrap())?;
        }
        Ok(())
    }
}