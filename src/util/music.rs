use ggez::{
    audio::{SoundSource, Source},
    Context, GameResult,
};
use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
// use std::sync::Arc;

type ASource = Rc<RefCell<Source>>;

const PAUSED_VOL: f32 = 0.3;

#[derive(Clone, Debug)]
pub struct MusicInfo {
    current_source: ASource,
    sources: MusicSources,
    playing: bool,
}

impl MusicInfo {
    pub fn new(ctx: &mut Context) -> GameResult<MusicInfo> {
        let f = |ctx: &mut Context, s| -> GameResult<ASource> {
            Ok(Rc::new(RefCell::new(Source::new(ctx, s)?)))
        };
        let sources = MusicSources {
            theme_a: f(ctx, "/audio/music/theme_a.ogg")?,
            theme_b: f(ctx, "/audio/music/theme_a.ogg")?,
        };
        Ok(MusicInfo {
            current_source: Rc::clone(&sources.theme_a),
            sources,
            playing: false,
        })
    }
    pub fn update(&mut self) {
        if !self.playing {
            // let mut cs = self.current_source.try_borrow_mut().unwrap();
            // cs.set_repeat(true);
            // cs.play();
            // self.playing = true;
        }
    }
    pub fn pause_menu(&self) {
        self.current_source
            .try_borrow_mut()
            .unwrap()
            .set_volume(PAUSED_VOL);
    }
    pub fn unpause(&self) {
        self.current_source.try_borrow_mut().unwrap().set_volume(1.);
    }
}

#[derive(Clone, Debug)]
struct MusicSources {
    theme_a: ASource,
    theme_b: ASource,
}
