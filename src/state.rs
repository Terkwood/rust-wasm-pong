use ggez::{event, graphics, Context, GameResult};

use crate::ball::Ball;
use crate::constants::*;
use crate::court::Court;
use crate::paddle::Paddle;
use crate::player::Player;
use crate::score::Score;

pub struct MainState {
    score: Score,
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    court: Court,
    images: Images,
    last_frame: f64,
    playing: bool,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        let (size_x, size_y) = canvas_size(ctx);
        let mut state = MainState {
            score: Score::new(),
            court: Court::new(
                graphics::Image::new(ctx, BLOCK_IMAGE_FILE).unwrap(),
                size_x,
                size_y,
            ),
            left_paddle: Paddle::new(
                graphics::Image::new(ctx, PADDLE_IMAGE_FILE).unwrap(),
                size_x,
                size_y,
                false,
            ),
            right_paddle: Paddle::new(
                graphics::Image::new(ctx, PADDLE_IMAGE_FILE).unwrap(),
                size_x,
                size_y,
                true,
            ),
            ball: Ball::new(
                graphics::Image::new(ctx, BALL_IMAGE_FILE).unwrap(),
                size_x,
                size_y,
            ),
            images: Images {
                press1: graphics::Image::new(ctx, PRESS1_IMAGE_FILE).unwrap(),
                press2: graphics::Image::new(ctx, PRESS2_IMAGE_FILE).unwrap(),
                winner: graphics::Image::new(ctx, WINNER_IMAGE_FILE).unwrap(),
            },
            playing: false,
            last_frame: timestamp(),
        };

        state.start_bots();
        state
    }

    fn _draw_instructions(&mut self, ctx: &mut Context, size_x: f32, size_y: f32) {
        graphics::draw(
            ctx,
            &self.images.press1,
            graphics::DrawParam::default()
                .dest([size_x as f32 * 0.05, size_y as f32 * 0.05])
                .scale([1., 1.]),
        )
        .unwrap();

        // line up the right edge with 95% of the screen's width
        let mostly_right = size_x as f32 * 0.95;
        graphics::draw(
            ctx,
            &self.images.press2,
            graphics::DrawParam::default()
                .dest([
                    mostly_right - self.images.press2.width() as f32,
                    size_y as f32 * 0.05,
                ])
                .scale([1., 1.]),
        )
        .unwrap();
    }

    fn start_bots(&mut self) {
        self.start(0)
    }

    fn _start_single_player(&mut self) {
        self.start(1)
    }

    fn _start_double_player(&mut self) {
        self.start(2)
    }

    fn start(&mut self, num_players: u32) {
        if !self.playing {
            self.score = Score::new();
            self.playing = true;
            self.left_paddle
                .set_auto(num_players < 1, Some(level(self.score, Player::One)));
            self.right_paddle
                .set_auto(num_players < 2, Some(level(self.score, Player::Two)));
            self.ball.reset(None);
            // TODO self._hide_cursor();
        }
    }

    fn stop(&mut self, ask: bool) {
        if self.playing && (!ask || self.alert("Abandon game in progress?")) {
            self.playing = false;
            self.left_paddle.set_auto(false, None);
            self.right_paddle.set_auto(false, None);
            // TODO self._show_cursor();
        }
    }

    fn alert(&self, msg: &str) -> bool {
        unimplemented!()
    }

    fn goal(&mut self, player: Player) {
        console!(log, format!("🥅 {:?} GOAL 🥅", player));
        // TODO self.sounds.goal();
        self.score = Score::incr(self.score, player);

        if self.score.of(player) == 9 {
            //self.menu.declare_winner(player);
            self.stop(false);
            console!(log, "🏆 W I N N E R 🏆");
        } else {
            self.ball.reset(Some(player));
            self.left_paddle.set_level(level(self.score, Player::One));
            self.right_paddle.set_level(level(self.score, Player::Two));
        }
    }

    fn _hide_cursor(&self) {
        unimplemented!()
    }

    fn _show_cursor(&self) {
        unimplemented!()
    }
}

fn level(score: Score, player: Player) -> u32 {
    let x = score.of(player);
    let y = score.of(player.other());
    (8 + (x as i32 - y as i32)) as u32
}

fn timestamp() -> f64 {
    stdweb::web::Date::now()
}

fn canvas_size(ctx: &Context) -> (f32, f32) {
    let (x, y) = ctx.gfx_context.canvas_context.size();
    (x as f32, y as f32)
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let start = timestamp();
        let dt_secs = (start - self.last_frame) as f32 / 1000.0;
        let (_, game_width) = canvas_size(ctx);
        self.left_paddle.update(dt_secs, &self.ball, game_width);
        self.right_paddle.update(dt_secs, &self.ball, game_width);
        if self.playing {
            let _dx = self.ball.dx;
            let _dy = self.ball.dy;
            self.ball
                .update(dt_secs, &self.left_paddle, &self.right_paddle);
            // TODO sounds
            /*  if (self.ball.dx < 0 && dx > 0) self.sounds.ping ();
            else if (self. ball.dx > 0 && dx < 0) self. sounds.pong ();
            else if (self. ball.dy * dy < 0) self. sounds.wall (); */

            let (game_width, _) = canvas_size(ctx);
            if self.ball.left > game_width {
                self.goal(Player::One)
            } else if self.ball.right < 0.0 {
                self.goal(Player::Two)
            }
        }

        self.last_frame = start;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: &str) {
        match key {
            "Escape" => console!(log, "ESC"),
            "Digit0" => console!(log, "0"),
            "Digit1" => console!(log, "1"),
            "Digit2" => console!(log, "2"),
            "KeyQ" => self.left_paddle.move_up(),
            "KeyA" => self.left_paddle.move_down(),
            "KeyP" => self.right_paddle.move_up(),
            "KeyL" => self.right_paddle.move_down(),
            &_ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, key: &str) {
        match key {
            "KeyQ" => self.left_paddle.stop_moving_up(),
            "KeyA" => self.left_paddle.stop_moving_down(),
            "KeyP" => self.right_paddle.stop_moving_up(),
            "KeyL" => self.right_paddle.stop_moving_down(),
            &_ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        self.court.draw(ctx, self.score);
        self.left_paddle.draw(ctx);
        self.right_paddle.draw(ctx);
        if self.playing {
            self.ball.draw(ctx);
        } else {
            // TODO self.menu.draw (ctx);
        }

        let (size_x, size_y) = canvas_size(ctx);
        // TODO self._draw_instructions(ctx, size_x, size_y);

        graphics::draw(
            ctx,
            &ggez::graphics::Text::new(
                "Perma-Bot Mode 🤖\n".to_string()
                    + &format!("Res {} x {}\n", size_x, size_y)
                    + &format!("Timestamp {:04}\n", self.last_frame as u64 % 10000),
            ),
            graphics::DrawParam::default()
                .dest([size_x as f32 * 0.75, size_y as f32 * 0.85])
                .scale([1.5, 1.5]),
        )
        .unwrap();

        graphics::present(ctx)
    }
}

pub struct Images {
    press1: graphics::Image,
    press2: graphics::Image,
    winner: graphics::Image,
}
