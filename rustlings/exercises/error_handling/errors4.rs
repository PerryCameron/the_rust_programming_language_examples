// errors4.rs
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a hint.



#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?

        match value {
            // compare using like types and return wrapped value if number is positive
            n if n > 0 =>  Ok(PositiveNonzeroInteger(n as u64)),
            // if number is zero
            0 => Err(CreationError::Zero),
            // for everything else
            _ => Err(CreationError::Negative),
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
