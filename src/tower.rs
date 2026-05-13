//! This module has the complete logical representation of the tower.

use std::mem::take;

/// The number of slices we put onto the tower.
pub const NUM_SLICES: u8 = 5;

/// The move command we get for moving a stones.
pub struct MoveCommand {
    /// The start tower we take a slice from-
    pub(crate) start_tower: usize,
    /// The end tower we move to.
    pub(crate) end_tower: usize,
    /// The starting height where we should start.
    pub(crate) start_height: usize,
    /// The ending height we should go to.
    pub(crate) end_height: usize,
    /// The slice we move.
    pub(crate) slice_index: u8,
}

/// The mode to manipulate the current state with the commands.
#[derive(Copy, Clone, Debug)]
pub enum ManipulationMode {
    /// We take and we drop the stones.
    FullTransfer,
    /// We take only the stones.
    TakeOnly,
    /// We drop only the stones.
    DropOnly,
}

/// The tower with the three pilons.
#[derive(Clone, Debug)]
pub struct Tower {
    /// The three towers we have.
    towers: [Vec<u8>; 3],
}

impl Default for Tower {
    fn default() -> Self {
        let mut towers = [
            Vec::with_capacity(NUM_SLICES as usize),
            Vec::with_capacity(NUM_SLICES as usize),
            Vec::with_capacity(NUM_SLICES as usize),
        ];

        for i in 0..NUM_SLICES {
            towers[0].push(NUM_SLICES - i - 1);
        }
        Self { towers }
    }
}

impl Tower {
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

    /// Constructs a move command from the indicated stat ane end tower.
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

    /// Applies a move in different modes. TakeOnly and DropOnly are used in combination with animation.
    pub fn apply_move(&mut self, command: &MoveCommand, mode: ManipulationMode) {
        use ManipulationMode::*;
        if matches!(mode, TakeOnly | FullTransfer) {
            let test = self.towers[command.start_tower]
                .pop()
                .expect("One element should be at start tower");
            debug_assert_eq!(test, command.slice_index, "Inconsistent slice");
        }

        if matches!(mode, DropOnly | FullTransfer) {
            self.towers[command.end_tower].push(command.slice_index);
        }
    }
}

