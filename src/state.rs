use ggez::{event, graphics, Context, GameResult};

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::document;
use stdweb::web::html_element::CanvasElement;

use crate::ball::Ball;
use crate::constants::*;
use crate::court::Court;
use crate::menu::Menu;
use crate::paddle::Paddle;
use crate::player::Player;
use crate::score::Score;
use crate::sounds;

const WINNING_SCORE: u8 = 9;

pub struct MainState {
    menu: Menu,
    score: Score,
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    court: Court,
    last_frame: f64,
    playing: bool,
    canvas: CanvasElement,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        let (size_x, size_y) = canvas_size(ctx);
        let canvas: CanvasElement = document()
            .get_element_by_id("canvas")
            .unwrap()
            .try_into()
            .unwrap();

        MainState {
            menu: Menu::new(size_x, size_y),
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
            playing: false,
            last_frame: timestamp(),
            canvas,
        }
    }

    fn start_bots(&mut self) {
        self.start(0)
    }

    fn start_single_player(&mut self) {
        self.start(1)
    }

    fn start_double_player(&mut self) {
        self.start(2)
    }

    fn start(&mut self, num_players: u32) {
        if !self.playing {
            self.menu.reset();
            self.score = Score::new();
            self.playing = true;
            self.left_paddle
                .set_auto(num_players < 1, Some(level(self.score, Player::One)));
            self.left_paddle.reset_pos();
            self.right_paddle
                .set_auto(num_players < 2, Some(level(self.score, Player::Two)));
            self.right_paddle.reset_pos();
            self.ball.reset(None);
            self.hide_cursor();
        }
    }

    fn stop(&mut self) {
        if self.playing {
            self.playing = false;
            self.left_paddle.stop();
            self.right_paddle.stop();
            self.show_cursor();
        }
    }

    fn goal(&mut self, player: Player) {
        sounds::goal();
        self.score = Score::incr(self.score, player);

        if self.score.of(player) == WINNING_SCORE {
            self.menu.declare_winner(player);
            self.stop();
        } else {
            self.ball.reset(Some(player));
            self.left_paddle.set_level(level(self.score, Player::One));
            self.right_paddle.set_level(level(self.score, Player::Two));
        }
    }

    fn _draw_stats_text(&self, ctx: &mut Context) {
        let (size_x, size_y) = canvas_size(ctx);

        graphics::draw(
            ctx,
            &ggez::graphics::Text::new((
                format!("Res {} x {}\n", size_x, size_y)
                    + &format!("Timestamp {:04}\n", self.last_frame as u64 % 10000),
                graphics::Font(_STATS_FONT.to_string()),
                1.0,
            )),
            graphics::DrawParam::default()
                .color(TEXT_COLOR)
                .dest([size_x as f32 * 0.75, size_y as f32 * 0.85])
                .scale([1.5, 1.5]),
        )
        .unwrap();
    }

    fn hide_cursor(&self) {
        self.canvas.set_attribute("style", "cursor: none;").unwrap();
    }

    fn show_cursor(&mut self) {
        self.canvas.set_attribute("style", "cursor: auto;").unwrap();
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
            let dx = self.ball.dx;
            self.ball
                .update(dt_secs, &self.left_paddle, &self.right_paddle);

            if self.ball.dx < 0.0 && dx > 0.0 {
                sounds::ping();
            } else if self.ball.dx > 0.0 && dx < 0.0 {
                sounds::pong();
            }

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
            "Escape" => self.stop(),
            "Digit0" => self.start_bots(),
            "Digit1" => self.start_single_player(),
            "Digit2" => self.start_double_player(),
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
            self.menu.draw(ctx);
        }

        graphics::present(ctx)
    }
}
