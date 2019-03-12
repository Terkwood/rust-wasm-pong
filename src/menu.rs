use ggez::{graphics, Context};

use crate::constants::{FONT, TEXT_COLOR};
use crate::player::Player;

#[derive(Clone)]
pub struct Menu {
    // TODO
    //player_one_wins: TextBox,
    //player_two_wins: TextBox,
    one_player_start: TextBox,
    // one_player_controls: TextBox,
    // two_player_start: TextBox,
    // two_player_controls: TextBox,
    // zero_player_start: TextBox,
    // misc_controls: TextBox,
    winner: Option<Player>,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            // TODO player_one_wins: TextBox { x: 0.0, y: 0.0, msg: "🏆 WINNER! 🏆".to_string() },
            one_player_start: TextBox {
                x: 0.0,
                y: 0.0,
                msg: "press '1' for\nsingle player".to_string(),
            },
            winner: None,
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        graphics::draw(
            ctx,
            &ggez::graphics::Text::new((
                self.one_player_start.msg.to_string(),
                graphics::Font(FONT.to_string()),
                1.0,
            )),
            graphics::DrawParam::default()
                .color(TEXT_COLOR)
                .dest([self.one_player_start.x, self.one_player_start.y])
                .scale([1.5, 1.5]),
        )
        .unwrap();
    }

    pub fn declare_winner(&mut self, player: Player) {
        self.winner = Some(player)
    }
}

#[derive(Clone)]
pub struct TextBox {
    pub x: f32,
    pub y: f32,
    pub msg: String,
}
