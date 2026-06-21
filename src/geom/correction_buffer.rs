/// A correction buffer archives pre-evaluated, pre-stabilised design signatures.
///
/// When a primary trajectory fails topology validation or exhibits drift beyond
/// threshold, the arbitration layer can pull a frozen fallback from this buffer
/// instantly — no recomputation required.
///
/// This type is shared across all DGCS reasoners; domain-specific arbitration
/// layers determine what constitutes a "valid" entry.
#[derive(Debug, Default)]
pub struct CorrectionBuffer {
    entries: Vec<(String, String)>, // (plan_id, signature)
}

impl CorrectionBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Archive a validated plan as a correction candidate.
    pub fn archive(&mut self, plan_id: &str, signature: &str) {
        self.entries.push((plan_id.to_string(), signature.to_string()));
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the most recently archived entry (LIFO fallback heuristic).
    pub fn fallback(&self) -> Option<(&str, &str)> {
        self.entries.last().map(|(id, sig)| (id.as_str(), sig.as_str()))
    }
}
