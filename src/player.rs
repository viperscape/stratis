extern crate rand;

#[derive(Debug,Clone)]
pub struct Player {
    pub nick: String,
}

impl Player {
    pub fn default () -> Player {
        let mut nick = "player_".to_string();
        nick.push_str(&rand::random::<u16>().to_string());

        Player { nick: nick }
    }
}
