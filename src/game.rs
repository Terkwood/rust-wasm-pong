use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::{CanvasElement, ImageElement};
use stdweb::web::{document, window, CanvasRenderingContext2d};

pub fn add_document_event() -> stdweb::Value {
    js! {
        //var obj = @{}
    }
}

// TODO: type of cb data is wrong, should be like a js dict with {src:..., image:...}
pub fn load_images(sources: Vec<String>, cb: Box<(FnOnce(Vec<String>) -> ())>) {
    js! {
        /* load multiple images and callback when ALL have finished loading */
        var images = {};
        var count = @{sources.len() as u32};
        // TODO may need Yew callback magic for this
        //var callback = @{cb};
        if (count == 0) {
            // TODO
            //callback (images);
        } else {
            for (var n = 0; n < sources.length; n++) {
                var source = sources[n];
                var image = document.createElement ("img");
                images[source] = image;
                Game.addEvent (image, "load", function () {
                    // TODO
                    //if (--count == 0) callback (images);
                });
                image.src = source;
            }
        }

        true
    }
}

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
    pub fn new(front_canvas_id: &str, back_canvas_id: &str) -> Self {
        let r = Runner::new(front_canvas_id, back_canvas_id);
        Game {
            runner: Box::new(r),
        }
    }
}

/**
 * Tracks various information about the canvases used, stats on the game, etc.
 */
pub struct Runner {
    stats: Stats,
    fps: u16,
    interval: f32,
    // TODO  dup of front.  Can we remove it?
    front_canvas: Box<CanvasElement>,
    pub width: i32,
    pub height: i32,
    back_canvas: Box<CanvasElement>,
    front_canvas_2d: CanvasRenderingContext2d,
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
        let r = Runner {
            stats: Stats::new(),
            fps: fps,
            interval: 1000.0 / fps as f32,
            front_canvas: Box::new(front_canvas),
            width: canvas_width,
            height: canvas_height,
            back_canvas: Box::new(back_canvas),
            front_canvas_2d: f2d,
            back_canvas_2d: b2d,
        };

        // TODO addEvents (keydown, keyup)
        unimplemented!();

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
