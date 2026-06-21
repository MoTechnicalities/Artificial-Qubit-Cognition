#![allow(dead_code)]

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
            Self::ReinforceBeam     => "ReinforceBeam",
            Self::TightenJoint      => "TightenJoint",
            Self::StrengthenSupport => "StrengthenSupport",
            Self::ExtendSpan        => "ExtendSpan",
        }
    }

    pub fn matrix(self) -> [[i32; 3]; 3] {
        match self {
            // Couples stability into drift axis — reinforces load path.
            Self::ReinforceBeam     => [[1, 0, 0], [0, 1, 0], [1, 0, 1]],
            // Identity — tightens joint by fixing current geometry.
            Self::TightenJoint      => [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
            // Couples symmetry into stability — adds vertical reinforcement.
            Self::StrengthenSupport => [[1, 0, 0], [0, 1, 0], [0, 1, 1]],
            // Couples symmetry into drift — expands horizontal reach.
            Self::ExtendSpan        => [[1, 1, 0], [0, 1, 0], [0, 0, 1]],
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
