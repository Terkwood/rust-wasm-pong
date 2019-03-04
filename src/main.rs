#![feature(custom_attribute)]
pub fn main() {}

#[js_export]
pub fn hello() -> String { "Hello".to_string() }