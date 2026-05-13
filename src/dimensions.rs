//! This module encapsulates anything that is related to the number of slices the tower has.

use colorous::VIRIDIS;
use macroquad::color::Color;

/// The number of slices we use for the tower simulator.
pub const NUM_OF_SLICES: u8 = 8;

/// The minimum and maximum width we use for the tower slices.
const SLICE_WIDTH_MIN_MAX: (f32, f32) = (10.0, 20.0);


/// The slice height for painting.
pub fn slice_height() -> f32 {
    60.0 / NUM_OF_SLICES as f32
}

/// The width of the slices for a specific index.
pub fn get_width_for_slice( slice: u8) -> f32 {
    SLICE_WIDTH_MIN_MAX.0
        + ((slice as f32) / (NUM_OF_SLICES as f32))
            * (SLICE_WIDTH_MIN_MAX.1 - SLICE_WIDTH_MIN_MAX.0)
}

/// Gets the color of the slice by a viridis color scale.
pub fn get_slice_color(slice: u8) -> Color {
    let base = VIRIDIS.eval_continuous(1.0 - slice as f64 / NUM_OF_SLICES as f64);
    Color::new(
        base.r as f32 / 255.0,
        base.g as f32 / 255.0,
        base.b as f32 / 255.0,
        1.0,
    )
}
