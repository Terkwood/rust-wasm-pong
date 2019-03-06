use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};

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
     * Removed the `game` and `cfg` args from the original version.
     */
    pub fn new(id: u32) -> Self {
        let r = Runner::new(id);
        unimplemented!()
    }
}

pub fn load_images(sources: Vec<String>, callback: Box<(FnOnce(Vec<String>) -> ())>) {
    unimplemented!()
}

pub struct Runner {
    stats: Stats,
    fps: u16,
    interval: f32,
    // TODO is this a dup of front?  Can we remove it?
    canvas: CanvasElement,
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
    pub fn new(id: u32) -> Runner {
        let r = Runner {
            stats: Stats::new(),
            fps: 60,
            interval: unimplemented!(),
            canvas: unimplemented!(),
            width: unimplemented!(),
            height: unimplemented!(),
            front: unimplemented!(),
            front_width: unimplemented!(),
            front_height: unimplemented!(),
            back: unimplemented!(),
            back_width: unimplemented!(),
            back_height: unimplemented!(),
            front_2d: unimplemented!(),
            back_2d: unimplemented!(),
        };

        r
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

/**
 * @frame update + draw
 */
struct Stats {
    count: u32,
    fps: u16,
    update: u32,
    draw: u32,
    frame: u32,
}
impl Stats {
    pub fn new() -> Stats {
        Stats {
            count: 0,
            fps: 0,
            update: 0,
            draw: 0,
            frame: 0,
        }
    }
}
