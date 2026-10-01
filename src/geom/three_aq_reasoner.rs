use crate::arithmetic::{checked_add, checked_dot, checked_mul, checked_neg, ArithmeticError};
use crate::creativity::{CreativityEvaluation, CreativityGate, CreativityPolicy, SelectionWeights};
use crate::geom::tournament::TournamentError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AQState {
    pub name: String,
    pub coords: [i32; 3],
}

impl AQState {
    pub fn new(name: impl Into<String>, coords: [i32; 3]) -> Self {
        Self {
            name: name.into(),
            coords,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator3AQ {
    Abstraction,
    Contrast,
    AxisFlip,
}

impl Operator3AQ {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Abstraction => "Abstraction",
            Self::Contrast => "Contrast",
            Self::AxisFlip => "AxisFlip",
        }
    }

    pub fn apply(&self, coords: [i32; 3]) -> Result<[i32; 3], ArithmeticError> {
        match self {
            Self::Abstraction => Ok([coords[0].signum(), 0, coords[2].signum()]),
            Self::Contrast => Ok([
                checked_neg(coords[0], "3-AQ contrast")?,
                checked_neg(coords[1], "3-AQ contrast")?,
                checked_neg(coords[2], "3-AQ contrast")?,
            ]),
            Self::AxisFlip => Ok([
                coords[0],
                checked_neg(coords[1], "3-AQ axis flip")?,
                coords[2],
            ]),
        }
    }

    pub fn is_creative_deviation(&self) -> bool {
        matches!(self, Self::AxisFlip)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriadicScenario {
    pub agent: AQState,
    pub context: AQState,
    pub baseline_value: AQState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidatePlan {
    pub name: String,
    pub action_seed: AQState,
    pub operators: Vec<Operator3AQ>,
}

impl CandidatePlan {
    pub fn new(name: impl Into<String>, action_seed: AQState, operators: Vec<Operator3AQ>) -> Self {
        Self {
            name: name.into(),
            action_seed,
            operators,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionLabel {
    Aligned,
    Risky,
    Rejected,
}

impl DecisionLabel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Aligned => "Aligned",
            Self::Risky => "Risky",
            Self::Rejected => "Rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanEvaluation {
    pub plan_name: String,
    pub evolved_action: [i32; 3],
    pub context_alignment: i32,
    pub value_alignment: i32,
    pub score: i32,
    pub label: DecisionLabel,
    pub creativity: Option<CreativityEvaluation>,
    pub selection_score: i64,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TournamentResult3AQ {
    pub winner: PlanEvaluation,
    pub evaluations: Vec<PlanEvaluation>,
    pub tournament_signature: String,
}

/// Runs utility-only arbitration.
///
/// Returns [`TournamentError::NoCandidates`] when `candidates` is empty.
pub fn run_triadic_tournament(
    scenario: &TriadicScenario,
    candidates: &[CandidatePlan],
) -> Result<TournamentResult3AQ, TournamentError> {
    run_tournament(scenario, candidates, None)
}

/// Runs creativity-weighted arbitration without rewarding invalid candidates.
///
/// Returns [`TournamentError::NoCandidates`] when `candidates` is empty.
pub fn run_creative_triadic_tournament(
    scenario: &TriadicScenario,
    candidates: &[CandidatePlan],
    policy: &CreativityPolicy,
    weights: SelectionWeights,
) -> Result<TournamentResult3AQ, TournamentError> {
    run_tournament(scenario, candidates, Some((policy, weights)))
}

fn run_tournament(
    scenario: &TriadicScenario,
    candidates: &[CandidatePlan],
    creativity: Option<(&CreativityPolicy, SelectionWeights)>,
) -> Result<TournamentResult3AQ, TournamentError> {
    if candidates.is_empty() {
        return Err(TournamentError::NoCandidates);
    }

    let mut evaluations: Vec<PlanEvaluation> = candidates
        .iter()
        .map(|candidate| evaluate_candidate(scenario, candidate, creativity))
        .collect::<Result<_, _>>()?;

    evaluations.sort_by(|a, b| {
        b.selection_score
            .cmp(&a.selection_score)
            .then_with(|| b.score.cmp(&a.score))
            .then_with(|| b.context_alignment.cmp(&a.context_alignment))
            .then_with(|| a.plan_name.cmp(&b.plan_name))
    });

    let winner = evaluations[0].clone();
    let tournament_signature = format!(
        "winner:{}|plans:{}",
        winner.plan_name,
        evaluations
            .iter()
            .map(|ev| format!("{}:{}", ev.plan_name, ev.selection_score))
            .collect::<Vec<_>>()
            .join(">")
    );

    Ok(TournamentResult3AQ {
        winner,
        evaluations,
        tournament_signature,
    })
}

fn evaluate_candidate(
    scenario: &TriadicScenario,
    candidate: &CandidatePlan,
    creativity: Option<(&CreativityPolicy, SelectionWeights)>,
) -> Result<PlanEvaluation, TournamentError> {
    let mut action = candidate.action_seed.coords;
    let mut op_trace = Vec::with_capacity(candidate.operators.len());

    for op in &candidate.operators {
        action = op.apply(action)?;
        op_trace.push(op.name());
    }

    // Deterministic correction pass: if the action is adversarial to context,
    // apply one abstraction to reduce drift and re-evaluate.
    let mut corrected = false;
    if checked_dot(action, scenario.context.coords, "3-AQ context alignment")? < 0 {
        action = Operator3AQ::Abstraction.apply(action)?;
        corrected = true;
    }

    let context_alignment = checked_dot(action, scenario.context.coords, "3-AQ context alignment")?;
    let value_alignment = checked_dot(
        action,
        scenario.baseline_value.coords,
        "3-AQ value alignment",
    )?;
    let agent_alignment = checked_dot(action, scenario.agent.coords, "3-AQ agent alignment")?;

    // Weighted deterministic arbitration functional.
    let context_score = checked_mul(2, context_alignment, "3-AQ weighted context score")?;
    let value_score = checked_mul(2, value_alignment, "3-AQ weighted value score")?;
    let score = checked_add(
        checked_add(context_score, value_score, "3-AQ combined alignment score")?,
        agent_alignment,
        "3-AQ total utility score",
    )?;

    let label = if score >= 8 {
        DecisionLabel::Aligned
    } else if score >= 0 {
        DecisionLabel::Risky
    } else {
        DecisionLabel::Rejected
    };

    let creativity_evidence = TriadicCreativityEvidence {
        label,
        declared_operator_count: candidate.operators.len(),
        applied_operator_count: op_trace.len(),
    };
    let creativity_evaluation = creativity.map(|(policy, _)| {
        let operator_sequence = candidate
            .operators
            .iter()
            .map(|operator| operator.name().to_string())
            .collect::<Vec<_>>();
        policy.evaluate(
            &operator_sequence,
            &creativity_evidence,
            &TriadicCreativityGate,
        )
    });
    let selection_score = match (creativity, &creativity_evaluation) {
        (Some((_, weights)), Some(evaluation)) => weights.score(score as i64, evaluation)?,
        _ => score as i64,
    };
    let creativity_tag = creativity_evaluation
        .as_ref()
        .map(|evaluation| format!("|{}", evaluation.canonical_tag()))
        .unwrap_or_default();

    let signature = format!(
        "plan:{}|ops:{}|corrected:{}|action:[{},{},{}]|ctx:{}|val:{}|score:{}|label:{}{}",
        candidate.name,
        if op_trace.is_empty() {
            "none".to_string()
        } else {
            op_trace.join(">")
        },
        corrected,
        action[0],
        action[1],
        action[2],
        context_alignment,
        value_alignment,
        score,
        label.as_str(),
        creativity_tag
    );

    Ok(PlanEvaluation {
        plan_name: candidate.name.clone(),
        evolved_action: action,
        context_alignment,
        value_alignment,
        score,
        label,
        creativity: creativity_evaluation,
        selection_score,
        signature,
    })
}

struct TriadicCreativityEvidence {
    label: DecisionLabel,
    declared_operator_count: usize,
    applied_operator_count: usize,
}

struct TriadicCreativityGate;

impl CreativityGate<TriadicCreativityEvidence> for TriadicCreativityGate {
    fn primary_invariants_preserved(&self, subject: &TriadicCreativityEvidence) -> bool {
        subject.label != DecisionLabel::Rejected
    }

    fn structurally_valid(&self, subject: &TriadicCreativityEvidence) -> bool {
        subject.declared_operator_count == subject.applied_operator_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_scenario() -> TriadicScenario {
        TriadicScenario {
            agent: AQState::new("Agent", [1, 1, 0]),
            context: AQState::new("Context", [1, 0, 1]),
            baseline_value: AQState::new("Value", [1, 0, 1]),
        }
    }

    fn default_candidates() -> Vec<CandidatePlan> {
        vec![
            CandidatePlan::new(
                "Assist",
                AQState::new("ActionAssist", [1, 1, 1]),
                vec![Operator3AQ::Abstraction],
            ),
            CandidatePlan::new("Ignore", AQState::new("ActionIgnore", [0, -1, -1]), vec![]),
            CandidatePlan::new(
                "Escalate",
                AQState::new("ActionEscalate", [-1, 1, -1]),
                vec![Operator3AQ::Contrast],
            ),
        ]
    }

    #[test]
    fn triadic_tournament_selects_expected_winner() {
        let result = run_triadic_tournament(&default_scenario(), &default_candidates()).unwrap();
        assert_eq!(result.winner.plan_name, "Assist");
        assert_eq!(result.winner.label, DecisionLabel::Aligned);
    }

    #[test]
    fn tournament_is_deterministic() {
        let scenario = default_scenario();
        let candidates = default_candidates();

        let first = run_triadic_tournament(&scenario, &candidates).unwrap();
        let second = run_triadic_tournament(&scenario, &candidates).unwrap();

        assert_eq!(first.tournament_signature, second.tournament_signature);
        assert_eq!(first.winner.signature, second.winner.signature);
    }

    #[test]
    fn creativity_weight_can_select_a_lawful_deviation() {
        let policy =
            CreativityPolicy::new(vec![vec![Operator3AQ::Abstraction.name().to_string()]], 1)
                .unwrap();
        let weights = SelectionWeights {
            utility: 1,
            creativity: 2,
        };

        let result = run_creative_triadic_tournament(
            &default_scenario(),
            &default_candidates(),
            &policy,
            weights,
        )
        .unwrap();

        assert_eq!(result.winner.plan_name, "Escalate");
        assert!(result.winner.creativity.as_ref().unwrap().is_creative());
        assert_eq!(result.winner.selection_score, 10);

        let rejected = result
            .evaluations
            .iter()
            .find(|evaluation| evaluation.plan_name == "Ignore")
            .unwrap();
        assert!(!rejected.creativity.as_ref().unwrap().is_creative());
    }

    #[test]
    fn axis_flip_is_a_tagged_creative_deviation_operator() {
        assert!(Operator3AQ::AxisFlip.is_creative_deviation());
        assert!(!Operator3AQ::Abstraction.is_creative_deviation());
    }

    #[test]
    fn arithmetic_overflow_is_rejected() {
        let candidates = vec![CandidatePlan::new(
            "Overflow",
            AQState::new("Action", [i32::MAX, 0, 0]),
            vec![],
        )];
        let scenario = TriadicScenario {
            agent: AQState::new("Agent", [2, 0, 0]),
            context: AQState::new("Context", [2, 0, 0]),
            baseline_value: AQState::new("Value", [2, 0, 0]),
        };

        assert!(matches!(
            run_triadic_tournament(&scenario, &candidates),
            Err(TournamentError::Arithmetic(ArithmeticError::Overflow(_)))
        ));
    }

    #[test]
    fn empty_tournament_returns_an_error() {
        assert_eq!(
            run_triadic_tournament(&default_scenario(), &[]),
            Err(TournamentError::NoCandidates)
        );
    }
}
