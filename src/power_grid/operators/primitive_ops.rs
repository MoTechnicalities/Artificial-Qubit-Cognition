#![allow(dead_code)]

use crate::arithmetic::{checked_matrix_vector, ArithmeticError};

/// Operators acting on individual Power Grid primitive AQs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveOp {
    Energize,
    DeEnergize,
    Protect,
    Isolate,
}

impl PrimitiveOp {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Energize => "Energize",
            Self::DeEnergize => "DeEnergize",
            Self::Protect => "Protect",
            Self::Isolate => "Isolate",
        }
    }

    pub fn matrix(self) -> [[i32; 3]; 3] {
        match self {
            Self::Energize => [[1, 0, 0], [0, 1, 0], [0, 0, 1]], // stabilise in place
            Self::DeEnergize => [[1, 0, -1], [0, 1, 0], [0, 0, 1]], // reduce stability
            Self::Protect => [[1, 0, 0], [0, 1, 1], [0, 0, 1]],  // couple sym→stab
            Self::Isolate => [[-1, 0, 0], [0, 1, 0], [0, 0, -1]], // invert drift+stab
        }
    }

    pub fn apply(self, coords: [i32; 3]) -> Result<[i32; 3], ArithmeticError> {
        checked_matrix_vector(self.matrix(), coords, "grid primitive operator")
    }
}
