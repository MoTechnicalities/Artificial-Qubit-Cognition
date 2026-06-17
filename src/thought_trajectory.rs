#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosureStatus {
    Open,
    Closed,
    Corrected,
}

impl ClosureStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
            Self::Corrected => "corrected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThoughtTrajectory {
    origin: String,
    operators_applied: Vec<String>,
    bindings_crossed: Vec<String>,
    closure_status: ClosureStatus,
    audit_history: Vec<String>,
}

impl ThoughtTrajectory {
    pub fn new(origin: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
            operators_applied: Vec::new(),
            bindings_crossed: Vec::new(),
            closure_status: ClosureStatus::Open,
            audit_history: Vec::new(),
        }
    }

    pub fn origin(&self) -> &str {
        &self.origin
    }

    pub fn operators_applied(&self) -> &[String] {
        &self.operators_applied
    }

    pub fn bindings_crossed(&self) -> &[String] {
        &self.bindings_crossed
    }

    pub fn closure_status(&self) -> ClosureStatus {
        self.closure_status
    }

    pub fn audit_history(&self) -> &[String] {
        &self.audit_history
    }

    pub fn apply_operator(&mut self, operator: impl Into<String>) {
        let operator = operator.into();
        self.audit_history
            .push(format!("operator:{operator}"));
        self.operators_applied.push(operator);
    }

    pub fn cross_binding(&mut self, binding: impl Into<String>) {
        let binding = binding.into();
        self.audit_history
            .push(format!("binding:{binding}"));
        self.bindings_crossed.push(binding);
    }

    pub fn set_closure_status(&mut self, status: ClosureStatus) {
        self.closure_status = status;
        self.audit_history
            .push(format!("closure:{}", status.as_str()));
    }

    pub fn append_audit_event(&mut self, event: impl Into<String>) {
        self.audit_history.push(event.into());
    }

    pub fn canonical_signature(&self) -> String {
        let operators = if self.operators_applied.is_empty() {
            "none".to_string()
        } else {
            self.operators_applied.join(">")
        };
        let bindings = if self.bindings_crossed.is_empty() {
            "none".to_string()
        } else {
            self.bindings_crossed.join(">")
        };
        let audit = if self.audit_history.is_empty() {
            "none".to_string()
        } else {
            self.audit_history.join(">")
        };

        format!(
            "origin={}|operators={}|bindings={}|closure={}|audit={}",
            self.origin,
            operators,
            bindings,
            self.closure_status.as_str(),
            audit
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{ClosureStatus, ThoughtTrajectory};

    #[test]
    fn trajectory_records_operator_binding_and_closure_history() {
        let mut trajectory = ThoughtTrajectory::new("dog-state");
        trajectory.apply_operator("abstraction");
        trajectory.cross_binding("dog<mammal");
        trajectory.set_closure_status(ClosureStatus::Closed);

        assert_eq!(trajectory.origin(), "dog-state");
        assert_eq!(trajectory.operators_applied(), &["abstraction".to_string()]);
        assert_eq!(trajectory.bindings_crossed(), &["dog<mammal".to_string()]);
        assert_eq!(trajectory.closure_status(), ClosureStatus::Closed);
        assert_eq!(
            trajectory.audit_history(),
            &[
                "operator:abstraction".to_string(),
                "binding:dog<mammal".to_string(),
                "closure:closed".to_string()
            ]
        );
    }

    #[test]
    fn canonical_signature_is_deterministic() {
        let mut trajectory = ThoughtTrajectory::new("origin-a");
        trajectory.apply_operator("contrast");
        trajectory.cross_binding("dog<wolf");
        trajectory.append_audit_event("resonance:stable");
        trajectory.set_closure_status(ClosureStatus::Corrected);

        assert_eq!(
            trajectory.canonical_signature(),
            "origin=origin-a|operators=contrast|bindings=dog<wolf|closure=corrected|audit=operator:contrast>binding:dog<wolf>resonance:stable>closure:corrected"
        );
    }
}