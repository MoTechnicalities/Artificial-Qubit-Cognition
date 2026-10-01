#![allow(dead_code)]

use crate::arithmetic::{checked_matrix_vector, ArithmeticError};

/// Integer-safe operators acting on super-AQ gestalt vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuperOp {
    ReinforceBeam,
    TightenJoint,
    StrengthenSupport,
    ExtendSpan,
}

impl SuperOp {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReinforceBeam => "ReinforceBeam",
            Self::TightenJoint => "TightenJoint",
            Self::StrengthenSupport => "StrengthenSupport",
            Self::ExtendSpan => "ExtendSpan",
        }
    }

    pub fn matrix(self) -> [[i32; 3]; 3] {
        match self {
            // Couples stability into drift axis — reinforces load path.
            Self::ReinforceBeam => [[1, 0, 0], [0, 1, 0], [1, 0, 1]],
            // Identity — tightens joint by fixing current geometry.
            Self::TightenJoint => [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
            // Couples symmetry into stability — adds vertical reinforcement.
            Self::StrengthenSupport => [[1, 0, 0], [0, 1, 0], [0, 1, 1]],
            // Couples symmetry into drift — expands horizontal reach.
            Self::ExtendSpan => [[1, 1, 0], [0, 1, 0], [0, 0, 1]],
        }
    }

    pub fn apply(self, gestalt: [i32; 3]) -> Result<[i32; 3], ArithmeticError> {
        checked_matrix_vector(self.matrix(), gestalt, "bridge super operator")
    }
}
