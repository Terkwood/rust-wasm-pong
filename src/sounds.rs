pub fn ping() -> stdweb::Value {
    js!(pongSounds.ping())
}

pub fn pong() -> stdweb::Value {
    js!(pongSounds.pong())
}

pub fn goal() -> stdweb::Value {
    js!(pongSounds.goal())
}
