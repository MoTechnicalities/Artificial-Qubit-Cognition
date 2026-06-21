/// A correction buffer archives pre-evaluated, pre-stabilized design signatures.
///
/// When a primary trajectory fails topology validation or exhibits drift beyond
/// threshold, the arbitration layer can pull a frozen correction trajectory from
/// this buffer instantly — no recomputation required.
#[derive(Debug, Default)]
pub struct CorrectionBuffer {
    entries: Vec<(String, String)>, // (plan_id, signature)
}

impl CorrectionBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Archive a valid design as a correction candidate.
    pub fn archive(&mut self, plan_id: &str, signature: &str) {
        self.entries.push((plan_id.to_string(), signature.to_string()));
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the most stable archived entry (highest insertion-order rank).
    /// In a full implementation this would select by stability score.
    pub fn fallback(&self) -> Option<(&str, &str)> {
        self.entries
            .last()
            .map(|(id, sig)| (id.as_str(), sig.as_str()))
    }
}
