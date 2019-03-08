use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{KeyDownEvent, KeyUpEvent, ReadyStateChangeEvent};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{
    document, window, CanvasRenderingContext2d, MutationObserver, MutationObserverHandle,
    MutationObserverInit,
};

use crate::{Cfg, Pong};

// From webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}
macro_rules! enclose_mut {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let mut $x = $x.clone();)*
            $y
        }
    };
}

#[derive(Clone)]
pub struct Game {
    pong: Box<Pong>,
}

impl Game {
    /**
     * Execute this code when the DOM content is loaded.
     *
     * [See stdweb docs](https://docs.rs/stdweb/0.4.0/stdweb/web/struct.MutationObserver.html)
     */
    /*pub fn ready() -> MutationObserverHandle {
        let mo = MutationObserver::new(|_changes, _self| {
            js! {console.log("Mutating")}
        });

        /*let is_doc_ready: bool = js! {document.readyState === "complete"}.try_into().unwrap_or(false);
        if is_doc_ready {
            js! {console.log("READY!")}
        };*/

        mo.observe(
            &document().query_selector("#sidebar").unwrap().unwrap(),
            MutationObserverInit {
                attributes: true,
                child_list: true,
                subtree: true,
                attribute_filter: None,
                attribute_old_value: true,
                character_data: true,
                character_data_old_value: true,
            },
        )
        .unwrap();

        mo
    }*/

    /**
    * Create a new instance of the game, exposing methods relating
    * to canvas manipulation, HTML audio, receiving keyboard input,
    * etc.
    *
    * Renamed from `start` in the original version.
    * Removed the `game` and `cfg` args from the original version.
    *
    * It's essential to use the `enclose!` macro to help clone
    * Game when it's called for keyup events. See
    * https://github.com/koute/stdweb/blob/dff1e06086124fe79e3393a99ae8e2d424f5b2f1/examples/canvas/src/main.rs
    *
    * Also see these for info on handling key press:
    * - https://github.com/koute/stdweb/blob/8f40599d744b77a9dc6fe532951f6e16a2eae671/src/webapi/events/keyboard.rs#L229
    * - https://steemit.com/utopian-io/@tensor/rust-web-assembly-using-stdweb-to-build-a-client-side-application-with-rust-and-wasm

    */
    pub fn new(front_canvas_id: &str, back_canvas_id: &str) -> Self {
        js! {console.log("PING PONG");}
        let runner = Box::new(Runner::new(front_canvas_id, back_canvas_id));
        let pong = Pong::new(runner, Cfg::default());

        let game = Game {
            pong: Box::new(pong),
        };

        window().add_event_listener(enclose_mut!((game) move |event: KeyDownEvent|
            game.on_key_down(event)));

        window().add_event_listener(enclose_mut!((game) move |event: KeyUpEvent|
            game.on_key_up(event)));

        game
    }

    /**
     * See
     * - https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
     * - https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
     */
    fn on_key_down(&mut self, event: KeyDownEvent) {
        match event.code().as_ref() {
            "Digit0" => self.pong.start_demo(),
            "Digit1" => self.pong.start_single_player(),
            "Digit2" => self.pong.start_double_player(),
            "Escape" => self.pong.stop(true),
            "KeyQ" | "KeyW" => {
                if !self.pong.left_paddle.auto {
                    self.pong.left_paddle.move_up()
                }
            }
            "KeyA" | "KeyS" => {
                if !self.pong.left_paddle.auto {
                    self.pong.left_paddle.move_down()
                }
            }
            "KeyP" | "ArrowUp" => {
                if !self.pong.right_paddle.auto {
                    self.pong.right_paddle.move_up()
                }
            }
            "KeyL" | "ArrowDown" => {
                if !self.pong.right_paddle.auto {
                    self.pong.right_paddle.move_down()
                }
            }
            &_ => (),
        };
        event.prevent_default()
    }

    fn on_key_up(&self, event: KeyUpEvent) {
        match event.code().as_ref() {
            "KeyQ" | "KeyW" => {
                if !self.pong.left_paddle.auto {
                    self.pong.left_paddle.stop_moving_up()
                }
            }
            "KeyA" | "KeyS" => {
                if !self.pong.left_paddle.auto {
                    self.pong.left_paddle.stop_moving_down()
                }
            }
            "KeyP" | "ArrowUp" => {
                if !self.pong.right_paddle.auto {
                    self.pong.right_paddle.stop_moving_up()
                }
            }
            "KeyL" | "ArrowDown" => {
                if !self.pong.right_paddle.auto {
                    self.pong.right_paddle.stop_moving_down()
                }
            }
            _ => (),
        };
        event.prevent_default()
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

        runner
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
