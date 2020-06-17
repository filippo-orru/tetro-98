use crate::block::BLOCK_SIZE;
use crate::piece::{hold_piece::HOLD_PIECE_SCALING, HoldPiece, Piece, PieceAppearance};
use crate::util::{colors, Align, BetterText};
use ggez::{
    graphics::{DrawMode, DrawParam, Drawable, Font, Mesh, Rect},
    Context, GameResult,
};

pub const SIDEBAR_MARGIN: f32 = 8.;
pub const SIDEBAR_WIDTH: f32 =
    SIDEBAR_MARGIN + BLOCK_SIZE * 4. * SIDEBAR_BLOCK_SCALING + SIDEBAR_MARGIN;
pub const SIDEBAR_BLOCK_SCALING: f32 = 0.5;

pub fn draw(
    ctx: &mut Context,
    maybe_hold_piece: Option<&HoldPiece>,
    next_pieces: Vec<Piece>,
    font: Font,
) -> GameResult {
    if let Some(hold_piece) = maybe_hold_piece {
        let (x_off, y_off) = (10., 50.);
        let hldp_border_margin = 6.;
        let size = 3. * HOLD_PIECE_SCALING * BLOCK_SIZE + hldp_border_margin * 2.;
        // let off = Align::TL(20., 50.).to_offset();
        Mesh::new_rectangle(
            ctx,
            DrawMode::Stroke(Default::default()),
            Rect::new(0., 0., size, size),
            colors::BG_LINE_COLOR.into(),
        )?
        .draw(ctx, DrawParam::default().dest([x_off, y_off]))?;
        BetterText::new("HOLD", font)
            .align(Align::TL(x_off, y_off + size + 8.))
            .color(colors::GREY)
            .size(13)
            .draw(ctx)?;
        hold_piece.draw(
            ctx,
            Align::TL(x_off + hldp_border_margin, y_off + hldp_border_margin),
        )?;
    }

    let mut next_pieces = next_pieces;
    let mut y = 50.;
    for piece in next_pieces[0..6].iter_mut() {
        piece.set_scaling(SIDEBAR_BLOCK_SCALING);

        piece.draw(ctx, Align::TR(10., y), true, PieceAppearance::Normal)?;
        y += 40.;
    }

    Ok(())
}
