//! This module encapsulates anything that is related to the number of slices the tower has

use colorous::VIRIDIS;
use macroquad::color::Color;



pub struct Dimensions {
    num_slices: u8,
}

const SLICE_WIDTH_MIN_MAX: (f32, f32) = (10.0, 20.0);

impl Dimensions {
    pub fn new(num_slices: u8) -> Dimensions {
        Dimensions { num_slices }
    }

    pub fn num_slices(&self) -> u8 {
        self.num_slices
    }

    pub fn slice_height(&self) -> f32 {
        60.0 / self.num_slices as f32
    }

    pub fn get_width_for_slice(&self, slice: u8) -> f32 {
        SLICE_WIDTH_MIN_MAX.0
            + ((slice as f32) / (self.num_slices as f32))
                * (SLICE_WIDTH_MIN_MAX.1 - SLICE_WIDTH_MIN_MAX.0)
    }

    /// Gets the color of the slice by a viridis color scale.
    pub fn get_slice_color(&self, slice: u8) -> Color {
        let base = VIRIDIS.eval_continuous(1.0 - slice as f64 / self.num_slices as f64);
        Color::new(
            base.r as f32 / 255.0,
            base.g as f32 / 255.0,
            base.b as f32 / 255.0,
            1.0,
        )
    }
}
