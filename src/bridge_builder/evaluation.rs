use crate::arithmetic::{checked_mul, checked_sum, ArithmeticError};
/// Bridge-specific topology and resonance evaluation functions.
///
/// These functions implement the domain contracts for BBR using the shared
/// DGCS substrate types from `crate::geom`. All new DGCS reasoners follow
/// this same pattern: domain logic here, shared types from geom.
use crate::bridge_builder::{
    meta_aq::MetaAQ, operators::meta_ops::MetaOp, primitive_aq::PrimitiveAQKind,
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
) -> Result<ResonanceScore, ArithmeticError> {
    let stability_sum = checked_sum(
        meta_aqs
            .iter()
            .flat_map(|m| m.components.iter())
            .flat_map(|s| s.components.iter())
            .map(|p| p.coords[2]),
        "bridge stability accumulation",
    )?;
    let stability = checked_mul(stability_sum, field.state[0], "bridge stability weighting")?;

    let symmetry_sum = checked_sum(
        meta_aqs
            .iter()
            .filter(|m| m.kind.is_span())
            .flat_map(|m| m.components.iter())
            .flat_map(|s| s.components.iter())
            .map(|p| p.coords[1]),
        "bridge symmetry accumulation",
    )?;
    let symmetry = checked_mul(symmetry_sum, field.state[1], "bridge symmetry weighting")?;

    let drift_sum = checked_sum(
        meta_aqs
            .iter()
            .flat_map(|m| m.components.iter())
            .flat_map(|s| s.components.iter())
            .map(|p| p.coords[0]),
        "bridge drift accumulation",
    )?;
    let drift = checked_mul(drift_sum, field.state[2], "bridge drift weighting")?;

    let structural_coherence = checked_sum(
        meta_ops.iter().map(|op| op.coherence_bonus()),
        "bridge coherence accumulation",
    )?;

    Ok(ResonanceScore {
        stability,
        symmetry,
        drift,
        structural_coherence,
    })
}
