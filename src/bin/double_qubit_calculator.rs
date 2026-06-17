use artificial_qubit_cognition::geom::double_qubit_calculator::{
    ControlledScientificRotation, DoubleQubitRegister, JointGeometricOperator,
    PrimarySuperpositionOperator,
};

fn format_readout(values: [f64; 4]) -> String {
    format!(
        "[|00>: {:.6}, |01>: {:.6}, |10>: {:.6}, |11>: {:.6}]",
        values[0], values[1], values[2], values[3]
    )
}

fn main() {
    let mut register = DoubleQubitRegister::new();

    println!("Double Qubit Scientific Calculator");
    println!("Initial Register: {}", format_readout(register.read_registers()));

    PrimarySuperpositionOperator.execute(&mut register);
    println!(
        "After Primary Superposition: {}",
        format_readout(register.read_registers())
    );

    ControlledScientificRotation {
        angle_radians: ControlledScientificRotation::PI_OVER_4,
    }
    .execute(&mut register);

    println!(
        "After Controlled Rotation (pi/4): {}",
        format_readout(register.read_registers())
    );
}