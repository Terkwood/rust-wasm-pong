#[cfg(not(target_arch = "wasm32"))]
extern crate ggez;
#[cfg(target_arch = "wasm32")]
extern crate good_web_game as ggez;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate stdweb;

mod ball;
mod constants;
mod court;
mod level;
mod paddle;
mod player;
mod score;
mod state;

use ggez::{event, GameResult};

use crate::constants::*;
use crate::score::Score;
use crate::state::MainState;

#[cfg(target_arch = "wasm32")]
fn main() -> GameResult {
    use ggez::conf;

    good_web_game::start(
        conf::Conf {
            cache: conf::Cache::List(vec![
                BALL_IMAGE_FILE,
                PADDLE_IMAGE_FILE,
                PRESS1_IMAGE_FILE,
                PRESS2_IMAGE_FILE,
                WINNER_IMAGE_FILE,
                BLOCK_IMAGE_FILE,
            ]),
            ..Default::default()
        },
        |mut context| {
            let state = MainState::new(&mut context);
            event::run(context, state)
        },
    )
}

// LEGACY "MAGIC"
/*
struct Defaults {
    width: u32,
    height: u32,
    wall_width: u32,
    paddle_width: u32,
    paddle_height: u32,
    paddle_speed: f32,
    ball_speed: f32,
    ball_accel: f32,
    ball_radius: u32,
    sound: bool,
}

impl Default for Defaults {
    fn default() -> Self {
        Defaults {
            width: 640, // logical canvas width (browser will scale to physical canvas size - which is controlled by @media css queries)
            height: 480, // logical canvas height (ditto)
            wall_width: 12,
            paddle_width: 12,
            paddle_height: 180,
            paddle_speed: 1.5, // should be able to cross court vertically   in 2 seconds
            ball_speed: 4.0, // should be able to cross court horizontally in 4 seconds, at starting speed ...
            ball_accel: 8.0, // ... but accelerate as time passes
            ball_radius: 10,
            sound: true,
        }
    }
}

struct Colors {
    walls: String,
    ball: String,
    score: String,
    footprint: String,
    prediction_guess: String,
    prediction_exact: String,
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            walls: "white".to_owned(),
            ball: "white".to_owned(),
            score: "white".to_owned(),
            footprint: "#333".to_owned(),
            prediction_guess: "yellow".to_owned(),
            prediction_exact: "red".to_owned(),
        }
    }
}


#[derive(Clone)]
pub struct Cfg {
    stats: bool,
    footprints: bool,
    predictions: bool,
    sound: bool,
}
impl Default for Cfg {
    fn default() -> Cfg {
        Cfg {
            stats: true,
            footprints: false,
            predictions: false,
            sound: false,
        }
    }
}

#[derive(Clone)]
pub struct Pong {
    cfg: Cfg,
    runner: Box<Runner>,
    width: u32,
    height: u32,
    playing: bool,
    score: Score,
    menu: Box<Menu>,
    court: Court,
    left_paddle: Box<Paddle>,
    right_paddle: Box<Paddle>,
    ball: Ball,
    sounds: Box<Sounds>,
}

impl Pong {

    fn draw(self, ctx: &CanvasRenderingContext2d) {
        self.court.draw(ctx, self.score);
        self.left_paddle.draw(ctx);
        self.right_paddle.draw(ctx);
        if self.playing {
            self.ball.draw(ctx);
        } else {
            self.menu.draw(ctx);
        }
    }

    fn show_stats(mut self, on: bool) {
        self.cfg.stats = on;
    }

    fn show_footprints(mut self, on: bool) {
        self.cfg.footprints = on;
        self.ball.footprints = vec![];
    }

    fn show_predictions(mut self, on: bool) {
        self.cfg.predictions = on;
    }

    fn enable_sound(mut self, on: bool) {
        self.cfg.sound = on;
    }
}


//=============================================================================
// MENU
//=============================================================================

#[derive(Clone)]
struct Menu {}

impl Menu {
    pub fn new() -> Menu {
        //TODO punted
        Menu {}
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        unimplemented!()
    }

    pub fn declare_winner(&self, player: Player) {
        unimplemented!()
    }
}

//=============================================================================
// SOUNDS
//=============================================================================

#[derive(Clone)]
struct Sounds {}
impl Sounds {
    pub fn new() -> Sounds {
        // TODO punted
        Sounds {}
    }

    pub fn goal(&self) {
        unimplemented!()
    }

    pub fn ping(&self) {
        unimplemented!()
    }

    pub fn pong(&self) {
        unimplemented!()
    }

    pub fn wall(&self) {
        unimplemented!()
    }
}

*/
