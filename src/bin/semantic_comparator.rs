use artificial_qubit_cognition::geom::semantic_comparator::{
    compare_semantic_states, GovernedOperator, SemanticState,
};

fn main() {
    let dog = SemanticState::new("DOG", [2, 1, 3]);
    let wolf = SemanticState::new("WOLF", [2, 1, 3]);

    let baseline = compare_semantic_states(dog.clone(), wolf.clone(), &[], &[])
        .expect("canonical semantic comparison must be in range");
    println!("Deterministic Semantic Comparator");
    println!(
        "SemanticRelation(DOG, WOLF) baseline = {}",
        baseline.relation.as_str()
    );
    println!("Baseline signature: {}", baseline.relation_signature);

    let contrasted = compare_semantic_states(dog, wolf, &[], &[GovernedOperator::Contrast])
        .expect("canonical semantic contrast must be in range");
    println!(
        "SemanticRelation(DOG, WOLF) after Contrast = {}",
        contrasted.relation.as_str()
    );
    println!("Contrast signature: {}", contrasted.relation_signature);
}
