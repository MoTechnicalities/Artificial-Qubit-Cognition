use artificial_qubit_cognition::geom::three_aq_reasoner::{
    run_triadic_tournament, AQState, CandidatePlan, Operator3AQ, TriadicScenario,
};

fn main() {
    let scenario = TriadicScenario {
        agent: AQState::new("Agent", [1, 1, 0]),
        context: AQState::new("Context", [1, 0, 1]),
        baseline_value: AQState::new("Value", [1, 0, 1]),
    };

    let candidates = vec![
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
    ];

    let result = run_triadic_tournament(&scenario, &candidates);

    println!("3-AQ Deterministic Relational Reasoner");
    for evaluation in &result.evaluations {
        println!(
            "Plan={} Label={} Score={} Signature={}",
            evaluation.plan_name,
            evaluation.label.as_str(),
            evaluation.score,
            evaluation.signature
        );
    }

    println!("Winner={}", result.winner.plan_name);
    println!("Tournament signature={}", result.tournament_signature);
}