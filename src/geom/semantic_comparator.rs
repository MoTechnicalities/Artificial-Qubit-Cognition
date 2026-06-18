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
}