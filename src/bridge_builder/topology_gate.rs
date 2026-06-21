use crate::bridge_builder::{
    meta_aq::MetaAQ,
    primitive_aq::PrimitiveAQKind,
};

/// Outcome of the topological invariant check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyStatus {
    Valid,
    Invalid(String),
}

impl TopologyStatus {
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid)
    }
}

/// Topological Verification Gate.
///
/// Checks the following invariants before accepting a trajectory:
/// - No Disconnected primitives in span meta-AQs (unsupported span violation).
/// - No super-AQ containing 2+ Unstable primitives (unstable joint violation).
///
/// Any violation causes the design to be rejected and a correction trajectory
/// is pulled from the buffer by the arbitration layer.
pub fn check_topology(meta_aqs: &[MetaAQ]) -> TopologyStatus {
    for meta_aq in meta_aqs {
        for super_aq in &meta_aq.components {
            // Unstable joint: 2+ Unstable primitives in a single structural component.
            let unstable_count = super_aq
                .components
                .iter()
                .filter(|p| p.kind == PrimitiveAQKind::Unstable)
                .count();
            if unstable_count >= 2 {
                return TopologyStatus::Invalid("unstable joint".to_string());
            }

            // Unsupported span: a Disconnected primitive inside a span meta-AQ.
            if meta_aq.kind.is_span() {
                for paq in &super_aq.components {
                    if paq.kind == PrimitiveAQKind::Disconnected {
                        return TopologyStatus::Invalid("unsupported span".to_string());
                    }
                }
            }
        }
    }
    TopologyStatus::Valid
}
