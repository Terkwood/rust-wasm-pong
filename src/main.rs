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
mod menu;
mod paddle;
mod player;
mod score;
mod sounds;
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
            cache: conf::Cache::List(vec![BALL_IMAGE_FILE, PADDLE_IMAGE_FILE, BLOCK_IMAGE_FILE]),
            ..Default::default()
        },
        |mut context| {
            let state = MainState::new(&mut context);
            event::run(context, state)
        },
    )
}

// TODO
/*

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

*/
