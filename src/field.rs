use crate::block::{Block, BlockColor, BLOCK_SIZE};
use crate::get_win_dim;
use crate::piece::Piece;
use crate::util::{colors, Align};
use ggez::graphics::*;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::convert::TryInto;

pub const FIELD_WIDTH: usize = 10;
pub const FIELD_HEIGHT: usize = 32;
pub const FIELD_HEIGHT_VIS: usize = 20;

pub const FIELD_WIDTH_REAL: f32 = FIELD_WIDTH as f32 * BLOCK_SIZE;
pub const FIELD_HEIGHT_REAL: f32 = FIELD_HEIGHT as f32 * BLOCK_SIZE;
pub const FIELD_HEIGHT_VIS_REAL: f32 = FIELD_HEIGHT_VIS as f32 * BLOCK_SIZE;

pub const FIELD_OFF: (f32, f32) = (BLOCK_SIZE * 4., BLOCK_SIZE);
pub const FIELD_ALIGN: Align = Align::CC(0., 0.);
// pub const FIELD_ALIGN: Align = Align::TL(FIELself.current_source.try_borrow_mut().unwrap().D_OFF.0, FIELD_OFF.1);

const GRID_WIDTH: f32 = 1.;

#[derive(Clone, Debug)]
pub struct PlayingField {
    field: [[Option<Block>; FIELD_WIDTH]; FIELD_HEIGHT],
    meshes: HashMap<BlockColor, Mesh>,
    grid_line_mesh_v: Mesh,
    grid_line_mesh_h: Mesh,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum CollidingReason {
    None,
    LeaveLeft,
    LeaveRight,
    Overlap,
    TopOut,
    HitFloor,
}

impl PlayingField {
    pub fn new(ctx: &mut Context) -> PlayingField {
        let grid_line_mesh_v = Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(Default::default()),
            Rect::new(
                -GRID_WIDTH / 2.,
                -GRID_WIDTH / 2.,
                GRID_WIDTH,
                FIELD_HEIGHT_VIS as f32 * BLOCK_SIZE,
            ),
            colors::BG_LINE_COLOR.into(),
        )
        .unwrap();
        let grid_line_mesh_h = Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(Default::default()),
            Rect::new(
                -GRID_WIDTH / 2.,
                -GRID_WIDTH / 2.,
                FIELD_WIDTH as f32 * BLOCK_SIZE,
                GRID_WIDTH,
            ),
            colors::BG_LINE_COLOR.into(),
        )
        .unwrap();

        PlayingField {
            field: [[None; FIELD_WIDTH]; FIELD_HEIGHT],
            meshes: HashMap::new(),
            grid_line_mesh_v,
            grid_line_mesh_h,
        }
    }

    // /// Gets cell with offset in TL corner translated to field offset
    // pub fn get_cell(&self, x: usize, y: usize) -> Option<Block> {
    //     self.field[y][x]
    // }

    // pub fn set_cell(&mut self, x: usize, y: usize, item: Option<Block>) {
    //     self.field[y][x] = item;
    // }

    pub fn set_row(&mut self, y: usize, row: [Option<Block>; FIELD_WIDTH]) {
        self.field[y] = row;
    }

    /// Freezes piece to field and returns whether should game_over
    /// Returns whether blocks overlap (should game over)
    pub fn add_piece(&mut self, piece: Piece) -> bool {
        let (fields, _, block, _) = piece.deconstruct();
        let mut should_game_over = false;
        // self.meshes
        //     .insert(color, block.gen_mesh(ctx, -40).unwrap();

        for (x, y) in fields {
            let y = if let Some(y) = PlayingField::adj_y(y) {
                y
            } else {
                panic!("field.add_piece: y < 0!");
            };
            if x < 0 {
                panic!("field.add_piece: x < 0!");
            }
            let x = x as usize;
            // let (x, y) = (x as usize, y as usize);
            if y < FIELD_HEIGHT - FIELD_HEIGHT_VIS {
                should_game_over = true; // exceeding height
            }

            if self.field[y][x].is_some() {
                // should_game_over = Some(true); // overlapping
                // panic!("overlapping blocks in field!");
                should_game_over = true;
            } else {
                self.field[y][x] = Some(block);
            }
        }
        should_game_over
    }

    pub fn to_vec(&self) -> Vec<Vec<Option<Block>>> {
        self.field.iter().map(|r| r.to_vec()).collect()
    }

    pub fn to_vec_shallow(&self) -> Vec<[Option<Block>; FIELD_WIDTH]> {
        self.field.to_vec()
    }
    pub fn width(&self) -> usize {
        self.field[0].len()
    }

    pub fn height(&self) -> usize {
        self.field.len()
    }

    /*pub fn to_real(&self) -> Vec<(f32, f32, Block)> {
        let mut vec = vec![];
        for (y, row) in self
            .field
            .iter()
            .enumerate()
            .map(|(y, row)| (FIELD_HEIGHT_VIS - y) as f32)
        {
            for (x, block) in row
                .iter()
                .enumerate()
                .filter_map(|(x, b)| if b.is_some() {Some(())})
                .collect()
            {
                vec.push((x, y));
            }
        }
        return vec;
        // .map(|(_, row))
    }*/

    pub fn colliding(&self, piece: &Piece) -> bool {
        self.colliding_reason(piece) != CollidingReason::None
    }

    pub fn colliding_reason(&self, piece: &Piece) -> CollidingReason {
        let width = self.width();
        for (x, y) in piece.fields_tuple_offset(None) {
            let field_y = if let Some(y) = Self::adj_y(y) {
                y
            } else {
                return CollidingReason::HitFloor;
            };
            if x < 0 {
                return CollidingReason::LeaveLeft;
            }
            let x = x as usize;
            if x + 1 > width {
                return CollidingReason::LeaveRight;
            } else if field_y >= FIELD_HEIGHT {
                return CollidingReason::TopOut;
            } else if self.field[field_y][x].is_some() {
                return CollidingReason::Overlap;
            }
        }
        CollidingReason::None
    }

    fn adj_y(y: isize) -> Option<usize> {
        ((FIELD_HEIGHT - FIELD_HEIGHT_VIS) as isize + y) //- 1
            .try_into()
            .ok()
        // .unwrap_or(0)
    }

    #[allow(dead_code)]
    pub fn colliding_print(&self, piece: &Piece) -> bool {
        // let width = self.width();
        let fields = piece.fields_tuple_offset(None);
        println!("fields: {:?}", fields);
        self.colliding(piece)
    }

    pub fn add_row(&mut self, y: usize, row: [Option<Block>; FIELD_WIDTH]) {
        let mut field = self.to_vec_shallow();
        field.insert(y, row);
        field.remove(0);
        self.field_from_vec_shallow(field);
    }

    /*pub fn remove_row(&mut self, y: usize) {
        let mut field = self.to_vec_shallow();
        let empty_row = [None; FIELD_WIDTH];
        field[y] = empty_row;
        field.remove(0);
        self.field_from_vec_shallow(field);
    }

    pub fn set_row_destroying(&mut self, y: usize) {
        let destroying_block = Block::new(BlockColor::Destroying, 1.);
        for row in self.field[y].iter_mut() {
            *row = Some(destroying_block.clone());
        }
    }*/

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (window_width, window_height) = get_win_dim(ctx);
        let offset = FIELD_ALIGN.to_offset(
            (window_width, window_height),
            (
                FIELD_WIDTH as f32 * BLOCK_SIZE,
                FIELD_HEIGHT_VIS as f32 * BLOCK_SIZE,
            ),
        );
        for i in 0..=FIELD_WIDTH {
            let mut dp = DrawParam::default().dest([offset.0 + i as f32 * BLOCK_SIZE, offset.1]);
            if i == 0 || i == FIELD_WIDTH {
                dp = dp.color(colors::LGREY.into());
            }
            self.grid_line_mesh_v.draw(ctx, dp)?;
        }
        for i in 0..=FIELD_HEIGHT_VIS {
            let mut dp = DrawParam::default().dest([offset.0, offset.1 + i as f32 * BLOCK_SIZE]);
            if i == 0 || i == FIELD_HEIGHT_VIS {
                dp = dp.color(colors::LGREY.into());
            }
            self.grid_line_mesh_h.draw(ctx, dp)?;
        }

        // println!("field off: {:?}", offset);
        for (y, row) in self.field[FIELD_HEIGHT - FIELD_HEIGHT_VIS..]
            .iter()
            .enumerate()
        {
            for (x, maybe_block) in row.iter().enumerate() {
                if let Some(block) = maybe_block {
                    let mesh = if let Some(mesh) = self.meshes.get(&block.block_color()) {
                        mesh.clone()
                    } else {
                        // println!("calling gen_mesh in draw. Not yet in hashmap");
                        let mesh = block.gen_mesh(ctx, -40)?;
                        self.meshes.insert(block.block_color(), mesh.clone());
                        mesh
                    };
                    let offset = DrawParam::default().dest([
                        x as f32 * BLOCK_SIZE + offset.0,
                        y as f32 * BLOCK_SIZE + offset.1,
                    ]);
                    // println!("offset: {:?}", offset);
                    mesh.draw(ctx, offset)?;
                }
            }
        }
        Ok(())
    }

    fn field_from_vec_shallow(&mut self, field: Vec<[Option<Block>; FIELD_WIDTH]>) {
        for (y, row) in field.iter().enumerate() {
            self.set_row(y, *row);
        }
    }
}
