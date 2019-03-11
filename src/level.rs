#[derive(Copy, Clone)]
pub struct Level {
    pub ai_reaction: f32,
    pub ai_error: u32,
}

lazy_static! {
    pub static ref LEVELS: Vec<Level> = vec! [
        Level{ai_reaction: 0.2, ai_error: 40}, // 0:  ai is losing by 8
        Level{ai_reaction: 0.3, ai_error: 50}, // 1:  ai is losing by 7
        Level{ai_reaction: 0.4, ai_error: 60}, // 2:  ai is losing by 6
        Level{ai_reaction: 0.5, ai_error: 70}, // 3:  ai is losing by 5
        Level{ai_reaction: 0.6, ai_error: 80}, // 4:  ai is losing by 4
        Level{ai_reaction: 0.7, ai_error: 90}, // 5:  ai is losing by 3
        Level{ai_reaction: 0.8, ai_error: 100}, // 6:  ai is losing by 2
        Level{ai_reaction: 0.9, ai_error: 110}, // 7:  ai is losing by 1
        Level{ai_reaction: 1.0, ai_error: 120}, // 8:  tie
        Level{ai_reaction: 1.1, ai_error: 130}, // 9:  ai is winning by 1
        Level{ai_reaction: 1.2, ai_error: 140}, // 10: ai is winning by 2
        Level{ai_reaction: 1.3, ai_error: 150}, // 11: ai is winning by 3
        Level{ai_reaction: 1.4, ai_error: 160}, // 12: ai is winning by 4
        Level{ai_reaction: 1.5, ai_error: 170}, // 13: ai is winning by 5
        Level{ai_reaction: 1.6, ai_error: 180}, // 14: ai is winning by 6
        Level{ai_reaction: 1.7, ai_error: 190}, // 15: ai is winning by 7
        Level{ai_reaction: 1.8, ai_error: 200}, // 16: ai is winning by 8
    ];
}
