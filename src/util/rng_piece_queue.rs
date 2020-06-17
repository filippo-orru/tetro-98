use crate::{
    piece::{Piece, PieceShape},
    scenes::playing::sidebar::SIDEBAR_BLOCK_SCALING,
};
use ggez::{Context, GameResult};
use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct RngPieceQueue {
    pieces: Vec<Piece>,
}

impl RngPieceQueue {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut rbq = Self { pieces: vec![] };
        rbq.fill(ctx)?;
        Ok(rbq)
    }
    pub fn get(&mut self, ctx: &mut Context) -> GameResult<Piece> {
        let mut piece = self.pieces.remove(0);
        self.fill(ctx)?;
        piece.set_scaling(1.);
        Ok(piece)
    }

    pub fn fill(&mut self, ctx: &mut Context) -> GameResult {
        let mut rng = rand::thread_rng();
        while self.pieces.len() <= 7 {
            let mut random_pieces = PieceShape::all();
            random_pieces.shuffle(&mut rng);
            for form in random_pieces {
                self.pieces
                    .push(Piece::new(ctx, form, SIDEBAR_BLOCK_SCALING)?);
            }
        }

        Ok(())
    }

    /// Returns a clone of the Vec<Piece>
    pub fn as_vec(&self) -> Vec<Piece> {
        self.pieces.clone()
    }
}
