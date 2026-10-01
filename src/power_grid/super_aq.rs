use crate::arithmetic::{checked_sum, ArithmeticError};
use crate::power_grid::primitive_aq::PrimitiveAQ;

/// Super-AQ kinds for the Power Grid domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuperAQKind {
    Circuit,
    Breaker,
    Load,
    Line,
}

impl SuperAQKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Circuit => "Circuit",
            Self::Breaker => "Breaker",
            Self::Load => "Load",
            Self::Line => "Line",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuperAQ {
    pub kind: SuperAQKind,
    pub components: Vec<PrimitiveAQ>,
    pub gestalt: [i32; 3],
    pub signature: String,
}

impl SuperAQ {
    pub fn new(kind: SuperAQKind, components: Vec<PrimitiveAQ>) -> Result<Self, ArithmeticError> {
        let gestalt = [
            checked_sum(
                components.iter().map(|component| component.coords[0]),
                "grid super-AQ drift aggregation",
            )?,
            checked_sum(
                components.iter().map(|component| component.coords[1]),
                "grid super-AQ symmetry aggregation",
            )?,
            checked_sum(
                components.iter().map(|component| component.coords[2]),
                "grid super-AQ stability aggregation",
            )?,
        ];
        let signature = format!(
            "superaq:{}|gestalt:[{},{},{}]|n:{}",
            kind.as_str(),
            gestalt[0],
            gestalt[1],
            gestalt[2],
            components.len()
        );
        Ok(Self {
            kind,
            components,
            gestalt,
            signature,
        })
    }
}
