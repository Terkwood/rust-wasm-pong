#![feature(custom_attribute)]
extern crate stdweb;

#[js_export]
pub fn hello() -> String {
    "Hello".to_string()
}

fn main() {
    stdweb::initialize();
    stdweb::event_loop();
}
