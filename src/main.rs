#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate stdweb;

mod game;

use game::Runner;

fn main() {
    stdweb::initialize();
    let message = "PING 🏓 PONG 🏓";
    js! {
        alert( @{message} );
    }
    stdweb::event_loop();
}

//=============================================================================
// PONG
//=============================================================================

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

lazy_static! {
    pub static ref IMAGES: Vec<String> = vec![
        "images/press1.png".to_owned(),
        "images/press2.png".to_owned(),
        "images/winner.png".to_owned(),
    ];
}

pub struct Level {
    ai_reaction: f32,
    ai_error: u32,
}

lazy_static! {
    pub static ref LEVELS: Vec<Level> = vec! [
        Level{ai_reaction: 0.2, ai_error: 40}, // 0:  ai is losing by 8
        Level{ai_reaction: 0.3, ai_error: 50}, // 1:  ai is losing by 7
        Level{ai_reaction: 0.4, ai_error: 60}, // 2:  ai is losing by 6
        Level{ai_reaction: 0.5, ai_error: 70}, // 3:  ai is losing by 5
        Level{ai_reaction: 0.6, ai_error: 80}, // 4:  ai is losing by 4
        Level{ai_reaction: 0.7, ai_error: 90}, // 5:  ai is losing by 3
        Level{ai_reaction: 0.8, ai_error: 100}, // 6:  ai is losing by 2
        Level{ai_reaction: 0.9, ai_error: 110}, // 7:  ai is losing by 1
        Level{ai_reaction: 1.0, ai_error: 120}, // 8:  tie
        Level{ai_reaction: 1.1, ai_error: 130}, // 9:  ai is winning by 1
        Level{ai_reaction: 1.2, ai_error: 140}, // 10: ai is winning by 2
        Level{ai_reaction: 1.3, ai_error: 150}, // 11: ai is winning by 3
        Level{ai_reaction: 1.4, ai_error: 160}, // 12: ai is winning by 4
        Level{ai_reaction: 1.5, ai_error: 170}, // 13: ai is winning by 5
        Level{ai_reaction: 1.6, ai_error: 180}, // 14: ai is winning by 6
        Level{ai_reaction: 1.7, ai_error: 190}, // 15: ai is winning by 7
        Level{ai_reaction: 1.8, ai_error: 200}, // 16: ai is winning by 8
    ];
}

struct Cfg {}
struct Menu {}
struct Court {}
struct Paddle {}
struct Ball {}
struct Sounds {}

struct Pong {
    cfg: Cfg,
    runner: Runner,
    width: u32,
    height: u32,
    images: Vec<String>,
    playing: bool,
    scores: (u32, u32),
    menu: Menu,
    court: Court,
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    sounds: Sounds,
}

impl Pong {
    fn initialize(mut self, runner: Runner, cfg: Cfg) {
        let cb = move |images| {
            self.cfg = cfg;
            self.runner = runner.clone();
            self.width = runner.width;
            self.height = runner.height;
            self.images = images;
            self.playing = false;
            self.scores = (0, 0);
            self.menu = unimplemented!();
            self.court = unimplemented!();
            self.left_paddle = unimplemented!();
            self.right_paddle = unimplemented!();
            self.ball = unimplemented!();
            self.sounds = unimplemented!();
            self.runner.start();
        };
        game::load_images(IMAGES.to_vec(), Box::new(cb))
    }
}
