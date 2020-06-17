mod block;
mod field;
mod game;
mod net;
mod piece;
mod scenes;
mod util;

extern crate rand;

use block::BLOCK_SIZE;
use field::*;
use game::*;
use ggez::{
    conf, event,
    graphics::{screen_coordinates, Font},
    Context, ContextBuilder,
};
use scenes::playing::sidebar::SIDEBAR_WIDTH;

const BOTTOM_MARGIN: f32 = 40.;
pub const DEFAULT_WINDOW_WIDTH: f32 = FIELD_WIDTH as f32 * BLOCK_SIZE + FIELD_OFF.0 + SIDEBAR_WIDTH;
pub const DEFAULT_WINDOW_HEIGHT: f32 =
    (FIELD_HEIGHT_VIS + 1) as f32 * BLOCK_SIZE + FIELD_OFF.1 + BOTTOM_MARGIN;

fn main() {
    let resource_dir = std::path::PathBuf::from("./resources");
    let (mut ctx, mut event_loop) = ContextBuilder::new("tetro-98", "ffactory")
        .window_setup(conf::WindowSetup {
            title: "Tetro 98!".into(),
            icon: "/icon.ico".into(),
            samples: conf::NumSamples::Two,
            ..Default::default()
        })
        .window_mode(conf::WindowMode {
            min_width: DEFAULT_WINDOW_WIDTH,
            min_height: DEFAULT_WINDOW_HEIGHT,
            width: DEFAULT_WINDOW_WIDTH,
            height: DEFAULT_WINDOW_HEIGHT,
            resizable: true,
            ..Default::default()
        })
        .add_resource_path(resource_dir)
        .build()
        .expect("Could not create ggez context!");

    let font = Font::new(&mut ctx, "/imagine.ttf") //&std::path::Path::new("./resources/imagine.ttf"))
        .expect("Could not load font!");
    // "C:\\Development/rust/ggez-test/resources/imagine.ttf",

    let mut my_game = Game::new(&mut ctx, font).expect("Could not initialize game");

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Bye!"),
        Err(e) => println!("Error occured: {}", e),
        // _ => {}
    }
}

fn get_win_dim(ctx: &mut Context) -> (f32, f32) {
    let [_, _, w, h]: [f32; 4] = screen_coordinates(ctx).into();
    (w, h)
}
