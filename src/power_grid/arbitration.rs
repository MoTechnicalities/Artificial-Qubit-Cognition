use crate::geom::{
    correction_buffer::CorrectionBuffer,
    resonance_field::ResonanceField,
    topology_gate::TopologyStatus,
};
use crate::power_grid::{
    evaluation::{check_topology, evaluate_resonance},
    meta_aq::MetaAQ,
    operators::meta_ops::MetaOp,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingScore {
    Valid(i32),
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingResult {
    pub id: String,
    pub stability: i32,
    pub symmetry: i32,
    pub drift: i32,
    pub topology: TopologyStatus,
    pub score: RoutingScore,
    pub signature: String,
}

pub struct GridDesign {
    pub id: String,
    pub meta_aqs: Vec<MetaAQ>,
    pub meta_ops: Vec<MetaOp>,
}

pub struct GridTournament {
    /// Sorted: valid results descending by score, rejected last.
    pub results: Vec<RoutingResult>,
    pub winner: RoutingResult,
    pub tournament_signature: String,
}

pub fn run_tournament(
    designs: &[GridDesign],
    buffer: &mut CorrectionBuffer,
) -> GridTournament {
    let field = ResonanceField::default_field();

    let results_ordered: Vec<RoutingResult> = designs
        .iter()
        .map(|d| evaluate_design(d, &field, buffer))
        .collect();

    // Build signature from original A→B→C→D order before sorting.
    let plan_summary = results_ordered
        .iter()
        .map(|r| match &r.score {
            RoutingScore::Valid(s) => format!("{}:{}", r.id, s),
            RoutingScore::Rejected => format!("{}:invalid", r.id),
        })
        .collect::<Vec<_>>()
        .join("|");

    let mut results = results_ordered;
    results.sort_by(|a, b| match (&a.score, &b.score) {
        (RoutingScore::Valid(sa), RoutingScore::Valid(sb)) => sb.cmp(sa),
        (RoutingScore::Valid(_), RoutingScore::Rejected) => std::cmp::Ordering::Less,
        (RoutingScore::Rejected, RoutingScore::Valid(_)) => std::cmp::Ordering::Greater,
        (RoutingScore::Rejected, RoutingScore::Rejected) => a.id.cmp(&b.id),
    });

    let winner = results
        .iter()
        .find(|r| r.score != RoutingScore::Rejected)
        .expect("at least one valid routing plan required")
        .clone();

    let tournament_signature =
        format!("grid:metaAQ|winner:{}|{}", winner.id, plan_summary);

    GridTournament { results, winner, tournament_signature }
}

fn evaluate_design(
    design: &GridDesign,
    field: &ResonanceField,
    buffer: &mut CorrectionBuffer,
) -> RoutingResult {
    let topology = check_topology(&design.meta_aqs);
    let scores = evaluate_resonance(field, &design.meta_aqs, &design.meta_ops);

    let (score, signature) = if topology.is_valid() {
        let total = scores.total_score();
        let label = if total >= 55 {
            "HighCapacity"
        } else if total >= 40 {
            "Nominal"
        } else {
            "Marginal"
        };
        let sig = format!(
            "grid:{}|label:{}|score:{}|stab:{}|sym:{}|drift:{}|coherence:{}",
            design.id, label, total,
            scores.stability, scores.symmetry, scores.drift, scores.structural_coherence
        );
        buffer.archive(&design.id, &sig);
        (RoutingScore::Valid(total), sig)
    } else {
        let sig = format!("grid:{}|label:Rejected|topology:invalid", design.id);
        (RoutingScore::Rejected, sig)
    };

    RoutingResult {
        id: design.id.clone(),
        stability: scores.stability,
        symmetry: scores.symmetry,
        drift: scores.drift,
        topology,
        score,
        signature,
    }
}
