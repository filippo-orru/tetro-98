#![allow(dead_code)]
pub type IntoColor = (u8, u8, u8, u8);

pub const WHITE: IntoColor = (255, 255, 255, 255);
pub const OFF_WHITE: IntoColor = (238, 238, 238, 255);
pub const DARK_WHITE: IntoColor = (220, 220, 220, 255);
pub const RED: IntoColor = (255, 0, 0, 255);
pub const LRED: IntoColor = (255, 76, 68, 255);
pub const GREEN: IntoColor = (0, 255, 0, 255);
pub const LBLUE: IntoColor = (30, 114, 255, 255);
pub const BLUE: IntoColor = (0, 84, 255, 255);
pub const LCYAN: IntoColor = (35, 255, 255, 255);
pub const CYAN: IntoColor = (0, 251, 252, 255);
pub const YELLOW: IntoColor = (255, 255, 1, 255);
pub const ORANGE: IntoColor = (255, 104, 1, 255);
pub const PURPLE: IntoColor = (255, 4, 252, 255);
pub const BLACK: IntoColor = (15, 15, 15, 255);
pub const FLOOR: IntoColor = (89, 89, 89, 255);
pub const GHOST: IntoColor = (255, 255, 255, 153);
pub const GARBAGE: IntoColor = (84, 84, 84, 255);
pub const DESTROYING: IntoColor = (255, 255, 255, 215);

pub const GAME_OVER: IntoColor = (255, 20, 25, 242);
pub const DARK_OVERLAY: IntoColor = (20, 20, 20, 193);
pub const BG: IntoColor = (23, 23, 23, 255);
pub const BG_ERR: IntoColor = (64, 23, 23, 255);
pub const BG_ERR_OVERLAY: IntoColor = (64, 23, 23, 205);
// pub const BG_GAME_OVER: IntoColor = (64, 23, 23, 255);
pub const BG_LINE_COLOR: IntoColor = (45, 45, 45, 255);

pub const LGREY: IntoColor = (110, 110, 110, 255);
pub const GREY: IntoColor = (76, 76, 76, 255);
pub const DGREY: IntoColor = (25, 25, 25, 255);
pub const TOP_SHINE: IntoColor = (255, 255, 255, 54);
pub const BOTTOM_SHADOW: IntoColor = (0, 0, 0, 73);
pub const TRANSPARENT: IntoColor = (0, 0, 0, 0);

pub fn shift((r, g, b, a): IntoColor, amount: i8) -> IntoColor {
    if amount > 0 {
        let amount = amount as u8;
        (
            r.saturating_add(amount),
            g.saturating_add(amount),
            b.saturating_add(amount),
            a,
        )
    } else {
        let amount = (-amount) as u8;
        (
            r.saturating_sub(amount),
            g.saturating_sub(amount),
            b.saturating_sub(amount),
            a,
        )
    }
}

pub fn shift_alpha((r, g, b, a): IntoColor, amount: i8) -> IntoColor {
    if amount > 0 {
        let amount = amount as u8;
        (r, g, b, a.saturating_add(amount))
    } else {
        let amount = (-amount) as u8;
        (r, g, b, a.saturating_sub(amount))
    }
}
