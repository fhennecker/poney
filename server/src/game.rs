use crate::poneyprotocol::*;
use std::collections::HashMap;

pub struct Team {
    pub name: String,
    pub players: Vec<String>,
}

impl Team {
    pub fn new(name: String) -> Team {
        Team {
            name: name,
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: String) {
        if !self.players.contains(&player) {
            self.players.push(player);
        }
    }
}

pub struct Game {
    pub teams: HashMap<String, Team>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            teams: HashMap::new(),
        }
    }

    pub fn add_player_to_team(&mut self, player: &str, team: &str) {
        self.teams
            .insert(team.to_string(), Team::new(team.to_string()));
        self.teams
            .get_mut(team)
            .unwrap()
            .add_player(player.to_string());
    }

    pub fn handle_up_conn_msg(&mut self, msg: UpConnMsg) -> DownConnMsg {
        match msg {
            UpConnMsg::JoinRequest { username, team } => {
                // TODO check if username does not exist yet
                self.add_player_to_team(&username, &team);
                DownConnMsg::PlayerConnected {
                    username: username,
                    team: team,
                }
                // broadcast to all
            }
            UpConnMsg::GameStartRequest { username } => {
                DownConnMsg::GameStarted { username: username }
                // broadcast to all
            }
        }
    }
}
