use ggez::{graphics, Context};

use crate::constants::{BLOCK_LENGTH_TO_SCREEN_HEIGHT, BLOCK_LENGTH_TO_SCREEN_WIDTH};
use crate::Score;

#[derive(Clone)]
pub struct Court {
    walls: Vec<DrawnBlock>,
    net: Vec<DrawnBlock>,
    score_boxes: (DrawnBlock, DrawnBlock),
    block_image: graphics::Image,
    ww: f32,
}

impl Court {
    pub fn new(block_image: graphics::Image, game_width: f32, game_height: f32) -> Court {
        let ww = BLOCK_LENGTH_TO_SCREEN_WIDTH * game_width;
        let hh = BLOCK_LENGTH_TO_SCREEN_HEIGHT * game_height;

        let sw = 3.0 * ww;
        let sh = 4.0 * ww;
        let score_boxes = (
            DrawnBlock {
                x: 0.5 + game_width / 2.0 - 1.5 * ww - sw,
                y: 2.0 * ww,
                width: sw,
                height: sh,
            },
            DrawnBlock {
                x: 0.5 + game_width / 2.0 + 1.5 * ww,
                y: 2.0 * ww,
                width: sw,
                height: sh,
            },
        );

        let mut c = Court {
            ww,
            block_image,
            walls: vec![
                DrawnBlock {
                    x: 0.0,
                    y: 0.0,
                    width: game_width,
                    height: hh,
                },
                DrawnBlock {
                    x: 0.0,
                    y: game_height - hh,
                    width: game_width,
                    height: hh,
                },
            ],
            net: vec![],
            score_boxes: score_boxes,
        };

        // draw net down middle of screen
        let n_max = (game_height / (ww * 2.0)) as u32 + 1;
        for n in 0..n_max {
            c.net.push(DrawnBlock {
                x: game_width / 2.0 - ww / 2.0,
                y: ww / 2.0 + ww * 2.0 * n as f32,
                width: ww,
                height: ww,
            });
        }

        c
    }

    pub fn draw(&self, ctx: &mut Context, score: Score) {
        for &wall in &self.walls {
            let _ = graphics::draw(
                ctx,
                &self.block_image,
                graphics::DrawParam::default()
                    .dest([wall.x, wall.y])
                    .scale([
                        wall.width / self.block_image.width() as f32,
                        wall.height / self.block_image.height() as f32,
                    ]),
            );
        }

        for &block in &self.net {
            let _ = graphics::draw(
                ctx,
                &self.block_image,
                graphics::DrawParam::default()
                    .dest([block.x, block.y])
                    .scale([
                        block.width / self.block_image.width() as f32,
                        block.height / self.block_image.height() as f32,
                    ]),
            );
        }

        self.draw_digit(ctx, score.0, self.score_boxes.0);
        self.draw_digit(ctx, score.1, self.score_boxes.1);
    }

    fn draw_digit(&self, ctx: &mut Context, n: u32, block: DrawnBlock) {
        let mut dr = |x: f32, y: f32, sx: f32, sy: f32| {
            let _ = graphics::draw(
                ctx,
                &self.block_image,
                graphics::DrawParam::default().dest([x, y]).scale([
                    sx / self.block_image.width() as f32,
                    sy / self.block_image.height() as f32,
                ]),
            );
        };

        let x = block.x;
        let y = block.y;
        let w = block.width;
        let h = block.height;

        let dw = self.ww * 4.0 / 5.0;
        let dh = dw;
        let panels = DIGITS[n as usize];
        if panels[0] {
            dr(x, y, w, dh)
        }
        if panels[1] {
            dr(x, y, dw, h / 2.0)
        }
        if panels[2] {
            dr(x + w - dw, y, dw, h / 2.0)
        };
        if panels[3] {
            dr(x, y + h / 2.0 - dh / 2.0, w, dh)
        };
        if panels[4] {
            dr(x, y + h / 2.0, dw, h / 2.0);
        };
        if panels[5] {
            dr(x + w - dw, y + h / 2.0, dw, h / 2.0)
        };
        if panels[6] {
            dr(x, y + h - dh, w, dh)
        };
    }
}

#[derive(Copy, Clone)]
pub struct DrawnBlock {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

lazy_static! {
    pub static ref DIGITS: Vec<[bool; 7]> = vec! [
      [true, true, true, false, true, true, true], // 0
      [false, false, true, false, false, true, false],  // 1
      [true, false, true, true, true, false, true], // 2
      [true, false, true, true, false, true, true], // 3
      [false, true, true, true, false, true, false],  // 4
      [true, true, false, true, false, true, true], // 5
      [true, true, false, true, true, true, true], // 6
      [true, false, true, false, false, true, false],  // 7
      [true, true, true, true, true, true, true], // 8
      [true, true, true, true, false, true, false],  // 9
    ];
}
