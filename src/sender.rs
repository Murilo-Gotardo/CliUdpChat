use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4};
use std::io;
use crate::message::Message;

pub struct Sender {
    multicast_address: Ipv4Addr,
    multicast_port: u16,
    socket: Option<UdpSocket>
}

impl Sender {
    pub fn new(address: Ipv4Addr, port: u16) -> Self {
        Self {
            multicast_address: address,
            multicast_port: port,
            socket: None,
        }
    }

    pub fn join_multicast_group(&mut self, local_ip_address: Ipv4Addr) -> io::Result<()> {
        let socket = UdpSocket::bind((local_ip_address, 0))?;
        socket.join_multicast_v4(&self.multicast_address, &local_ip_address)?;
        self.socket = Some(socket);
        Ok(())
    }

    pub fn broadcast_message(&self, message: Message) -> io::Result<()> {
        let json = serde_json::to_string(&message)?;
        
        if let Some(ref socket) = self.socket {
            let end_point = SocketAddrV4::new(self.multicast_address, self.multicast_port);
            socket.send_to(json.as_bytes(), end_point)?;
        }
        Ok(())
    }
}
