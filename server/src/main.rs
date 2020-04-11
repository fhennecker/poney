use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::protocol::Message;
use tungstenite::server::accept;

pub mod poneyprotocol;

#[cfg(debug_assertions)]
const BIND_IP: &str = "127.0.0.1:9001";

#[cfg(not(debug_assertions))]
const BIND_IP: &str = "0.0.0.0:9001";

fn main() {
    use poneyprotocol::*;

    println!("Started server on {}", BIND_IP);

    let server = TcpListener::bind(BIND_IP).unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

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
                if msg.is_binary() || msg.is_text() {
                    println!("received: {}", msg);
                    //websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}
