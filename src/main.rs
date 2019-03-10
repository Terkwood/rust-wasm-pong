#![recursion_limit = "128"]

#[cfg(not(target_arch = "wasm32"))]
extern crate ggez;
#[cfg(target_arch = "wasm32")]
extern crate good_web_game as ggez;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::error::Error;

use stdweb::web::{document, window, CanvasRenderingContext2d};

use ggez::{event, graphics, Context, GameResult};

struct MainState {
    score: Score,
    left_paddle: Paddle,
    right_paddle: Paddle,
    images: Images,
    last_frame: f64,
    // TODO interval: f32,
    // TODO fps: u16,
}

const PADDLE_IMAGE_FILE: &str = "/paddle.png";
const PRESS1_IMAGE_FILE: &str = "/press1.png";
const PRESS2_IMAGE_FILE: &str = "/press2.png";
const WINNER_IMAGE_FILE: &str = "/winner.png";

const PADDLE_HEIGHT: f32 = 60.0;
/**
 * Paddle should be able to cross court vertically in 2 seconds
 */
const PADDLE_SPEED: f32 = 2.0;
const WALL_WIDTH: f32 = 12.0;
const BALL_RADIUS: f32 = 5.0;
/**
 * Ball should be able to cross court horizontally in 4 seconds,
 * at starting speed.
 */
const BALL_SPEED: f32 = 4.0;
/**
 * Accelerate as time passes.
 */
const BALL_ACCEL: f32 = 8.0;

impl MainState {
    fn new(ctx: &mut Context) -> MainState {
        let (size_x, size_y) = canvas_size(ctx);
        MainState {
            score: Score::new(),
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
            images: Images {
                press1: graphics::Image::new(ctx, PRESS1_IMAGE_FILE).unwrap(),
                press2: graphics::Image::new(ctx, PRESS2_IMAGE_FILE).unwrap(),
                winner: graphics::Image::new(ctx, WINNER_IMAGE_FILE).unwrap(),
            },
            last_frame: timestamp(),
        }
    }
}

fn timestamp() -> f64 {
    stdweb::web::Date::now()
}

fn canvas_size(ctx: &Context) -> (f32, f32) {
    let (x, y) = ctx.gfx_context.canvas_context.size();
    (x as f32, y as f32)
}
impl event::EventHandler for MainState {
    fn update(&mut self, _: &mut Context) -> GameResult {
        let start = timestamp();
        let dt_seconds = (start - self.last_frame) as f32 / 1000.0;
        self.left_paddle.update(dt_seconds);
        self.right_paddle.update(dt_seconds);

        // TODO differs from js impl, which assigns last_frame after drawing
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

        let (size_x, size_y) = canvas_size(ctx);

        graphics::draw(
            ctx,
            &self.images.press1,
            graphics::DrawParam::default()
                .dest([size_x as f32 * 0.05, size_y as f32 * 0.05])
                .scale([1., 1.]),
        )
        .unwrap();

        graphics::draw(
            ctx,
            &self.images.press2,
            graphics::DrawParam::default()
                .dest([size_x as f32 * 0.75, size_y as f32 * 0.05])
                .scale([1., 1.]),
        )
        .unwrap();

        self.left_paddle.draw(ctx);
        self.right_paddle.draw(ctx);

        graphics::draw(
            ctx,
            &ggez::graphics::Text::new(
                format!("Res {} x {}\n", size_x, size_y) + &format!("Frame {}\n", self.last_frame),
            ),
            graphics::DrawParam::default()
                .dest([size_x as f32 * 0.75, size_y as f32 * 0.90])
                .scale([1.5, 1.5]),
        )
        .unwrap();

        graphics::present(ctx)
    }
}

#[cfg(target_arch = "wasm32")]
fn main() -> GameResult {
    use ggez::conf;

    good_web_game::start(
        conf::Conf {
            cache: conf::Cache::List(vec![
                PADDLE_IMAGE_FILE,
                PRESS1_IMAGE_FILE,
                PRESS2_IMAGE_FILE,
                WINNER_IMAGE_FILE,
            ]),
            ..Default::default()
        },
        |mut context| {
            let state = MainState::new(&mut context);
            event::run(context, state)
        },
    )
}

struct Images {
    press1: graphics::Image,
    press2: graphics::Image,
    winner: graphics::Image,
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

    /*pub fn incr(mut self, player: Player) {
        match player {
            Player::One => self.0 = self.0 + 1,
            Player::Two => self.1 = self.1 + 1,
        }
    }*/
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

#[derive(Clone)]
struct Paddle {
    auto: bool,
    width: f32,
    height: f32,
    min_y: f32,
    max_y: f32,
    speed: f32,
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
    x: f32,
    y: f32,
    down: f32,
    up: f32,
    image: ggez::graphics::Image,
}

impl Paddle {
    pub fn new(
        image: ggez::graphics::Image,
        canvas_width: f32,
        canvas_height: f32,
        rhs: bool,
    ) -> Paddle {
        let mut paddle = Paddle {
            auto: false,
            width: 12.0,
            height: PADDLE_HEIGHT,
            speed: 0.0,
            min_y: WALL_WIDTH,
            max_y: canvas_height - WALL_WIDTH - PADDLE_HEIGHT,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
            top: 0.0,
            x: 0.0,
            y: 0.0,
            up: 0.0,
            down: 0.0,
            image,
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
                .scale([1., 1.]),
        )
        .unwrap()
    }

    pub fn move_down(&mut self) {
        self.down = 1.0;
    }

    pub fn move_up(&mut self) {
        self.up = 1.0;
    }

    pub fn set_auto(&self, on: bool, level: Option<u32>) {
        unimplemented!()
    }

    pub fn set_level(&self, level: u32) {
        unimplemented!()
    }

    pub fn stop_moving_down(&mut self) {
        self.down = 0.0;
    }

    pub fn stop_moving_up(&mut self) {
        self.up = 0.0;
    }

    pub fn update(&mut self, dt_secs: f32) {
        if self.auto {
            unimplemented!();
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
}

#[derive(Clone)]
struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    footprints: Vec<bool>,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    radius: f32,
    speed: f32,
    accel: f32,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    dx_changed: bool,
    dy_changed: bool,
}
impl Ball {
    pub fn new(game_width: u32, game_height: u32) -> Ball {
        let max_x = game_width as f32 - BALL_RADIUS;
        let min_x = BALL_RADIUS;
        let ball = Ball {
            radius: BALL_RADIUS,
            min_x,
            max_x,
            min_y: WALL_WIDTH + BALL_RADIUS,
            max_y: game_height as f32 - WALL_WIDTH - BALL_RADIUS,
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
            speed: max_x - min_x / BALL_SPEED,
            accel: BALL_ACCEL,
        };

        ball
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        unimplemented!()
    }

    pub fn reset(&mut self, player: Option<Player>) {
        self.footprints = vec![];
        self.set_pos(
            match player.unwrap_or(Player::One) {
                Player::One => self.min_x,
                Player::Two => self.max_x,
            },
            unimplemented!(),
        ); // TODO Game.random(this.minY, this.maxY)

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

        if let Some(pt) = Ball::intercept(self, paddle, pos.nx, pos.ny) {
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
        }
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

    fn intercept(ball: &Ball, paddle: &Paddle, nx: f32, ny: f32) -> Option<BallIntercept> {
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
struct BallPosition {
    nx: f32,
    ny: f32,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}
struct BallIntercept {
    x: f32,
    y: f32,
    d: Side,
}
enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

// LEGACY "MAGIC"

//mod game;
//use game::{Game, Runner};
/*
fn main() {
    stdweb::initialize();

    stdweb::event_loop();
}

pub fn log_wip() {
    console!(log, "PING 🏓 PONG 🏓");
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
    pub fn new(runner: Box<Runner>, cfg: Cfg) -> Pong {
        let w = runner.width as u32;
        let h = runner.height as u32;

        let pong = Pong {
            cfg: cfg,
            runner: runner,
            width: w,
            height: h,
            playing: false,
            score: Score::new(),
            menu: Box::new(Menu::new()),
            court: Court::new(),
            left_paddle: Box::new(Paddle::new()),
            right_paddle: Box::new(Paddle::new()),
            ball: Ball::new(),
            sounds: Box::new(Sounds::new()),
        };

        pong
    }

    fn start_demo(&mut self) {
        self.start(0)
    }

    fn start_single_player(&mut self) {
        self.start(1)
    }

    fn start_double_player(&mut self) {
        self.start(2)
    }

    fn start(&mut self, num_players: u32) {
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

    fn stop(&mut self, ask: bool) {
        if self.playing && (!ask || self.runner.confirm("Abandon game in progress?")) {
            self.playing = false;
            self.left_paddle.set_auto(false, None);
            self.right_paddle.set_auto(false, None);
            self.runner.show_cursor();
        }
    }

    fn goal(&mut self, player: Player) {
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

    fn update(&mut self, dt: i32) {
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

//=============================================================================
// COURT
//=============================================================================

#[derive(Clone)]
struct Court {}
impl Court {
    pub fn new() -> Court {
        //TODO punted
        Court {}
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, score: Score) {
        unimplemented!()
    }
}

*/
