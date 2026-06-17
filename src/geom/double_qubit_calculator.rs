const SCALE: i32 = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoubleQubitRegister {
    amplitudes: [i32; 4],
}

impl DoubleQubitRegister {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.amplitudes = [SCALE, 0, 0, 0];
    }

    pub fn read_registers(&self) -> [f64; 4] {
        [
            self.amplitudes[0] as f64 / SCALE as f64,
            self.amplitudes[1] as f64 / SCALE as f64,
            self.amplitudes[2] as f64 / SCALE as f64,
            self.amplitudes[3] as f64 / SCALE as f64,
        ]
    }

    pub fn amplitudes(&self) -> [i32; 4] {
        self.amplitudes
    }

    fn set_amplitudes(&mut self, amplitudes: [i32; 4]) {
        self.amplitudes = amplitudes;
    }
}

impl Default for DoubleQubitRegister {
    fn default() -> Self {
        Self {
            amplitudes: [SCALE, 0, 0, 0],
        }
    }
}

pub trait JointGeometricOperator {
    fn execute(&self, register: &mut DoubleQubitRegister);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimarySuperpositionOperator;

impl JointGeometricOperator for PrimarySuperpositionOperator {
    fn execute(&self, register: &mut DoubleQubitRegister) {
        register.set_amplitudes([SCALE / 2, SCALE / 2, SCALE / 2, SCALE / 2]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlledScaleOperator {
    pub target_index: usize,
    pub scale_numerator: i32,
    pub scale_denominator: i32,
}

impl JointGeometricOperator for ControlledScaleOperator {
    fn execute(&self, register: &mut DoubleQubitRegister) {
        if self.target_index >= register.amplitudes.len() || self.scale_denominator == 0 {
            return;
        }

        let current = register.amplitudes[self.target_index] as i64;
        let updated = current * self.scale_numerator as i64 / self.scale_denominator as i64;
        register.amplitudes[self.target_index] = updated.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlledScientificRotation {
    pub angle_radians: i32,
}

impl ControlledScientificRotation {
    pub const PI_OVER_4: i32 = 785_398;
    pub const PI_OVER_2: i32 = 1_570_796;

    fn rotation_profile(&self) -> (i32, i32, i32, i32) {
        match self.angle_radians {
            Self::PI_OVER_4 => (707_107, 923_880, 382_683, 923_880),
            Self::PI_OVER_2 => (0, SCALE, SCALE, 0),
            _ => (SCALE, 0, 0, SCALE),
        }
    }
}

impl JointGeometricOperator for ControlledScientificRotation {
    fn execute(&self, register: &mut DoubleQubitRegister) {
        let profile = self.rotation_profile();
        let previous = register.amplitudes();
        let transformed = [
            (previous[0] as i64 * profile.0 as i64 / SCALE as i64) as i32,
            (previous[1] as i64 * profile.1 as i64 / SCALE as i64) as i32,
            (previous[2] as i64 * profile.2 as i64 / SCALE as i64) as i32,
            (previous[3] as i64 * profile.3 as i64 / SCALE as i64) as i32,
        ];
        register.set_amplitudes(transformed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_register_projects_to_zeroes() {
        let mut register = DoubleQubitRegister::default();
        register.clear();

        assert_eq!(register.amplitudes(), [SCALE, 0, 0, 0]);
        assert_eq!(register.read_registers(), [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn superposition_then_pi_over_4_controlled_rotation_matches_expected_projection() {
        let mut register = DoubleQubitRegister::new();
        PrimarySuperpositionOperator.execute(&mut register);
        ControlledScientificRotation {
            angle_radians: ControlledScientificRotation::PI_OVER_4,
        }
        .execute(&mut register);

        assert_eq!(register.amplitudes(), [353_553, 461_940, 191_341, 461_940]);
    }

    #[test]
    fn controlled_scale_operator_scales_one_register() {
        let mut register = DoubleQubitRegister::default();
        PrimarySuperpositionOperator.execute(&mut register);

        ControlledScaleOperator {
            target_index: 1,
            scale_numerator: 3,
            scale_denominator: 2,
        }
        .execute(&mut register);

        assert_eq!(register.amplitudes()[1], 750_000);
    }
}