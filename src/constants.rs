use ggez::graphics::Color;

pub const BALL_IMAGE_FILE: &str = "/ball.png";
pub const BLOCK_IMAGE_FILE: &str = "/block.png";
pub const PADDLE_IMAGE_FILE: &str = "/paddle.png";

pub const BLOCK_LENGTH_TO_SCREEN_WIDTH: f32 = 0.025;
pub const BLOCK_LENGTH_TO_SCREEN_HEIGHT: f32 = 0.01875;

pub const PADDLE_WIDTH_TO_SCREEN_WIDTH: f32 = 0.025;
pub const PADDLE_HEIGHT_TO_SCREEN_HEIGHT: f32 = 0.125;

/**
 * Paddle should be able to cross court vertically in 2 seconds
 */
pub const PADDLE_SPEED: f32 = 2.0;

pub const BALL_RADIUS: f32 = 8.0;
/**
 * Ball should be able to cross court horizontally in this many seconds,
 * at starting speed. (Original used 4)
 */
pub const BALL_SPEED: f32 = 4.0;

pub const BALL_ACCEL: f32 = 8.0;

pub const WINNER_FONT: &str = "24px Orbitron";
pub const MENU_FONT: &str = "18px Orbitron";
pub const STATS_FONT: &str = "10px Orbitron";
pub const TEXT_COLOR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

pub const WINNER_MSG: &str = "🏆 WINNER 🏆";
pub const ONE_PLAYER_START_MSG: &str = "press '1' for\n\nsingle player";
pub const ONE_PLAYER_CONTROLS_MSG: &str = "'q': move up\n\n'a': move down";
pub const TWO_PLAYER_START_MSG: &str = "press '2' for\n\ndouble player";
pub const TWO_PLAYER_CONTROLS_MSG: &str = "'p': move up\n\n'l': move down";
