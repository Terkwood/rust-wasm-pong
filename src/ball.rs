use ggez::{graphics, Context};
use rand::Rng;

use crate::constants::{BALL_ACCEL, BALL_RADIUS, BALL_SPEED, BLOCK_LENGTH_TO_SCREEN_HEIGHT};
use crate::paddle;
use crate::paddle::Paddle;
use crate::player::Player;

#[derive(Clone)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub footprints: Vec<bool>,
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub radius: f32,
    pub speed: f32,
    pub accel: f32,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub dx_changed: bool,
    pub dy_changed: bool,
    pub image: ggez::graphics::Image,
}
impl Ball {
    pub fn new(image: ggez::graphics::Image, game_width: f32, game_height: f32) -> Ball {
        let max_x = game_width - BALL_RADIUS;
        let min_x = BALL_RADIUS;
        let bh = BLOCK_LENGTH_TO_SCREEN_HEIGHT * game_height;
        let ball = Ball {
            radius: BALL_RADIUS,
            min_x,
            max_x,
            min_y: bh + BALL_RADIUS,
            max_y: game_height - bh - BALL_RADIUS,
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
            x: 0.0,
            y: 0.0,
            dx: 0.0,
            dy: 0.0,
            dx_changed: false,
            dy_changed: false,
            footprints: vec![],
            speed: (max_x - min_x) / BALL_SPEED,
            accel: BALL_ACCEL,
            image,
        };

        ball
    }

    pub fn draw(&self, ctx: &mut Context) {
        let w = self.radius * 2.0;
        let h = w;

        graphics::draw(
            ctx,
            &self.image,
            graphics::DrawParam::default()
                .dest([self.x - self.radius, self.y - self.radius])
                .scale([
                    w / self.image.width() as f32,
                    h / self.image.height() as f32,
                ]),
        )
        .unwrap();
        if !self.footprints.is_empty() {
            let max = self.footprints.len();
            // TODO ctx.strokeStyle = Pong.Colors.footprint;
            for _n in 0..max {
                // TODO
                //   ctx.strokeRect (
                //     self.footprints[n].x - self.radius,
                //     self.footprints[n].y - self.radius,
                //     w,
                //     h
                //   );
            }
        }
    }

    pub fn reset(&mut self, player: Option<Player>) {
        self.footprints = vec![];
        let mut rng = rand::thread_rng();
        self.set_pos(
            match player.unwrap_or(Player::One) {
                Player::One => self.min_x,
                Player::Two => self.max_x,
            },
            rng.gen_range(self.min_y, self.max_y),
        );

        self.set_dir(
            match player.unwrap_or(Player::One) {
                Player::One => self.speed,
                Player::Two => -self.speed,
            },
            self.speed,
        )
    }

    fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.left = x - self.radius;
        self.top = y - self.radius;
        self.right = x + self.radius;
        self.bottom = y + self.radius;
    }

    fn set_dir(&mut self, dx: f32, dy: f32) {
        self.dx_changed = (self.dx < 0.0) != (dx < 0.0); // did horizontal direction change
        self.dy_changed = (self.dy < 0.0) != (dy < 0.0); // did vertical direction change
        self.dx = dx;
        self.dy = dy;
    }

    pub fn update(&mut self, dt: f32, left_paddle: &Paddle, right_paddle: &Paddle) {
        let mut pos = Ball::accelerate(self.x, self.y, self.dx, self.dy, self.accel, dt);

        if pos.dy > 0.0 && pos.y > self.max_y {
            pos.y = self.max_y;
            pos.dy = -pos.dy;
        } else if pos.dy < 0.0 && pos.y < self.min_y {
            pos.y = self.min_y;
            pos.dy = -pos.dy;
        }

        let paddle = if pos.dx < 0.0 {
            left_paddle
        } else {
            right_paddle
        };

        if let Some(pt) = Ball::intercept(self, paddle::Rect::from(paddle.clone()), pos.nx, pos.ny)
        {
            match pt.d {
                Side::Left | Side::Right => {
                    pos.x = pt.x;
                    pos.dx = -pos.dx;
                }
                Side::Top | Side::Bottom => {
                    pos.y = pt.y;
                    pos.dy = -pos.dy
                }
            }

            // add/remove spin based on paddle direction
            if paddle.up != 0.0 {
                pos.dy = pos.dy * if pos.dy < 0.0 { 0.5 } else { 1.5 };
            } else if paddle.down != 0.0 {
                pos.dy = pos.dy * if pos.dy > 0.0 { 0.5 } else { 1.5 };
            }
        }

        self.set_pos(pos.x, pos.y);
        self.set_dir(pos.dx, pos.dy);
        // TODO self.footprint();
    }

    fn accelerate(x: f32, y: f32, dx: f32, dy: f32, accel: f32, dt_secs: f32) -> BallPosition {
        let x2 = x + dt_secs * dx + accel * dt_secs * dt_secs * 0.5;
        let y2 = y + dt_secs * dy + accel * dt_secs * dt_secs * 0.5;
        let dx2 = dx + accel * dt_secs * if dx > 0.0 { 1.0 } else { -1.0 };
        let dy2 = dy + accel * dt_secs * if dy > 0.0 { 1.0 } else { -1.0 };
        BallPosition {
            nx: x2 - x,
            ny: y2 - y,
            x: x2,
            y: y2,
            dx: dx2,
            dy: dy2,
        }
    }

    pub fn intercept(ball: &Ball, paddle: paddle::Rect, nx: f32, ny: f32) -> Option<BallIntercept> {
        fn solve(
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32,
            x3: f32,
            y3: f32,
            x4: f32,
            y4: f32,
            d: Side,
        ) -> Option<BallIntercept> {
            let denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
            if denom != 0.0 {
                let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denom;
                if ua >= 0.0 && ua <= 1.0 {
                    let ub = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denom;
                    if ub >= 0.0 && ub <= 1.0 {
                        let x = x1 + ua * (x2 - x1);
                        let y = y1 + ua * (y2 - y1);
                        return Some(BallIntercept { x, y, d });
                    }
                }
            }

            None
        }

        let mut pt = None;

        if nx < 0.0 {
            pt = solve(
                ball.x,
                ball.y,
                ball.x + nx,
                ball.y + ny,
                paddle.right + ball.radius,
                paddle.top - ball.radius,
                paddle.right + ball.radius,
                paddle.bottom + ball.radius,
                Side::Right,
            );
        } else if nx > 0.0 {
            pt = solve(
                ball.x,
                ball.y,
                ball.x + nx,
                ball.y + ny,
                paddle.left - ball.radius,
                paddle.top - ball.radius,
                paddle.left - ball.radius,
                paddle.bottom + ball.radius,
                Side::Left,
            )
        }

        if pt.is_none() {
            if ny < 0.0 {
                pt = solve(
                    ball.x,
                    ball.y,
                    ball.x + nx,
                    ball.y + ny,
                    paddle.left - ball.radius,
                    paddle.bottom + ball.radius,
                    paddle.right + ball.radius,
                    paddle.bottom + ball.radius,
                    Side::Bottom,
                );
            } else if ny > 0.0 {
                pt = solve(
                    ball.x,
                    ball.y,
                    ball.x + nx,
                    ball.y + ny,
                    paddle.left - ball.radius,
                    paddle.top - ball.radius,
                    paddle.right + ball.radius,
                    paddle.top - ball.radius,
                    Side::Top,
                );
            }
        }

        pt
    }
}

pub struct BallPosition {
    nx: f32,
    ny: f32,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

pub struct BallIntercept {
    pub x: f32,
    pub y: f32,
    pub d: Side,
}

pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}
