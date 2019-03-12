use ggez::{graphics, Context};

use crate::constants::*;
use crate::player::Player;

#[derive(Clone)]
pub struct Menu {
    player_one_wins: TextBox,
    player_two_wins: TextBox,
    one_player_start: TextBox,
    one_player_controls: TextBox,
    two_player_start: TextBox,
    two_player_controls: TextBox,
    winner: Option<Player>,
}

impl Menu {
    pub fn new(game_width: f32, game_height: f32) -> Menu {
        Menu {
            player_one_wins: TextBox {
                x: game_width * 0.18,
                y: game_height * 0.45,
                msg: WINNER_MSG.to_string(),
            },
            player_two_wins: TextBox {
                x: game_width * 0.66,
                y: game_height * 0.45,
                msg: WINNER_MSG.to_string(),
            },
            one_player_start: TextBox {
                x: 0.05 * game_width,
                y: 2.0 * BLOCK_LENGTH_TO_SCREEN_HEIGHT * game_height,
                msg: ONE_PLAYER_START_MSG.to_string(),
            },
            one_player_controls: TextBox {
                x: 0.085 * game_width,
                y: 6.0 * BLOCK_LENGTH_TO_SCREEN_HEIGHT * game_height,
                msg: ONE_PLAYER_CONTROLS_MSG.to_string(),
            },
            two_player_start: TextBox {
                x: 0.835 * game_width,
                y: 2.0 * BLOCK_LENGTH_TO_SCREEN_HEIGHT * game_height,
                msg: TWO_PLAYER_START_MSG.to_string(),
            },
            two_player_controls: TextBox {
                x: 0.80 * game_width,
                y: 6.0 * BLOCK_LENGTH_TO_SCREEN_HEIGHT * game_height,
                msg: TWO_PLAYER_CONTROLS_MSG.to_string(),
            },

            winner: None,
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        for text_box in &self.menu_boxes() {
            draw_text(ctx, MENU_FONT, text_box)
        }

        if let Some(Player::One) = self.winner {
            draw_text(ctx, WINNER_FONT, &self.player_one_wins);
        }
        if let Some(Player::Two) = self.winner {
            draw_text(ctx, WINNER_FONT, &self.player_two_wins)
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

fn draw_text(ctx: &mut Context, font: &str, text_box: &TextBox) {
    graphics::draw(
        ctx,
        &ggez::graphics::Text::new((
            text_box.msg.to_string(),
            graphics::Font(font.to_string()),
            1.0,
        )),
        graphics::DrawParam::default()
            .color(TEXT_COLOR)
            .dest([text_box.x, text_box.y])
            .scale([1.0, 1.0]),
    )
    .unwrap();
}

#[derive(Clone)]
pub struct TextBox {
    pub x: f32,
    pub y: f32,
    pub msg: String,
}
