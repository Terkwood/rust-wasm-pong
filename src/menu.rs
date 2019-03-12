use ggez::{graphics, Context};

use crate::constants::{BLOCK_LENGTH_TO_SCREEN_HEIGHT, MENU_FONT, TEXT_COLOR};
use crate::player::Player;

#[derive(Clone)]
pub struct Menu {
    // TODO
    //player_one_wins: TextBox,
    //player_two_wins: TextBox,
    one_player_start: TextBox,
    one_player_controls: TextBox,
    two_player_start: TextBox,
    two_player_controls: TextBox,
    // zero_player_start: TextBox,
    // misc_controls: TextBox,
    winner: Option<Player>,
}

impl Menu {
    pub fn new(game_width: f32, game_height: f32) -> Menu {
        Menu {
            // TODO player_one_wins: TextBox { x: 0.0, y: 0.0, msg: "🏆 WINNER! 🏆".to_string() },
            one_player_start: TextBox {
                x: 0.05 * game_width,
                y: 2.0 * BLOCK_LENGTH_TO_SCREEN_HEIGHT * game_height,
                msg: "press '1' for\n\nsingle player".to_string(),
            },
            one_player_controls: TextBox {
                x: 0.085 * game_width,
                y: 6.0 * BLOCK_LENGTH_TO_SCREEN_HEIGHT * game_height,
                msg: "'q': move up\n\n'a': move down".to_string(),
            },
            two_player_start: TextBox {
                x: 0.835 * game_width,
                y: 2.0 * BLOCK_LENGTH_TO_SCREEN_HEIGHT * game_height,
                msg: "press '2' for\n\ndouble player".to_string(),
            },
            two_player_controls: TextBox {
                x: 0.80 * game_width,
                y: 6.0 * BLOCK_LENGTH_TO_SCREEN_HEIGHT * game_height,
                msg: "'p': move up\n\n'l': move down".to_string(),
            },

            winner: None,
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        for text_box in &self.menu_boxes() {
            graphics::draw(
                ctx,
                &ggez::graphics::Text::new((
                    text_box.msg.to_string(),
                    graphics::Font(MENU_FONT.to_string()),
                    1.0,
                )),
                graphics::DrawParam::default()
                    .color(TEXT_COLOR)
                    .dest([text_box.x, text_box.y])
                    .scale([1.0, 1.0]),
            )
            .unwrap();
        }
    }

    fn menu_boxes(&self) -> Vec<&TextBox> {
        vec![
            &self.one_player_start,
            &self.one_player_controls,
            &self.two_player_start,
            &self.two_player_controls,
        ]
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
