/// A primitive AQ encodes a minimal structural polarity.
/// coords = [drift_axis, symmetry_axis, stability_axis]
/// - coords[0]: drift contribution (deviation from structural baseline)
/// - coords[1]: symmetry contribution (relevant for span meta-AQs)
/// - coords[2]: stability contribution (structural load capacity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveAQKind {
    Stable,
    Unstable,
    LoadBearing,
    NonLoadBearing,
    Aligned,
    Misaligned,
    Connected,
    Disconnected,
}

impl PrimitiveAQKind {
    /// Canonical integer coords in Z^3 for this primitive distinction.
    pub fn canonical_coords(self) -> [i32; 3] {
        match self {
            // [drift, symmetry, stability]
            Self::Stable         => [0, 3, 4],
            Self::Unstable       => [3, 1, 1],
            Self::LoadBearing    => [1, 2, 3],
            Self::NonLoadBearing => [0, 1, 2],
            Self::Aligned        => [0, 3, 3],
            Self::Misaligned     => [2, 1, 2],
            Self::Connected      => [0, 2, 3],
            Self::Disconnected   => [1, 3, 1],
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stable         => "Stable",
            Self::Unstable       => "Unstable",
            Self::LoadBearing    => "LoadBearing",
            Self::NonLoadBearing => "NonLoadBearing",
            Self::Aligned        => "Aligned",
            Self::Misaligned     => "Misaligned",
            Self::Connected      => "Connected",
            Self::Disconnected   => "Disconnected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveAQ {
    pub kind: PrimitiveAQKind,
    /// coords[0]=drift, coords[1]=symmetry, coords[2]=stability
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
