use std::f64::consts::PI;

pub const SCALE: i32 = 1_000_000;

/// A fixed-point coordinate on the Bloch sphere representing calculator state.
/// All values are scaled by SCALE to preserve integer surface representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalculatorQubit {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl CalculatorQubit {
    /// Initialize register at logical 0.0 (north pole).
    pub fn clear() -> Self {
        Self { x: 0, y: 0, z: SCALE }
    }

    /// Read the state as a decimal in the [0.0, 1.0] range.
    pub fn read_value(&self) -> String {
        let z_clamped = (self.z as f64 / SCALE as f64).clamp(-1.0, 1.0);
        let theta = z_clamped.acos();
        let value = theta / PI;
        format!("{value:.1}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeometricAddOperator {
    pub value_to_add: f64,
}

impl GeometricAddOperator {
    /// Apply deterministic rotation around the Y axis in the X-Z plane.
    pub fn execute(&self, state: &CalculatorQubit) -> CalculatorQubit {
        let delta_theta = self.value_to_add * PI;
        let current_z = (state.z as f64 / SCALE as f64).clamp(-1.0, 1.0);
        let current_theta = current_z.acos();
        let new_theta = current_theta + delta_theta;

        let new_x = (new_theta.sin() * SCALE as f64).round() as i32;
        let new_y = 0;
        let new_z = (new_theta.cos() * SCALE as f64).round() as i32;

        CalculatorQubit {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeometricSubOperator {
    pub value_to_subtract: f64,
}

impl GeometricSubOperator {
    pub fn execute(&self, state: &CalculatorQubit) -> CalculatorQubit {
        let add = GeometricAddOperator {
            value_to_add: -self.value_to_subtract,
        };
        add.execute(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometric_addition_0_1_plus_0_2_is_0_3() {
        let mut register = CalculatorQubit::clear();
        register = GeometricAddOperator { value_to_add: 0.1 }.execute(&register);
        register = GeometricAddOperator { value_to_add: 0.2 }.execute(&register);
        assert_eq!(register.read_value(), "0.3");
    }

    #[test]
    fn geometric_subtraction_0_7_minus_0_2_is_0_5() {
        let mut register = CalculatorQubit::clear();
        register = GeometricAddOperator { value_to_add: 0.7 }.execute(&register);
        register = GeometricSubOperator { value_to_subtract: 0.2 }.execute(&register);
        assert_eq!(register.read_value(), "0.5");
    }
}
