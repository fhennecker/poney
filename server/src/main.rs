use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use tungstenite::protocol::{Message, WebSocket};
use tungstenite::server::accept;

pub mod poneyprotocol;

#[cfg(debug_assertions)]
const BIND_IP: &str = "127.0.0.1:9001";

#[cfg(not(debug_assertions))]
const BIND_IP: &str = "0.0.0.0:9001";

fn client_thread(mut websocket: WebSocket<TcpStream>) {
    use poneyprotocol::*;

    // send fake teams message upfront for debug
    let msg = DownConnMsg::AvailableTeams {
        teams: vec![
            "chaussettes".to_string(),
            "saucettes".to_string(),
        ]
    };
    let serialized = serde_json::to_string(&msg).unwrap();
    let fake_available_teams_msg = Message::from(serialized);
    websocket.write_message(fake_available_teams_msg).unwrap();

    loop {
        let msg = websocket.read_message().unwrap();
        if ! msg.is_text() {
            println!("Received non-text message. Too bad.");
            continue;
        }

        let text = msg.into_text().unwrap();

        let deserialized: Result<UpConnMsg,serde_json::error::Error> = serde_json::from_str(&text);
        match deserialized {
            Ok(x) => println!("Deserialized: {:?}", x),
            Err(x) => println!("Cannot deserialize {}: {}", &text, x),
        }
    }
}

fn game_manager_thread() {

}

fn main() {
    println!("Started server on {}", BIND_IP);

    spawn(game_manager_thread);

    let server = TcpListener::bind(BIND_IP).unwrap();
    for stream in server.incoming() {
        spawn(move || {
            client_thread(accept(stream.unwrap()).unwrap());
        });
    }
}
