pub const BALL_IMAGE_FILE: &str = "/ball.png";
pub const PADDLE_IMAGE_FILE: &str = "/paddle.png";
pub const PRESS1_IMAGE_FILE: &str = "/press1.png";
pub const PRESS2_IMAGE_FILE: &str = "/press2.png";
pub const WINNER_IMAGE_FILE: &str = "/winner.png";

pub const PADDLE_HEIGHT_TO_SCREEN_HEIGHT: f32 = 0.125;
pub const PADDLE_WIDTH_TO_SCREEN_WIDTH: f32 = 0.025;
/**
 * Paddle should be able to cross court vertically in 2 seconds
 */
pub const PADDLE_SPEED: f32 = 2.0;
pub const WALL_WIDTH: f32 = 12.0;

pub const BALL_RADIUS: f32 = 8.0;
/**
 * Ball should be able to cross court horizontally in this many seconds,
 * at starting speed. (Original used 4)
 */
pub const BALL_SPEED: f32 = 4.0;

// FIXME original says "accelerate as time passes"... but we don't?
pub const BALL_ACCEL: f32 = 8.0;
