use std::net::SocketAddr;

use axum::body::Bytes;
use axum::extract::ConnectInfo;
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse};
use tokio::net::TcpListener;

use axum::{
    routing::{get, post},
    extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    http::StatusCode,
    Json, Router,
};
use tracing::{info, error};

// mod board;
// mod server;

#[tokio::main]
async fn main() {
    
    tracing_subscriber::fmt::init();

    let port: u16 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "8080".to_string())
        .parse().unwrap_or_else(|e| panic!("Could not parse port argument with error: {e}"));

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

        // build our application with a route
    let app = Router::new()
        .route("/", get(home))
        .fallback(handle404);

    // let cancellation_token = CancellationToken::new();
    axum::serve(listener, app).await.unwrap_or_else(|e| panic!("Could not start web server with error {e}"));
}

async fn home() -> Html<&'static str> {
    Html(include_str!("../static/home.html"))
}

// async fn play() -> Html<&'static str> {
//     Html(include_str!("../static/play.html"))
// }

async fn handle404() -> Html<&'static str> {
    Html(include_str!("../static/404.html"))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    header: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = header.get(axum::http::header::USER_AGENT).and_then(|val| val.to_str().ok()).unwrap_or("Unknown browser");
    info!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}


/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    // send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket
        .send(Message::Ping(Bytes::from_static(&[1, 2, 3])))
        .await
        .is_ok()
    {
        info!("Pinged {who}...");
    } else {
        info!("Could not send ping {who}!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    // receive single message from a client (we can either receive or send with socket).
    // this will likely be the Pong for our Ping or a hello message from client.
    // waiting for message from a client will block this task, but will not block other client's
    // connections.
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who).is_break() {
                return;
            }
        } else {
            info!("client {who} abruptly disconnected");
            return;
        }
    }

    // Since each client gets individual statemachine, we can pause handling
    // when necessary to wait for some external event (in this case illustrated by sleeping).
    // Waiting for this client to finish getting its greetings does not prevent other clients from
    // connecting to server and receiving their greetings.
    for i in 1..5 {
        if socket
            .send(Message::Text(format!("Hi {i} times!").into()))
            .await
            .is_err()
        {
            info!("client {who} abruptly disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        let n_msg = 20;
        for i in 0..n_msg {
            // In case of any websocket error, we exit.
            if sender
                .send(Message::Text(format!("Server message {i} ...").into()))
                .await
                .is_err()
            {
                return i;
            }

            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }

        info!("Sending close to {who}...");
        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Utf8Bytes::from_static("Goodbye"),
            })))
            .await
        {
            info!("Could not send Close due to {e}, probably it is ok?");
        }
        n_msg
    });

    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            // print message and break if instructed to do so
            if process_message(msg, who).is_break() {
                break;
            }
        }
        cnt
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => info!("{a} messages sent to {who}"),
                Err(a) => error!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => info!("Received {b} messages"),
                Err(b) => error!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    info!("Websocket context {who} destroyed");
}
