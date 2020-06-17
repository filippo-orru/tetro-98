use std::ops::*;

#[derive(Clone, Debug)]
pub enum LevelingType {
    Online(Online),
    Single(Singleplayer),
}

impl LevelingType {
    pub fn single() -> LevelingType {
        LevelingType::Single(Singleplayer::new())
    }
    pub fn online() -> LevelingType {
        LevelingType::Online(Online::new())
    }

    // pub fn get_score(&self, lines: usize) -> usize {
    //     if let

    // pub fn cleared_lines(&mut self, amount: usize) {
    //     if let LevelingType::Single(ref mut single) = self {
    //         single.cleared_lines(amount);
    //     }
    // }

    // pub fn update(&mut self, dt: f64) {
    //     if let LevelingType::Online(ref mut online) = self {
    //         online.update(dt);
    //     }
    // }

    pub fn get_gravity(&self) -> f64 {
        use LevelingType::*;
        match self {
            Single(single) => single.get_gravity(),
            Online(online) => online.get_gravity(),
        }
    }

    // pub fn get_level(&self) -> usize {
    //     use LevelingType::*;
    //     match self {
    //         Single(single) => single.get_level(),
    //         Online(online) => online.get_level(),
    //     }
    // }
}

// pub trait LevelingTypeT {
//     fn default() -> Self;
//     fn get_gravity(&self) -> f64;
//     fn cleared_lines(&mut self);
// }

#[derive(Clone, Debug)]
pub struct Online {
    level: Level,
    time_passed: f64,
}

impl Online {
    pub fn new() -> Online {
        Online {
            level: Level::L1,
            time_passed: 0.,
        }
    }
    pub fn update(&mut self, dt: f64) {
        self.time_passed += dt;
        self.level = Level::L1 + ((self.time_passed + 30.).floor() / 20.).floor() as usize;
    }
    pub fn get_gravity(&self) -> f64 {
        self.level.get_gravity()
    }
}

#[derive(Clone, Debug)]
pub struct Singleplayer {
    score: usize,
    level: Level,
    lines: u16,
}

impl Singleplayer {
    fn new() -> Self {
        Self {
            score: 0,
            level: Level::L1,
            lines: 0,
        }
    }

    pub fn get_level(&self) -> Level {
        self.level.clone()
    }

    pub fn get_gravity(&self) -> f64 {
        self.level.get_gravity()
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    fn get_score_from_lines(&self, lines: usize) -> usize {
        let lines_adj = match lines {
            1 => 1,
            2 => 3,
            3 => 5,
            4 => 8,
            _ => 0,
        };
        40 * (lines_adj + 1)
    }

    pub fn cleared_lines(&mut self, lines: usize) {
        self.score += self.get_score_from_lines(lines);
        self.lines += lines as u16;
        let lvl = self.level.clone();
        if self.lines_missing() < 0 {
            self.level = lvl + 1;
        }
    }

    fn lines_missing(&self) -> isize {
        let lvl = self.level.clone();
        (lvl * 5_usize) as isize - self.lines as isize
    }
}

#[derive(Clone, Debug)]
pub enum Level {
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
    L9,
    L10,
    L11,
    L12,
}

impl Level {
    pub fn as_usize(&self) -> usize {
        self.clone().into()
    }

    fn get_gravity(&self) -> f64 {
        use Level::*;
        let frames = match self {
            L1 | L2 | L3 | L4 | L5 | L6 => (60 - (self.as_usize() - 1) * 10) as f64,
            L7 => 8.,
            L8 => 6.,
            L9 => 4.,
            L10 => 2.,
            L11 => 1.,
            L12 => 0.3,
        };

        frames / 60.
    }
}

impl Add<usize> for Level {
    type Output = Level;
    fn add(self, other: usize) -> Level {
        use Level::*;
        let mut lvl = self;
        for _ in 0..other {
            lvl = match lvl {
                L1 => L2,
                L2 => L3,
                L3 => L4,
                L4 => L5,
                L5 => L6,
                L6 => L7,
                L7 => L8,
                L8 => L9,
                L9 => L10,
                L10 => L11,
                L11 => L12,
                L12 => L12,
            }
        }
        lvl
    }
}

impl Mul<usize> for Level {
    type Output = usize;
    fn mul(self, other: usize) -> Self::Output {
        let me: usize = self.into();
        me * other
    }
}

impl Into<usize> for Level {
    fn into(self) -> usize {
        use Level::*;
        match self {
            L1 => 1,
            L2 => 2,
            L3 => 3,
            L4 => 4,
            L5 => 5,
            L6 => 6,
            L7 => 7,
            L8 => 8,
            L9 => 9,
            L10 => 10,
            L11 => 11,
            L12 => 12,
        }
    }
}
