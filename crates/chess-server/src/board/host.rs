use chess_lib::{Board, DefaultSides};

use super::client::{Client, ClientList};
use super::message::{ClientError, ServerMessage};

pub type ServerId = usize;

pub struct ServerList(std::collections::HashMap<ServerId, Server>);

pub struct Server {
    board: Board,
    pub players: [Option<Side>; 2],
    // pub spectators: Vec<usize>,
}

pub struct Side {
    pub id: usize,
}

impl ServerList {

    pub fn join(&mut self, id: ServerId, client: &mut Client, side: Option<DefaultSides>) -> Result<(), ClientError> {
        self.0.get_mut(&id).map(|s| s.join(id, client, side)).ok_or(ClientError::NoServer)
    }

    pub fn leave(&self, id: ServerId, clients: ()) -> Result<(), ClientError> {
        self.0.get_mut(&id).map(|s| s.leave(clients)).ok_or(ClientError::NoServer)
    }

}

impl Server {

    pub fn join(&mut self, this: ServerId, client: &mut Client, side: Option<DefaultSides>) {

        let playing = side.and_then(|side| {
            if self.players[side as usize].is_none() {
                self.players[side as usize] = Some(Side { id: client.id().clone() });
                Some(side)
            } else if self.players[side.other() as usize].is_none() {
                Some(side.other())
            } else {
                None
            }
        });

        client.game = Some(this);
        client.send(ServerMessage::Join { playing, turn: self.board.turn }); 
        self.board.pieces.iter().for_each(|(pos, piece)| {
            client.send(ServerMessage::Update { position: *pos, with: Some(*piece) });
        });
    }

    pub fn leave(&mut self, clients: &ClientList, side: DefaultSides) {
        self.players
    }
    
}