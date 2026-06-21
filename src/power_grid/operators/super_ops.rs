#![allow(dead_code)]

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
            Self::Trip      => "Trip",
            Self::Reset     => "Reset",
            Self::Reinforce => "Reinforce",
        }
    }

    pub fn matrix(self) -> [[i32; 3]; 3] {
        match self {
            Self::Trip      => [[1, 0, -1], [0, 1, 0], [0, 0, 1]], // reduce stab
            Self::Reset     => [[1, 0, 0], [0, 1, 0], [0, 0, 1]],  // identity
            Self::Reinforce => [[1, 0, 0], [0, 1, 0], [0, 1, 1]],  // sym→stab coupling
        }
    }

    pub fn apply(self, gestalt: [i32; 3]) -> [i32; 3] {
        let m = self.matrix();
        [
            m[0][0] * gestalt[0] + m[0][1] * gestalt[1] + m[0][2] * gestalt[2],
            m[1][0] * gestalt[0] + m[1][1] * gestalt[1] + m[1][2] * gestalt[2],
            m[2][0] * gestalt[0] + m[2][1] * gestalt[1] + m[2][2] * gestalt[2],
        ]
    }
}
