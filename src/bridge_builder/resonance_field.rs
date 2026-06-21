use crate::bridge_builder::{meta_aq::MetaAQ, operators::meta_ops::MetaOp};

/// The resonance field is itself a state in Z^3 — recursive resonance.
///
/// state[0] = stability weight multiplier
/// state[1] = symmetry weight multiplier
/// state[2] = drift penalty multiplier
///
/// The default field uses weight [1, 1, 1], meaning no amplification or
/// attenuation of any resonance axis. Future recursive resonance layers
/// can evolve this state via operator application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResonanceField {
    pub state: [i32; 3],
    pub signature: String,
}

impl ResonanceField {
    pub fn default_field() -> Self {
        let state = [1, 1, 1];
        let signature = format!(
            "resonance|state:[{},{},{}]|layer:0",
            state[0], state[1], state[2]
        );
        Self { state, signature }
    }
}

/// Decomposed resonance evaluation results.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResonanceScore {
    /// Sum of stability-axis (coords[2]) values across all primitives.
    pub stability: i32,
    /// Sum of symmetry-axis (coords[1]) values across span meta-AQs only.
    pub symmetry: i32,
    /// Sum of drift-axis (coords[0]) values across all primitives.
    pub drift: i32,
    /// Coherence bonus from applied meta-operators (e.g. DistributeLoad = +6).
    pub structural_coherence: i32,
}

impl ResonanceScore {
    /// score = stability + symmetry - drift + structural_coherence
    pub fn total_score(&self) -> i32 {
        self.stability + self.symmetry - self.drift + self.structural_coherence
    }
}

/// Evaluate the resonance field over a set of meta-AQs and applied meta-ops.
///
/// Axis semantics:
/// - stability  = sum of primitive coords[2] × field.state[0]
/// - symmetry   = sum of span-primitive coords[1] × field.state[1]
/// - drift      = sum of primitive coords[0] × field.state[2]
/// - coherence  = sum of meta_op.coherence_bonus()
pub fn evaluate(
    field: &ResonanceField,
    meta_aqs: &[MetaAQ],
    meta_ops: &[MetaOp],
) -> ResonanceScore {
    let all_primitives = meta_aqs
        .iter()
        .flat_map(|m| m.components.iter())
        .flat_map(|s| s.components.iter());

    let stability: i32 = all_primitives.clone().map(|p| p.coords[2]).sum::<i32>()
        * field.state[0];

    let symmetry: i32 = meta_aqs
        .iter()
        .filter(|m| m.kind.is_span())
        .flat_map(|m| m.components.iter())
        .flat_map(|s| s.components.iter())
        .map(|p| p.coords[1])
        .sum::<i32>()
        * field.state[1];

    let drift: i32 = all_primitives.map(|p| p.coords[0]).sum::<i32>() * field.state[2];

    let structural_coherence: i32 = meta_ops.iter().map(|op| op.coherence_bonus()).sum();

    ResonanceScore { stability, symmetry, drift, structural_coherence }
}
