use crate::bridge_builder::{
    correction_buffer::CorrectionBuffer,
    meta_aq::MetaAQ,
    operators::meta_ops::MetaOp,
    resonance_field::{evaluate, ResonanceField},
    topology_gate::{check_topology, TopologyStatus},
};

/// Score variant for a design result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesignScore {
    Valid(i32),
    Rejected,
}

/// Full evaluation result for a single candidate bridge design.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignResult {
    pub id: String,
    pub stability: i32,
    pub symmetry: i32,
    pub drift: i32,
    pub topology: TopologyStatus,
    pub score: DesignScore,
    pub signature: String,
}

/// A candidate bridge design: meta-AQs + applied meta-operators.
pub struct BridgeDesign {
    pub id: String,
    pub meta_aqs: Vec<MetaAQ>,
    pub meta_ops: Vec<MetaOp>,
}

/// Complete tournament output.
pub struct Tournament {
    /// All results, sorted: valid designs descending by score, rejected last.
    pub results: Vec<DesignResult>,
    pub winner: DesignResult,
    /// Canonical, replay-stable tournament signature.
    pub tournament_signature: String,
}

/// Run the full arbitration tournament over a set of candidate designs.
///
/// For each design:
/// 1. Check topology invariants.
/// 2. Compute resonance scores (stability, symmetry, drift, coherence).
/// 3. Produce a canonical signature.
/// 4. Archive valid designs to the correction buffer.
///
/// Selects the winner as the valid design with the highest total score.
pub fn run_tournament(designs: &[BridgeDesign], buffer: &mut CorrectionBuffer) -> Tournament {
    let field = ResonanceField::default_field();

    // Evaluate in original design order (preserves A→B→C→D ordering for signature).
    let results_ordered: Vec<DesignResult> = designs
        .iter()
        .map(|d| evaluate_design(d, &field, buffer))
        .collect();

    // Build tournament signature from original order before sorting.
    let plan_summary = results_ordered
        .iter()
        .map(|r| match &r.score {
            DesignScore::Valid(s) => format!("{}:{}", r.id, s),
            DesignScore::Rejected => format!("{}:invalid", r.id),
        })
        .collect::<Vec<_>>()
        .join("|");

    // Sort: valid designs descending by score; rejected last.
    let mut results = results_ordered;
    results.sort_by(|a, b| match (&a.score, &b.score) {
        (DesignScore::Valid(sa), DesignScore::Valid(sb)) => sb.cmp(sa),
        (DesignScore::Valid(_), DesignScore::Rejected) => std::cmp::Ordering::Less,
        (DesignScore::Rejected, DesignScore::Valid(_)) => std::cmp::Ordering::Greater,
        (DesignScore::Rejected, DesignScore::Rejected) => a.id.cmp(&b.id),
    });

    let winner = results
        .iter()
        .find(|r| r.score != DesignScore::Rejected)
        .expect("at least one valid design required")
        .clone();

    let tournament_signature =
        format!("bridge:metaAQ|winner:{}|{}", winner.id, plan_summary);

    Tournament { results, winner, tournament_signature }
}

fn evaluate_design(
    design: &BridgeDesign,
    field: &ResonanceField,
    buffer: &mut CorrectionBuffer,
) -> DesignResult {
    let topology = check_topology(&design.meta_aqs);
    let scores = evaluate(field, &design.meta_aqs, &design.meta_ops);

    let (score, signature) = if topology.is_valid() {
        let total = scores.total_score();
        let label = if total >= 60 {
            "HighPerformance"
        } else if total >= 45 {
            "Aligned"
        } else {
            "Adequate"
        };
        let sig = format!(
            "design:{}|label:{}|score:{}|stab:{}|sym:{}|drift:{}|coherence:{}",
            design.id, label, total,
            scores.stability, scores.symmetry, scores.drift, scores.structural_coherence
        );
        buffer.archive(&design.id, &sig);
        (DesignScore::Valid(total), sig)
    } else {
        let sig = format!(
            "design:{}|label:Rejected|topology:invalid",
            design.id
        );
        (DesignScore::Rejected, sig)
    };

    DesignResult {
        id: design.id.clone(),
        stability: scores.stability,
        symmetry: scores.symmetry,
        drift: scores.drift,
        topology,
        score,
        signature,
    }
}
