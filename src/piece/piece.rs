pub use super::abstraction::PieceShape;
use crate::block::*;
use crate::field::FIELD_WIDTH;
use crate::get_win_dim;
use crate::util::types::*;
use crate::util::Align;
use ggez::graphics::*;
use ggez::*;
pub use PieceState::*;

#[derive(Clone, Debug)]
pub struct Piece {
    // fields: Vec<(usize, usize)>,
    pub shape: PieceShape,
    pub offset: GridPoint,
    rotation: RotationState,
    scaling: f32,
    block: Block,
    meshes: PieceMeshes,
}

#[derive(Clone, Debug)]
struct PieceMeshes {
    normal: Mesh,
    ghost: Mesh,
    blocked: Mesh,
}

#[derive(Clone, Debug)]
pub enum PieceAppearance {
    Normal,
    Ghost,
    Blocked,
}

impl Piece {
    pub fn new(ctx: &mut Context, shape: PieceShape, scaling: f32) -> GameResult<Piece> {
        let block = Block::new(shape.block_color()); //, scaling

        // let mesh = if let Some(ctx) = ctx {
        // Some(
        // println!("calling gen_mesh in Piece::new");
        let meshes = PieceMeshes {
            normal: block.gen_mesh(ctx, 0)?,
            ghost: block.gen_ghost_mesh(ctx)?,
            blocked: Block::new(BlockColor::Blocked).gen_mesh(ctx, 0)?,
        };
        // } else {
        //     None
        // };
        let offset = Self::default_offset(&shape);
        Ok(Piece {
            shape,
            offset,
            rotation: RotationState::Rs0,
            scaling,
            block,
            meshes,
        })
    }

    pub fn reset(&mut self) {
        self.offset = Self::default_offset(&self.shape);
        self.rotation = RotationState::Rs0;
    }

    pub fn get_rotation(&self) -> RotationState {
        self.rotation.clone()
    }

    pub fn set_rotation(&mut self, rot: RotationState) {
        self.rotation = rot;
    }

    pub fn set_scaling(&mut self, scaling: f32) {
        //, ctx: &mut Context
        // if (self.block.get_scaling() - scaling).abs() < 10e-5 {
        // println!("calling gen_mesh in set_scaling. Need to update mesh w scale");
        // self.block.set_scaling(scaling);
        // self.mesh = self.block.gen_mesh(ctx, "changing scaling")?;
        // self.ghost_mesh = self.block.gen_ghost_mesh(ctx)?;
        self.scaling = scaling;
        // }
        // Ok(())
    }

    /// Returns the size of a single block. Result of multiplying scaling factor by block size
    pub fn get_block_size(&self) -> f32 {
        BLOCK_SIZE * self.scaling
    }

    /// Returns the fields of the piece as Vec<(isize, isize)>.
    /// If offset is specified it uses that, else uses self.offset.
    pub fn fields_tuple_offset(&self, offset: Option<(isize, isize)>) -> Vec<(isize, isize)> {
        let offset = offset.unwrap_or_else(|| self.offset.into());
        self.shape.as_field(&self.rotation).as_vec_offset(offset)
    }

    // pub fn fields_tuple_adj(&self) -> Vec<(isize, isize)> {
    //     self.fields_tuple_offset((0,0))
    // }

    // pub fn fields_tuple(&self) -> Vec<(isize, isize)> {
    //     self.fields_tuple_offset(self.offset.into())
    // }

    /// Returns Vec<GridPoint> with the piece's offset from top left.
    pub fn fields_gp(&self) -> Vec<GridPoint> {
        self.fields_tuple_offset(None)
            .iter()
            .map(|(x, y)| GridPoint::from((*x, *y)))
            .collect()
    }

    pub fn step(&mut self, dir: Dir) {
        self.offset.step(dir, 1);
    }
    pub fn color(&self) -> BlockColor {
        self.shape.block_color()
    }
    pub fn width(&self) -> usize {
        self.shape.width()
    }

    // pub fn mesh(&self) -> &Mesh {
    //     // Ok(if let Some(ref mesh) = self.mesh {
    //     //     mesh.clone()
    //     // } else {
    //     //     let mesh = self.block.gen_mesh(ctx)?;
    //     //     // self.mesh = Some(mesh.clone());
    //     //     mesh
    //     // })
    //     &self.mesh
    // }

    #[allow(dead_code)]
    pub fn size(&self) -> f32 {
        BLOCK_SIZE * self.scaling * self.shape.width() as f32
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        align: Align,
        ignore_offset: bool,
        appearance: PieceAppearance,
    ) -> GameResult<()> {
        let mesh = match appearance {
            PieceAppearance::Normal => &self.meshes.normal,
            PieceAppearance::Ghost => &self.meshes.ghost,
            PieceAppearance::Blocked => &self.meshes.blocked,
        };

        let block_size = self.get_block_size();
        let window_offset: (f32, f32) = {
            // let mesh_dimensions = mesh.dimensions(ctx).unwrap();
            let mesh_dimensions = (
                block_size * self.shape.width() as f32,
                block_size * self.shape.width() as f32,
            );
            // println!("mesh dimensions: {:?}", mesh_dimensions);
            align.to_offset(get_win_dim(ctx), mesh_dimensions)
        };
        // println!("window off: {:?}", window_offset);
        // if let Some(align) = align {
        // } else {
        //     (0., 0.)
        // };
        // println!("piece off: {:?}", window_offset);

        let fields = {
            let tuple_offset = if ignore_offset { Some((0, 0)) } else { None };
            self.fields_tuple_offset(tuple_offset)
        };
        // println!("piece fields: {:?}", fields);
        // if ghost {
        //     println!("Ghost start");
        // }
        for (x, y) in fields {
            // if ghost {
            //     print!("Ghost (scale {}) off: {:?} -> ", self.scaling, (x, y));
            // }
            let (x, y) = (x as f32 * block_size, y as f32 * block_size);
            let mesh_offset = [x + window_offset.0, y + window_offset.1];
            // if ghost {
            //     println!("{:?}", mesh_offset);
            // }
            // let c: u8 = (x as u8).saturating_mul(25);
            let param = DrawParam::default()
                .dest(mesh_offset)
                .scale([self.scaling; 2]);
            mesh.draw(ctx, param)?;
            // mesh.draw(
            //     ctx,
            //     DrawParam::default().dest([x, y]).scale([self.scaling; 2]), // .color((c, c, c, 255).into()),
            // )?;
        }

        Ok(())
    }

    /// Returns important fields in a tuple, then drops itself
    /// Return: (fields_tuple, block_color, block, Option<mesh>)
    pub fn deconstruct(self) -> (Vec<(isize, isize)>, BlockColor, Block, Mesh) {
        (
            self.fields_tuple_offset(None),
            self.color(),
            self.block,
            self.meshes.normal,
        )
    }

    fn default_offset(shape: &PieceShape) -> GridPoint {
        let x = (FIELD_WIDTH as f32 / 2. - shape.width() as f32 / 2.).round() as isize;
        // let y = FIELD_HEIGHT_VIS as isize - (shape.width() as f32 / 2.).floor() as isize;
        let y = -1;
        (x, y).into()
    }
}

#[derive(Clone, Debug)]
pub enum PieceState {
    // NoPiece(f64), // time waiting
    NoPiece(f64, f64), // time waiting, time to wait
    Piece(Piece),
}

#[derive(Clone, Debug)]
pub enum RotationState {
    Rs0,
    Rs1,
    Rs2,
    Rs3,
}

impl RotationState {
    #[allow(dead_code)]
    pub fn rotated(&self, rdir: RDir) -> RotationState {
        use RDir::*;
        use RotationState::*;
        match rdir {
            Clockwise => match self {
                Rs0 => Rs1,
                Rs1 => Rs2,
                Rs2 => Rs3,
                Rs3 => Rs0,
            },

            CClockwise => match self {
                Rs0 => Rs3,
                Rs1 => Rs0,
                Rs2 => Rs1,
                Rs3 => Rs2,
            },
        }
    }

    pub fn count(&self) -> usize {
        use RotationState::*;
        match self {
            Rs0 => 0,
            Rs1 => 1,
            Rs2 => 2,
            Rs3 => 3,
        }
    }
}

impl std::ops::Add<RDir> for RotationState {
    type Output = Self;
    fn add(self, other: RDir) -> Self::Output {
        use RDir::*;
        use RotationState::*;
        match other {
            Clockwise => match self {
                Rs0 => Rs1,
                Rs1 => Rs2,
                Rs2 => Rs3,
                Rs3 => Rs0,
            },

            CClockwise => match self {
                Rs0 => Rs3,
                Rs1 => Rs0,
                Rs2 => Rs1,
                Rs3 => Rs2,
            },
        }
    }
}
