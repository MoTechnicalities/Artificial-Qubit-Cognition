/// Bridge-specific topology and resonance evaluation functions.
///
/// These functions implement the domain contracts for BBR using the shared
/// DGCS substrate types from `crate::geom`. All new DGCS reasoners follow
/// this same pattern: domain logic here, shared types from geom.
use crate::bridge_builder::{
    meta_aq::MetaAQ,
    operators::meta_ops::MetaOp,
    primitive_aq::PrimitiveAQKind,
};
use crate::geom::{
    resonance_field::{ResonanceField, ResonanceScore},
    topology_gate::TopologyStatus,
};

/// Check bridge-specific topological invariants.
///
/// Violations:
/// - `Disconnected` primitive in a span meta-AQ → unsupported span
/// - 2+ `Unstable` primitives in a single super-AQ → unstable joint
pub fn check_topology(meta_aqs: &[MetaAQ]) -> TopologyStatus {
    for meta_aq in meta_aqs {
        for super_aq in &meta_aq.components {
            let unstable = super_aq
                .components
                .iter()
                .filter(|p| p.kind == PrimitiveAQKind::Unstable)
                .count();
            if unstable >= 2 {
                return TopologyStatus::Invalid("unstable joint".to_string());
            }
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

/// Evaluate bridge-specific resonance scores from meta-AQ geometry.
///
/// Axis semantics (coords = [drift, symmetry, stability]):
/// - stability = sum of coords[2] over all primitives × field.state[0]
/// - symmetry  = sum of coords[1] over span primitives only × field.state[1]
/// - drift     = sum of coords[0] over all primitives × field.state[2]
/// - coherence = sum of meta_op.coherence_bonus()
pub fn evaluate_resonance(
    field: &ResonanceField,
    meta_aqs: &[MetaAQ],
    meta_ops: &[MetaOp],
) -> ResonanceScore {
    let stability: i32 = meta_aqs
        .iter()
        .flat_map(|m| m.components.iter())
        .flat_map(|s| s.components.iter())
        .map(|p| p.coords[2])
        .sum::<i32>()
        * field.state[0];

    let symmetry: i32 = meta_aqs
        .iter()
        .filter(|m| m.kind.is_span())
        .flat_map(|m| m.components.iter())
        .flat_map(|s| s.components.iter())
        .map(|p| p.coords[1])
        .sum::<i32>()
        * field.state[1];

    let drift: i32 = meta_aqs
        .iter()
        .flat_map(|m| m.components.iter())
        .flat_map(|s| s.components.iter())
        .map(|p| p.coords[0])
        .sum::<i32>()
        * field.state[2];

    let structural_coherence: i32 = meta_ops.iter().map(|op| op.coherence_bonus()).sum();

    ResonanceScore { stability, symmetry, drift, structural_coherence }
}
