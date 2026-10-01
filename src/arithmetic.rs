#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticError {
    Overflow(&'static str),
    OutOfRange(&'static str),
}

pub fn checked_add(left: i32, right: i32, context: &'static str) -> Result<i32, ArithmeticError> {
    left.checked_add(right)
        .ok_or(ArithmeticError::Overflow(context))
}

pub fn checked_mul(left: i32, right: i32, context: &'static str) -> Result<i32, ArithmeticError> {
    left.checked_mul(right)
        .ok_or(ArithmeticError::Overflow(context))
}

pub fn checked_sub(left: i32, right: i32, context: &'static str) -> Result<i32, ArithmeticError> {
    left.checked_sub(right)
        .ok_or(ArithmeticError::Overflow(context))
}

pub fn checked_abs(value: i32, context: &'static str) -> Result<i32, ArithmeticError> {
    value
        .checked_abs()
        .ok_or(ArithmeticError::Overflow(context))
}

pub fn checked_sum(
    values: impl IntoIterator<Item = i32>,
    context: &'static str,
) -> Result<i32, ArithmeticError> {
    values
        .into_iter()
        .try_fold(0, |total, value| checked_add(total, value, context))
}

pub fn checked_neg(value: i32, context: &'static str) -> Result<i32, ArithmeticError> {
    value
        .checked_neg()
        .ok_or(ArithmeticError::Overflow(context))
}

pub fn checked_dot(
    left: [i32; 3],
    right: [i32; 3],
    context: &'static str,
) -> Result<i32, ArithmeticError> {
    let mut total = 0i32;
    for axis in 0..3 {
        let product = checked_mul(left[axis], right[axis], context)?;
        total = checked_add(total, product, context)?;
    }
    Ok(total)
}

pub fn checked_matrix_vector(
    matrix: [[i32; 3]; 3],
    vector: [i32; 3],
    context: &'static str,
) -> Result<[i32; 3], ArithmeticError> {
    Ok([
        checked_dot(matrix[0], vector, context)?,
        checked_dot(matrix[1], vector, context)?,
        checked_dot(matrix[2], vector, context)?,
    ])
}

pub fn checked_i64_add(
    left: i64,
    right: i64,
    context: &'static str,
) -> Result<i64, ArithmeticError> {
    left.checked_add(right)
        .ok_or(ArithmeticError::Overflow(context))
}

pub fn checked_i64_mul(
    left: i64,
    right: i64,
    context: &'static str,
) -> Result<i64, ArithmeticError> {
    left.checked_mul(right)
        .ok_or(ArithmeticError::Overflow(context))
}

pub fn usize_to_i64(value: usize, context: &'static str) -> Result<i64, ArithmeticError> {
    i64::try_from(value).map_err(|_| ArithmeticError::OutOfRange(context))
}

#[cfg(test)]
mod tests {
    use super::{checked_dot, checked_matrix_vector, checked_neg, usize_to_i64, ArithmeticError};

    #[test]
    fn checked_geometry_preserves_canonical_results() {
        assert_eq!(checked_dot([1, 2, 3], [4, 5, 6], "dot"), Ok(32));
        assert_eq!(
            checked_matrix_vector([[1, 0, -1], [0, 1, 1], [0, 0, 1]], [1, 2, 3], "matrix",),
            Ok([-2, 5, 3])
        );
    }

    #[test]
    fn checked_geometry_reports_overflow() {
        assert_eq!(
            checked_neg(i32::MIN, "negation"),
            Err(ArithmeticError::Overflow("negation"))
        );
        assert_eq!(
            checked_dot([i32::MAX, 0, 0], [2, 0, 0], "dot"),
            Err(ArithmeticError::Overflow("dot"))
        );
    }

    #[test]
    fn narrowing_is_explicit() {
        assert_eq!(usize_to_i64(3, "distance"), Ok(3));
    }
}
