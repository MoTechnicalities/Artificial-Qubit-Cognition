/// Meta-operators acting on Power Grid zone-level meta-AQs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaOp {
    BalanceZones,
    ShedLoad,
    StabilizeGrid,
}

impl MetaOp {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BalanceZones   => "BalanceZones",
            Self::ShedLoad       => "ShedLoad",
            Self::StabilizeGrid  => "StabilizeGrid",
        }
    }

    /// Deterministic structural coherence bonus.
    /// `ShedLoad` = +6: intentionally dropping load redistributes power
    /// across zones, creating a globally coherent routing geometry.
    pub fn coherence_bonus(self) -> i32 {
        match self {
            Self::ShedLoad      => 6,
            Self::BalanceZones  => 0,
            Self::StabilizeGrid => 0,
        }
    }
}
