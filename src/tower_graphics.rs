//! This module is in charge of rendering the tower and the dealing with coordinates of single slices.

use crate::tower::{Tower};
use macroquad::prelude::*;
use crate::dimensions::{get_slice_color, get_width_for_slice, slice_height};

/// The virtual window dimension we use. The origin is in the upper left corner.
const WINDOW_DIMENSIONS: f32 = 100.0;


/// The height of the tower.
const TOWER_HEIGHT: f32 = 70.0;

/// The width of the tower.
const TOWER_WIDTH: f32 = 4.0;

/// The tower positions in x dimension.
const TOWER_POSITIONS: [f32; 3] = [15.0, 50.0, 85.0];




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
            WINDOW_DIMENSIONS - slice_index as f32 * slice_height() - slice_height() / 2.0,
        )
    }

    /// Gets the turning point for the tower where we change from horizontal to vertical movement.
    pub fn get_turning_point(tower: usize) -> Vec2 {
        Vec2::new(TOWER_POSITIONS[tower], slice_height())
    }

    /// Draws a slice at the indicated windows position.
    pub fn draw_slice(slice_index: u8, position: Vec2) {
        let width = get_width_for_slice(slice_index);
        let top_left = position - Vec2::new(width * 0.5, slice_height() * 0.5);
        draw_rectangle(
            top_left.x,
            top_left.y,
            width,
            slice_height(),
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
