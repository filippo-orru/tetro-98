use crate::game::GameState; //, GAME_HEIGHT_VIS, GAME_WIDTH};
use crate::get_win_dim;
use crate::scenes::playing::PlayingState;
use crate::util::colors;
use crate::util::{Align, BetterText};

use ggez::event::KeyCode;
use ggez::graphics::*;
use ggez::*;

#[derive(Clone, Debug)]
pub struct PausedMenuState {
    selected: PausedMenuItem,
}

#[derive(Clone, Debug)]
pub enum PausedMenuItem {
    Continue,
    Exit,
}

impl PausedMenuState {
    pub fn default() -> Self {
        Self {
            selected: PausedMenuItem::Continue,
        }
    }
    pub fn draw(&self, ctx: &mut Context, font: Font) -> GameResult<()> {
        let (win_w, win_h) = get_win_dim(ctx);
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0., 0., win_w, win_h),
            colors::DARK_OVERLAY.into(),
        )?
        .draw(ctx, DrawParam::default())?;

        let mut cont = BetterText::new("Continue", font)
            .color(colors::LGREY)
            .align(Align::TL(50., 50.));
        let mut exit = BetterText::new("Exit", font)
            .color(colors::LGREY)
            .align(Align::TL(50., 150.));
        use PausedMenuItem::*;
        match self.selected {
            Continue => cont = cont.color(colors::WHITE).text("> Continue!"),
            Exit => exit = exit.color(colors::WHITE).text("> Exit!"),
        }
        cont.draw(ctx)?;
        exit.draw(ctx)?;
        Ok(())
    }
    pub fn pressed(&mut self, key: KeyCode, playing_state: &PlayingState) -> Option<GameState> {
        use KeyCode::*;
        use PausedMenuItem::*;
        match key {
            Up | Down => {
                self.selected = match self.selected {
                    Continue => Exit,
                    Exit => Continue,
                };
                None
            }
            Return | Space => match self.selected {
                Continue => Some(GameState::Playing(playing_state.clone())),
                Exit => Some(GameState::default()),
            },
            Escape => Some(GameState::Playing(playing_state.clone())),
            _ => None,
        }
    }
}
