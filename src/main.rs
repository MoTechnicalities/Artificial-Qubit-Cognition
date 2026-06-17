mod geom;

use geom::qubit_calculator::{CalculatorQubit, GeometricAddOperator, GeometricSubOperator};

fn main() {
    // Deterministic 0.1 + 0.2 verification
    let mut register = CalculatorQubit::clear();
    println!("Initial Register Value: {}", register.read_value());

    register = GeometricAddOperator { value_to_add: 0.1 }.execute(&register);
    println!("After Adding 0.1:        {}", register.read_value());

    register = GeometricAddOperator { value_to_add: 0.2 }.execute(&register);
    println!("Final Result (0.1 + 0.2): {}", register.read_value());

    // Bonus subtraction example
    let mut sub_register = CalculatorQubit::clear();
    sub_register = GeometricAddOperator { value_to_add: 0.7 }.execute(&sub_register);
    sub_register = GeometricSubOperator { value_to_subtract: 0.2 }.execute(&sub_register);
    println!("Subtraction Check (0.7 - 0.2): {}", sub_register.read_value());
}
