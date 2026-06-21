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
            Self::Load    => "Load",
            Self::Line    => "Line",
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
    pub fn new(kind: SuperAQKind, components: Vec<PrimitiveAQ>) -> Self {
        let gestalt = components.iter().fold([0, 0, 0], |acc, p| {
            [acc[0] + p.coords[0], acc[1] + p.coords[1], acc[2] + p.coords[2]]
        });
        let signature = format!(
            "superaq:{}|gestalt:[{},{},{}]|n:{}",
            kind.as_str(), gestalt[0], gestalt[1], gestalt[2], components.len()
        );
        Self { kind, components, gestalt, signature }
    }
}
