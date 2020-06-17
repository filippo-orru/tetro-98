const LOCK_DELAY: f64 = 1.;
const LOCK_MOVE_DELAY: f64 = 0.6;
const LOCK_MOVES_MAX: usize = 15;

// pub enum HoldReturn {
//     OnFloor,
// }

#[derive(Clone, Debug)]
pub struct OnHoldState {
    last_time_passed: f64,
    total_time_passed: f64,
    moves_made: usize,
}

impl OnHoldState {
    pub fn new() -> OnHoldState {
        Self {
            last_time_passed: 0.,
            total_time_passed: 0.,
            moves_made: 0,
        }
    }
    pub fn pressed(&mut self) {
        self.last_time_passed = 0.;
        self.moves_made += 1;
    }
    pub fn update(&mut self, delta: f64) {
        self.last_time_passed += delta;
        self.total_time_passed += delta;
    }
    pub fn hold_exceeded(&self, delta: f64) -> bool {
        self.last_time_passed + delta > LOCK_MOVE_DELAY
            || self.total_time_passed > LOCK_DELAY
            || self.moves_made > LOCK_MOVES_MAX
    }
}
