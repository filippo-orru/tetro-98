use crate::util::colors;
use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect};
use ggez::{Context, GameResult};

pub const BLOCK_SIZE: f32 = 25.;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Block {
    color: BlockColor,
    // scaling: f32,
    // mesh: Mesh
}

impl Block {
    pub fn new(color: BlockColor) -> Block {
        //, scaling: f32
        //, ctx: &mut Context
        Block { color } //, mesh: Self::gen_mesh(ctx, ) } // , scaling
    }

    pub fn block_color(self) -> BlockColor {
        self.color
    }

    // pub fn empty_row(len: usize, scaling: f32) -> Vec<Option<Block>> {
    //     vec![
    //         Some(Block {
    //             color: BlockColor::Destroying,
    //             scaling,
    //         });
    //         len
    //     ]
    // }
    // pub fn get_scaling(&mut self) -> f32 {
    //     self.scaling
    // }
    // pub fn set_scaling(&mut self, scaling: f32) {
    //     self.scaling = scaling;
    // }
    // pub fn size(self) -> f32 {
    //     self.scaling * BLOCK_SIZE
    // }

    pub fn gen_mesh(self, ctx: &mut Context, shift_amount: i8) -> GameResult<Mesh> {
        // println!(
        //     "generating block mesh! Color: {:?}. Reason: {}",
        //     self.color, why
        // );
        use colors::*;

        let blsi = BLOCK_SIZE; //block size
        let efwi = blsi / 6.; // effect width
        let ma = 3.; // margin
        let mut mesh = MeshBuilder::new();
        mesh.rectangle(
            DrawMode::Fill(Default::default()),
            Rect::new(0., 0., blsi, blsi),
            shift(self.color.color_tuple(), shift_amount).into(),
        );
        let (top_shine_shift, left_shadow_shift) = match self.color.dark_or_light() {
            BlockColorLightness::Dark => (-25, -45),

            BlockColorLightness::Light => (-40, -15),
        };
        if self.color != BlockColor::Destroying {
            mesh.polygon(
                DrawMode::Fill(Default::default()),
                &[
                    // shiny effect
                    [ma, ma],
                    [blsi - ma, ma],
                    [blsi - ma, ma + efwi],
                    [blsi / 2. - ma, blsi / 2. - ma / 2.],
                    [ma, ma + 2. * efwi],
                    [ma, ma],
                ],
                TOP_SHINE.into(),
            )?
            .polygon(
                DrawMode::Fill(Default::default()),
                &[
                    // dark bottom
                    [ma, blsi - ma],
                    [blsi - ma, blsi - ma],
                    [blsi, blsi],
                    [0., blsi],
                    [ma, blsi - ma],
                ],
                BOTTOM_SHADOW.into(),
            )?
            .polygon(
                // top shine
                DrawMode::Fill(Default::default()),
                &[[0., 0.], [blsi, 0.], [blsi - ma, ma], [ma, ma], [0., 0.]],
                shift_alpha(TOP_SHINE, top_shine_shift).into(),
            )?
            .polygon(
                // left shadow
                DrawMode::Fill(Default::default()),
                &[[0., 0.], [ma, ma], [ma, blsi - ma], [0., blsi], [0., 0.]],
                shift_alpha(BOTTOM_SHADOW, left_shadow_shift).into(),
            )?
            .polygon(
                DrawMode::Fill(Default::default()),
                &[
                    // right shadow
                    [blsi, 0.],
                    [blsi, blsi],
                    [blsi - ma, blsi - ma],
                    [blsi - ma, ma],
                    [blsi, 0.],
                ],
                shift_alpha(BOTTOM_SHADOW, -13).into(),
            )?;
        }
        mesh.build(ctx)
        // Ok(mesh)
    }

    #[allow(non_snake_case)]
    pub fn gen_ghost_mesh(self, ctx: &mut Context) -> GameResult<Mesh> {
        let blsi = BLOCK_SIZE;
        // println!(
        //     "generating block ghost mesh! Color: {:?}, size: {}",
        //     self.color, blsi
        // );
        let inner_margin_half = blsi / 5.;
        let stroke_w = blsi / 12.;
        let mut mesh = MeshBuilder::new();
        let ghost_color = colors::shift(self.block_color().color_tuple(), -40).into();

        // Top left
        let poly_TL = &[
            [stroke_w + 0., 0. + stroke_w],
            [stroke_w + blsi / 2. - inner_margin_half, 0. + stroke_w],
            [
                stroke_w + blsi / 2. - inner_margin_half,
                stroke_w + stroke_w,
            ],
            [stroke_w + stroke_w, stroke_w + stroke_w],
            [
                stroke_w + stroke_w,
                blsi / 2. - inner_margin_half + stroke_w,
            ],
            [stroke_w + 0., blsi / 2. - inner_margin_half + stroke_w],
            [stroke_w + 0., 0. + stroke_w],
        ];

        // Top right
        let poly_TR = &[
            [blsi / 2. + inner_margin_half - stroke_w, 0. + stroke_w],
            [blsi - stroke_w, 0. + stroke_w],
            [blsi - stroke_w, blsi / 2. - inner_margin_half + stroke_w],
            [
                blsi - stroke_w - stroke_w,
                blsi / 2. - inner_margin_half + stroke_w,
            ],
            [blsi - stroke_w - stroke_w, stroke_w + stroke_w],
            [
                blsi / 2. + inner_margin_half - stroke_w,
                stroke_w + stroke_w,
            ],
            [blsi / 2. + inner_margin_half - stroke_w, 0. + stroke_w],
        ];

        // Bottom right
        let poly_BR = &[
            [blsi - stroke_w, blsi / 2. + inner_margin_half - stroke_w],
            [blsi - stroke_w, blsi - stroke_w],
            [blsi / 2. + inner_margin_half - stroke_w, blsi - stroke_w],
            [
                blsi / 2. + inner_margin_half - stroke_w,
                blsi - stroke_w - stroke_w,
            ],
            [blsi - stroke_w - stroke_w, blsi - stroke_w - stroke_w],
            [
                blsi - stroke_w - stroke_w,
                blsi / 2. + inner_margin_half - stroke_w,
            ],
            [blsi - stroke_w, blsi / 2. + inner_margin_half - stroke_w],
        ];

        // Bottom left
        let poly_BL = &[
            [stroke_w + 0., blsi / 2. + inner_margin_half - stroke_w],
            [
                stroke_w + stroke_w,
                blsi / 2. + inner_margin_half - stroke_w,
            ],
            [stroke_w + stroke_w, blsi - stroke_w - stroke_w],
            [
                stroke_w + blsi / 2. - inner_margin_half,
                blsi - stroke_w - stroke_w,
            ],
            [stroke_w + blsi / 2. - inner_margin_half, blsi - stroke_w],
            [stroke_w + 0., blsi - stroke_w],
            [stroke_w + 0., blsi / 2. + inner_margin_half - stroke_w],
        ];

        for poly in &[poly_TL, poly_TR, poly_BR, poly_BL] {
            mesh.polygon(DrawMode::Fill(Default::default()), *poly, ghost_color)?;
        }

        mesh.build(ctx)
    }
}

enum BlockColorLightness {
    Dark,
    Light,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlockColor {
    Red,
    Blue,
    Green,
    Destroying,
    Purple,
    Cyan,
    Yellow,
    Orange,
    Garbage,
    Blocked,
}

impl BlockColor {
    pub fn color(self) -> Color {
        let c = self.color_tuple();
        c.into()
    }
    pub fn color_tuple(self) -> colors::IntoColor {
        use BlockColor::*;
        match self {
            Red => colors::RED,
            Blue => colors::BLUE,
            Green => colors::GREEN,
            Destroying => colors::DESTROYING,
            Purple => colors::PURPLE,
            Cyan => colors::CYAN,
            Yellow => colors::YELLOW,
            Orange => colors::ORANGE,
            Garbage => colors::GARBAGE,
            Blocked => colors::GREY,
        }
    }
    pub fn dark_or_light(&self) -> BlockColorLightness {
        use BlockColor::*;
        match self {
            Red | Purple | Orange | Blue | Garbage | Blocked => BlockColorLightness::Dark,
            Green | Yellow | Cyan | Destroying => BlockColorLightness::Light,
        }
    }
}
