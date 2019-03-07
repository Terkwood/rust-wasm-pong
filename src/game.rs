use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{KeyDownEvent, KeyUpEvent};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
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
#[derive(Clone)]
pub struct Runner {
    stats: Stats,
    fps: u16,
    interval: f32,
    pub width: i32,
    pub height: i32,
    front_canvas: Box<CanvasElement>,
    front_canvas_2d: CanvasRenderingContext2d,
    back_canvas: Box<CanvasElement>,
    back_canvas_2d: CanvasRenderingContext2d,
}

impl Runner {
    /**
     * Create a new `Runner` and attach keyup/keydown events.
     *
     * It's essential to use the `enclose!` macro to help clone
     * Runner when it's called for keyup events. See
     * https://github.com/koute/stdweb/blob/dff1e06086124fe79e3393a99ae8e2d424f5b2f1/examples/canvas/src/main.rs
     *
     * Also see these for info on handling key press:
     * - https://github.com/koute/stdweb/blob/8f40599d744b77a9dc6fe532951f6e16a2eae671/src/webapi/events/keyboard.rs#L229
     * - https://steemit.com/utopian-io/@tensor/rust-web-assembly-using-stdweb-to-build-a-client-side-application-with-rust-and-wasm
     */
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
            front_canvas: Box::new(front_canvas),
            width: canvas_width,
            height: canvas_height,
            back_canvas: Box::new(back_canvas),
            front_canvas_2d: f2d,
            back_canvas_2d: b2d,
        };

        window().add_event_listener(enclose!((runner) move |event: KeyDownEvent|
            runner.on_key_down(event)));

        window().add_event_listener(enclose!((runner) move |event: KeyUpEvent|
            runner.on_key_up(event)));

        runner
    }

    /**
     * See
     * - https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
     * - https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
     */
    fn on_key_down(&self, event: KeyDownEvent) {
        match event.code().as_ref() {
            // TODO missing link to game object for, e.g.
            // self.game.start_demo(),
            "Digit0" => unimplemented!(),
            "Digit1" => unimplemented!(),
            "Digit2" => unimplemented!(),
            "KeyA" => unimplemented!(),
            &_ => unimplemented!(),
        };
        event.prevent_default()
    }

    fn on_key_up(&self, event: KeyUpEvent) {
        match event.code() {
            _ => unimplemented!(),
        };
        event.prevent_default()
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
