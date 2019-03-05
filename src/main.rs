#[macro_use]
extern crate stdweb;

fn main() {
    stdweb::initialize();
    let message = "PING 🏓 PONG 🏓";
    js! {
        alert( @{message} );
    }
    stdweb::event_loop();
}
