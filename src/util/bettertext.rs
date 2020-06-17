use super::alignment::Alignment;
use crate::get_win_dim;
use crate::util::colors;
use ggez::graphics::Scale;
use ggez::graphics::*;
use ggez::{Context, GameResult};

pub struct BetterText {
    s: String,
    color: Color,
    align: Alignment,
    size: usize,
    font: Font, // margin: (f32, f32),
}

impl BetterText {
    pub fn new(s: &str, font: Font) -> Self {
        Self {
            s: String::from(s),
            color: colors::WHITE.into(),
            align: Alignment::TL(0., 0.),
            size: 18,
            font,
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut text = Text::new(self.s.clone());
        text.set_font(self.font, Scale::uniform(self.size as f32 * 2.));

        let (ww, wh): (f32, f32) = get_win_dim(ctx);
        let (tw, th) = text.dimensions(ctx);
        let (tw, th) = (tw as f32 / 2., th as f32 / 2.);
        let offset: (f32, f32) = self.align.to_offset((ww, wh), (tw, th));
        let param = DrawParam::default()
            .dest([offset.0, offset.1])
            .scale([0.5, 0.5])
            .color(self.color);
        text.draw(ctx, param)
    }

    pub fn text(mut self, s: &str) -> Self {
        self.s = String::from(s);
        self
    }

    pub fn align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = color.into();
        self
    }
    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }
}

// impl Drawable for BetterText {
//     fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
//         Ok(())
//     }
//     fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
//         None
//     }

//     fn set_blend_mode(&mut self, mode: Option<BlendMode>) {}

//     fn blend_mode(&self) -> Option<BlendMode> {
//         None
//     }
// }
