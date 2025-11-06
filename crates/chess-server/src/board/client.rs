use std::sync::mpsc::Sender;

use dashmap::DashMap;

use super::message::{ClientMessage, ServerMessage};

pub type ClientId = usize;

#[derive(Clone)]
pub struct ClientList(std::sync::Arc<DashMap<ClientId, Client>>);

pub struct ClientAction {
    pub id: usize,
    pub action: ClientActionType,
}

pub enum ClientActionType {
    // Connect,
    Message(ClientMessage),
    Disconnect,
}

pub struct Client {
    endpoint: ClientEndpoint,
    pub game: Option<usize>,
}

#[derive(Clone)]
pub struct ClientEndpoint {
    id: usize,
    queue: Sender<ClientAction>,
    endpoint: ws::Sender,
}

impl Client {

    pub fn new(clients: &DashMap<usize, Self>, endpoint: ws::Sender, queue: &Sender<ClientAction>) -> ClientEndpoint {
        let id = clients.iter().map(|e| e.endpoint.id + 1).max().unwrap_or(0);
        let endpoint = ClientEndpoint { id, queue: queue.clone(), endpoint };
        clients.insert(id, Client { endpoint: endpoint.clone(), game: None });
        endpoint
    }
    
    pub fn id(&self) -> &ClientId {
        &self.endpoint.id
    }

    pub fn send(&self, message: ServerMessage) {
        let message = serde_json::to_string(&message).expect("Could not serialize server message!");
        println!("sent message to client #{}: {}", self.endpoint.id, &message);
        self.endpoint.endpoint.send(message).expect("Could not send server message!");
    }

    pub fn close(&self) {
        self.endpoint.endpoint.close(ws::CloseCode::Normal).expect("Could not close endpoint!");
    }
    
}

impl ClientEndpoint {

    fn queue(&self, message: impl FnOnce() -> ClientAction) {
        self.queue.send((message)()).expect("Could not send client message through queue!");
    }

}

impl ws::Handler for ClientEndpoint {

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        println!("Client from {:?} connected!", shake.remote_addr());
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        match msg {
            ws::Message::Text(msg) => match serde_json::from_str::<ClientMessage>(&msg) {
                Ok(message) => self.queue(|| ClientAction {
                    id: self.id, 
                    action: ClientActionType::Message(message),
            }),
                Err(err) => eprintln!("Could not read client message \"{}\" from client #{}", err, self.id),
            },
            ws::Message::Binary(_) => todo!(),
        }

        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        println!("Client #{} disconnected with code {:?} and reason: {}", self.id, code, reason);
        self.queue(|| ClientAction { id: self.id, action: ClientActionType::Disconnect });
    }

}