use artificial_qubit_cognition::geom::semantic_comparator::{
    compare_is_a_analogy, compare_semantic_states, compare_semantic_states_with_hierarchy,
    GovernedOperator, IsAHierarchy, SemanticState,
};

fn run_pair(dog: &SemanticState, other: &SemanticState) {
    let baseline = compare_semantic_states(dog.clone(), other.clone(), &[], &[]);
    println!(
        "SemanticRelation({}, {}) baseline = {}",
        dog.name,
        other.name,
        baseline.relation.as_str()
    );
    println!("Baseline signature: {}", baseline.relation_signature);

    let contrasted = compare_semantic_states(
        dog.clone(),
        other.clone(),
        &[],
        &[GovernedOperator::Contrast],
    );
    println!(
        "SemanticRelation({}, {}) after Contrast = {}",
        dog.name,
        other.name,
        contrasted.relation.as_str()
    );
    println!("Contrast signature: {}", contrasted.relation_signature);
    println!();
}

fn run_hierarchical_pair(left: &SemanticState, right: &SemanticState, hierarchy: &IsAHierarchy) {
    let result = compare_semantic_states_with_hierarchy(
        left.clone(),
        right.clone(),
        &[],
        &[],
        hierarchy,
    );

    println!(
        "TypedRelation({}, {}) = {}",
        left.name,
        right.name,
        result.typed_relation.as_str()
    );
    println!("Typed signature: {}", result.typed_signature);
    println!();
}

fn main() {
    // Canonical demo coordinates for deterministic animal-pair comparisons.
    let dog = SemanticState::new("DOG", [2, 1, 3]);
    let wolf = SemanticState::new("WOLF", [2, 1, 3]);
    let cat = SemanticState::new("CAT", [1, 1, 2]);
    let lion = SemanticState::new("LION", [3, 1, 4]);
    let hierarchy = IsAHierarchy::new(&[("LION", "CAT"), ("WOLF", "DOG")]);

    println!("Deterministic Semantic Comparator");
    run_pair(&dog, &wolf);
    run_pair(&dog, &cat);
    run_pair(&dog, &lion);
    run_pair(&cat, &wolf);
    run_pair(&cat, &lion);
    run_pair(&wolf, &lion);

    println!("Deterministic Semantic Comparator (Hierarchy Mode)");
    run_hierarchical_pair(&lion, &cat, &hierarchy);
    run_hierarchical_pair(&wolf, &dog, &hierarchy);
    run_hierarchical_pair(&cat, &lion, &hierarchy);

    let analogy_true = compare_is_a_analogy("LION", "CAT", "WOLF", "DOG", &hierarchy);
    println!(
        "IsAAnalogy(LION:CAT :: WOLF:DOG) = {}",
        if analogy_true.analogous { "Analogous" } else { "NotAnalogous" }
    );
    println!("Analogy signature: {}", analogy_true.analogy_signature);

    let analogy_false = compare_is_a_analogy("LION", "CAT", "WOLF", "LION", &hierarchy);
    println!(
        "IsAAnalogy(LION:CAT :: WOLF:LION) = {}",
        if analogy_false.analogous {
            "Analogous"
        } else {
            "NotAnalogous"
        }
    );
    println!("Analogy signature: {}", analogy_false.analogy_signature);
}
