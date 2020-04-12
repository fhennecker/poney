use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    name: String,
    price: f64
}

// Downstream message (server->client) in the connection management context
#[derive(Serialize, Debug)]
#[serde(tag="type", content="payload", rename_all="snake_case")]
pub enum DownConnMsg<'a> {
    Welcome,
    AvailableTeams {teams: Vec<&'a String>},
    JoinedGame,
    PlayerConnected {username: String, team: String},
    GameStarted {username: String},
}

// {"type": "join_request", "payload": {"username": "Plop", "team": "Team Team"}}

// Upstream message (client->server) in the connection management context
#[derive(Deserialize, Debug)]
#[serde(tag="type", content="payload", rename_all="snake_case")]
pub enum UpConnMsg {
    JoinRequest {username: String, team: String},
    GameStartRequest {username: String},
}

// Downstream message in a game context
#[derive(Serialize, Debug)]
#[serde(tag="type", content="payload", rename_all="snake_case")]
pub enum DownGameMsg {
    PoneyDragged {team: String},
    ItemsAvailable {items: Vec<Item>},
    BudgetUpdate {budget: f64},
    ItemBuyConfirm {item: String},
    PoneyPosition {position: f64},
    GameOver {winner: usize},
}

// Upstream message in a game context
#[derive(Deserialize, Debug)]
#[serde(tag="type", content="payload", rename_all="snake_case")]
pub enum UpGameMsg {
    ItemBuyRequest {item: String},
}
