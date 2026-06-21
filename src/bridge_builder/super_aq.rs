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
            Self::Beam    => "Beam",
            Self::Joint   => "Joint",
            Self::Support => "Support",
            Self::Span    => "Span",
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
    pub fn new(kind: SuperAQKind, components: Vec<PrimitiveAQ>) -> Self {
        let gestalt = aggregate_gestalt(&components);
        let signature = format!(
            "superaq:{}|gestalt:[{},{},{}]|n:{}",
            kind.as_str(), gestalt[0], gestalt[1], gestalt[2], components.len()
        );
        Self { kind, components, gestalt, signature }
    }
}

fn aggregate_gestalt(components: &[PrimitiveAQ]) -> [i32; 3] {
    components.iter().fold([0, 0, 0], |acc, p| {
        [acc[0] + p.coords[0], acc[1] + p.coords[1], acc[2] + p.coords[2]]
    })
}
