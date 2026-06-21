use crate::bridge_builder::super_aq::SuperAQ;

/// A meta-AQ is a governed composite of super-AQs representing
/// an entire structural region of the bridge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaAQKind {
    LeftSpan,
    CentralSupport,
    RightSpan,
}

impl MetaAQKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LeftSpan       => "LeftSpan",
            Self::CentralSupport => "CentralSupport",
            Self::RightSpan      => "RightSpan",
        }
    }

    /// Span meta-AQs contribute to symmetry scoring.
    pub fn is_span(self) -> bool {
        matches!(self, Self::LeftSpan | Self::RightSpan)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaAQ {
    pub kind: MetaAQKind,
    pub components: Vec<SuperAQ>,
    /// gestalt = component-wise sum of all super-AQ gestalts
    pub gestalt: [i32; 3],
    pub signature: String,
}

impl MetaAQ {
    pub fn new(kind: MetaAQKind, components: Vec<SuperAQ>) -> Self {
        let gestalt = aggregate_gestalt(&components);
        let signature = format!(
            "metaaq:{}|gestalt:[{},{},{}]|n:{}",
            kind.as_str(), gestalt[0], gestalt[1], gestalt[2], components.len()
        );
        Self { kind, components, gestalt, signature }
    }
}

fn aggregate_gestalt(components: &[SuperAQ]) -> [i32; 3] {
    components.iter().fold([0, 0, 0], |acc, s| {
        [acc[0] + s.gestalt[0], acc[1] + s.gestalt[1], acc[2] + s.gestalt[2]]
    })
}
