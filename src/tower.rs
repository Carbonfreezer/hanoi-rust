//! This module has the complete logical representation of the tower.

use crate::dimensions::NUM_OF_SLICES;

/// The move command we get for moving a stones.
pub struct MoveCommand {
    /// The start tower we take a slice from-
    pub start_tower: usize,
    /// The end tower we move to.
    pub end_tower: usize,
    /// The starting height where we should start.
    pub start_height: usize,
    /// The ending height we should go to.
    pub end_height: usize,
    /// The slice we move.
    pub slice_index: u8,
}

/// The tower with the three pilons.
#[derive(Clone, Debug)]
pub struct Tower {
    /// The three towers we have.
    towers: [Vec<u8>; 3],
}

impl Tower {
    /// We crate a new tower with the slides being on the first or last pilon.
    pub fn new(is_forward: bool) -> Self {
        let start_pilon = if is_forward { 0 } else { 2 };

        let mut towers = [
            Vec::with_capacity(NUM_OF_SLICES as usize),
            Vec::with_capacity(NUM_OF_SLICES as usize),
            Vec::with_capacity(NUM_OF_SLICES as usize),
        ];

        for i in 0..NUM_OF_SLICES  {
            towers[start_pilon].push(NUM_OF_SLICES - i - 1);
        }
        Self { towers }
    }
    /// Checks if we can legally move from start to destination.
    pub fn is_legal_move(&self, start: usize, destination: usize) -> bool {
        let Some(start_slice) = self.towers[start].last() else {
            return false;
        };
        let Some(destination_slice) = self.towers[destination].last() else {
            return true;
        };
        start_slice < destination_slice
    }

    /// Constructs a move command from the indicated start and end tower.
    pub fn get_move_command(&self, start_tower: usize, end_tower: usize) -> MoveCommand {
        debug_assert!(start_tower <= 2);
        debug_assert!(end_tower <= 2);
        debug_assert!(!self.towers[start_tower].is_empty());

        MoveCommand {
            start_tower,
            end_tower,
            start_height: (self.towers[start_tower].len() - 1),
            end_height: (self.towers[end_tower].len()),
            slice_index: *(self.towers[start_tower]
                .last()
                .expect("One element should be at start tower")),
        }
    }

    /// Gets  the towers.
    pub fn get_towers(&self) -> &[Vec<u8>; 3] {
        &self.towers
    }

    /// Executes the taking part of the move.
    pub fn take_slice(&mut self, command: &MoveCommand) {
        let test = self.towers[command.start_tower]
            .pop()
            .expect("One element should be at start tower");
        debug_assert_eq!(test, command.slice_index, "Inconsistent slice");
    }

    /// Executes the drop part of a move.
    pub fn drop_slice(&mut self, command: &MoveCommand) {
        self.towers[command.end_tower].push(command.slice_index);
    }
}
