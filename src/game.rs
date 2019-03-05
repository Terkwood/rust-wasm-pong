pub fn load_images(sources: Vec<String>, callback: Box<(FnOnce(Vec<String>) -> ())>) {
    unimplemented!()
}

#[derive(Clone, Copy)]
pub struct Runner {
    pub width: u32,
    pub height: u32,
}

impl Runner {
    pub fn confirm(self, _arg: &str) -> bool {
        unimplemented!()
    }

    pub fn hide_cursor(self) {
        unimplemented!()
    }

    pub fn show_cursor(self) {
        unimplemented!()
    }

    pub fn start(self) {
        unimplemented!()
    }
}
