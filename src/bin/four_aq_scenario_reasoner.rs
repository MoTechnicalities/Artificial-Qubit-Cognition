use artificial_qubit_cognition::geom::four_aq_scenario_reasoner::run_demo;

fn main() {
    let tournament = run_demo();

    println!("4-AQ Deterministic Scenario Reasoner");
    println!();

    for result in &tournament.results {
        println!("Plan {}: {}", result.plan_id, result.description);
        println!("Label: {}", result.label.as_str());
        println!("Score: {}", result.score);
        println!("Signature: {}", result.signature);
        println!();
    }

    println!(
        "Winner: Plan {} ({})",
        tournament.winner.plan_id,
        tournament.winner.label.as_str()
    );
    println!("Tournament signature: {}", tournament.tournament_signature);
}
