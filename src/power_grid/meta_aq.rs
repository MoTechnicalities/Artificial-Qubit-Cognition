use crate::power_grid::super_aq::SuperAQ;

/// Meta-AQ kinds for the Power Grid domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaAQKind {
    NorthZone,
    CentralSubstation,
    SouthZone,
}

impl MetaAQKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NorthZone         => "NorthZone",
            Self::CentralSubstation => "CentralSubstation",
            Self::SouthZone         => "SouthZone",
        }
    }

    /// Active zones contribute to symmetry scoring.
    pub fn is_active_zone(self) -> bool {
        matches!(self, Self::NorthZone | Self::SouthZone)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaAQ {
    pub kind: MetaAQKind,
    pub components: Vec<SuperAQ>,
    pub gestalt: [i32; 3],
    pub signature: String,
}

impl MetaAQ {
    pub fn new(kind: MetaAQKind, components: Vec<SuperAQ>) -> Self {
        let gestalt = components.iter().fold([0, 0, 0], |acc, s| {
            [acc[0] + s.gestalt[0], acc[1] + s.gestalt[1], acc[2] + s.gestalt[2]]
        });
        let signature = format!(
            "metaaq:{}|gestalt:[{},{},{}]|n:{}",
            kind.as_str(), gestalt[0], gestalt[1], gestalt[2], components.len()
        );
        Self { kind, components, gestalt, signature }
    }
}
