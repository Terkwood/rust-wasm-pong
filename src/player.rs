#[derive(Debug, Copy, Clone)]
pub enum Player {
    One,
    Two,
}
impl Player {
    pub fn other(self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}
