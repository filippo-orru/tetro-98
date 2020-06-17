use crate::piece::{Piece, PieceShape, RotationState};
use crate::util::types::Dir;

pub trait RotSteps {
    fn rot_steps(&self, start: RotationState, end: RotationState) -> [Vec<Option<Dir>>; 5];
}

impl RotSteps for Piece {
    fn rot_steps(&self, start: RotationState, end: RotationState) -> [Vec<Option<Dir>>; 5] {
        use Dir::*;
        use PieceShape::*;
        use RotationState::*;
        let empty: [Vec<Option<Dir>>; 5] = [vec![], vec![], vec![], vec![], vec![]];
        match self.shape {
            J | L | Z | S | T => match (start, end) {
                (Rs0, Rs1) => [
                    vec![None, None],
                    vec![Some(Left), None],
                    vec![Some(Left), Some(Up)],
                    vec![None, Some(Down), Some(Down)],
                    vec![Some(Left), Some(Down), Some(Down)],
                ],
                (Rs1, Rs0) => [
                    vec![None, None],
                    vec![Some(Right), None],
                    vec![Some(Right), Some(Down)],
                    vec![None, Some(Up), Some(Up)],
                    vec![Some(Right), Some(Up), Some(Up)],
                ],
                (Rs1, Rs2) => [
                    vec![None, None],
                    vec![Some(Right), None],
                    vec![Some(Right), Some(Down)],
                    vec![None, Some(Up), Some(Up)],
                    vec![Some(Right), Some(Up), Some(Up)],
                ],
                (Rs2, Rs1) => [
                    vec![None, None],
                    vec![Some(Left), None],
                    vec![Some(Left), Some(Up)],
                    vec![None, Some(Down), Some(Down)],
                    vec![Some(Left), Some(Down), Some(Down)],
                ],
                (Rs2, Rs3) => [
                    vec![None, None],
                    vec![Some(Right), None],
                    vec![Some(Right), Some(Up)],
                    vec![None, Some(Down), Some(Down)],
                    vec![Some(Right), Some(Down), Some(Down)],
                ],
                (Rs3, Rs2) => [
                    vec![None, None],
                    vec![Some(Left), None],
                    vec![Some(Left), Some(Down)],
                    vec![None, Some(Up), Some(Up)],
                    vec![Some(Left), Some(Up), Some(Up)],
                ],
                (Rs3, Rs0) => [
                    vec![None, None],
                    vec![Some(Left), None],
                    vec![Some(Left), Some(Down)],
                    vec![None, Some(Up), Some(Up)],
                    vec![Some(Left), Some(Up), Some(Up)],
                ],
                (Rs0, Rs3) => [
                    vec![None, None],
                    vec![Some(Right), None],
                    vec![Some(Right), Some(Up)],
                    vec![None, Some(Down), Some(Down)],
                    vec![Some(Right), Some(Down), Some(Down)],
                ],
                _ => empty,
            },

            I => match (start, end) {
                (Rs0, Rs1) => [
                    vec![None, None],
                    vec![Some(Left), Some(Left), None],
                    vec![Some(Right), None],
                    vec![Some(Left), Some(Left), Some(Down)],
                    vec![Some(Right), Some(Up), Some(Up)],
                ],
                (Rs1, Rs0) => [
                    vec![None, None],
                    vec![Some(Right), Some(Right), None],
                    vec![Some(Left), None],
                    vec![Some(Right), Some(Right), Some(Up)],
                    vec![Some(Left), Some(Down), Some(Down)],
                ],
                (Rs1, Rs2) => [
                    vec![None, None],
                    vec![Some(Left), None],
                    vec![Some(Right), Some(Right), None],
                    vec![Some(Left), Some(Up), Some(Up)],
                    vec![Some(Right), Some(Right), Some(Down)],
                ],
                (Rs2, Rs1) => [
                    vec![None, None],
                    vec![Some(Right), None],
                    vec![Some(Left), Some(Left), None],
                    vec![Some(Right), Some(Down), Some(Down)],
                    vec![Some(Left), Some(Left), Some(Up)],
                ],
                (Rs2, Rs3) => [
                    vec![None, None],
                    vec![Some(Right), Some(Right), None],
                    vec![Some(Left), None],
                    vec![Some(Right), Some(Right), Some(Up)],
                    vec![Some(Left), Some(Down), Some(Down)],
                ],
                (Rs3, Rs2) => [
                    vec![None, None],
                    vec![Some(Left), Some(Left), None],
                    vec![Some(Right), None],
                    vec![Some(Left), Some(Left), Some(Down)],
                    vec![Some(Right), Some(Up), Some(Up)],
                ],
                (Rs3, Rs0) => [
                    vec![None, None],
                    vec![Some(Right), None],
                    vec![Some(Left), Some(Left), None],
                    vec![Some(Right), Some(Down), Some(Down)],
                    vec![Some(Left), Some(Left), Some(Up)],
                ],
                (Rs0, Rs3) => [
                    vec![None, None],
                    vec![Some(Left), None],
                    vec![Some(Right), Some(Right), None],
                    vec![Some(Left), Some(Up), Some(Up)],
                    vec![Some(Right), Some(Right), Some(Down)],
                ],
                _ => empty,
            },
            O => empty,
        }
    }
}
