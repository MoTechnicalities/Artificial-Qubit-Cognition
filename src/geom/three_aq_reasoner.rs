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

    pub fn apply(&self, coords: [i32; 3]) -> [i32; 3] {
        match self {
            Self::Abstraction => [coords[0].signum(), 0, coords[2].signum()],
            Self::Contrast => [-coords[0], -coords[1], -coords[2]],
            Self::AxisFlip => [coords[0], -coords[1], coords[2]],
        }
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
    pub fn new(
        name: impl Into<String>,
        action_seed: AQState,
        operators: Vec<Operator3AQ>,
    ) -> Self {
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
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TournamentResult3AQ {
    pub winner: PlanEvaluation,
    pub evaluations: Vec<PlanEvaluation>,
    pub tournament_signature: String,
}

pub fn run_triadic_tournament(
    scenario: &TriadicScenario,
    candidates: &[CandidatePlan],
) -> TournamentResult3AQ {
    let mut evaluations: Vec<PlanEvaluation> = candidates
        .iter()
        .map(|candidate| evaluate_candidate(scenario, candidate))
        .collect();

    evaluations.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| b.context_alignment.cmp(&a.context_alignment))
            .then_with(|| a.plan_name.cmp(&b.plan_name))
    });

    let winner = evaluations[0].clone();
    let tournament_signature = format!(
        "winner:{}|plans:{}",
        winner.plan_name,
        evaluations
            .iter()
            .map(|ev| format!("{}:{}", ev.plan_name, ev.score))
            .collect::<Vec<_>>()
            .join(">")
    );

    TournamentResult3AQ {
        winner,
        evaluations,
        tournament_signature,
    }
}

fn evaluate_candidate(scenario: &TriadicScenario, candidate: &CandidatePlan) -> PlanEvaluation {
    let mut action = candidate.action_seed.coords;
    let mut op_trace = Vec::with_capacity(candidate.operators.len());

    for op in &candidate.operators {
        action = op.apply(action);
        op_trace.push(op.name());
    }

    // Deterministic correction pass: if the action is adversarial to context,
    // apply one abstraction to reduce drift and re-evaluate.
    let mut corrected = false;
    if dot(action, scenario.context.coords) < 0 {
        action = Operator3AQ::Abstraction.apply(action);
        corrected = true;
    }

    let context_alignment = dot(action, scenario.context.coords);
    let value_alignment = dot(action, scenario.baseline_value.coords);
    let agent_alignment = dot(action, scenario.agent.coords);

    // Weighted deterministic arbitration functional.
    let score = (2 * context_alignment) + (2 * value_alignment) + agent_alignment;

    let label = if score >= 8 {
        DecisionLabel::Aligned
    } else if score >= 0 {
        DecisionLabel::Risky
    } else {
        DecisionLabel::Rejected
    };

    let signature = format!(
        "plan:{}|ops:{}|corrected:{}|action:[{},{},{}]|ctx:{}|val:{}|score:{}|label:{}",
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
        label.as_str()
    );

    PlanEvaluation {
        plan_name: candidate.name.clone(),
        evolved_action: action,
        context_alignment,
        value_alignment,
        score,
        label,
        signature,
    }
}

fn dot(a: [i32; 3], b: [i32; 3]) -> i32 {
    (a[0] * b[0]) + (a[1] * b[1]) + (a[2] * b[2])
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
            CandidatePlan::new(
                "Ignore",
                AQState::new("ActionIgnore", [0, -1, -1]),
                vec![],
            ),
            CandidatePlan::new(
                "Escalate",
                AQState::new("ActionEscalate", [-1, 1, -1]),
                vec![Operator3AQ::Contrast],
            ),
        ]
    }

    #[test]
    fn triadic_tournament_selects_expected_winner() {
        let result = run_triadic_tournament(&default_scenario(), &default_candidates());
        assert_eq!(result.winner.plan_name, "Assist");
        assert_eq!(result.winner.label, DecisionLabel::Aligned);
    }

    #[test]
    fn tournament_is_deterministic() {
        let scenario = default_scenario();
        let candidates = default_candidates();

        let first = run_triadic_tournament(&scenario, &candidates);
        let second = run_triadic_tournament(&scenario, &candidates);

        assert_eq!(first.tournament_signature, second.tournament_signature);
        assert_eq!(first.winner.signature, second.winner.signature);
    }
}