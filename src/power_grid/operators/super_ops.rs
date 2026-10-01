#![allow(dead_code)]

use crate::arithmetic::{checked_matrix_vector, ArithmeticError};

/// Operators acting on Power Grid super-AQ gestalt vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuperOp {
    Trip,
    Reset,
    Reinforce,
}

impl SuperOp {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Trip => "Trip",
            Self::Reset => "Reset",
            Self::Reinforce => "Reinforce",
        }
    }

    pub fn matrix(self) -> [[i32; 3]; 3] {
        match self {
            Self::Trip => [[1, 0, -1], [0, 1, 0], [0, 0, 1]], // reduce stab
            Self::Reset => [[1, 0, 0], [0, 1, 0], [0, 0, 1]], // identity
            Self::Reinforce => [[1, 0, 0], [0, 1, 0], [0, 1, 1]], // sym→stab coupling
        }
    }

    pub fn apply(self, gestalt: [i32; 3]) -> Result<[i32; 3], ArithmeticError> {
        checked_matrix_vector(self.matrix(), gestalt, "grid super operator")
    }
}
