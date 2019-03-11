use crate::player::Player;

#[derive(Copy, Clone)]
pub struct Score(pub u32, pub u32);
impl Score {
    pub fn new() -> Score {
        Score(0, 0)
    }

    pub fn of(self, player: Player) -> u32 {
        match player {
            Player::One => self.0,
            Player::Two => self.1,
        }
    }

    pub fn incr(mut self, player: Player) {
        match player {
            Player::One => self.0 = self.0 + 1,
            Player::Two => self.1 = self.1 + 1,
        }
    }
}
