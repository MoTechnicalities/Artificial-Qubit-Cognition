#![allow(dead_code)]

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
            Self::Stabilize  => "Stabilize",
            Self::Align      => "Align",
            Self::LoadShift  => "LoadShift",
            Self::Connect    => "Connect",
        }
    }

    pub fn matrix(self) -> [[i32; 3]; 3] {
        match self {
            // Reinforces stability axis without disturbing drift or symmetry.
            Self::Stabilize  => [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
            // Reduces drift contribution while preserving stability.
            Self::Align      => [[1, 0, -1], [0, 1, 0], [0, 0, 1]],
            // Couples drift to stability axis — represents load redistribution.
            Self::LoadShift  => [[1, 0, 1], [0, 1, 0], [0, 0, 1]],
            // Strengthens symmetry axis by coupling it to stability.
            Self::Connect    => [[1, 0, 0], [0, 1, 1], [0, 0, 1]],
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
