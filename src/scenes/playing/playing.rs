use super::level::*;
use super::sidebar;
use crate::net::{self, Netinfo};
use crate::piece::{rot_steps::RotSteps, *};
use crate::scenes::{paused::PausedMenuState, playing_online::PlayingOnlineState};
use crate::util::{colors, music, types::*, Align, BetterText, OnHoldState, RngPieceQueue};
use crate::{block::*, field::*, game::GameState, get_win_dim};

use ggez::{event::KeyCode, graphics::*, timer::delta, *};
use rand::{thread_rng, Rng};

// const TICK_DELTA: f64 = 0.6;
const ADD_PIECE_DELAY: f64 = 0.11;
const ADD_PIECE_DELAY_DESTROYED: f64 = 0.74;
const DESTROY_DELTA: f64 = 0.3;
const DESTROY_HIDE_DELTA: f64 = 0.12;

#[derive(Clone, Debug)]
pub struct PlayingState {
    delta: f64,
    music: music::MusicInfo,
    leveling: LevelingType, // level and time to next
    field: PlayingField,
    piece: PieceState,
    next_pieces: RngPieceQueue,
    hold_piece: Option<HoldPiece>,
    score: usize,
    on_hold: Option<OnHoldState>,
    destroying_rows_indices: Option<(f64, Vec<usize>)>,
    garbage_to_add: usize,
    game_over: bool,
    online: bool,
}

impl PlayingState {
    pub fn new(ctx: &mut Context, online: bool) -> PlayingState {
        let next_pieces = RngPieceQueue::new(ctx).expect("Failed to generate piece queue");
        let leveling = if online {
            LevelingType::online()
        } else {
            LevelingType::single()
        };
        let music = music::MusicInfo::new(ctx).expect("Failed to load music fileds");
        PlayingState {
            delta: 0.,
            music,
            leveling,
            field: PlayingField::new(ctx),
            piece: NoPiece(0., 0.),
            next_pieces,
            hold_piece: None,
            score: 0,
            on_hold: None,
            destroying_rows_indices: None,
            garbage_to_add: 0,
            game_over: false,
            online,
        }
    }
    pub fn update(&mut self, ctx: &mut Context) -> GameResult<Option<GameState>> {
        if self.game_over {
            return Ok(None);
        }
        self.music.update();
        // self.music.theme.play();

        let dt = delta(ctx).as_secs_f64();
        self.delta += dt;

        if let LevelingType::Online(ref mut online) = self.leveling {
            online.update(dt);
        }

        match self.piece {
            Piece(ref piece) => {
                let mut piece = piece.clone();
                for _ in 0..(self.delta / self.leveling.get_gravity()).floor() as usize {
                    if !self.check_hold(&piece, dt) {
                        self.delta = 0.;
                        piece = if let Some(p) = self.step_piece(piece, Dir::Down, true) {
                            p
                        } else {
                            break;
                        }
                    } else {
                        // println!("holding");
                    }
                }
            }
            NoPiece(ref mut time_elapsed, ref time_to_wait) => {
                *time_elapsed += dt;

                if *time_elapsed > *time_to_wait {
                    self.add_new_piece(ctx)?;
                    self.delta = 0.;
                }
            }
        }

        // else {
        //     println!("spawn piece");
        //     self.piece = Some(self.next_pieces.get(ctx));
        // }
        // else {
        // }
        // }
        if let Some((mut ddelta, indices)) = self.destroying_rows_indices.clone() {
            ddelta += dt;

            if ddelta > DESTROY_DELTA {
                self.drop_blocks(indices);
                self.destroying_rows_indices = None;
            } else {
                if ddelta >= DESTROY_HIDE_DELTA && ddelta - dt < DESTROY_HIDE_DELTA {
                    self.animate_destroying_invis(&indices);
                }
                self.destroying_rows_indices = Some((ddelta, indices));
            }
        }
        if self.garbage_to_add > 0 {
            self.add_one_garbage_line();
            self.garbage_to_add -= 1;
        }
        Ok(None)
    }

    pub fn update_net(
        &mut self,
        delta: f64,
        prev_state: PlayingState,
        net: &mut Netinfo,
    ) -> Result<(), String> {
        net.delta(delta);
        if net.last_sent >= net::HEARTBEAT_INTERVAL {
            net.heartbeat();
        }
        if net.last_response >= net::TIMEOUT {
            return Err("Connection timed out".to_string());
        }
        if self.game_over && !prev_state.game_over {
            net.game_over();
        }
        if let Some(indices) = &self.destroying_rows_indices {
            if prev_state.destroying_rows_indices.is_none() {
                net.lines(indices.1.len());
            }
        }
        if self.field.height() != prev_state.field.height() {
            net.height(self.field.height());
        }
        Ok(())
    }

    pub fn pressed(
        &mut self,
        ctx: &mut Context,
        key: KeyCode,
        net: Option<&Netinfo>,
    ) -> Option<GameState> {
        use KeyCode::*;
        let mut ret = None;
        if self.game_over {
            match key {
                Space | Return => self.reset(ctx),
                Escape => ret = Some(GameState::default()),
                _ => {}
            }
        } else if let Piece(piece) = self.piece.clone() {
            match key {
                Up => {
                    self.step_piece(piece, Dir::Up, false);
                }
                Down => {
                    self.step_piece(piece, Dir::Down, false);
                }
                Left => {
                    self.step_piece(piece, Dir::Left, false);
                }
                Right => {
                    self.step_piece(piece, Dir::Right, false);
                }
                Space | Return => {
                    //  else {
                    self.rotate_piece(RDir::CClockwise);
                    // }
                }
                Tab => {
                    self.rotate_piece(RDir::Clockwise);
                }
                Q | J => {
                    self.swap_hold(ctx).expect("Failed to swap hold");
                }
                P | Escape => {
                    if let Some(net) = net {
                        ret = Some(GameState::PausedOnline(
                            PlayingOnlineState::Connected(self.clone()),
                            PausedMenuState::default(),
                            net.clone(),
                        ));
                    } else {
                        ret = Some(GameState::Paused(self.clone(), PausedMenuState::default()));
                    }
                }
                _ => {}
            }
        }
        self.check_hold_pressed();
        ret
    }

    pub fn draw(&mut self, ctx: &mut Context, font: Font) -> GameResult<()> {
        clear(ctx, colors::BG.into());
        self.field.draw(ctx)?;
        sidebar::draw(
            ctx,
            self.hold_piece.as_ref(),
            self.next_pieces.as_vec(),
            font,
        )?;

        if !self.game_over {
            if let LevelingType::Single(single) = &self.leveling {
                let lvl = single.get_level();
                let score = single.get_score();
                BetterText::new(&format!("Level {}", lvl.as_usize()), font)
                    .align(Align::TL(8., 8.))
                    .color(colors::LGREY)
                    .draw(ctx)?;

                BetterText::new(&format!("{}", score), font)
                    .align(Align::TR(8., 8.))
                    .color(colors::LGREY)
                    .draw(ctx)?;
            }
            let field_offset =
                FIELD_ALIGN.to_offset(get_win_dim(ctx), (FIELD_WIDTH_REAL, FIELD_HEIGHT_VIS_REAL));
            let field_inner_align = Align::TL(field_offset.0, field_offset.1);
            if let Piece(piece) = self.piece.clone() {
                {
                    let mut ghost_piece = piece.clone();
                    while !self.field.colliding(&ghost_piece) {
                        ghost_piece.step(Dir::Down);
                    }
                    ghost_piece.step(Dir::Up);
                    ghost_piece.draw(ctx, field_inner_align, false, PieceAppearance::Ghost)?;
                }
                piece.draw(ctx, field_inner_align, false, PieceAppearance::Normal)?;
            }
        } else {
            let (win_w, win_h) = crate::get_win_dim(ctx);
            Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0., 0., win_w, win_h),
                colors::BG_ERR_OVERLAY.into(),
            )?
            .draw(ctx, DrawParam::default())?;
            BetterText::new("GAME OVER", font)
                .color(colors::LRED)
                .size(23)
                .align(Align::CC(0., -50.))
                .draw(ctx)?;

            if let LevelingType::Single(single) = &self.leveling {
                let score = single.get_score();
                BetterText::new(&format!("score:{}", score), font)
                    .align(Align::BC(0., 30.))
                    .color(colors::LGREY)
                    .draw(ctx)?;
            }
        }

        Ok(())
    }

    pub fn add_garbage_lines(&mut self, amount: usize) {
        self.garbage_to_add += amount;
    }

    fn add_new_piece(&mut self, ctx: &mut Context) -> GameResult {
        let mut piece = self.next_pieces.get(ctx)?;
        piece.step(Dir::Down);
        match self.field.colliding_reason(&piece) {
            CollidingReason::None | CollidingReason::TopOut => {}
            _ => {
                piece.step(Dir::Up);
                self.game_over = match self.field.colliding_reason(&piece) {
                    CollidingReason::None | CollidingReason::TopOut => false,
                    _ => true,
                };
            }
        }
        // } else {
        self.piece = Piece(piece);
        // }
        Ok(())
    }

    fn add_one_garbage_line(&mut self) {
        let block = Block::new(BlockColor::Garbage);
        let mut row = [Some(block); FIELD_WIDTH];
        row[thread_rng().gen_range(0, FIELD_WIDTH)] = None;
        self.field.add_row(self.field.height() - 1, row);
    }

    /*fn remove_destroying(&mut self, indices: &Vec<usize>) {
        for index in indices {
            self.field.remove_row(*index);
        }
    }*/

    fn swap_hold(&mut self, ctx: &mut Context) -> GameResult {
        use HoldPiece::*;
        if let Piece(mut old_piece) = self.piece.clone() {
            if let Some(Unlocked(mut new_piece)) = self.hold_piece.clone() {
                new_piece.set_scaling(1.);
                self.piece = Piece(new_piece);
                // Reset piece state
                old_piece.reset();
                old_piece.set_scaling(HOLD_PIECE_SCALING);
                self.hold_piece = Some(Locked(old_piece));
            } else if self.hold_piece.is_none() {
                self.add_new_piece(ctx)?;
                old_piece.set_scaling(HOLD_PIECE_SCALING);
                self.hold_piece = Some(Locked(old_piece));
            }
        }
        Ok(())
    }

    fn animate_destroying_invis(&mut self, indices: &[usize]) {
        let row = [None; FIELD_WIDTH];
        for index in indices {
            self.field.set_row(*index, row);
        }
    }

    fn drop_blocks(&mut self, indices_unsorted: Vec<usize>) {
        let mut field_cpy = self.field.to_vec_shallow();
        let empty_row = [None; FIELD_WIDTH];
        // let mut rows_removed = 0;
        let mut indices = indices_unsorted;
        indices.sort();
        for index in indices {
            field_cpy.remove(index); //- rows_removed
            field_cpy.insert(0, empty_row.clone());
            // rows_removed += 1;
        }
        // for _ in 0..rows_removed {}
        for (y, row) in field_cpy.iter().enumerate() {
            self.field.set_row(y, *row);
        }
    }

    /// Returns the modified piece if it still exists
    fn step_piece(&mut self, piece: Piece, dir: Dir, lock: bool) -> Option<Piece> {
        // let mut piece = piece.clone();
        let mut piece = piece;
        let before_piece = piece.clone();
        // let mut mod_piece  = piece.clone();
        // if self.field.colliding(&before_piece) {
        //     println!("called step piece on piece that is already colliding!");
        // }

        // let maybe_mod_piece =
        match dir {
            Dir::Left | Dir::Right => {
                piece.step(dir);
                if !self.field.colliding(&piece) {
                    self.piece = Piece(piece.clone());
                    Some(piece)
                } else {
                    None
                }
            }
            Dir::Down => {
                piece.step(dir);
                // println!("stepped down. Offset now: {:?}", piece.offset);
                if !self.field.colliding(&piece) {
                    self.piece = Piece(piece.clone());
                    Some(piece)
                } else if lock {
                    self.add_piece(before_piece.clone());
                    // Some(before_piece)
                    None
                } else {
                    None
                }
            }
            Dir::Up => {
                while !self.field.colliding(&piece) {
                    piece.step(Dir::Down);
                    // println!("stepping fast");
                }
                piece.step(Dir::Up);
                self.add_piece(piece.clone());
                None
            }
        }
    }

    fn rotate_piece(&mut self, rdir: RDir) {
        // println!("rotating");
        if let Piece(mut piece) = self.piece.clone() {
            let start_rot = piece.get_rotation();
            let end_rot = piece.get_rotation() + rdir;
            piece.set_rotation(end_rot.clone());

            for step_dir_pairs in piece.rot_steps(start_rot, end_rot).iter() {
                let mut moved_piece = piece.clone();
                for step_dir in step_dir_pairs {
                    if let Some(dir) = step_dir {
                        moved_piece.step(*dir);
                    }
                }
                // println!("trying...");
                if !self.field.colliding(&moved_piece) {
                    self.piece = Piece(moved_piece);
                    // println!("Rot worked with offset {:?}", step_dir_pairs);
                    return;
                }
            }
            // println!("didnt work");
        }
    }

    fn reset(&mut self, ctx: &mut Context) {
        *self = PlayingState::new(ctx, self.online);
    }

    fn add_piece(&mut self, piece: Piece) {
        use HoldPiece::*;
        let mut should_game_over = false;
        if let Some(Locked(piece)) = &self.hold_piece {
            self.hold_piece = Some(Unlocked(piece.clone()));
        }

        if self.field.add_piece(piece) {
            // if overlapping {
            //     println!("game over because of overlapping");
            //     self.game_over = true;
            // } else {
            //
            should_game_over = true;
            // }
        }

        let destroyed_rows_indices = self.check_rows_destroying();
        let time_to_wait = if destroyed_rows_indices.is_empty() {
            ADD_PIECE_DELAY
        } else {
            ADD_PIECE_DELAY_DESTROYED
        };
        self.piece = PieceState::NoPiece(0., time_to_wait);
        if should_game_over && destroyed_rows_indices.is_empty() {
            // top out with no rows about to be destroyed / marked destroying
            self.game_over = true;
        }
    }

    /// Call once when a key is pressed
    fn check_hold_pressed(&mut self) {
        if let Piece(mut piece) = self.piece.clone() {
            if !self.field.colliding(&piece) {
                piece.step(Dir::Down);
                if self.field.colliding(&piece) {
                    if let Some(ref mut ohd) = self.on_hold {
                        ohd.pressed();
                    } else {
                        self.on_hold = Some(OnHoldState::new());
                    }
                }
            }
        }
    }

    /// returns whether game should hold the piece
    fn check_hold(&mut self, piece: &Piece, delta: f64) -> bool {
        // if let Piece(mut piece) = self.piece.clone() {
        let mut piece = piece.clone();
        piece.step(Dir::Down);
        // let would_collide = match self.field.colliding_reason(&piece) {
        //     CollidingReason::None => false,
        //     CollidingReason::HitFloor => return false,
        //     _ => true,
        // };
        let would_collide = self.field.colliding(&piece);

        if let Some(ref mut ohds) = self.on_hold {
            if would_collide {
                if ohds.hold_exceeded(delta) {
                    // println!(
                    //     "exceeded hold. moves: {}, last_time: {}, total_time: {}",
                    //     ohds.moves_made,
                    //     ohds.last_time_passed + delta,
                    //     ohds.total_time_passed
                    // );
                    println!("Hold exceeded");
                    self.on_hold = None;
                    return false;
                } else {
                    //increased hold
                    // self.on_hold = Some(ohds.update(delta));
                    ohds.update(delta);
                    // println!("Holding (state: )");
                    return true;
                }
            } else {
                self.on_hold = None;
                return false;
            }
        // } else {
        //     // self.on_hold = Some(0.);
        //     return false;
        // }

        // self.on_hold = Some(0.);
        // return true;
        // } else {
        // not holding, no action. Hold gets initiated within keypress
        // self.on_hold = None;
        // return false;
        } else if would_collide {
            // println!("holding automatically");
            self.on_hold = Some(OnHoldState::new());
            return true;
        }
        false
        // } else {
        // println!("no piece, no hold");
        // }
        // return false;
    }

    /// returns Vec <index of rows that need to be destroyed>
    fn check_rows_destroying(&mut self) -> Vec<usize> {
        let mut row_indices_to_destroy: Vec<usize> = vec![];
        let field_cpy = self.field.clone();
        let row_destroying = [Some(Block::new(BlockColor::Destroying)); FIELD_WIDTH];

        for (y, row) in field_cpy.to_vec().iter().enumerate() {
            if !row.contains(&None) {
                row_indices_to_destroy.push(y);
                self.field.set_row(y, row_destroying.clone());
            }
        }

        if !row_indices_to_destroy.is_empty() {
            self.destroyed_lines(row_indices_to_destroy.len() as usize);
            self.destroying_rows_indices = Some((0., row_indices_to_destroy.clone()));
        }
        row_indices_to_destroy
    }

    fn destroyed_lines(&mut self, lines: usize) {
        if let LevelingType::Single(ref mut single) = self.leveling {
            single.cleared_lines(lines);
        }
    }
}
