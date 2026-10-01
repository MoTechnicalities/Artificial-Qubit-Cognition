use crate::arithmetic::{checked_i64_add, checked_i64_mul, usize_to_i64, ArithmeticError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreativityError {
    EmptyCanonicalSet,
    ZeroMinimumDistance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreativityPolicy {
    canonical_trajectories: Vec<Vec<String>>,
    minimum_distance: usize,
}

pub trait CreativityGate<T> {
    fn primary_invariants_preserved(&self, subject: &T) -> bool;
    fn structurally_valid(&self, subject: &T) -> bool;
}

impl CreativityPolicy {
    pub fn new(
        canonical_trajectories: Vec<Vec<String>>,
        minimum_distance: usize,
    ) -> Result<Self, CreativityError> {
        if canonical_trajectories.is_empty() {
            return Err(CreativityError::EmptyCanonicalSet);
        }
        if minimum_distance == 0 {
            return Err(CreativityError::ZeroMinimumDistance);
        }

        Ok(Self {
            canonical_trajectories,
            minimum_distance,
        })
    }

    pub fn evaluate<T>(
        &self,
        operator_sequence: &[String],
        subject: &T,
        gate: &impl CreativityGate<T>,
    ) -> CreativityEvaluation {
        let primary_invariants_preserved = gate.primary_invariants_preserved(subject);
        let structurally_valid = gate.structurally_valid(subject);
        let distance_from_expected = self
            .canonical_trajectories
            .iter()
            .map(|canonical| sequence_distance(operator_sequence, canonical))
            .min()
            .expect("creativity policy requires at least one canonical trajectory");
        let is_creative = primary_invariants_preserved
            && structurally_valid
            && distance_from_expected >= self.minimum_distance;

        CreativityEvaluation {
            primary_invariants_preserved,
            structurally_valid,
            distance_from_expected,
            minimum_distance: self.minimum_distance,
            is_creative,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreativityEvaluation {
    primary_invariants_preserved: bool,
    structurally_valid: bool,
    distance_from_expected: usize,
    minimum_distance: usize,
    is_creative: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreativityStatus {
    Creative,
    Routine,
    Invalid,
}

impl CreativityEvaluation {
    pub fn primary_invariants_preserved(&self) -> bool {
        self.primary_invariants_preserved
    }

    pub fn structurally_valid(&self) -> bool {
        self.structurally_valid
    }

    pub fn distance_from_expected(&self) -> usize {
        self.distance_from_expected
    }

    pub fn minimum_distance(&self) -> usize {
        self.minimum_distance
    }

    pub fn is_creative(&self) -> bool {
        self.status() == CreativityStatus::Creative
    }

    pub fn status(&self) -> CreativityStatus {
        if !self.primary_invariants_preserved || !self.structurally_valid {
            CreativityStatus::Invalid
        } else if self.is_creative {
            CreativityStatus::Creative
        } else {
            CreativityStatus::Routine
        }
    }

    pub fn creativity_score(&self) -> usize {
        if self.status() == CreativityStatus::Creative {
            self.distance_from_expected
        } else {
            0
        }
    }

    pub fn canonical_tag(&self) -> String {
        let status = match self.status() {
            CreativityStatus::Creative => "creative",
            CreativityStatus::Routine => "routine",
            CreativityStatus::Invalid => "invalid",
        };
        format!(
            "creativity:{}|distance:{}|minimum:{}|invariants:{}|structure:{}",
            status,
            self.distance_from_expected,
            self.minimum_distance,
            self.primary_invariants_preserved,
            self.structurally_valid
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionWeights {
    pub utility: i64,
    pub creativity: i64,
}

impl SelectionWeights {
    pub fn score(
        &self,
        utility: i64,
        creativity: &CreativityEvaluation,
    ) -> Result<i64, ArithmeticError> {
        let creativity_score = usize_to_i64(
            creativity.creativity_score(),
            "creativity distance conversion",
        )?;
        let utility_term = checked_i64_mul(self.utility, utility, "weighted utility score")?;
        let creativity_term = checked_i64_mul(
            self.creativity,
            creativity_score,
            "weighted creativity score",
        )?;
        checked_i64_add(utility_term, creativity_term, "combined selection score")
    }
}

fn sequence_distance(left: &[String], right: &[String]) -> usize {
    let mut previous: Vec<usize> = (0..=right.len()).collect();
    let mut current = vec![0; right.len() + 1];

    for (left_index, left_operator) in left.iter().enumerate() {
        current[0] = left_index + 1;
        for (right_index, right_operator) in right.iter().enumerate() {
            let substitution_cost = usize::from(left_operator != right_operator);
            current[right_index + 1] = (previous[right_index + 1] + 1)
                .min(current[right_index] + 1)
                .min(previous[right_index] + substitution_cost);
        }
        std::mem::swap(&mut previous, &mut current);
    }

    previous[right.len()]
}

#[cfg(test)]
mod tests {
    use crate::arithmetic::ArithmeticError;

    use super::{
        CreativityError, CreativityGate, CreativityPolicy, CreativityStatus, SelectionWeights,
    };

    struct TestSubject {
        primary_valid: bool,
        structurally_valid: bool,
    }

    struct TestGate;

    impl CreativityGate<TestSubject> for TestGate {
        fn primary_invariants_preserved(&self, subject: &TestSubject) -> bool {
            subject.primary_valid
        }

        fn structurally_valid(&self, subject: &TestSubject) -> bool {
            subject.structurally_valid
        }
    }

    fn operators(names: &[&str]) -> Vec<String> {
        names.iter().map(|name| (*name).to_string()).collect()
    }

    #[test]
    fn distinguishes_routine_creative_and_invalid_trajectories() {
        let policy =
            CreativityPolicy::new(vec![operators(&["abstraction", "alignment"])], 2).unwrap();
        let valid = TestSubject {
            primary_valid: true,
            structurally_valid: true,
        };
        let primary_invalid = TestSubject {
            primary_valid: false,
            structurally_valid: true,
        };

        let routine = policy.evaluate(&operators(&["abstraction", "alignment"]), &valid, &TestGate);
        let creative = policy.evaluate(&operators(&["contrast", "axis-flip"]), &valid, &TestGate);
        let invalid = policy.evaluate(
            &operators(&["contrast", "axis-flip"]),
            &primary_invalid,
            &TestGate,
        );

        assert!(!routine.is_creative());
        assert!(creative.is_creative());
        assert!(!invalid.is_creative());
        assert_eq!(routine.status(), CreativityStatus::Routine);
        assert_eq!(creative.status(), CreativityStatus::Creative);
        assert_eq!(invalid.status(), CreativityStatus::Invalid);
        assert_eq!(invalid.creativity_score(), 0);
    }

    #[test]
    fn weighted_score_cannot_reward_invalid_deviation() {
        let policy = CreativityPolicy::new(vec![operators(&["alignment"])], 1).unwrap();
        let valid = TestSubject {
            primary_valid: true,
            structurally_valid: true,
        };
        let structurally_invalid = TestSubject {
            primary_valid: true,
            structurally_valid: false,
        };
        let creative = policy.evaluate(&operators(&["contrast"]), &valid, &TestGate);
        let invalid = policy.evaluate(&operators(&["contrast"]), &structurally_invalid, &TestGate);
        let weights = SelectionWeights {
            utility: 2,
            creativity: 3,
        };

        assert_eq!(weights.score(10, &creative), Ok(23));
        assert_eq!(weights.score(10, &invalid), Ok(20));
    }

    #[test]
    fn weighted_score_reports_overflow() {
        let policy = CreativityPolicy::new(vec![operators(&["alignment"])], 1).unwrap();
        let valid = TestSubject {
            primary_valid: true,
            structurally_valid: true,
        };
        let creative = policy.evaluate(&operators(&["contrast"]), &valid, &TestGate);
        let weights = SelectionWeights {
            utility: i64::MAX,
            creativity: 1,
        };

        assert_eq!(
            weights.score(2, &creative),
            Err(ArithmeticError::Overflow("weighted utility score"))
        );
    }

    #[test]
    fn policy_requires_a_non_empty_baseline_and_positive_threshold() {
        assert_eq!(
            CreativityPolicy::new(Vec::new(), 1),
            Err(CreativityError::EmptyCanonicalSet)
        );
        assert_eq!(
            CreativityPolicy::new(vec![Vec::new()], 0),
            Err(CreativityError::ZeroMinimumDistance)
        );
    }
}
