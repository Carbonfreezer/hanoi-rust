//! This module encapsulates everything related to animation.

use crate::tower::{MoveCommand};
use crate::tower_graphics::TowerGraphics;
use macroquad::math::Vec2;

/// Our velocity for moving the stones.
const MOVEMENT_VELOCITY: f32 = 100.0;

/// One line segment along the way.
struct MovementSegment {
    /// Where does the segment start.
    starting_point: Vec2,
    /// Where does it end.
    ending_point: Vec2,
    /// The way length part we are responsible of.
    way_length_from_to: (f32, f32),
}

impl MovementSegment {
    fn new(starting_point: Vec2, ending_point: Vec2, start_way_length: f32) -> Self {
        Self {
            starting_point,
            ending_point,
            way_length_from_to: (
                start_way_length,
                start_way_length + (ending_point - starting_point).length(),
            ),
        }
    }

    /// If we are responsible for the way length segment we return the position otherwise None.
    fn get_point_along_length(&self, distance: f32) -> Option<Vec2> {
        if distance >= self.way_length_from_to.0 && distance <= self.way_length_from_to.1 {
            let alpha = (distance - self.way_length_from_to.0)
                / (self.way_length_from_to.1 - self.way_length_from_to.0);
            let smoothed = alpha * alpha * (3.0 - 2.0 * alpha);
            Some(self.starting_point + smoothed * (self.ending_point - self.starting_point))
        } else {
            None
        }
    }

    fn get_end_length(&self) -> f32 {
        self.way_length_from_to.1
    }
}

/// This is the stone that currently undergoes animation.
pub struct AnimatingStone {
    /// The total distance we need to cover.
    total_way_length: f32,
    /// The distance we have already covered.
    covered_way_length: f32,
    /// The stone that is moving.
    moving_slice: u8,
    /// The list of path segments we need to cover.
    path_segments: Vec<MovementSegment>,
    /// The final point we reach.
    final_point: Vec2,
}

impl AnimatingStone {
    /// Given a move command from the tower we generate a corresponding animation for this one.
    pub fn new(command: &MoveCommand) -> Self {
        #[allow(clippy::useless_vec)]
        let control_points = vec![
            TowerGraphics::get_tower_point(command.start_tower, command.start_height),
            TowerGraphics::get_turning_point(command.start_tower),
            TowerGraphics::get_turning_point(command.end_tower),
            TowerGraphics::get_tower_point(command.end_tower, command.end_height),
        ];

        let path_segments: Vec<_> = control_points[1..]
            .iter()
            .scan(
                (control_points[0], 0.0_f32),
                |(previous_point, way_length), new_point| {
                    let result = MovementSegment::new(*previous_point, *new_point, *way_length);
                    *way_length = result.get_end_length();
                    *previous_point = *new_point;
                    Some(result)
                },
            )
            .collect();

        Self {
            total_way_length: path_segments.last().unwrap().get_end_length(),
            covered_way_length: 0.0,
            moving_slice: command.slice_index,
            path_segments,
            final_point: *control_points.last().unwrap(),
        }
    }

    /// Updates and returns if the animation is finished.
    pub fn update(&mut self, delta_time: f32) -> bool {
        self.covered_way_length += delta_time * MOVEMENT_VELOCITY;
        self.covered_way_length >= self.total_way_length
    }

    /// Draws the stone.
    pub fn render_stone(&self) {
        let position = self
            .path_segments
            .iter()
            .find_map(|seg| seg.get_point_along_length(self.covered_way_length))
            .unwrap_or(self.final_point);

        TowerGraphics::draw_slice(self.moving_slice, position);
    }
}

