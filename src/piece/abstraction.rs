use super::*;
#[derive(Clone, Debug, PartialEq)]
pub enum PieceShape {
    T,
    L,
    J,
    S,
    Z,
    O,
    I,
}

impl PieceShape {
    pub fn all() -> Vec<PieceShape> {
        use PieceShape::*;
        vec![T, L, J, Z, S, O, I]
        // vec![O, I]
    }
    pub fn block_color(&self) -> BlockColor {
        use super::BlockColor::*;
        use PieceShape::*;
        match self {
            T => Purple,
            L => Orange,
            J => Blue,
            S => Green,
            Z => Red,
            O => Yellow,
            I => Cyan,
        }
        // .into()
    }

    pub fn as_field(&self, rotation: &RotationState) -> PieceField {
        let mut field = PieceField::new(self.bitmap());
        for _ in 0..rotation.count() {
            field.rotate();
        }
        field
    }

    pub fn width(&self) -> usize {
        use PieceShape::*;
        match self {
            O => 2,
            J | L | S | Z | T => 3,
            I => 4,
        }
    }
    fn bitmap(&self) -> Vec<bool> {
        use PieceShape::*;
        match self {
            T => vec![0, 1, 0, 1, 1, 1, 0, 0, 0],
            L => vec![0, 0, 1, 1, 1, 1, 0, 0, 0],
            J => vec![1, 0, 0, 1, 1, 1, 0, 0, 0],
            S => vec![0, 1, 1, 1, 1, 0, 0, 0, 0],
            Z => vec![1, 1, 0, 0, 1, 1, 0, 0, 0],
            O => vec![1, 1, 1, 1],
            I => vec![0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        }
        .iter()
        .map(|n| n != &0)
        .collect()
    }

    /*pub fn fields(&self) -> Vec<GridPoint> {
        self.fields_tuple()
            .iter()
            .map(|t| GridPoint::from(*t))
            .collect()
    }

    pub fn fields_off(&self, offset: GridPoint) -> Vec<GridPoint> {
        self.fields()
            .iter()
            .map(|gp: &GridPoint| gp.with_offset_gp(offset.into()))
            .collect()
    }

    pub fn fields_off_tuple(&self, offset: (usize, usize)) -> Vec<(usize, usize)> {
        self.fields_tuple()
            .iter()
            .map(|(x, y)| (x + offset.0, y + offset.1))
            .collect()
    }*/
}

pub struct PieceField {
    size: usize,
    field: Vec<bool>,
}

impl PieceField {
    pub fn new(field: Vec<bool>) -> Self {
        let size = (field.len() as f64).sqrt() as usize;
        // let mut field = in_field.clone();
        // for _ in 0..rotation.count() {

        // field[y * size..(y + 1) * size]
        //     .iter()
        //     .for_each(|b| print!("{}", if *b { "X" } else { "O" }));
        // .collect::<Vec<_>>()
        // println!("");
        // }
        // println!("+rot+");
        // }
        // println!("rot done");
        PieceField { size, field }
    }

    pub fn rotate(&mut self) {
        let mut field = vec![false; self.size * self.size];
        for (y, row) in self.as_2d().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                // field[x * size + (size - 1 - y)]
                // if cell {
                field[x * self.size + (self.size - 1 - y)] = *cell;
                // }
                // field[y * size + x] = in_field[(size - 1 - x) * size + y];
            }
        }
        self.field = field;
    }

    /// Returns the vector of point positions with offset and anchor in TL corner
    pub fn as_vec_offset(&self, (x_off, y_off): (isize, isize)) -> Vec<(isize, isize)> {
        // let (x_off, y_off): (isize, isize) = gp.into();
        let mut vec: Vec<(isize, isize)> = vec![];
        for y in 0..self.size {
            for x in 0..self.size {
                if self.field[self.size * y + x] {
                    vec.push((x as isize + x_off, y_off + y as isize));
                }
            }
        }
        vec
    }

    /// Returns [[bool]] with anchor on TL corner
    pub fn as_2d(&self) -> Vec<Vec<bool>> {
        let mut vec: Vec<Vec<bool>> = vec![];
        for i in 0..self.size {
            vec.push(self.field[i * self.size..(i + 1) * self.size].to_vec());
        }
        vec
    }
}
