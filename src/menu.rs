use ggez::{graphics, Context};

use crate::constants::*;
use crate::player::Player;

const WINNER_MSG: &str = "🏆 WINNER 🏆";
const ONE_PLAYER_START_MSG: &str = "press '1' for\n\nsingle player";
const ONE_PLAYER_CONTROLS_MSG: &str = "'q': move up\n\n'a': move down";
const TWO_PLAYER_START_MSG: &str = "press '2' for\n\ndouble player";
const TWO_PLAYER_CONTROLS_MSG: &str = "'p': move up\n\n'l': move down";
const ZERO_PLAYER_START_MSG: &str = "press '0' for\n\nbots-only 🤖";

#[derive(Clone)]
pub struct Menu {
    player_one_wins: TextBox,
    player_two_wins: TextBox,
    one_player_start: TextBox,
    one_player_controls: TextBox,
    two_player_start: TextBox,
    two_player_controls: TextBox,
    zero_player_start: TextBox,
    additional_instructions: TextBox,
    winner: Option<Player>,
}

impl Menu {
    pub fn new(game_width: f32, game_height: f32) -> Menu {
        let p1_win_x = game_width * 0.18;
        let p2_win_x = game_width * 0.66;
        let p1_start_x = 0.05 * game_width;
        let p1_controls_x = 0.085 * game_width;
        let controls_y = 6.0 * BLOCK_LENGTH_TO_SCREEN_HEIGHT * game_height;

        let p2_start_x = 0.835 * game_width;
        let bottom_start_y = game_height - 8.0 * BLOCK_LENGTH_TO_SCREEN_HEIGHT * game_height;
        let p2_controls_x = 0.80 * game_width;
        let top_y = 2.0 * BLOCK_LENGTH_TO_SCREEN_HEIGHT * game_height;
        let win_y = game_height * 0.45;

        Menu {
            player_one_wins: TextBox {
                x: p1_win_x,
                y: win_y,
                msg: WINNER_MSG.to_string(),
            },
            player_two_wins: TextBox {
                x: p2_win_x,
                y: win_y,
                msg: WINNER_MSG.to_string(),
            },
            one_player_start: TextBox {
                x: p1_start_x,
                y: top_y,
                msg: ONE_PLAYER_START_MSG.to_string(),
            },
            one_player_controls: TextBox {
                x: p1_controls_x,
                y: controls_y,
                msg: ONE_PLAYER_CONTROLS_MSG.to_string(),
            },
            two_player_start: TextBox {
                x: p2_start_x,
                y: top_y,
                msg: TWO_PLAYER_START_MSG.to_string(),
            },
            two_player_controls: TextBox {
                x: p2_controls_x,
                y: controls_y,
                msg: TWO_PLAYER_CONTROLS_MSG.to_string(),
            },
            zero_player_start: TextBox {
                x: p1_start_x,
                y: bottom_start_y,
                msg: ZERO_PLAYER_START_MSG.to_string(),
            },
            additional_instructions: TextBox {
                x: 0.0,
                y: 0.0,
                msg: "".to_string(),
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
            &self.zero_player_start,
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
