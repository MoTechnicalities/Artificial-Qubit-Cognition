/// A resonance field state in Z^3.
///
/// The field is itself a cognitive object (recursive resonance): it can be
/// stored, transformed, and stabilized using the same operator machinery as
/// ordinary trajectories. Different DGCS reasoners may evolve their field
/// state differently; the struct is shared.
///
/// `state[0]` = stability weight multiplier
/// `state[1]` = symmetry weight multiplier
/// `state[2]` = drift penalty multiplier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResonanceField {
    pub state: [i32; 3],
    pub signature: String,
}

impl ResonanceField {
    pub fn new(state: [i32; 3]) -> Self {
        let signature = format!(
            "resonance|state:[{},{},{}]|layer:0",
            state[0], state[1], state[2]
        );
        Self { state, signature }
    }

    /// Default unit field: all three axes weighted equally at 1.
    pub fn default_field() -> Self {
        Self::new([1, 1, 1])
    }
}

/// Decomposed resonance evaluation results shared across all DGCS reasoners.
///
/// Domain-specific evaluators populate the four components; the total score
/// formula is invariant:
///
/// ```text
/// score = stability + symmetry - drift + structural_coherence
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResonanceScore {
    /// Sum of stability-axis contributions across all primitives × field weight.
    pub stability: i32,
    /// Sum of symmetry-axis contributions across active-zone primitives × field weight.
    pub symmetry: i32,
    /// Sum of drift-axis contributions across all primitives × field weight.
    pub drift: i32,
    /// Deterministic bonus from meta-operators (e.g. DistributeLoad = +6).
    pub structural_coherence: i32,
}

impl ResonanceScore {
    /// `score = stability + symmetry - drift + structural_coherence`
    pub fn total_score(&self) -> i32 {
        self.stability + self.symmetry - self.drift + self.structural_coherence
    }
}
