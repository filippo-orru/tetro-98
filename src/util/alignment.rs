#[allow(dead_code)]
#[derive(Clone, Debug, Copy)]
pub enum Alignment {
    /// Top Left
    TL(f32, f32),
    /// Center Top
    TC(f32, f32),
    /// Top Right
    TR(f32, f32),
    /// Center Right
    CR(f32, f32),
    /// Bottom Right
    BR(f32, f32),
    /// Center Bottom
    BC(f32, f32),
    /// Center Bottom
    BL(f32, f32),
    /// Center Left
    CL(f32, f32),
    /// Center Center
    CC(f32, f32),
}

impl Alignment {
    /// Converts an alignment to a screen offset for the top left corner
    /// of the provided object dimensions. Requires window height and width.
    pub fn to_offset(&self, (ww, wh): (f32, f32), (ow, oh): (f32, f32)) -> (f32, f32) {
        use Alignment::*;
        match *self {
            TL(x, y) => (x, y),
            TC(x, y) => (ww / 2. - ow / 2. + x, y),
            TR(x, y) => (ww - ow - x, y),
            CR(x, y) => (ww - ow - x, wh / 2. - oh / 2. + y),
            BR(x, y) => (ww - ow - x, wh - oh - y),
            BC(x, y) => (ww / 2. - ow / 2. - x, wh - oh - y),
            BL(x, y) => (x, wh - oh - y),
            CL(x, y) => (x, wh / 2. - oh / 2. + y),
            CC(x, y) => (ww / 2. - ow / 2. + x, wh / 2. - oh / 2. + y),
        }
    }
}
