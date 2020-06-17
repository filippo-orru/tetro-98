use super::{MainMenuItem, MenuScreen};
use crate::net::Netinfo;
use crate::util::{colors, Align, BetterText};
use crate::GameState;
use ggez::{event::KeyCode, graphics::Font, Context, GameResult};

#[derive(Clone, Debug, PartialEq)]
pub struct OnlineInputState {
    pub connection_state: ConnectionState,
    pub selected_digit: usize,
    pub peer_ip: [u8; 12],
    // error: Option<(f64, String)>,
}

impl OnlineInputState {
    pub fn new() -> OnlineInputState {
        OnlineInputState {
            connection_state: ConnectionState::Idle,
            peer_ip: [1, 2, 7, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            selected_digit: 0,
            // error: None,
        }
    }
    pub fn peer_ip_str(&self) -> String {
        let mut s = String::new();
        for (i, d) in self.peer_ip.iter().enumerate() {
            s += format!("{}", d).as_str();
            if i != self.peer_ip.len() - 1 && (i + 1) % 3 == 0 {
                s += ".";
            }
        }
        s
    }

    pub fn update(&mut self, ctx: &mut Context) -> Option<GameState> {
        if let ConnectionState::Waiting(true) = self.connection_state {
            match Netinfo::new(self.peer_ip_str()) {
                Ok(n) => {
                    self.connection_state = ConnectionState::Connecting;
                    return Some(GameState::playing_online(ctx, None, n));
                }
                Err(err) => {
                    println!("Netinfo error:\n{}", err);
                    // self.error = Some((0., err));
                    self.connection_state = ConnectionState::Failed(err);
                }
            }
        }
        None
    }

    pub fn pressed(&mut self, _ctx: &mut Context, key: KeyCode) -> Option<MenuScreen> {
        use KeyCode::*;
        match key {
            Return => {
                // if valid_ip()
                self.connection_state = ConnectionState::Waiting(false);
            }
            Up => {
                if self.peer_ip[self.selected_digit] < 9 {
                    self.peer_ip[self.selected_digit] += 1;
                }
            }
            Down => {
                if let Some(d) = self.peer_ip[self.selected_digit].checked_sub(1) {
                    self.peer_ip[self.selected_digit] = d;
                }
            }
            Left => {
                if let Some(d) = self.selected_digit.checked_sub(1) {
                    self.selected_digit = d;
                }
            }
            Right => {
                if self.selected_digit < self.peer_ip.len() {
                    self.selected_digit += 1;
                }
            }
            Escape => {
                return Some(MenuScreen::Main(MainMenuItem::PlayOnline));
            }
            _ => {}
        }
        None
    }

    pub fn draw(&mut self, ctx: &mut Context, font: Font) -> GameResult {
        BetterText::new("Peer ip adress:", font)
            .align(Align::TL(50., 50.))
            .color(colors::WHITE)
            .draw(ctx)?;
        // txt(ctx, "Peer ip adress:", 50., 100., colors::WHITE.into());
        // draw_queued_text(ctx, param(0., 0.), None, FilterMode::Linear)?;
        for (i, digit) in self.peer_ip.iter().enumerate() {
            let (mut txt_str, mut offset) = if i == self.selected_digit {
                (format!("({})", digit), -3.5)
            } else {
                (digit.to_string(), 0.)
            };
            if (i + 1) % 3 == 0 && i < self.peer_ip.len() - 1 {
                txt_str += ".";
            }
            offset += (i as f32 / 3.).floor() * 18.;

            BetterText::new(&txt_str, font)
                .align(Align::TL(50. + i as f32 * 18. + offset, 70.))
                .size(16)
                .color(colors::OFF_WHITE)
                .draw(ctx)?;
        }

        BetterText::new("Use Arrow Keys to set address:", font)
            .align(Align::BC(0., 50.))
            .size(16)
            .color(colors::LGREY)
            .draw(ctx)?;

        if let ConnectionState::Waiting(ref mut b) = self.connection_state {
            // println!("called draw with busy connection");
            BetterText::new("connecting...", font)
                .align(Align::BC(0., 110.))
                .size(20)
                .color(colors::LGREY)
                .draw(ctx)?;
            *b = true;
        }

        if let ConnectionState::Failed(err) = &self.connection_state {
            BetterText::new("Error! Could not connect.", font)
                .align(Align::BC(0., 110.))
                .size(20)
                .color(colors::LRED)
                .draw(ctx)?;
            BetterText::new(err, font)
                .align(Align::BC(0., 90.))
                .size(16)
                .color(colors::LRED)
                .draw(ctx)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionState {
    Idle,
    Waiting(bool), // true when text has been drawn
    Connecting,
    Failed(String),
}

/*impl ConnectionState {
    fn is_busy(&self) -> bool {
        use ConnectionState::*;
        match self {
            Waiting(_) | Connecting => true,
            _ => false,
        }
    }
}
*/
