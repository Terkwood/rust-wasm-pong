use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use crate::Cfg;

struct Game {
    runner: Box<Runner>,
}

impl Game {
    /**
     * Create a new instance of the game, exposing methods relating
     * to canvas manipulation, HTML audio, receiving keyboard input,
     * etc.
     *
     * Renamed from `start` in the original version.
     */
    // TODO type for game arg
    pub fn new(id: u32, game: u32, cfg: Cfg) -> Self {
        unimplemented!()
    }
}

pub fn load_images(sources: Vec<String>, callback: Box<(FnOnce(Vec<String>) -> ())>) {
    unimplemented!()
}

pub struct Runner {
    pub cfg: Cfg,
    pub fps: u16,
    pub interval: f32,
    // TODO is this a dup of front?  Can we remove it?
    pub canvas: CanvasElement,
    pub width: u32,
    pub height: u32,
    front: CanvasElement,
    front_width: u32,
    front_height: u32,
    back: CanvasElement,
    back_width: u32,
    back_height: u32,
    front_2d: CanvasRenderingContext2d,
    back_2d: CanvasRenderingContext2d,
}

impl Runner {
    pub fn new(id: u32, game: u32, cfg: Cfg) -> Runner {
        unimplemented!()
    }

    pub fn confirm(&self, _arg: &str) -> bool {
        unimplemented!()
    }

    pub fn hide_cursor(&self) {
        unimplemented!()
    }

    pub fn show_cursor(&self) {
        unimplemented!()
    }

    pub fn start(&self) {
        unimplemented!()
    }
}
