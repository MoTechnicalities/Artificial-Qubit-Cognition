use crate::arithmetic::{checked_sum, ArithmeticError};
use crate::bridge_builder::primitive_aq::PrimitiveAQ;

/// A super-AQ is a governed composite of 2–4 primitive AQs representing
/// a structural component (Beam, Joint, Support, Span).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuperAQKind {
    Beam,
    Joint,
    Support,
    Span,
}

impl SuperAQKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Beam => "Beam",
            Self::Joint => "Joint",
            Self::Support => "Support",
            Self::Span => "Span",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuperAQ {
    pub kind: SuperAQKind,
    pub components: Vec<PrimitiveAQ>,
    /// gestalt = component-wise sum of all primitive coords
    pub gestalt: [i32; 3],
    pub signature: String,
}

impl SuperAQ {
    pub fn new(kind: SuperAQKind, components: Vec<PrimitiveAQ>) -> Result<Self, ArithmeticError> {
        let gestalt = aggregate_gestalt(&components)?;
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

fn aggregate_gestalt(components: &[PrimitiveAQ]) -> Result<[i32; 3], ArithmeticError> {
    Ok([
        checked_sum(
            components.iter().map(|component| component.coords[0]),
            "bridge super-AQ drift aggregation",
        )?,
        checked_sum(
            components.iter().map(|component| component.coords[1]),
            "bridge super-AQ symmetry aggregation",
        )?,
        checked_sum(
            components.iter().map(|component| component.coords[2]),
            "bridge super-AQ stability aggregation",
        )?,
    ])
}
