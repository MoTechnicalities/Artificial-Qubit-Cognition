/// Operators acting on entire bridge sections (meta-AQ level).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaOp {
    BalanceSpans,
    DistributeLoad,
    StabilizeBridge,
}

impl MetaOp {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BalanceSpans    => "BalanceSpans",
            Self::DistributeLoad  => "DistributeLoad",
            Self::StabilizeBridge => "StabilizeBridge",
        }
    }

    /// Deterministic structural coherence bonus contributed by this operator.
    /// DistributeLoad is the only operator that actively reorganizes load paths,
    /// yielding a +6 structural coherence bonus in the resonance field.
    pub fn coherence_bonus(self) -> i32 {
        match self {
            Self::DistributeLoad  => 6,
            Self::BalanceSpans    => 0,
            Self::StabilizeBridge => 0,
        }
    }
}
