use super::{Piece, PieceAppearance};
// use crate::scenes::playing::sidebar::SIDEBAR_BLOCK_SCALING;
use crate::util::Align;

use ggez::{Context, GameResult};

pub const HOLD_PIECE_SCALING: f32 = 0.65;

#[derive(Clone, Debug)]
pub enum HoldPiece {
    Unlocked(Piece),
    Locked(Piece),
}

impl HoldPiece {
    /*pub fn swap(&mut self, in_piece: Piece, ctx: &mut Context) -> GameResult<Option<Piece>> {
        use HoldPiece::*;
        match *self {
            Unlocked(ref mut p) => {
                let mut in_piece = in_piece;
                let mut out_piece = p.clone();
                out_piece.set_scaling(ctx, 1.)?;
                in_piece.set_scaling(ctx, SIDEBAR_BLOCK_SCALING)?;
                *p = in_piece;
                Ok(Some(out_piece))
            }
            Locked(ref mut p) => Ok(None),
        }
    }*/
    pub fn draw(&self, ctx: &mut Context, align: Align) -> GameResult<()> {
        // let mut piece = self.unwrap();
        let (mut piece, appearance) = match self.clone() {
            HoldPiece::Unlocked(p) => (p, PieceAppearance::Normal),
            HoldPiece::Locked(p) => (p, PieceAppearance::Blocked),
        };
        let adj_scaling = 3. / piece.width() as f32;
        piece.set_scaling(adj_scaling * HOLD_PIECE_SCALING);
        piece.draw(ctx, align, true, appearance)
    }

    /*fn unwrap_mut(&mut self) -> &mut Piece {
        use HoldPiece::*;
        match self {
            Unlocked(ref mut p) | Locked(ref mut p) => p,
        }
    }
    pub fn unwrap(&self) -> Piece {
        use HoldPiece::*;
        match &self {
            Unlocked(piece) | Locked(piece) => piece.clone(),
        }
    }*/
}
