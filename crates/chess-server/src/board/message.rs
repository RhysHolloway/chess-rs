use chess_lib::{Move, MoveError, Piece, Pos, DefaultSides, Turn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ClientMessage {
    // hosting

    // in game
    RequestMoves { pos: Pos },
    Move { from: Move },
    Resign,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ServerMessage {
    Join { playing: Option<DefaultSides>, turn: Turn, },
    End { winner: Option<DefaultSides> },

    // in game
    Moves { from: Pos, to: Vec<Pos> },
    Update { position: Pos, with: Option<Piece> },
    Error { kind: ClientError },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ClientError {
    Move { error: MoveError },
    NoServer,
    Spectator,
}

#[cfg(test)]
mod tests {
    use super::ServerMessage;


    #[test]
    fn server_message() {

        assert!(serde_json::from_str::<ServerMessage>(r#"{
        "type":"moves",
        "from":"e2",
        "to":["e3", "e4"]
        }"#).unwrap() == ServerMessage::Moves { from: "e2".parse().unwrap(), to: vec!["e3".parse().unwrap(), "e4".parse().unwrap()] });

    }

}