use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::spawn;
use tungstenite::protocol::{Message, WebSocket};
use tungstenite::server::accept;

pub mod poneyprotocol;
use poneyprotocol::*;

#[cfg(debug_assertions)]
const BIND_IP: &str = "127.0.0.1:9001";

#[cfg(not(debug_assertions))]
const BIND_IP: &str = "0.0.0.0:9001";

fn client_thread(
    mut websocket: WebSocket<TcpStream>,
    client_to_gm_tx: Sender<UpConnMsg>,
    gm_to_client_rx: Receiver<DownConnMsg>,
) {
    // send fake teams message upfront for debug
    let msg = DownConnMsg::AvailableTeams {
        teams: vec!["chaussettes".to_string(), "saucettes".to_string()],
    };
    let serialized = serde_json::to_string(&msg).unwrap();
    let fake_available_teams_msg = Message::from(serialized);
    websocket.write_message(fake_available_teams_msg).unwrap();

    loop {
        let msg = websocket.read_message().unwrap();
        if !msg.is_text() {
            println!("Received non-text message. Too bad.");
            continue;
        }

        let text = msg.into_text().unwrap();

        let deserialized: Result<UpConnMsg, serde_json::error::Error> = serde_json::from_str(&text);
        match deserialized {
            Ok(x) => {
                println!("Deserialized: {:?}", x);
                client_to_gm_tx.send(x);
            }
            Err(x) => println!("Cannot deserialize {}: {}", &text, x),
        }
    }
}

fn game_manager_thread(
    client_to_gm_rx: Receiver<UpConnMsg>,
    main_to_gm_rx: Receiver<Sender<DownConnMsg>>,
) {
    // pleath poll here
    loop {
        println!(
            "The Master Of the Games received: {:?}",
            client_to_gm_rx.recv().unwrap()
        );
    }
}

fn main() {
    println!("Started server on {}", BIND_IP);

    let (client_to_gm_tx, gm_to_client_rx) = channel::<UpConnMsg>();

    let (main_to_gm_tx, gm_to_main_rx) = channel::<Sender<DownConnMsg>>();

    spawn(move || {
        game_manager_thread(gm_to_client_rx, gm_to_main_rx);
    });

    let server = TcpListener::bind(BIND_IP).unwrap();
    for stream in server.incoming() {
        let client_to_gm_tx_clone = client_to_gm_tx.clone();
        let (gm_to_client_tx, client_to_gm_rx) = channel::<DownConnMsg>();
        spawn(move || {
            client_thread(
                accept(stream.unwrap()).unwrap(),
                client_to_gm_tx_clone,
                client_to_gm_rx,
            );
        });
        // sending gm->client communication channel to the game master
        main_to_gm_tx.send(gm_to_client_tx);
    }
}
