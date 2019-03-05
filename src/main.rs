#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate stdweb;

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
