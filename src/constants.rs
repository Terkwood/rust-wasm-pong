use ggez::graphics::Color;

pub const BALL_IMAGE_FILE: &str = "/ball.png";
pub const BLOCK_IMAGE_FILE: &str = "/block.png";
pub const PADDLE_IMAGE_FILE: &str = "/paddle.png";
pub const PRESS1_IMAGE_FILE: &str = "/press1.png";
pub const PRESS2_IMAGE_FILE: &str = "/press2.png";
pub const WINNER_IMAGE_FILE: &str = "/winner.png";

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

pub const FONT: &str = "10px Orbitron";
pub const TEXT_COLOR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
