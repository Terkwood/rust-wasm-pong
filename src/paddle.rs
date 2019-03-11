use ggez::{graphics, Context};
use rand::prelude::*;

use crate::ball::{Ball, BallIntercept};
use crate::constants::*;
use crate::level::{Level, LEVELS};

#[derive(Clone)]
pub struct Paddle {
    pub auto: bool,
    pub width: f32,
    pub height: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub speed: f32,
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
    pub x: f32,
    pub y: f32,
    pub down: f32,
    pub up: f32,
    pub image: ggez::graphics::Image,
    pub level: Option<Level>,
    pub prediction: Option<Prediction>,
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl From<Paddle> for Rect {
    fn from(paddle: Paddle) -> Rect {
        Rect {
            top: paddle.top,
            bottom: paddle.bottom,
            left: paddle.left,
            right: paddle.right,
        }
    }
}

impl Paddle {
    pub fn new(
        image: ggez::graphics::Image,
        canvas_width: f32,
        canvas_height: f32,
        rhs: bool,
    ) -> Paddle {
        let paddle_height = canvas_height * PADDLE_HEIGHT_TO_SCREEN_HEIGHT;
        let mut paddle = Paddle {
            auto: false,
            level: None,
            width: canvas_width * PADDLE_WIDTH_TO_SCREEN_WIDTH,
            height: paddle_height,
            speed: 0.0,
            min_y: WALL_WIDTH,
            max_y: canvas_height - WALL_WIDTH - paddle_height,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
            top: 0.0,
            x: 0.0,
            y: 0.0,
            up: 0.0,
            down: 0.0,
            image,
            prediction: None,
        };
        paddle.speed = (paddle.max_y - paddle.min_y) / PADDLE_SPEED;
        paddle.set_pos(
            if rhs {
                canvas_width - paddle.width
            } else {
                0.0
            },
            paddle.min_y + (paddle.max_y - paddle.min_y) / 2.0,
        );

        paddle.set_dir(0.0);
        paddle
    }

    fn set_dir(&mut self, dy: f32) {
        self.up = if dy < 0.0 { -dy } else { 0.0 };
        self.down = if dy > 0.0 { dy } else { 0.0 };
    }

    fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.left = self.x;
        self.right = self.left + self.width;
        self.top = self.y;
        self.bottom = self.y + self.height;
    }

    pub fn draw(&self, ctx: &mut Context) {
        graphics::draw(
            ctx,
            &self.image,
            graphics::DrawParam::default()
                .dest([self.x, self.y])
                .scale([
                    self.width / self.image.width() as f32,
                    self.height / self.image.height() as f32,
                ]),
        )
        .unwrap()
    }

    pub fn move_down(&mut self) {
        self.down = 1.0;
    }

    pub fn move_up(&mut self) {
        self.up = 1.0;
    }

    pub fn set_auto(&mut self, on: bool, level: Option<u32>) {
        if on && !self.auto {
            self.auto = true;
            self.set_level(level.unwrap_or(0))
        } else if !on && self.auto {
            self.auto = false;
            self.set_dir(0.0);
        }
    }

    pub fn set_level(&mut self, level: u32) {
        if self.auto {
            self.level = Some(LEVELS[level as usize])
        }
    }

    pub fn stop_moving_down(&mut self) {
        self.down = 0.0;
    }

    pub fn stop_moving_up(&mut self) {
        self.up = 0.0;
    }

    pub fn update(&mut self, dt_secs: f32, ball: &Ball, game_width: f32) {
        if self.auto {
            self.bot(dt_secs, ball, game_width)
        }

        let amount: f32 = self.down - self.up;
        //console!(log,format!("Paddle update amount: {}", amount));
        if amount != 0.0 {
            let mut y = self.y + amount * dt_secs * self.speed;
            if y < self.min_y {
                y = self.min_y;
            } else if y > self.max_y {
                y = self.max_y
            };

            //console!(log,format!("Paddle set_pos: x {}, y {}", self.x, y));
            self.set_pos(self.x, y);
        }
    }

    fn bot(&mut self, dt_secs: f32, ball: &Ball, game_width: f32) {
        if (ball.x < self.left && ball.dx < 0.0) || (ball.x > self.right && ball.dx > 0.0) {
            self.stop_moving_up();
            self.stop_moving_down();
            return;
        }

        self.predict(ball, dt_secs, game_width);

        if let Some(p) = self.prediction {
            if p.y < self.top + self.height / 2.0 - 5.0 {
                self.stop_moving_down();
                self.move_up();
            } else if p.y > self.bottom - self.height / 2.0 + 5.0 {
                self.stop_moving_up();
                self.move_down();
            } else {
                self.stop_moving_up();
                self.stop_moving_down();
            }
        }
    }

    fn predict(&mut self, ball: &Ball, dt_secs: f32, game_width: f32) {
        // only re-predict if the ball changed direction, or its been some amount of time since last prediction
        if let Some(mut p) = self.prediction {
            if p.dx * ball.dx > 0.0
                && p.dy * ball.dy > 0.0
                && p.since < self.level.map(|l| l.ai_reaction).unwrap_or(0.0)
            {
                p.since = p.since + dt_secs;
                self.prediction = Some(p);
                return;
            }
        }

        let maybe_pt = Ball::intercept(
            ball,
            Rect {
                left: self.left,
                right: self.right,
                top: -10000.0,
                bottom: 10000.0,
            },
            ball.dx * 10.0,
            ball.dy * 10.0,
        );
        if let Some(mut pt) = maybe_pt {
            let t = self.min_y + ball.radius;
            let b = self.max_y + self.height - ball.radius;

            while pt.y < t || pt.y > b {
                if pt.y < t {
                    pt.y = t + (t - pt.y);
                } else if pt.y > b {
                    pt.y = t + (b - t) - (pt.y - b);
                }
            }

            self.prediction = Some(Prediction::from(pt));
        } else {
            self.prediction = None;
        }

        if let Some(mut p) = self.prediction {
            p.since = 0.0;
            p.dx = ball.dx;
            p.dy = ball.dy;
            p.radius = ball.radius;
            let closeness = if ball.dx < 0.0 {
                ball.x - self.right
            } else {
                self.left - ball.x
            } / game_width;
            // TODO is the unwrap_or ok ?
            let error = self.level.map(|l| l.ai_error).unwrap_or(0) as f32 * closeness;
            let mut rng = rand::thread_rng();
            p.y = p.y + rng.gen_range(-error, error);
            self.prediction = Some(p);
        }
    }
}

impl From<BallIntercept> for Prediction {
    fn from(bi: BallIntercept) -> Self {
        Prediction {
            x: bi.x,
            y: bi.y,
            ..Default::default()
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Prediction {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub since: f32,
    pub radius: f32,
}
