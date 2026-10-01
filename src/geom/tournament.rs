use crate::arithmetic::ArithmeticError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TournamentError {
    NoCandidates,
    NoValidCandidates,
    Arithmetic(ArithmeticError),
}

impl From<ArithmeticError> for TournamentError {
    fn from(error: ArithmeticError) -> Self {
        Self::Arithmetic(error)
    }
}
