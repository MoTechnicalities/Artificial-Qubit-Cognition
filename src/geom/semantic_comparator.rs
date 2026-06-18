#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticState {
    pub name: String,
    pub coords: [i32; 3],
}

impl SemanticState {
    pub fn new(name: impl Into<String>, coords: [i32; 3]) -> Self {
        Self {
            name: name.into(),
            coords,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernedOperator {
    Abstraction,
    Contrast,
    AxisFlip,
}

impl GovernedOperator {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Abstraction => "Abstraction",
            Self::Contrast => "Contrast",
            Self::AxisFlip => "AxisFlip",
        }
    }

    pub fn apply(&self, coords: [i32; 3]) -> [i32; 3] {
        match self {
            // Project to the primary semantic axis while preserving sign.
            Self::Abstraction => {
                let z = if coords[2] == 0 { 0 } else { coords[2].signum() };
                [0, 0, z]
            }
            // Negate all axes to represent explicit contrast.
            Self::Contrast => [-coords[0], -coords[1], -coords[2]],
            // Invert the secondary axis while preserving primary orientation.
            Self::AxisFlip => [coords[0], -coords[1], coords[2]],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrajectoryStep {
    pub operator: GovernedOperator,
    pub coords: [i32; 3],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticTrajectory {
    pub origin: SemanticState,
    pub steps: Vec<TrajectoryStep>,
    pub final_state: SemanticState,
    pub signature: String,
}

impl SemanticTrajectory {
    pub fn build(origin: SemanticState, operators: &[GovernedOperator]) -> Self {
        let mut coords = origin.coords;
        let mut steps = Vec::with_capacity(operators.len());

        for operator in operators {
            coords = operator.apply(coords);
            steps.push(TrajectoryStep {
                operator: *operator,
                coords,
            });
        }

        let final_state = SemanticState {
            name: origin.name.clone(),
            coords,
        };

        let signature = canonical_trajectory_signature(&origin, &steps);

        Self {
            origin,
            steps,
            final_state,
            signature,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticRelation {
    Reinforcement,
    Alignment,
    Contrast,
    Conflict,
}

impl SemanticRelation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Reinforcement => "Reinforcement",
            Self::Alignment => "Alignment",
            Self::Contrast => "Contrast",
            Self::Conflict => "Conflict",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonResult {
    pub left_trajectory: SemanticTrajectory,
    pub right_trajectory: SemanticTrajectory,
    pub relation: SemanticRelation,
    pub relation_signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsAHierarchy {
    parent_of: std::collections::HashMap<String, String>,
}

impl IsAHierarchy {
    pub fn new(edges: &[(&str, &str)]) -> Self {
        let mut parent_of = std::collections::HashMap::with_capacity(edges.len());

        for (child, parent) in edges {
            parent_of.insert((*child).to_string(), (*parent).to_string());
        }

        Self { parent_of }
    }

    pub fn is_a(&self, child: &str, ancestor: &str) -> bool {
        if child == ancestor {
            return true;
        }

        let mut cursor = child;
        while let Some(parent) = self.parent_of.get(cursor) {
            if parent == ancestor {
                return true;
            }
            cursor = parent;
        }

        false
    }

    pub fn distance_to_ancestor(&self, child: &str, ancestor: &str) -> Option<usize> {
        if child == ancestor {
            return Some(0);
        }

        let mut cursor = child;
        let mut depth = 0usize;
        while let Some(parent) = self.parent_of.get(cursor) {
            depth += 1;
            if parent == ancestor {
                return Some(depth);
            }
            cursor = parent;
        }

        None
    }

    pub fn nearest_shared_ancestor(&self, left: &str, right: &str) -> Option<(String, usize, usize)> {
        let left_ancestors = self.ancestors_with_depth(left);
        let right_ancestors = self.ancestors_with_depth(right);

        let mut best: Option<(String, usize, usize)> = None;

        for (ancestor, left_depth) in left_ancestors {
            if let Some(right_depth) = right_ancestors.get(&ancestor) {
                let candidate = (ancestor, left_depth, *right_depth);
                match &best {
                    None => best = Some(candidate),
                    Some((_, best_left, best_right)) => {
                        if left_depth + right_depth < *best_left + *best_right {
                            best = Some(candidate);
                        }
                    }
                }
            }
        }

        best
    }

    fn ancestors_with_depth(&self, node: &str) -> std::collections::HashMap<String, usize> {
        let mut out = std::collections::HashMap::new();
        let mut cursor = node;
        let mut depth = 0usize;

        out.insert(cursor.to_string(), depth);

        while let Some(parent) = self.parent_of.get(cursor) {
            depth += 1;
            out.insert(parent.clone(), depth);
            cursor = parent;
        }

        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypedSemanticRelation {
    SameConcept,
    LeftIsAOfRight,
    RightIsAOfLeft,
    SharedAncestor,
    Unrelated,
}

impl TypedSemanticRelation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SameConcept => "SameConcept",
            Self::LeftIsAOfRight => "LeftIsAOfRight",
            Self::RightIsAOfLeft => "RightIsAOfLeft",
            Self::SharedAncestor => "SharedAncestor",
            Self::Unrelated => "Unrelated",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchicalComparisonResult {
    pub base: ComparisonResult,
    pub typed_relation: TypedSemanticRelation,
    pub shared_ancestor: Option<String>,
    pub typed_signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsAAnalogyResult {
    pub left_pair_is_a: bool,
    pub right_pair_is_a: bool,
    pub left_depth: Option<usize>,
    pub right_depth: Option<usize>,
    pub analogous: bool,
    pub analogy_signature: String,
}

pub fn compare_semantic_states(
    left: SemanticState,
    right: SemanticState,
    left_ops: &[GovernedOperator],
    right_ops: &[GovernedOperator],
) -> ComparisonResult {
    let left_trajectory = SemanticTrajectory::build(left, left_ops);
    let right_trajectory = SemanticTrajectory::build(right, right_ops);

    let relation = measure_relation(
        left_trajectory.final_state.coords,
        right_trajectory.final_state.coords,
    );

    let relation_signature = format!(
        "relation:{}|left:{}|right:{}",
        relation.as_str(),
        left_trajectory.signature,
        right_trajectory.signature
    );

    ComparisonResult {
        left_trajectory,
        right_trajectory,
        relation,
        relation_signature,
    }
}

pub fn compare_semantic_states_with_hierarchy(
    left: SemanticState,
    right: SemanticState,
    left_ops: &[GovernedOperator],
    right_ops: &[GovernedOperator],
    hierarchy: &IsAHierarchy,
) -> HierarchicalComparisonResult {
    let base = compare_semantic_states(left.clone(), right.clone(), left_ops, right_ops);

    let (typed_relation, shared_ancestor) = if left.name == right.name {
        (TypedSemanticRelation::SameConcept, Some(left.name.clone()))
    } else if hierarchy.is_a(&left.name, &right.name) {
        (TypedSemanticRelation::LeftIsAOfRight, Some(right.name.clone()))
    } else if hierarchy.is_a(&right.name, &left.name) {
        (TypedSemanticRelation::RightIsAOfLeft, Some(left.name.clone()))
    } else if let Some((ancestor, _, _)) = hierarchy.nearest_shared_ancestor(&left.name, &right.name) {
        (TypedSemanticRelation::SharedAncestor, Some(ancestor))
    } else {
        (TypedSemanticRelation::Unrelated, None)
    };

    let shared_ancestor_str = shared_ancestor.as_deref().unwrap_or("none");
    let typed_signature = format!(
        "typed:{}|shared_ancestor:{}|base:{}",
        typed_relation.as_str(),
        shared_ancestor_str,
        base.relation_signature
    );

    HierarchicalComparisonResult {
        base,
        typed_relation,
        shared_ancestor,
        typed_signature,
    }
}

pub fn compare_is_a_analogy(
    left_child: &str,
    left_parent: &str,
    right_child: &str,
    right_parent: &str,
    hierarchy: &IsAHierarchy,
) -> IsAAnalogyResult {
    let left_depth = hierarchy.distance_to_ancestor(left_child, left_parent);
    let right_depth = hierarchy.distance_to_ancestor(right_child, right_parent);

    let left_pair_is_a = left_depth.is_some();
    let right_pair_is_a = right_depth.is_some();

    let analogous = match (left_depth, right_depth) {
        (Some(ld), Some(rd)) => ld == rd,
        _ => false,
    };

    let analogy_signature = format!(
        "analogy:{}:{}::{}:{}|left_depth:{}|right_depth:{}|result:{}",
        left_child,
        left_parent,
        right_child,
        right_parent,
        left_depth
            .map(|d| d.to_string())
            .unwrap_or_else(|| "none".to_string()),
        right_depth
            .map(|d| d.to_string())
            .unwrap_or_else(|| "none".to_string()),
        if analogous { "Analogous" } else { "NotAnalogous" }
    );

    IsAAnalogyResult {
        left_pair_is_a,
        right_pair_is_a,
        left_depth,
        right_depth,
        analogous,
        analogy_signature,
    }
}

pub fn measure_relation(left: [i32; 3], right: [i32; 3]) -> SemanticRelation {
    if left == right {
        return SemanticRelation::Reinforcement;
    }

    let dot = left[0] * right[0] + left[1] * right[1] + left[2] * right[2];

    if dot > 0 {
        SemanticRelation::Alignment
    } else if dot < 0 {
        SemanticRelation::Contrast
    } else {
        SemanticRelation::Conflict
    }
}

fn canonical_trajectory_signature(origin: &SemanticState, steps: &[TrajectoryStep]) -> String {
    let step_str = if steps.is_empty() {
        "none".to_string()
    } else {
        steps
            .iter()
            .map(|step| {
                format!(
                    "{}:[{},{},{}]",
                    step.operator.name(),
                    step.coords[0],
                    step.coords[1],
                    step.coords[2]
                )
            })
            .collect::<Vec<_>>()
            .join(">")
    };

    format!(
        "origin:{}:[{},{},{}]|steps:{}",
        origin.name, origin.coords[0], origin.coords[1], origin.coords[2], step_str
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_states_yield_reinforcement() {
        let dog = SemanticState::new("DOG", [2, 1, 3]);
        let wolf = SemanticState::new("WOLF", [2, 1, 3]);

        let result = compare_semantic_states(dog, wolf, &[], &[]);
        assert_eq!(result.relation, SemanticRelation::Reinforcement);
    }

    #[test]
    fn contrast_operator_yields_contrast_relation() {
        let dog = SemanticState::new("DOG", [2, 1, 3]);
        let wolf = SemanticState::new("WOLF", [2, 1, 3]);

        let result = compare_semantic_states(dog, wolf, &[], &[GovernedOperator::Contrast]);
        assert_eq!(result.relation, SemanticRelation::Contrast);
    }

    #[test]
    fn orthogonal_states_yield_conflict() {
        let a = SemanticState::new("A", [1, 0, 0]);
        let b = SemanticState::new("B", [0, 1, 0]);

        let result = compare_semantic_states(a, b, &[], &[]);
        assert_eq!(result.relation, SemanticRelation::Conflict);
    }

    #[test]
    fn signature_is_deterministic() {
        let dog = SemanticState::new("DOG", [2, 1, 3]);
        let wolf = SemanticState::new("WOLF", [2, 1, 3]);

        let first = compare_semantic_states(
            dog.clone(),
            wolf.clone(),
            &[GovernedOperator::AxisFlip],
            &[GovernedOperator::Contrast],
        );
        let second = compare_semantic_states(
            dog,
            wolf,
            &[GovernedOperator::AxisFlip],
            &[GovernedOperator::Contrast],
        );

        assert_eq!(first.relation_signature, second.relation_signature);
    }

    #[test]
    fn hierarchy_mode_detects_subtype() {
        let hierarchy = IsAHierarchy::new(&[("LION", "CAT"), ("WOLF", "DOG")]);
        let lion = SemanticState::new("LION", [3, 1, 4]);
        let cat = SemanticState::new("CAT", [1, 1, 2]);

        let result = compare_semantic_states_with_hierarchy(lion, cat, &[], &[], &hierarchy);
        assert_eq!(result.typed_relation, TypedSemanticRelation::LeftIsAOfRight);
        assert!(result.typed_signature.contains("typed:LeftIsAOfRight"));
    }

    #[test]
    fn hierarchy_mode_preserves_base_signature() {
        let hierarchy = IsAHierarchy::new(&[("LION", "CAT"), ("WOLF", "DOG")]);
        let lion = SemanticState::new("LION", [3, 1, 4]);
        let cat = SemanticState::new("CAT", [1, 1, 2]);

        let base = compare_semantic_states(lion.clone(), cat.clone(), &[], &[]);
        let typed = compare_semantic_states_with_hierarchy(lion, cat, &[], &[], &hierarchy);

        assert!(typed.typed_signature.contains(&base.relation_signature));
    }

    #[test]
    fn is_a_analogy_detects_parallel_subtyping() {
        let hierarchy = IsAHierarchy::new(&[("LION", "CAT"), ("WOLF", "DOG")]);

        let analogy = compare_is_a_analogy("LION", "CAT", "WOLF", "DOG", &hierarchy);
        assert!(analogy.analogous);
    }

    #[test]
    fn is_a_analogy_rejects_invalid_chain() {
        let hierarchy = IsAHierarchy::new(&[("LION", "CAT"), ("WOLF", "DOG")]);

        let analogy = compare_is_a_analogy("LION", "CAT", "WOLF", "LION", &hierarchy);
        assert!(!analogy.analogous);
    }
}