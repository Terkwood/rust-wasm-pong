#![feature(custom_attribute)]
extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    window,
    CanvasRenderingContext2d
};

#[js_export]
pub fn hello() -> String {
    "Hello".to_string()
}

fn main() {
    stdweb::initialize();
    stdweb::event_loop();
}