//! This module is in charge of rendering the tower and the dealing with coordinates of single slices.

use crate::tower::{NUM_SLICES, Tower};
use macroquad::prelude::*;
use colorous::VIRIDIS;

/// The virtual window dimension we use. The origin is in the upper left corner.
const WINDOW_DIMENSIONS: f32 = 100.0;

/// The height of a single slice.
const SLICE_HEIGHT: f32 = 60.0 / NUM_SLICES as f32;

/// The height of the tower.
const TOWER_HEIGHT: f32 = SLICE_HEIGHT * (NUM_SLICES + 1) as f32;

/// The width of the tower.
const TOWER_WIDTH: f32 = 4.0;

/// The tower positions in x dimension.
const TOWER_POSITIONS: [f32; 3] = [15.0, 50.0, 85.0];

/// The topping height, where the pieces change from / to vertical direction.
const FLYING_HEIGHT: f32 = SLICE_HEIGHT;

const SLICE_WIDTH_MIN_MAX: (f32, f32) = (10.0, 20.0);

fn get_width_for_slice(slice: u8) -> f32 {
    SLICE_WIDTH_MIN_MAX.0
        + ((slice as f32) / (NUM_SLICES as f32)) * (SLICE_WIDTH_MIN_MAX.1 - SLICE_WIDTH_MIN_MAX.0)
}


/// Gets the color of the slice by a viridis color scale.
fn get_slice_color(slice : u8 ) -> Color {
    let base = VIRIDIS.eval_continuous(1.0 - slice as f64 / NUM_SLICES as f64);
    Color::new(
        base.r as f32 / 255.0,
        base.g as f32 / 255.0,
        base.b as f32 / 255.0,
        1.0,
    )
}

#[derive(Default)]
pub struct TowerGraphics {
    camera: Camera2D,
}

impl TowerGraphics {
    /// Asks for the position of a certain pilon.
    /// public to be used for animation later on.
    pub fn get_tower_point(tower: usize, slice_index: usize) -> Vec2 {
        Vec2::new(
            TOWER_POSITIONS[tower],
            WINDOW_DIMENSIONS - slice_index as f32 * SLICE_HEIGHT - SLICE_HEIGHT / 2.0,
        )
    }

    /// Gets the turning point for the tower where we change from horizontal to vertical movement.
    pub fn get_turning_point(tower: usize) -> Vec2 {
        Vec2::new(TOWER_POSITIONS[tower], FLYING_HEIGHT)
    }

    /// Draws a slice at the indicated windows position.
    pub fn draw_slice(slice_index: u8, position: Vec2) {
        let width = get_width_for_slice(slice_index);
        let top_left = position - Vec2::new(width * 0.5, SLICE_HEIGHT * 0.5);
        draw_rectangle(
            top_left.x,
            top_left.y,
            width,
            SLICE_HEIGHT,
            get_slice_color(slice_index),
        );
    }

    /// Makes sure that the coordinate system is always the same independent of window size and aspect ratio.
    pub fn update_camera_position(&mut self) {
        self.camera.target = vec2(WINDOW_DIMENSIONS / 2.0, WINDOW_DIMENSIONS / 2.0);
        let screen_aspect = screen_width() / screen_height();

        // Aspect Ratio
        if screen_aspect > 1.0 {
            self.camera.zoom.x = 2.0 / (WINDOW_DIMENSIONS * screen_aspect);
            self.camera.zoom.y = 2.0 / WINDOW_DIMENSIONS;
        } else {
            self.camera.zoom.x = 2.0 / WINDOW_DIMENSIONS;
            self.camera.zoom.y = 2.0 / (WINDOW_DIMENSIONS / screen_aspect);
        }

        set_camera(&self.camera);
    }

    /// Renders the complete scene with the pillars and the slices on them.
    pub fn draw_tower(tower: &Tower) {
        // First the three pillars.
        for x_pos in TOWER_POSITIONS {
            draw_rectangle(
                x_pos - TOWER_WIDTH * 0.5,
                WINDOW_DIMENSIONS - TOWER_HEIGHT,
                TOWER_WIDTH,
                TOWER_HEIGHT,
                GRAY,
            );
        }

        // Draw towers. This can be done faster with iterator expressions, see later.
        let tower_collection = tower.get_towers();
        for (tower_idx, tower) in tower_collection.iter().enumerate() {
            for (slice_idx, _) in tower.iter().enumerate() {
                Self::draw_slice(
                    tower[slice_idx],
                    Self::get_tower_point(tower_idx, slice_idx),
                );
            }
        }
    }
}
