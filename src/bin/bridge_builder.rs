use artificial_qubit_cognition::bridge_builder::{
    arbitration::DesignScore, demo::run_demo, topology_gate::TopologyStatus,
};

fn main() {
    let tournament = run_demo();

    println!("Bridge-Builder Reasoner Demo");
    println!("--------------------------------");

    // Print results in original design order (A, B, C, D).
    let mut display: Vec<_> = tournament.results.iter().collect();
    display.sort_by_key(|r| r.id.as_str());

    for result in &display {
        println!("Evaluating Design {}...", result.id);
        println!("  Stability: {}", result.stability);
        println!("  Symmetry: {}", result.symmetry);
        println!("  Drift: {}", result.drift);
        match &result.topology {
            TopologyStatus::Valid => println!("  Topology: Valid"),
            TopologyStatus::Invalid(reason) => println!("  Topology: Invalid ({})", reason),
        }
        match &result.score {
            DesignScore::Valid(s) => println!("  Score: {}", s),
            DesignScore::Rejected => println!("  Score: Rejected"),
        }
        println!();
    }

    println!("Winner: Design {}", tournament.winner.id);
    println!("Tournament Signature:");
    println!("{}", tournament.tournament_signature);
}
