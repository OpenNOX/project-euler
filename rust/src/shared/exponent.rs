/// Represents an exponent.
#[derive(Debug, Eq, PartialEq)]
pub struct Exponent {
    /// Base value.
    pub base_value: u64,

    /// Power to raise base value by.
    pub power: u64,
}
