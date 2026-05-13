#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::animation::AnimatingStone;
use crate::tower::{MoveCommand, Tower};
use crate::tower_graphics::TowerGraphics;
use macroquad::prelude::{BLACK, clear_background, get_frame_time, next_frame};
use crate::dimensions::NUM_OF_SLICES;

mod animation;
mod tower;
mod tower_graphics;
mod dimensions;

/// The resulting type of the tower iterator to paint animations and the non moving game state.
struct TowerStateMove {
    /// The tower inbetween moves.
    tower: Tower,
    /// The currently executed move command.
    command: MoveCommand,
}

/// The iterator to generate a solution for the tower in one pass.
struct TowerIterator {
    /// The tower we iterrate on.
    tower: Tower,
    /// The next move we execute
    move_index: usize,
    /// The amount of moves we execute.
    amount_of_moves: usize,
    /// Indicates the poles from source, inbetween, desintation
    pole_characteristic: [usize; 3],
}

impl TowerIterator {
    fn new(is_forward: bool) -> Self {
        let (start, end) = if is_forward { (0, 2) } else { (2, 0) };

        Self {
            tower: Tower::new(is_forward),
            move_index: 1,
            amount_of_moves: 2_usize.pow(NUM_OF_SLICES as u32) - 1,
            pole_characteristic: if NUM_OF_SLICES.is_multiple_of(2) {
                [start, end, 1]
            } else {
                [start, 1, end]
            },
        }
    }
}

impl Iterator for TowerIterator {
    type Item = TowerStateMove;

    /// The solver of the hanoi problem in iterative form.
    fn next(&mut self) -> Option<Self::Item> {
        if self.move_index > self.amount_of_moves {
            return None;
        }

        let (mut start, mut end) = match self.move_index % 3 {
            // Case 1: Move between start and end
            1 => (self.pole_characteristic[0], self.pole_characteristic[2]),
            // Case 2: Move between Source and Auxiliary
            2 => (self.pole_characteristic[0], self.pole_characteristic[1]),
            // Case 0: Move between Auxiliary and Destination
            0 => (self.pole_characteristic[1], self.pole_characteristic[2]),
            _ => unreachable!("%3 should result in 0,1 and 2."),
        };

        if !self.tower.is_legal_move(start, end) {
            (start, end) = (end, start);
        }

        let command = self.tower.get_move_command(start, end);
        self.tower.take_slice(&command);
        let between_tower = self.tower.clone();
        self.tower.drop_slice(&command);
        self.move_index += 1;
        Some(TowerStateMove {
            tower: between_tower,
            command,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.amount_of_moves - (self.move_index - 1);
        (remaining, Some(remaining))
    }
}

/// The amount of  elements is known upfront, wo we can make an exact size iterator out of it.
impl ExactSizeIterator for TowerIterator {}


#[macroquad::main("Towers of Hanoi")]
async fn main() {
    let mut graphics = TowerGraphics::default();
    let mut forward = true;

    loop {
        let tower_iterator = TowerIterator::new(forward);
        forward = !forward;
        for action in tower_iterator {
            let mut animation = AnimatingStone::new(&action.command);
            while !animation.update(get_frame_time()) {
                graphics.update_camera_position();
                clear_background(BLACK);
                TowerGraphics::draw_tower(&action.tower);
                animation.render_stone();
                next_frame().await
            }
        }
    }
}
