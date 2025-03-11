use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};

pub trait GameState {
    type Event;
    type Error;

    fn validate(&self, event: &Self::Event) -> Result<(), Self::Error>;
    fn consume(&mut self, event: &Self::Event);

    fn next(&mut self, event: &Self::Event) -> Result<(), Self::Error> {
        self.validate(event)?;
        self.consume(event);
        Ok(())
    }
}

pub fn run() {
    let addr: SocketAddr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5555).into();
    let connection_config = ConnectionConfig::default();
    let mut server = RenetServer::new(connection_config);

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 2,
        protocol_id: 1,
        public_addresses: vec![addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let socket = UdpSocket::bind(addr).unwrap();

    let mut transport = NetcodeServerTransport::new(server_config, socket).unwrap();

    let mut received_messages = vec![];
    let mut last_updated = Instant::now();

    loop {
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;

        server.update(duration);
        transport.update(duration, &mut server).unwrap();

        received_messages.clear();

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    server.broadcast_message_except(
                        client_id,
                        DefaultChannel::ReliableOrdered,
                        format!("User \"{}\" connected", client_id),
                    );
                    println!("Client {} connected.", client_id)
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {} disconnected: {}", client_id, reason);
                    server.broadcast_message_except(
                        client_id,
                        DefaultChannel::ReliableOrdered,
                        format!("User \"{}\" disconnected", client_id),
                    );
                }
            }
        }

        for client_id in server.clients_id() {
            while let Some(message) =
                server.receive_message(client_id, DefaultChannel::ReliableOrdered)
            {
                let text = String::from_utf8(message.into()).unwrap();
                println!("Client {} sent text: {}", client_id, text);
                let text = format!("{}: {}", client_id, text);
                received_messages.push(text);
            }
        }

        for text in received_messages.iter() {
            server.broadcast_message(DefaultChannel::ReliableOrdered, text.as_bytes().to_vec());
        }

        transport.send_packets(&mut server);
        thread::sleep(Duration::from_millis(50));
    }
}
