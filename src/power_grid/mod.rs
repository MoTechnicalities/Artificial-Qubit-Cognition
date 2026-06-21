pub mod arbitration;
pub mod demo;
pub mod evaluation;
pub mod meta_aq;
pub mod operators;
pub mod primitive_aq;
pub mod super_aq;

#[cfg(test)]
mod tests {
    use super::arbitration::RoutingScore;
    use super::demo::run_demo;

    #[test]
    fn plan_b_wins_tournament() {
        let t = run_demo();
        assert_eq!(t.winner.id, "B");
        assert_eq!(t.winner.score, RoutingScore::Valid(57));
    }

    #[test]
    fn plan_c_is_rejected_with_topology_violation() {
        let t = run_demo();
        let c = t.results.iter().find(|r| r.id == "C").expect("plan C");
        assert_eq!(c.score, RoutingScore::Rejected);
        assert!(!c.topology.is_valid());
    }

    #[test]
    fn tournament_signature_is_deterministic() {
        let t1 = run_demo();
        let t2 = run_demo();
        assert_eq!(t1.tournament_signature, t2.tournament_signature);
    }

    #[test]
    fn routing_scores_match_spec() {
        let t = run_demo();
        let find = |id: &str| t.results.iter().find(|r| r.id == id).unwrap();
        assert_eq!(find("A").score, RoutingScore::Valid(41));
        assert_eq!(find("B").score, RoutingScore::Valid(57));
        assert_eq!(find("D").score, RoutingScore::Valid(50));
        assert_eq!(find("A").stability, 30);
        assert_eq!(find("A").symmetry,  16);
        assert_eq!(find("A").drift,      5);
        assert_eq!(find("B").stability, 39);
        assert_eq!(find("B").symmetry,  20);
        assert_eq!(find("B").drift,      2);
        assert_eq!(find("D").stability, 33);
        assert_eq!(find("D").symmetry,  14);
        assert_eq!(find("D").drift,      3);
    }
}
