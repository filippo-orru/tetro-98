use crate::game::GameState;
use crate::net::Netinfo;
use crate::scenes::playing::PlayingState;
use crate::util::colors;
use crate::util::{Align, BetterText};
use ggez::graphics::{clear, Font};
use ggez::{event::KeyCode, timer::delta, Context, GameResult};

#[derive(Clone, Debug)]
pub enum PlayingOnlineState {
    Connected(PlayingState),
    Disconnected(String),
}

impl PlayingOnlineState {
    pub fn pressed(
        &mut self,
        ctx: &mut Context,
        key: KeyCode,
        net: &mut Netinfo,
    ) -> Option<GameState> {
        use PlayingOnlineState::*;

        let dt = delta(ctx).as_secs_f64();
        // let mut ret = None;

        match self {
            Connected(ref mut playing_state) => {
                let prev_state = playing_state.clone();
                let ret = playing_state.pressed(ctx, key, Some(net));

                if let Err(msg) = playing_state.update_net(dt, prev_state, net) {
                    Some(GameState::PlayingOnline(
                        PlayingOnlineState::Disconnected(msg),
                        net.clone(),
                    ))
                } else {
                    ret
                }
            }
            Disconnected(_) => match key {
                KeyCode::Escape => Some(GameState::default()),
                _ => None,
            },
        }
    }
    pub fn draw(&mut self, ctx: &mut Context, font: Font) -> GameResult {
        use PlayingOnlineState::*;
        match self {
            Disconnected(msg) => {
                clear(ctx, colors::BG_ERR.into());
                BetterText::new("Connection Lost!", font)
                    .align(Align::TC(0., 50.))
                    .size(26)
                    .draw(ctx)?;
                BetterText::new(&format!("Details:\n{}", msg), font)
                    .align(Align::BC(0., 50.))
                    .size(14)
                    .draw(ctx)?;
            }
            Connected(ps) => {
                ps.draw(ctx, font)?;
            }
        }

        Ok(())
    }
    pub fn as_option(&mut self) -> Option<&mut PlayingState> {
        if let PlayingOnlineState::Connected(ref mut ps) = *self {
            Some(ps)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnemyState {
    pub height: usize,
    pub game_over: bool,
}

impl EnemyState {
    pub fn new() -> EnemyState {
        EnemyState {
            height: 0,
            game_over: false,
        }
    }
}
