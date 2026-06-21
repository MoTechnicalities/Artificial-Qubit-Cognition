use artificial_qubit_cognition::{
    geom::topology_gate::TopologyStatus,
    power_grid::{arbitration::RoutingScore, demo::run_demo},
};

fn main() {
    let tournament = run_demo();

    println!("Power Grid Reasoner Demo");
    println!("--------------------------------");

    let mut display: Vec<_> = tournament.results.iter().collect();
    display.sort_by_key(|r| r.id.as_str());

    for result in &display {
        println!("Evaluating Routing Plan {}...", result.id);
        println!("  Stability: {}", result.stability);
        println!("  Symmetry: {}", result.symmetry);
        println!("  Drift: {}", result.drift);
        match &result.topology {
            TopologyStatus::Valid => println!("  Topology: Valid"),
            TopologyStatus::Invalid(reason) => println!("  Topology: Invalid ({})", reason),
        }
        match &result.score {
            RoutingScore::Valid(s) => println!("  Score: {}", s),
            RoutingScore::Rejected => println!("  Score: Rejected"),
        }
        println!();
    }

    println!("Winner: Routing Plan {}", tournament.winner.id);
    println!("Tournament Signature:");
    println!("{}", tournament.tournament_signature);
}
