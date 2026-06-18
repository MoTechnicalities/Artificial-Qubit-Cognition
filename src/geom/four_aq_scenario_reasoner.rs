#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioState {
    pub situation: [i32; 3],
    pub agent: [i32; 3],
    pub action: [i32; 3],
    pub outcome: [i32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioOperator {
    AlignWithValues,
    RiskAmplify,
    ResponsibilityContrast,
}

impl ScenarioOperator {
    pub fn name(&self) -> &'static str {
        match self {
            Self::AlignWithValues => "AlignWithValues",
            Self::RiskAmplify => "RiskAmplify",
            Self::ResponsibilityContrast => "ResponsibilityContrast",
        }
    }

    pub fn matrix(&self) -> [[i32; 3]; 3] {
        match self {
            // Pulls vectors toward value-stable orientation.
            Self::AlignWithValues => [[1, 0, -1], [0, 1, 1], [0, 0, 1]],
            // Pushes trajectories toward harm-like geometry in risky contexts.
            Self::RiskAmplify => [[1, 0, 1], [0, 1, 0], [0, 0, 1]],
            // Contrasts responsibility axis while preserving primary direction.
            Self::ResponsibilityContrast => [[1, 0, 0], [0, -1, 0], [0, 0, 1]],
        }
    }

    pub fn apply(&self, v: [i32; 3]) -> [i32; 3] {
        apply_matrix(self.matrix(), v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorTarget {
    Situation,
    Agent,
    Action,
    Outcome,
    AgentAndAction,
    ActionAndOutcome,
}

impl OperatorTarget {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Situation => "Situation",
            Self::Agent => "Agent",
            Self::Action => "Action",
            Self::Outcome => "Outcome",
            Self::AgentAndAction => "AgentAndAction",
            Self::ActionAndOutcome => "ActionAndOutcome",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorStep {
    pub operator: ScenarioOperator,
    pub target: OperatorTarget,
}

impl OperatorStep {
    pub fn new(operator: ScenarioOperator, target: OperatorTarget) -> Self {
        Self { operator, target }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioPlan {
    pub id: String,
    pub description: String,
    pub state: ScenarioState,
    pub steps: Vec<OperatorStep>,
}

impl ScenarioPlan {
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        state: ScenarioState,
        steps: Vec<OperatorStep>,
    ) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            state,
            steps,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioLabel {
    Aligned,
    MildAlignment,
    Risky,
    Negligent,
    Overkill,
}

impl ScenarioLabel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Aligned => "Aligned",
            Self::MildAlignment => "MildAlignment",
            Self::Risky => "Risky",
            Self::Negligent => "Negligent",
            Self::Overkill => "Overkill",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioResult {
    pub plan_id: String,
    pub description: String,
    pub label: ScenarioLabel,
    pub score: i32,
    pub trajectory_signature: String,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioTournament {
    pub results: Vec<ScenarioResult>,
    pub winner: ScenarioResult,
    pub tournament_signature: String,
}

pub const S_SAFE_CONTEXT: [i32; 3] = [1, 1, 1];
pub const S_RISKY_CONTEXT: [i32; 3] = [2, 1, 3];

pub const A_HELPFUL: [i32; 3] = [1, 2, 2];
pub const A_SELFISH: [i32; 3] = [2, 2, 1];

pub const ACT_HELP: [i32; 3] = [1, 3, 2];
pub const ACT_IGNORE: [i32; 3] = [2, 1, 1];

pub const O_STABLE: [i32; 3] = [1, 2, 3];
pub const O_HARM: [i32; 3] = [3, 1, 1];

pub fn default_plans() -> Vec<ScenarioPlan> {
    vec![
        ScenarioPlan::new(
            "A",
            "Helpful in Risky Context",
            ScenarioState {
                situation: S_RISKY_CONTEXT,
                agent: A_HELPFUL,
                action: ACT_HELP,
                outcome: O_STABLE,
            },
            vec![
                OperatorStep::new(ScenarioOperator::RiskAmplify, OperatorTarget::Outcome),
                OperatorStep::new(ScenarioOperator::AlignWithValues, OperatorTarget::ActionAndOutcome),
            ],
        ),
        ScenarioPlan::new(
            "B",
            "Ignore in Risky Context",
            ScenarioState {
                situation: S_RISKY_CONTEXT,
                agent: A_SELFISH,
                action: ACT_IGNORE,
                outcome: O_HARM,
            },
            vec![
                OperatorStep::new(ScenarioOperator::RiskAmplify, OperatorTarget::Outcome),
                OperatorStep::new(
                    ScenarioOperator::ResponsibilityContrast,
                    OperatorTarget::AgentAndAction,
                ),
            ],
        ),
        ScenarioPlan::new(
            "C",
            "Helpful in Safe Context",
            ScenarioState {
                situation: S_SAFE_CONTEXT,
                agent: A_HELPFUL,
                action: ACT_HELP,
                outcome: O_STABLE,
            },
            vec![OperatorStep::new(
                ScenarioOperator::AlignWithValues,
                OperatorTarget::ActionAndOutcome,
            )],
        ),
        ScenarioPlan::new(
            "D",
            "Escalatory Help in Safe Context",
            ScenarioState {
                situation: S_SAFE_CONTEXT,
                agent: A_HELPFUL,
                action: ACT_HELP,
                outcome: O_STABLE,
            },
            vec![
                OperatorStep::new(ScenarioOperator::AlignWithValues, OperatorTarget::Action),
                OperatorStep::new(ScenarioOperator::AlignWithValues, OperatorTarget::Action),
                OperatorStep::new(ScenarioOperator::RiskAmplify, OperatorTarget::Outcome),
            ],
        ),
    ]
}

pub fn run_tournament(plans: &[ScenarioPlan]) -> ScenarioTournament {
    let mut results: Vec<ScenarioResult> = plans.iter().map(evaluate_plan).collect();

    results.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.plan_id.cmp(&b.plan_id)));
    let winner = results[0].clone();

    let tournament_signature = format!(
        "scenario:4AQ|winner:{}|plans:{}",
        winner.plan_id,
        results
            .iter()
            .map(|r| format!("{}:{}:{}", r.plan_id, r.label.as_str(), r.score))
            .collect::<Vec<_>>()
            .join(">")
    );

    ScenarioTournament {
        results,
        winner,
        tournament_signature,
    }
}

pub fn run_demo() -> ScenarioTournament {
    run_tournament(&default_plans())
}

fn evaluate_plan(plan: &ScenarioPlan) -> ScenarioResult {
    let mut state = plan.state.clone();
    let mut op_trace: Vec<String> = Vec::with_capacity(plan.steps.len());

    for step in &plan.steps {
        apply_step(&mut state, step);
        op_trace.push(format!("{}:{}", step.operator.name(), step.target.name()));
    }

    let outcome_score = dot(state.outcome, O_STABLE) - dot(state.outcome, O_HARM);

    let agent_helpfulness = dot(state.agent, A_HELPFUL) - dot(state.agent, A_SELFISH);
    let action_helpfulness = dot(state.action, ACT_HELP) - dot(state.action, ACT_IGNORE);

    let agent_action_coherence = if agent_helpfulness > 0 && action_helpfulness > 0 {
        3
    } else if agent_helpfulness < 0 && action_helpfulness < 0 {
        1
    } else {
        -3
    };

    // Positive means geometrically closer to risky context, negative means closer to safe context.
    let situation_risk = l1_distance(state.situation, S_SAFE_CONTEXT)
        - l1_distance(state.situation, S_RISKY_CONTEXT);
    let situation_appropriateness = if situation_risk >= 0 {
        if action_helpfulness > 0 {
            11
        } else {
            -10
        }
    } else if action_helpfulness > 0 {
        1
    } else {
        0
    };

    // Fixed deterministic governance coefficients.
    let alpha = 1;
    let beta = 2;
    let gamma = 3;

    let score = (alpha * outcome_score)
        + (beta * agent_action_coherence)
        + (gamma * situation_appropriateness);

    let action_intensity = l1_norm(state.action);
    let label = if situation_risk < 0 && action_helpfulness > 0 && action_intensity >= 10 {
        ScenarioLabel::Overkill
    } else if score >= 30 {
        ScenarioLabel::Aligned
    } else if score >= 10 {
        ScenarioLabel::MildAlignment
    } else if score >= 0 {
        ScenarioLabel::Risky
    } else {
        ScenarioLabel::Negligent
    };

    let trajectory_signature = format!(
        "ops:{}|S:[{},{},{}]|A:[{},{},{}]|ACT:[{},{},{}]|O:[{},{},{}]",
        if op_trace.is_empty() {
            "none".to_string()
        } else {
            op_trace.join(">")
        },
        state.situation[0],
        state.situation[1],
        state.situation[2],
        state.agent[0],
        state.agent[1],
        state.agent[2],
        state.action[0],
        state.action[1],
        state.action[2],
        state.outcome[0],
        state.outcome[1],
        state.outcome[2],
    );

    let signature = format!(
        "plan:{}|label:{}|score:{}|traj:{}",
        plan.id,
        label.as_str(),
        score,
        trajectory_signature
    );

    ScenarioResult {
        plan_id: plan.id.clone(),
        description: plan.description.clone(),
        label,
        score,
        trajectory_signature,
        signature,
    }
}

fn apply_step(state: &mut ScenarioState, step: &OperatorStep) {
    match step.target {
        OperatorTarget::Situation => state.situation = step.operator.apply(state.situation),
        OperatorTarget::Agent => state.agent = step.operator.apply(state.agent),
        OperatorTarget::Action => state.action = step.operator.apply(state.action),
        OperatorTarget::Outcome => state.outcome = step.operator.apply(state.outcome),
        OperatorTarget::AgentAndAction => {
            state.agent = step.operator.apply(state.agent);
            state.action = step.operator.apply(state.action);
        }
        OperatorTarget::ActionAndOutcome => {
            state.action = step.operator.apply(state.action);
            state.outcome = step.operator.apply(state.outcome);
        }
    }
}

fn apply_matrix(m: [[i32; 3]; 3], v: [i32; 3]) -> [i32; 3] {
    [
        (m[0][0] * v[0]) + (m[0][1] * v[1]) + (m[0][2] * v[2]),
        (m[1][0] * v[0]) + (m[1][1] * v[1]) + (m[1][2] * v[2]),
        (m[2][0] * v[0]) + (m[2][1] * v[1]) + (m[2][2] * v[2]),
    ]
}

fn dot(a: [i32; 3], b: [i32; 3]) -> i32 {
    (a[0] * b[0]) + (a[1] * b[1]) + (a[2] * b[2])
}

fn l1_norm(v: [i32; 3]) -> i32 {
    v[0].abs() + v[1].abs() + v[2].abs()
}

fn l1_distance(a: [i32; 3], b: [i32; 3]) -> i32 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_a_wins_tournament() {
        let tournament = run_demo();
        assert_eq!(tournament.winner.plan_id, "A");
        assert_eq!(tournament.winner.label, ScenarioLabel::Aligned);
    }

    #[test]
    fn plan_b_is_negligent() {
        let tournament = run_demo();
        let plan_b = tournament
            .results
            .iter()
            .find(|r| r.plan_id == "B")
            .expect("plan B should exist");

        assert_eq!(plan_b.label, ScenarioLabel::Negligent);
        assert!(plan_b.score < 0);
    }

    #[test]
    fn signatures_are_deterministic() {
        let first = run_demo();
        let second = run_demo();

        assert_eq!(first.tournament_signature, second.tournament_signature);
        assert_eq!(first.results, second.results);
    }
}
