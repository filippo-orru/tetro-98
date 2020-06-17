use ggez::event::KeyCode;
use ggez::mint::Point2;
use std::ops::Mul;

#[derive(Clone, Debug, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum RDir {
    Clockwise,
    CClockwise,
}

#[derive(Copy, Clone, Debug)]
pub struct GridPoint {
    pub x: isize,
    pub y: isize,
}

impl GridPoint {
    // pub fn with_offset_gp(&self, offset: GridPoint) -> GridPoint {
    //     Self {
    //         x: self.x + offset.x,
    //         y: self.y + offset.y,
    //     }
    // }

    /*/// Converts grid point with offset to Point2 used for rendering
    /// **Attention: Flips the y-axis!** Not!
    pub fn with_offset(&self, (x, y): (f32, f32)) -> Point2<f32> {
        Point2 {
            x: self.x as f32 + x,
            y: self.y as f32 + y,
        }
    }*/

    pub fn step(&mut self, dir: Dir, steps: usize) {
        use Dir::*;
        let steps = steps as isize;
        match dir {
            Down => self.y += steps,
            Up => self.y -= steps,
            Left => self.x -= steps,
            Right => self.x += steps,
        }
    }

    // pub fn to_real(&self, block_size: f32) -> [f32; 2] {
    //     //offset: GridPoint
    //     // println!("to_real, self: {}, off: {}", self.y, offset.y);
    //     [
    //         self.x as f32 * block_size,
    //         self.y as f32 * block_size, // (FIELD_HEIGHT_VIS as isize - self.y) as f32 * block_size,
    //     ]
    // }
}

impl Mul<isize> for GridPoint {
    type Output = GridPoint;
    fn mul(mut self, other: isize) -> Self::Output {
        self.x *= other;
        self.y *= other;
        self
    }
}

impl Mul<(isize, isize)> for GridPoint {
    type Output = GridPoint;
    fn mul(mut self, other: (isize, isize)) -> Self::Output {
        self.x *= other.0;
        self.y *= other.1;
        self
    }
}

impl Mul<f32> for GridPoint {
    type Output = (f32, f32);
    fn mul(self, other: f32) -> Self::Output {
        (self.x as f32 * other, self.y as f32 * other)
    }
}

impl Mul<(f32, f32)> for GridPoint {
    type Output = (f32, f32);
    fn mul(self, other: (f32, f32)) -> Self::Output {
        (self.x as f32 * other.0, self.y as f32 * other.1)
    }
}

// impl From<(f32,f32)> for Point2<f32> {
//     fn from(self) -> Point2<f32> {
//         Point2 { x: self}
//     }
// }

impl From<(isize, isize)> for GridPoint {
    fn from((x, y): (isize, isize)) -> GridPoint {
        GridPoint { x, y }
    }
}
impl From<(usize, usize)> for GridPoint {
    fn from((x, y): (usize, usize)) -> GridPoint {
        GridPoint {
            x: x as isize,
            y: y as isize,
        }
    }
}

impl Into<Point2<f32>> for GridPoint {
    fn into(self) -> Point2<f32> {
        Point2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl Into<(f32, f32)> for GridPoint {
    fn into(self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }
}

impl Into<(isize, isize)> for GridPoint {
    fn into(self) -> (isize, isize) {
        (self.x, self.y)
    }
}

impl Into<[f32; 2]> for GridPoint {
    fn into(self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }
}

#[derive(Clone, Debug)]
pub struct KeypressInfos {
    kpis: Vec<KeypressInfo>,
}

impl KeypressInfos {
    pub fn empty() -> Self {
        Self { kpis: vec![] }
    }
    pub fn contains(&self, key: KeyCode) -> bool {
        self.keys().contains(&key)
    }
    pub fn push(&mut self, key_info: KeypressInfo) {
        self.kpis.push(key_info);
    }
    pub fn pop_if_exists(&mut self, key: KeyCode) {
        if self.contains(key) {
            if let Some(i) = self.keys().iter().position(|&k| k == key) {
                self.kpis.remove(i);
            }
        }
    }
    pub fn kpis(&self) -> Vec<KeypressInfo> {
        self.kpis.clone()
    }

    pub fn set(&mut self, kpis: Vec<KeypressInfo>) {
        self.kpis = kpis;
    }

    fn keys(&self) -> Vec<KeyCode> {
        self.kpis
            .iter()
            .map(|kpi| kpi.key)
            .collect::<Vec<KeyCode>>()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeypressInfo {
    pub delta: f64,
    pub key: KeyCode,
    pub repeat_count: usize,
}

impl KeypressInfo {
    pub fn new(key: KeyCode) -> Self {
        KeypressInfo {
            delta: 0.,
            key,
            repeat_count: 1,
        }
    }
}

pub trait ToResTString<T, E>
where
    E: std::error::Error,
{
    fn to_str_err(self) -> Result<T, String>;
}

impl<T, E> ToResTString<T, E> for Result<T, E>
where
    E: std::error::Error,
    // T: Clone,
    // E: Clone,
{
    fn to_str_err(self) -> Result<T, String> {
        self.or_else(|e| Err(format!("{}", e)))
    }
}
