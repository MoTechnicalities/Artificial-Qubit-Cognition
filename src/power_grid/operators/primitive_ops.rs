#![allow(dead_code)]

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
            Self::Energize    => "Energize",
            Self::DeEnergize  => "DeEnergize",
            Self::Protect     => "Protect",
            Self::Isolate     => "Isolate",
        }
    }

    pub fn matrix(self) -> [[i32; 3]; 3] {
        match self {
            Self::Energize    => [[1, 0, 0], [0, 1, 0], [0, 0, 1]],   // stabilise in place
            Self::DeEnergize  => [[1, 0, -1], [0, 1, 0], [0, 0, 1]],  // reduce stability
            Self::Protect     => [[1, 0, 0], [0, 1, 1], [0, 0, 1]],   // couple sym→stab
            Self::Isolate     => [[-1, 0, 0], [0, 1, 0], [0, 0, -1]], // invert drift+stab
        }
    }

    pub fn apply(self, coords: [i32; 3]) -> [i32; 3] {
        let m = self.matrix();
        [
            m[0][0] * coords[0] + m[0][1] * coords[1] + m[0][2] * coords[2],
            m[1][0] * coords[0] + m[1][1] * coords[1] + m[1][2] * coords[2],
            m[2][0] * coords[0] + m[2][1] * coords[1] + m[2][2] * coords[2],
        ]
    }
}
