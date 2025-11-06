mod client;
mod message;
mod host;

use std::sync::Arc;

use chess_lib::{Board, DefaultSides};
use dashmap::DashMap;
use host::*;
use message::{ClientMessage, ServerMessage};
use client::Client;

fn main() {
    let clients = Arc::new(DashMap::<usize, Client>::new());

    let mut games = ServerList::new();

    let (sender, receiver) = std::sync::mpsc::channel();

    let clients_clone = clients.clone();
    // let sender_clone = sender.clone();

    std::thread::spawn(move || ws::listen("127.0.0.1:9001", |endpoint| Client::new(&clients_clone, endpoint, &sender)));

    let mut random = rand::thread_rng();

    while let Ok(client::ClientAction { id, action }) = receiver.recv() {
        match action {
            // client::ClientActionType::Connect => todo!(),
            client::ClientActionType::Message(command) => {
                println!("Recieved message from {id}: {command:?}");
        match command {
            // ClientMessage::Host { side } => {
            //     let num = servers.keys().max().copied().unwrap_or(0) + 1;
            //     servers.insert(num, Default::default());
            //     let side = Some(side.unwrap_or([DefaultSides::White, DefaultSides::Black][random.gen_range(0..=1)]));                
            //     join_board(&servers, clients.get_mut(&id).unwrap().value_mut(), num, || side)
            // },
            // ClientMessage::RequestList => {
            //     clients.get(&id).unwrap().send(ServerMessage::ServerList { servers: servers.keys().copied().collect() });
            // },
            // ClientMessage::Join { game, side } => {
            //     let other = side.as_ref().map(DefaultSides::other);
            //     join_board(&servers, clients.get_mut(&id).unwrap().value_mut(), game, || side.filter(|side| !clients.iter().any(|c| c.is_of_side(game, &side))).or_else(|| other.filter(|other| !clients.iter().any(|c| c.is_of_side(game, &other)))))
            // }
            ClientMessage::RequestMoves { pos } => {
                let endpoint = clients.get(&id).unwrap();
                match clients.get(&id).unwrap().game.as_ref() {
                    Some(game) => {
                        let host = &games[game];
                        endpoint.value().send(ServerMessage::Moves { from: pos, to : board.pieces.at(&pos).into_iter().flat_map(|p| p.moves(&board, pos)).collect() });
                    }
                    None => {
                        endpoint.value().send(ServerMessage::Error { kind: message::ClientError::Move { error: chess_lib::MoveError::NoPiece } });
                    }
                }
            }
            ClientMessage::Move { from } => {
                match clients.get(&id).unwrap().game.as_ref() {
                    Some(game) => {
                        match game.side {
                            Some(side) => {
                                let board = servers.get_mut(&game.id).unwrap();
                                if side != board.turn.side {
                                    continue;
                                }
                                match board.move_piece(from) {
                                    Ok(()) => {
                                        for position in board.pieces.drain_changes().collect::<Vec<_>>() {
                                            for client in clients.iter() {
                                                client.value().send(ServerMessage::Update { position, with: board.pieces.at(&position).copied() });
                                            }
                                        }
                                    }
                                    Err(err) => eprintln!("error: {err:?}"),
                                }
                            }
                            None => {
                                clients.get(&id).unwrap().send(ServerMessage::Error { kind: message::ClientError::Spectator });
                            }
                        }

                    }
                    None => {
                    },
                }
            },
            ClientMessage::Resign => {
                if let Some(game) = clients.get_mut(&id).unwrap().value_mut().game.take() {
                    for mut client in clients.iter_mut() {
                        if client.value().game.as_ref().filter(|g| g.id == game.id).is_some() {
                            client.value().send(ServerMessage::EndGame);
                            client.value_mut().game = None;
                        }
                    }
                }
            },
        }

            },
            client::ClientActionType::Disconnect => {
                clients.get_mut(&id).unwrap().game = None;
            },
        }        
    }
}
