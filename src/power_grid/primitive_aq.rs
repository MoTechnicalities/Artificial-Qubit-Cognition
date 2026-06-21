/// Primitive AQs for the Power Grid domain.
/// coords = [drift_axis, symmetry_axis, stability_axis]
///
/// - coords[0]: drift  — contribution to structural deviation / instability
/// - coords[1]: sym    — zone balance contribution (active zones only)
/// - coords[2]: stab   — load capacity / power delivery reliability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveAQKind {
    Energized,
    DeEnergized,
    Overloaded,
    Nominal,
    Connected,
    Isolated,
    Protected,
    Unprotected,
    Standby,
    Idle,
}

impl PrimitiveAQKind {
    pub fn canonical_coords(self) -> [i32; 3] {
        match self {
            // [drift, sym, stab]
            Self::Energized    => [0, 3, 4],
            Self::DeEnergized  => [3, 1, 2],
            Self::Overloaded   => [2, 1, 3],
            Self::Nominal      => [1, 2, 3],
            Self::Connected    => [0, 2, 3],
            Self::Isolated     => [1, 3, 1],
            Self::Protected    => [0, 3, 3],
            Self::Unprotected  => [2, 1, 2],
            Self::Standby      => [0, 1, 2],
            Self::Idle         => [0, 1, 1],
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Energized    => "Energized",
            Self::DeEnergized  => "DeEnergized",
            Self::Overloaded   => "Overloaded",
            Self::Nominal      => "Nominal",
            Self::Connected    => "Connected",
            Self::Isolated     => "Isolated",
            Self::Protected    => "Protected",
            Self::Unprotected  => "Unprotected",
            Self::Standby      => "Standby",
            Self::Idle         => "Idle",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveAQ {
    pub kind: PrimitiveAQKind,
    pub coords: [i32; 3],
    pub signature: String,
}

impl PrimitiveAQ {
    pub fn new(kind: PrimitiveAQKind) -> Self {
        let coords = kind.canonical_coords();
        let signature = format!(
            "paq:{}|coords:[{},{},{}]",
            kind.as_str(), coords[0], coords[1], coords[2]
        );
        Self { kind, coords, signature }
    }
}
