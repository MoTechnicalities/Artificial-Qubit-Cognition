/// Shared outcome type for topological invariant gates across all DGCS reasoners.
///
/// Domain-specific gate implementations return this type, ensuring that
/// arbitration layers, correction buffers, and cross-system handshakes (STIF)
/// can all interpret topology results without domain coupling.
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
