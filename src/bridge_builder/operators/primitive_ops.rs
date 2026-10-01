#![allow(dead_code)]

use crate::arithmetic::{checked_matrix_vector, ArithmeticError};

/// Integer-safe operators acting on primitive AQ coordinates in Z^3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveOp {
    Stabilize,
    Align,
    LoadShift,
    Connect,
}

impl PrimitiveOp {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stabilize => "Stabilize",
            Self::Align => "Align",
            Self::LoadShift => "LoadShift",
            Self::Connect => "Connect",
        }
    }

    pub fn matrix(self) -> [[i32; 3]; 3] {
        match self {
            // Reinforces stability axis without disturbing drift or symmetry.
            Self::Stabilize => [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
            // Reduces drift contribution while preserving stability.
            Self::Align => [[1, 0, -1], [0, 1, 0], [0, 0, 1]],
            // Couples drift to stability axis — represents load redistribution.
            Self::LoadShift => [[1, 0, 1], [0, 1, 0], [0, 0, 1]],
            // Strengthens symmetry axis by coupling it to stability.
            Self::Connect => [[1, 0, 0], [0, 1, 1], [0, 0, 1]],
        }
    }

    pub fn apply(self, coords: [i32; 3]) -> Result<[i32; 3], ArithmeticError> {
        checked_matrix_vector(self.matrix(), coords, "bridge primitive operator")
    }
}
