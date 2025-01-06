use core::fmt::{Debug, Display};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};


#[derive(Copy, Clone)]
enum NumericTerm {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Always finite.
    Float(f64),
}

/// Represents a Term number, whether integer or floating point.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd)]
pub struct Number {
    n: NumericTerm,
}

impl Number {
    /// Returns true if the `Number` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any Number on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer value.
    pub fn is_i64(&self) -> bool {
        match self.n {
            NumericTerm::PosInt(v) => v <= i64::MAX as u64,
            NumericTerm::NegInt(_) => true,
            NumericTerm::Float(_) => false,
        }
    }

    /// Returns true if the `Number` is an integer between zero and `u64::MAX`.
    ///
    /// For any Number on which `is_u64` returns true, `as_u64` is guaranteed to
    /// return the integer value.
    pub fn is_u64(&self) -> bool {
        match self.n {
            NumericTerm::PosInt(_) => true,
            NumericTerm::NegInt(_) | NumericTerm::Float(_) => false,
        }
    }

    /// Returns true if the `Number` can be represented by f64.
    ///
    /// For any Number on which `is_f64` returns true, `as_f64` is guaranteed to
    /// return the floating point value.
    ///
    /// Currently, this function returns true if and only if both `is_i64` and
    /// `is_u64` return false but this is not a guarantee in the future.
    pub fn is_f64(&self) -> bool {
        match self.n {
            NumericTerm::Float(_) => true,
            NumericTerm::PosInt(_) | NumericTerm::NegInt(_) => false,
        }
    }

    /// If the `Number` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            NumericTerm::PosInt(n) => {
                if n <= i64::MAX as u64 {
                    Some(n as i64)
                } else {
                    None
                }
            }
            NumericTerm::NegInt(n) => Some(n),
            NumericTerm::Float(_) => None,
        }
    }

    /// If the `Number` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            NumericTerm::PosInt(n) => Some(n),
            NumericTerm::NegInt(_) | NumericTerm::Float(_) => None,
        }
    }

    /// Represents the number as f64 if possible. Returns None otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            NumericTerm::PosInt(n) => Some(n as f64),
            NumericTerm::NegInt(n) => Some(n as f64),
            NumericTerm::Float(n) => Some(n),
        }
    }

    /// Converts a finite `f64` to a `Number`. Infinite or NaN values are not JSON
    /// numbers.
    pub fn from_f64(f: f64) -> Option<Number> {
        if f.is_finite() {
            let n = {
                {
                    NumericTerm::Float(f)
                }
            };
            Some(Number { n })
        } else {
            None
        }
    }

    /// If the `Number` is an integer, represent it as i128 if possible. Returns
    /// None otherwise.
    pub fn as_i128(&self) -> Option<i128> {
        match self.n {
            NumericTerm::PosInt(n) => Some(n as i128),
            NumericTerm::NegInt(n) => Some(n as i128),
            NumericTerm::Float(_) => None,
        }
    }

    /// If the `Number` is an integer, represent it as u128 if possible. Returns
    /// None otherwise.
    pub fn as_u128(&self) -> Option<u128> {
        match self.n {
            NumericTerm::PosInt(n) => Some(n as u128),
            NumericTerm::NegInt(_) | NumericTerm::Float(_) => None,
        }
    }

    /// Converts an `i128` to a `Number`. Numbers smaller than i64::MIN or
    /// larger than u64::MAX can only be represented in `Number` if serde_json's
    /// "arbitrary_precision" feature is enabled.
    pub fn from_i128(i: i128) -> Option<Number> {
        let n = {
            {
                if let Ok(u) = u64::try_from(i) {
                    NumericTerm::PosInt(u)
                } else if let Ok(i) = i64::try_from(i) {
                    NumericTerm::NegInt(i)
                } else {
                    return None;
                }
            }
        };
        Some(Number { n })
    }

    /// Converts a `u128` to a `Number`. Numbers greater than u64::MAX can only
    /// be represented in `Number` if serde_json's "arbitrary_precision" feature
    /// is enabled.
    pub fn from_u128(i: u128) -> Option<Number> {
        let n = {
            {
                if let Ok(u) = u64::try_from(i) {
                    NumericTerm::PosInt(u)
                } else {
                    return None;
                }
            }
        };
        Some(Number { n })
    }

    pub(crate) fn as_f32(&self) -> Option<f32> {
        match self.n {
            NumericTerm::PosInt(n) => Some(n as f32),
            NumericTerm::NegInt(n) => Some(n as f32),
            NumericTerm::Float(n) => Some(n as f32),
        }
    }

    pub(crate) fn from_f32(f: f32) -> Option<Number> {
        if f.is_finite() {
            let n = {
                {
                    NumericTerm::Float(f as f64)
                }
            };
            Some(Number { n })
        } else {
            None
        }
    }
}


impl PartialEq for NumericTerm {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NumericTerm::PosInt(a), NumericTerm::PosInt(b)) => a == b,
            (NumericTerm::NegInt(a), NumericTerm::NegInt(b)) => a == b,
            (NumericTerm::Float(a), NumericTerm::Float(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for NumericTerm {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self < other {
            Some(Ordering::Less)
        } else if self > other  {
            Some(Ordering::Greater)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (NumericTerm::PosInt(a), NumericTerm::PosInt(b)) => a < b,
            (NumericTerm::NegInt(a), NumericTerm::NegInt(b)) => a < b,
            (NumericTerm::Float(a), NumericTerm::Float(b)) => a < b,
            _ => false,
        }
    }

    fn le(&self, other: &Self) -> bool {
        match (self, other) {
            (NumericTerm::PosInt(a), NumericTerm::PosInt(b)) => a <= b,
            (NumericTerm::NegInt(a), NumericTerm::NegInt(b)) => a <= b,
            (NumericTerm::Float(a), NumericTerm::Float(b)) => a <= b,
            _ => false,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (NumericTerm::PosInt(a), NumericTerm::PosInt(b)) => a > b,
            (NumericTerm::NegInt(a), NumericTerm::NegInt(b)) => a > b,
            (NumericTerm::Float(a), NumericTerm::Float(b)) => a > b,
            _ => false,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            (NumericTerm::PosInt(a), NumericTerm::PosInt(b)) => a >= b,
            (NumericTerm::NegInt(a), NumericTerm::NegInt(b)) => a >= b,
            (NumericTerm::Float(a), NumericTerm::Float(b)) => a >= b,
            _ => false,
        }
    }
}

// Implementing Eq is fine since any float values are always finite.
impl Eq for NumericTerm {}

impl Hash for NumericTerm {
    fn hash<H: Hasher>(&self, h: &mut H) {
        match *self {
            NumericTerm::PosInt(i) => i.hash(h),
            NumericTerm::NegInt(i) => i.hash(h),
            NumericTerm::Float(f) => {
                if f == 0.0f64 {
                    // There are 2 zero representations, +0 and -0, which
                    // compare equal but have different bits. We use the +0 hash
                    // for both so that hash(+0) == hash(-0).
                    0.0f64.to_bits().hash(h);
                } else {
                    f.to_bits().hash(h);
                }
            }
        }
    }
}

impl Display for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.n {
            NumericTerm::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
            NumericTerm::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
            NumericTerm::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
        }
    }
}

impl Debug for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Number({})", self)
    }
}