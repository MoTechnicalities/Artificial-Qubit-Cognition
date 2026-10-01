use crate::arithmetic::{checked_mul, checked_sum, ArithmeticError};
/// Power-Grid-specific topology and resonance evaluation.
///
/// Uses the shared DGCS substrate from `crate::geom`. Domain logic lives here;
/// substrate types (TopologyStatus, ResonanceField, ResonanceScore) are shared
/// with every other DGCS reasoner.
use crate::geom::{
    resonance_field::{ResonanceField, ResonanceScore},
    topology_gate::TopologyStatus,
};
use crate::power_grid::{
    meta_aq::MetaAQ, operators::meta_ops::MetaOp, primitive_aq::PrimitiveAQKind,
};

/// Check power-grid-specific topological invariants.
///
/// Violations:
/// - `Isolated` primitive in an active zone (NorthZone / SouthZone) →
///   "isolated circuit node"
/// - 2+ `Overloaded` primitives in the same super-AQ →
///   "overload cascade"
pub fn check_topology(meta_aqs: &[MetaAQ]) -> TopologyStatus {
    for meta_aq in meta_aqs {
        for super_aq in &meta_aq.components {
            let overloaded = super_aq
                .components
                .iter()
                .filter(|p| p.kind == PrimitiveAQKind::Overloaded)
                .count();
            if overloaded >= 2 {
                return TopologyStatus::Invalid("overload cascade".to_string());
            }
            if meta_aq.kind.is_active_zone() {
                for paq in &super_aq.components {
                    if paq.kind == PrimitiveAQKind::Isolated {
                        return TopologyStatus::Invalid("isolated circuit node".to_string());
                    }
                }
            }
        }
    }
    TopologyStatus::Valid
}

/// Evaluate power-grid resonance scores from zone geometry.
///
/// Axis semantics (coords = [drift, symmetry, stability]):
/// - stability = sum of coords[2] over all primitives × field.state[0]
/// - symmetry  = sum of coords[1] over active-zone primitives × field.state[1]
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
        "grid stability accumulation",
    )?;
    let stability = checked_mul(stability_sum, field.state[0], "grid stability weighting")?;

    let symmetry_sum = checked_sum(
        meta_aqs
            .iter()
            .filter(|m| m.kind.is_active_zone())
            .flat_map(|m| m.components.iter())
            .flat_map(|s| s.components.iter())
            .map(|p| p.coords[1]),
        "grid symmetry accumulation",
    )?;
    let symmetry = checked_mul(symmetry_sum, field.state[1], "grid symmetry weighting")?;

    let drift_sum = checked_sum(
        meta_aqs
            .iter()
            .flat_map(|m| m.components.iter())
            .flat_map(|s| s.components.iter())
            .map(|p| p.coords[0]),
        "grid drift accumulation",
    )?;
    let drift = checked_mul(drift_sum, field.state[2], "grid drift weighting")?;

    let structural_coherence = checked_sum(
        meta_ops.iter().map(|op| op.coherence_bonus()),
        "grid coherence accumulation",
    )?;

    Ok(ResonanceScore {
        stability,
        symmetry,
        drift,
        structural_coherence,
    })
}
