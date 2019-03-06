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
    pub fn new(canvas_id: &str) -> Self {
        let r = Runner::new(canvas_id);
        Game {
            runner: Box::new(r),
        }
    }
}

pub fn load_images(sources: Vec<String>, callback: Box<(FnOnce(Vec<String>) -> ())>) {
    unimplemented!()
}

/**
 * Tracks various information about the canvases used, stats on the game, etc.
 */
pub struct Runner {
    stats: Stats,
    fps: u16,
    interval: f32,
    // TODO  dup of front.  Can we remove it?
    canvas: Box<CanvasElement>,
    pub width: i32,
    pub height: i32,
    front: CanvasElement,
    front_width: i32,
    front_height: i32,
    back: CanvasElement,
    back_width: i32,
    back_height: i32,
    front_2d: CanvasRenderingContext2d,
    back_2d: CanvasRenderingContext2d,
}

impl Runner {
    pub fn new(canvas_id: &str) -> Runner {
        let fps = 60;
        let canvas: CanvasElement = document()
            .get_element_by_id(canvas_id)
            .unwrap()
            .try_into()
            .unwrap();
        let canvas_width: i32 = canvas.offset_width();
        let canvas_height: i32 = canvas.offset_height();
        let r = Runner {
            stats: Stats::new(),
            fps: fps,
            interval: 1000.0 / fps as f32,
            canvas: Box::new(canvas),
            width: canvas_width,
            height: canvas_height,
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
