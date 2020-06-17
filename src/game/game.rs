use crate::net::Netinfo;
use crate::scenes::menu::MenuScreen;
use crate::scenes::paused::PausedMenuState;
use crate::scenes::playing::PlayingState;
use crate::scenes::playing_online::PlayingOnlineState;
use crate::util::{
    colors,
    types::{KeypressInfo, KeypressInfos},
};

use ggez::event::{self, Button, EventHandler, GamepadId, KeyCode, KeyMods};
use ggez::graphics;
use ggez::timer::delta;
use ggez::{Context, GameResult};

const LOOPING_KEYS_PLAYING: [KeyCode; 3] = [KeyCode::Down, KeyCode::Right, KeyCode::Left];
const LOOPING_KEYS_MENU: [KeyCode; 2] = [KeyCode::Down, KeyCode::Up];
const KEYPRESS_INIT_DELAY: f64 = 0.182;
const KEYPRESS_DELAY: f64 = 0.05;

pub struct Game {
    // delta: f64,
    font: graphics::Font,
    state: GameState,
    keydown: KeypressInfos,
}

impl Game {
    pub fn new(_ctx: &mut Context, font: graphics::Font) -> GameResult<Game> {
        Ok(Game {
            font,
            state: GameState::default(),
            keydown: KeypressInfos::empty(),
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let GameState::Exiting = self.state {
            event::quit(ctx);
            return Ok(());
        }
        let dt = delta(ctx).as_secs_f64();
        let mut kpis = self.keydown.kpis();
        for kpi in kpis.iter_mut() {
            use GameState::*;
            let looping_keys: &[KeyCode] = match &self.state {
                Playing(_) | PlayingOnline(_, _) => &LOOPING_KEYS_PLAYING,
                Menu(screen) => {
                    if let MenuScreen::OnlineInput(_) = screen {
                        &LOOPING_KEYS_MENU
                    } else {
                        &[]
                    }
                }
                Paused(_, _) | PausedOnline(_, _, _) | Exiting => &[],
            };

            if !looping_keys.contains(&kpi.key) && kpi.repeat_count > 0 {
                continue; // shouldnt repeat and has already fired
            }
            kpi.delta += dt;
            if (kpi.repeat_count <= 1 && kpi.delta >= KEYPRESS_INIT_DELAY)
                || (kpi.repeat_count > 1 && kpi.delta >= KEYPRESS_DELAY)
            {
                // println!("press repeat! Delta: {} > {}", kpi.delta, KEYPRESS_DELAY);

                kpi.repeat_count += 1;
                kpi.delta = 0.;
                self.state.pressed(ctx, kpi.key);
            }
        }
        self.keydown.set(kpis);
        self.state.update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // let font: Font = self.font;
        self.state.draw(ctx, self.font)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if !self.keydown.contains(keycode) {
            let key_info = KeypressInfo::new(keycode);
            self.state.pressed(ctx, keycode);
            self.keydown.push(key_info);
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.keydown.pop_if_exists(keycode);
    }

    fn gamepad_button_down_event(&mut self, ctx: &mut Context, btn: Button, _id: GamepadId) {
        if let Some(keycode) = btn_to_keycode(btn) {
            if !self.keydown.contains(keycode) {
                let key_info = KeypressInfo::new(keycode);
                self.state.pressed(ctx, keycode);
                self.keydown.push(key_info);
            }
        }
    }

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, btn: Button, _id: GamepadId) {
        if let Some(keycode) = btn_to_keycode(btn) {
            self.keydown.pop_if_exists(keycode);
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }
}

pub enum GameState {
    Menu(MenuScreen),
    Playing(PlayingState),
    Paused(PlayingState, PausedMenuState),
    PlayingOnline(PlayingOnlineState, Netinfo),
    PausedOnline(PlayingOnlineState, PausedMenuState, Netinfo),
    Exiting,
}

impl GameState {
    pub fn default() -> GameState {
        GameState::Menu(MenuScreen::default())
    }
    pub fn playing_online(ctx: &mut Context, ps: Option<PlayingState>, n: Netinfo) -> GameState {
        let ps = ps.unwrap_or_else(|| PlayingState::new(ctx, true));
        GameState::PlayingOnline(PlayingOnlineState::Connected(ps), n)
    }
    // pub fn playing() -> GameState {
    //     GameState::Playing(PlayingState::new())
    // }
    // pub fn paused(playing_state: PlayingState) -> GameState {
    //     GameState::Paused(playing_state, PausedMenuState::default())
    // }
    // pub fn paused_online(playing_state: PlayingState, net: Netinfo) -> GameState {
    //     GameState::PausedOnline(playing_state, PausedMenuState::default(), net)
    // }

    pub fn draw(&mut self, ctx: &mut Context, font: graphics::Font) -> GameResult<()> {
        use GameState::*;
        match self {
            Playing(ref mut playing_state) => {
                playing_state.draw(ctx, font)?;
            }
            Paused(ref mut playing_state, ref paused_menu) => {
                playing_state.draw(ctx, font)?;
                paused_menu.draw(ctx, font)?;
            }
            PlayingOnline(ref mut playing_online_state, _) => {
                playing_online_state.draw(ctx, font)?;
            }
            PausedOnline(ref mut playing_online_state, ref paused_menu, _) => {
                playing_online_state.draw(ctx, font)?;
                paused_menu.draw(ctx, font)?;
            }
            Menu(ref mut menu_screen) => menu_screen.draw(ctx, font)?,
            Exiting => {}
        }
        graphics::present(ctx)
    }
    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        use GameState::*;
        let dt = delta(ctx).as_secs_f64();
        match self {
            Playing(playing_state) => {
                if let Some(new_state) = playing_state.update(ctx)? {
                    *self = new_state;
                }
            }
            PlayingOnline(ref mut playing_online_state, ref mut net)
            | PausedOnline(ref mut playing_online_state, _, ref mut net) => {
                if let Some(playing_state) = playing_online_state.as_option() {
                    net.receive(playing_state);
                    if !net.enemy.game_over {
                        let prev_state = playing_state.clone();
                        let optional_new_state = playing_state.update(ctx)?;
                        if let Err(_msg) = playing_state.update_net(dt, prev_state, net) {
                            // TODO
                        }
                        if let Some(new_state) = optional_new_state {
                            *self = new_state;
                        }
                    }
                }
            }
            Menu(screen) => {
                if let Some(new_state) = screen.update(ctx) {
                    *self = new_state;
                }
            }
            Paused(_, _) => {}
            Exiting => {} // Menu(ref mut menu_state) => menu_state.update(ctx)?,
        }
        Ok(())
    }

    pub fn pressed(&mut self, ctx: &mut Context, key: KeyCode) {
        use GameState::*;
        if let Some(new_state) = match self {
            Playing(ref mut playing_state) => playing_state.pressed(ctx, key, None),
            PlayingOnline(ref mut playing_online_state, ref mut net) => {
                playing_online_state.pressed(ctx, key, net)
            }
            Paused(ref playing_state, ref mut paused_menu) => {
                paused_menu.pressed(key, playing_state)
            }
            Menu(ref mut menu_state) => menu_state.pressed(ctx, key),
            _ => None,
        } {
            *self = new_state;
        }
    }
}

fn btn_to_keycode(btn: Button) -> Option<KeyCode> {
    use Button::*;
    use KeyCode::*;
    match btn {
        DPadDown => Some(Down),
        DPadLeft => Some(Left),
        DPadRight => Some(Right),
        DPadUp => Some(Up),
        East => Some(Tab),
        South => Some(Space),
        LeftTrigger | RightTrigger => Some(Q),
        Start | Select => Some(Escape),
        _ => None,
    }
}
