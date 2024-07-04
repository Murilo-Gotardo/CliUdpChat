mod sender;
mod listener;
mod message;

use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};
use std::{io, thread};
use std::io::Write;
use crate::listener::Listener;
use crate::message::Message;
use crate::sender::Sender;

fn main() -> io::Result<()> {
    let multicast_address = Ipv4Addr::new(224, 168, 100, 2);
    let multicast_port = 11000;

    let sender = Arc::new(Mutex::new(Sender::new(multicast_address, multicast_port)));
    let listener = Arc::new(Mutex::new(Listener::new(multicast_address, multicast_port)));
    
    print!("Digite seu ip: ");
    io::stdout().flush().expect("falha ao limpar buffer do stdout");
    let mut ip_input = String::new();
    io::stdin().read_line(&mut ip_input)?;
    let ip_address = ip_input.trim().parse::<Ipv4Addr>().expect("Ip inválido");

    print!("Digite seu nome: ");
    io::stdout().flush().expect("falha ao limpar buffer do stdout");
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).expect("falha ao ler nome");
    
    let sender_clone = Arc::clone(&sender);
    let thread_tx = thread::spawn(move || {
        let mut sender = sender_clone.lock().unwrap();
        sender.join_multicast_group(ip_address).expect("falha ao entrar no grupo multicast");
        println!("Entrando no grupo...");
        println!("Pode digitar!");
        loop {
            io::stdout().flush().expect("falha ao limpar buffer do stdout");
            let mut message_to_send = String::new();
            io::stdin().read_line(&mut message_to_send).expect("falha ao ler mensagem");
            let message = Message::new(name_input.clone(), message_to_send);
            sender.broadcast_message(message.unwrap()).expect("falha ao enviar mensagem");
        }
    });

    let listener_clone = Arc::clone(&listener);
    let thread_rx = thread::spawn(move || {
        let mut listener = listener_clone.lock().unwrap();
        listener.start_multicast(ip_address).expect("falha ao iniciar multicast");
        listener.receive_broadcast_messages().expect("falha ao receber mensagem");
    });

    thread_tx.join().unwrap();
    thread_rx.join().unwrap();

    Ok(())
}