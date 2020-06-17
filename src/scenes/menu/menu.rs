use super::online::*;
use crate::game::GameState;
use crate::scenes::playing::PlayingState;
use crate::util::colors;
use crate::util::{Align, BetterText};

use ggez::event::KeyCode;
use ggez::graphics::*;
use ggez::{Context, GameResult};

#[derive(Clone, Debug)]
pub enum MenuScreen {
    Main(MainMenuItem),
    OnlineInput(OnlineInputState),
}

impl MenuScreen {
    pub fn default() -> MenuScreen {
        MenuScreen::Main(MainMenuItem::Play)
    }
    pub fn online() -> MenuScreen {
        MenuScreen::OnlineInput(OnlineInputState::new())
    }

    pub fn draw(&mut self, ctx: &mut Context, font: Font) -> GameResult<()> {
        clear(ctx, colors::BG.into());
        use MenuScreen::*;
        match self {
            Main(selected) => {
                let mut play = BetterText::new("Play", font)
                    .align(Align::TL(50., 50.))
                    .color(colors::GREY);
                let mut play_online = BetterText::new("Play Online", font)
                    .align(Align::TL(50., 100.))
                    .color(colors::GREY);
                let mut exit = BetterText::new("Exit", font)
                    .align(Align::BL(50., 50.))
                    .color(colors::GREY);

                use MainMenuItem::*;
                match selected {
                    Play => play = play.color(colors::WHITE).text("> Play!"),
                    PlayOnline => {
                        play_online = play_online.color(colors::LBLUE).text("> Play Online!")
                    }
                    Exit => exit = exit.color(colors::LRED).text("> Exit"),
                }
                play.draw(ctx)?;
                play_online.draw(ctx)?;
                exit.draw(ctx)?;
            }
            OnlineInput(ref mut state) => {
                // let mut play =
                state.draw(ctx, font)?;
                // txt(ctx, s, 10., 445., colors::LRED.into());
                // draw_queued_text(ctx, param(0., 0.), None, FilterMode::Linear)?;
                // }
            }
        }
        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) -> Option<GameState> {
        use MenuScreen::*;
        if let OnlineInput(ref mut state) = self {
            // if let Some(new_state) = state.update(ctx) {
            //     return new_state;
            // }
            return state.update(ctx);
        }
        None
    }

    pub fn pressed(&mut self, ctx: &mut Context, key: KeyCode) -> Option<GameState> {
        use KeyCode::*;
        use MenuScreen::*;
        match *self {
            Main(ref mut selected) => {
                use MainMenuItem::*;
                match key {
                    Return | Space => match selected {
                        Play => Some(GameState::Playing(PlayingState::new(ctx, false))),
                        PlayOnline => {
                            *self = MenuScreen::online();
                            None
                        }
                        Exit => Some(GameState::Exiting),
                    },
                    Down => {
                        *selected = match selected {
                            Play => PlayOnline,
                            PlayOnline => Exit,
                            Exit => Play,
                        };
                        None
                    }
                    Up => {
                        *selected = match selected {
                            Play => Exit,
                            PlayOnline => Play,
                            Exit => PlayOnline,
                        };
                        None
                    }
                    Escape => Some(GameState::Exiting),
                    _ => None,
                }
            }
            OnlineInput(ref mut state) => {
                if let Some(new_state) = state.pressed(ctx, key) {
                    *self = new_state;
                }
                None
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MainMenuItem {
    Play,
    PlayOnline,
    Exit,
}
