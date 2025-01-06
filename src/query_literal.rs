use core::fmt::{Debug, Display, Write};
use std::{fmt, io};
use crate::number::Number;

/// Represents any valid RepoQuery Term.
#[derive(Clone, Eq, PartialEq, PartialOrd, Hash)]
pub enum QueryLiteral {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
}

impl QueryLiteral {
    /// Returns true if the `QueryLiteral` is a String. Returns false otherwise.
    ///
    /// For any QueryLiteral on which `is_string` returns true, `as_str` is guaranteed
    /// to return the string slice.
    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    /// If the `QueryLiteral` is a String, returns the associated str. Returns None
    /// otherwise.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            QueryLiteral::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns true if the `QueryLiteral` is a Number. Returns false otherwise.
    pub fn is_number(&self) -> bool {
        match *self {
            QueryLiteral::Number(_) => true,
            _ => false,
        }
    }

    /// If the `QueryLiteral` is a Number, returns the associated [`Number`]. Returns
    /// None otherwise.
    pub fn as_number(&self) -> Option<&Number> {
        match self {
            QueryLiteral::Number(number) => Some(number),
            _ => None,
        }
    }

    /// Returns true if the `QueryLiteral` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any QueryLiteral on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer QueryLiteral.
    pub fn is_i64(&self) -> bool {
        match self {
            QueryLiteral::Number(n) => n.is_i64(),
            _ => false,
        }
    }

    /// Returns true if the `QueryLiteral` is an integer between zero and `u64::MAX`.
    ///
    /// For any QueryLiteral on which `is_u64` returns true, `as_u64` is guaranteed to
    /// return the integer QueryLiteral.
    pub fn is_u64(&self) -> bool {
        match self {
            QueryLiteral::Number(n) => n.is_u64(),
            _ => false,
        }
    }

    /// Returns true if the `QueryLiteral` is a number that can be represented by f64.
    ///
    /// For any QueryLiteral on which `is_f64` returns true, `as_f64` is guaranteed to
    /// return the floating point QueryLiteral.
    ///
    /// Currently, this function returns true if and only if both `is_i64` and
    /// `is_u64` return false but this is not a guarantee in the future.
    pub fn is_f64(&self) -> bool {
        match self {
            QueryLiteral::Number(n) => n.is_f64(),
            _ => false,
        }
    }

    /// If the `QueryLiteral` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            QueryLiteral::Number(n) => n.as_i64(),
            _ => None,
        }
    }

    /// If the `QueryLiteral` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            QueryLiteral::Number(n) => n.as_u64(),
            _ => None,
        }
    }

    /// If the `QueryLiteral` is a number, represent it as f64 if possible. Returns
    /// None otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            QueryLiteral::Number(n) => n.as_f64(),
            _ => None,
        }
    }

    /// Returns true if the `QueryLiteral` is a Boolean. Returns false otherwise.
    ///
    /// For any QueryLiteral on which `is_boolean` returns true, `as_bool` is
    /// guaranteed to return the boolean QueryLiteral.
    pub fn is_boolean(&self) -> bool {
        self.as_bool().is_some()
    }

    /// If the `QueryLiteral` is a Boolean, returns the associated bool. Returns None
    /// otherwise.
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            QueryLiteral::Bool(b) => Some(b),
            _ => None,
        }
    }

    /// Returns true if the `QueryLiteral` is a Null. Returns false otherwise.
    ///
    /// For any QueryLiteral on which `is_null` returns true, `as_null` is guaranteed
    /// to return `Some(())`.
    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    /// If the `QueryLiteral` is a Null, returns (). Returns None otherwise.
    pub fn as_null(&self) -> Option<()> {
        match *self {
            QueryLiteral::Null => Some(()),
            _ => None,
        }
    }
}

impl Debug for QueryLiteral {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "QueryLiteral({})", self)
    }
}

impl Display for QueryLiteral {
    /// Display a QueryLiteral as a string.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryLiteral::Null => formatter.write_str("Null"),
            QueryLiteral::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            QueryLiteral::Number(number) => Debug::fmt(number, formatter),
            QueryLiteral::String(string) => write!(formatter, "String({:?})", string),
        }
    }
}