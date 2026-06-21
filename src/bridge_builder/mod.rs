pub mod arbitration;
pub mod correction_buffer;
pub mod demo;
pub mod meta_aq;
pub mod operators;
pub mod primitive_aq;
pub mod resonance_field;
pub mod super_aq;
pub mod topology_gate;

#[cfg(test)]
mod tests {
    use super::arbitration::DesignScore;
    use super::demo::run_demo;

    #[test]
    fn design_b_wins_tournament() {
        let t = run_demo();
        assert_eq!(t.winner.id, "B");
        assert_eq!(t.winner.score, DesignScore::Valid(62));
    }

    #[test]
    fn design_c_is_rejected_with_topology_violation() {
        let t = run_demo();
        let c = t.results.iter().find(|r| r.id == "C").expect("design C");
        assert_eq!(c.score, DesignScore::Rejected);
        assert!(!c.topology.is_valid());
    }

    #[test]
    fn all_scores_are_deterministic() {
        let t1 = run_demo();
        let t2 = run_demo();
        assert_eq!(t1.tournament_signature, t2.tournament_signature);
        for (r1, r2) in t1.results.iter().zip(t2.results.iter()) {
            assert_eq!(r1.score, r2.score);
            assert_eq!(r1.signature, r2.signature);
        }
    }

    #[test]
    fn design_scores_match_spec() {
        let t = run_demo();
        let find = |id: &str| t.results.iter().find(|r| r.id == id).unwrap();
        assert_eq!(find("A").score, DesignScore::Valid(46));
        assert_eq!(find("B").score, DesignScore::Valid(62));
        assert_eq!(find("D").score, DesignScore::Valid(50));
        // A, B, D intermediate values
        assert_eq!(find("A").stability, 32);
        assert_eq!(find("A").symmetry,  18);
        assert_eq!(find("A").drift,      4);
        assert_eq!(find("B").stability, 41);
        assert_eq!(find("B").symmetry,  22);
        assert_eq!(find("B").drift,      1);
        assert_eq!(find("D").stability, 35);
        assert_eq!(find("D").symmetry,  12);
        assert_eq!(find("D").drift,      3);
    }
}
