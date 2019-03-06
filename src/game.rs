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
    pub width: u32,
    pub height: u32,
}

impl Runner {
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
