use stdweb::traits::*;
use stdweb::unstable::TryInto;

use stdweb::web::event::KeyDownEvent;
use stdweb::web::html_element::CanvasElement;

use stdweb::web::{document, window, CanvasRenderingContext2d};


/**
 * Tracks various information about the canvases used, stats on the game, etc.
 */
#[derive(Clone)]
pub struct Runner {
    stats: Stats,
    fps: u16,
    interval: f32,
    stopped: bool,
    pub width: i32,
    pub height: i32,
    front_canvas: Box<CanvasElement>,
    front_canvas_2d: CanvasRenderingContext2d,
    back_canvas: Box<CanvasElement>,
    back_canvas_2d: CanvasRenderingContext2d,
}

impl Runner {
    pub fn new(front_canvas_id: &str, back_canvas_id: &str) -> Runner {
        let fps = 60;
        let front_canvas: CanvasElement = document()
            .get_element_by_id(front_canvas_id)
            .unwrap()
            .try_into()
            .unwrap();
        let canvas_width: i32 = front_canvas.offset_width();
        let canvas_height: i32 = front_canvas.offset_height();
        let back_canvas: CanvasElement = document()
            .get_element_by_id(back_canvas_id)
            .unwrap()
            .try_into()
            .unwrap();
        let f2d = front_canvas.get_context().unwrap();
        let b2d = back_canvas.get_context().unwrap();
        let runner = Runner {
            stats: Stats::new(),
            fps: fps,
            interval: 1000.0 / fps as f32,
            stopped: false,
            front_canvas: Box::new(front_canvas),
            width: canvas_width,
            height: canvas_height,
            back_canvas: Box::new(back_canvas),
            front_canvas_2d: f2d,
            back_canvas_2d: b2d,
        };

        runner
    }

    pub fn confirm(&self, _arg: &str) -> bool {
        unimplemented!()
    }

    /**
     * game instance should call runner.start() when its finished initializing
     * and is ready to start the game loop.
     */
    pub fn start(&'static self) {
        // TODO need to be mut
        // self.stopped = false;
        // self.game_loop()
        // TODO self.last_frame = unimplemented!();
        // TODO self.timer = unimplemented!();
    }

    pub fn stop(&'static mut self) {}

    /**
     * javascript `alert` blocks the thread, so we need
     * to stop the game loop before calling it.
     */
    fn alert(&'static self, msg: &str) {
        // TODO need to be mut
        // self.stopped = true;
        let r = window().alert(msg);
        self.start();
        r
    }
}

/**
 * @frame update + draw
 */
#[derive(Clone)]
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
