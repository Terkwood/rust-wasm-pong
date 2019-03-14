use crate::player::Player;

#[derive(Debug, Copy, Clone)]
pub struct Score(pub u8, pub u8);
impl Score {
    pub fn new() -> Score {
        Score(0, 0)
    }

    pub fn of(self, player: Player) -> u8 {
        match player {
            Player::One => self.0,
            Player::Two => self.1,
        }
    }

    pub fn incr(score: Score, player: Player) -> Score {
        match player {
            Player::One => Score(score.0 + 1, score.1),
            Player::Two => Score(score.0, score.1 + 1),
        }
    }
}
