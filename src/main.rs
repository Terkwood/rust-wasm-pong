#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate stdweb;

mod game;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d};

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

pub struct Cfg {
    stats: bool,
    footprints: bool,
    predictions: bool,
    sound: bool,
}

struct Pong {
    cfg: Cfg,
    runner: Box<Runner>,
    width: u32,
    height: u32,
    images: Vec<String>,
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
    fn initialize(mut self, runner: Box<Runner>, cfg: Cfg) {
        let cb = move |images| {
            let w = runner.width;
            let h = runner.height;
            self.cfg = cfg;
            self.runner = Box::from(runner);
            self.width = w;
            self.height = h;
            self.images = images;
            self.playing = false;
            self.score = Score::new();
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

    fn start_demo(self) {
        self.start(0)
    }

    fn start_single_player(self) {
        self.start(1)
    }

    fn start_double_player(self) {
        self.start(2)
    }

    fn start(mut self, num_players: u32) {
        if (!self.playing) {
            self.score = Score::new();
            self.playing = true;
            self.left_paddle.set_auto(num_players < 1, unimplemented!());
            self.right_paddle
                .set_auto(num_players < 2, unimplemented!());
            self.ball.reset(None);
            self.runner.hide_cursor();
        }
    }

    fn stop(mut self, ask: bool) {
        if self.playing && (!ask || self.runner.confirm("Abandon game in progress?")) {
            self.playing = false;
            self.left_paddle.set_auto(false, None);
            self.right_paddle.set_auto(false, None);
            self.runner.show_cursor();
        }
    }

    fn goal(self, player: Player) {
        self.sounds.goal();
        self.score.incr(player);
        if self.score.of(player) == 9 {
            self.menu.declare_winner(player);
            self.stop(false);
        } else {
            self.ball.reset(Some(player));
            self.left_paddle.set_level(level(self.score, Player::One));
            self.right_paddle.set_level(level(self.score, Player::Two));
        }
    }

    fn update(self, dt: i32) {
        self.left_paddle.update(dt, &self.ball);
        self.right_paddle.update(dt, &self.ball);
        if self.playing {
            let dx = self.ball.dx;
            let dy = self.ball.dy;
            self.ball.update(dt, &self.left_paddle, &self.right_paddle);
            if self.ball.dx < 0 && dx > 0 {
                self.sounds.ping()
            } else if self.ball.dx > 0 && dx < 0 {
                self.sounds.pong()
            } else if self.ball.dy * dy < 0 {
                self.sounds.wall();
            };

            if self.ball.left > self.width as i32 {
                self.goal(Player::One)
            } else if self.ball.right < 0 {
                self.goal(Player::Two)
            }
        }
    }

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

    fn onkeydown(key_code: u16) {
        match key_code {
            _ => unimplemented!(),
        }
    }

    fn onkeyup(key_code: u16) {
        match key_code {
            _ => unimplemented!(),
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

fn level(score: Score, player: Player) -> u32 {
    let x = score.of(player);
    let y = score.of(player.other());
    8 + (x - y)
}

#[derive(Copy, Clone)]
struct Score(u32, u32);
impl Score {
    pub fn new() -> Score {
        Score(0, 0)
    }

    pub fn of(self, player: Player) -> u32 {
        match player {
            Player::One => self.0,
            Player::Two => self.1,
        }
    }

    pub fn incr(mut self, player: Player) {
        match player {
            Player::One => self.0 = self.0 + 1,
            Player::Two => self.1 = self.1 + 1,
        }
    }
}

#[derive(Copy, Clone)]
enum Player {
    One,
    Two,
}
impl Player {
    pub fn other(self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

//=============================================================================
// MENU
//=============================================================================

struct Menu {}

impl Menu {
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

struct Sounds {}
impl Sounds {
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

//=============================================================================
// COURT
//=============================================================================

struct Court {}
impl Court {
    pub fn draw(&self, ctx: &CanvasRenderingContext2d, score: Score) {
        unimplemented!()
    }
}

//=============================================================================
// PADDLE
//=============================================================================

struct Paddle {}
impl Paddle {
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        unimplemented!()
    }

    pub fn set_auto(&self, on: bool, level: Option<u32>) {
        unimplemented!()
    }

    pub fn set_level(&self, level: u32) {
        unimplemented!()
    }

    pub fn update(&self, dt: i32, ball: &Ball) {
        unimplemented!()
    }
}

//=============================================================================
// BALL
//=============================================================================

struct Ball {
    left: i32,
    right: i32,
    dx: i32,
    dy: i32,
    footprints: Vec<bool>,
}
impl Ball {
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        unimplemented!()
    }

    pub fn reset(&self, player: Option<Player>) {
        unimplemented!()
    }

    pub fn update(&self, dt: i32, left: &Paddle, right: &Paddle) {}
}

//=============================================================================
// HELPER
//=============================================================================
