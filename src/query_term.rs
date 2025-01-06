use core::fmt::{Debug, Display};
use std::fmt;
use crate::number::Number;
use crate::query_literal::QueryLiteral;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash)]
pub enum QueryTerm {
    Literal(QueryLiteral),
    Field(String, Box<QueryLiteral>),
}

impl QueryTerm {
    /// Returns true if the `QueryTerm` is a String. Returns false otherwise.
    ///
    /// For any QueryLiteral on which `is_string` returns true, `as_str` is guaranteed
    /// to return the string slice.
    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    /// If the `QueryTerm` is a String, returns the associated str. Returns None
    /// otherwise.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            QueryTerm::Literal(l) => l.as_str(),
            QueryTerm::Field(_, l) => l.as_str()
        }
    }

    /// Returns true if the `QueryTerm` is a Number. Returns false otherwise.
    ///
    pub fn is_number(&self) -> bool {
        match self {
            QueryTerm::Literal(l) => l.is_number(),
            QueryTerm::Field(_, l) => l.is_number()
        }
    }

    /// If the `QueryLiteral` is a Number, returns the associated [`Number`]. Returns
    /// None otherwise.
    pub fn as_number(&self) -> Option<&Number> {
        match self {
            QueryTerm::Literal(l) => l.as_number(),
            QueryTerm::Field(_, l) => l.as_number()
        }
    }

    /// Returns true if the `QueryLiteral` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any QueryLiteral on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer QueryLiteral.
    pub fn is_i64(&self) -> bool {
        match self {
            QueryTerm::Literal(l) => l.is_i64(),
            QueryTerm::Field(_, l) => l.is_i64()
        }
    }

    /// Returns true if the `QueryLiteral` is an integer between zero and `u64::MAX`.
    ///
    /// For any QueryLiteral on which `is_u64` returns true, `as_u64` is guaranteed to
    /// return the integer QueryLiteral.
    pub fn is_u64(&self) -> bool {
        match self {
            QueryTerm::Literal(l) => l.is_u64(),
            QueryTerm::Field(_, l) => l.is_u64()
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
            QueryTerm::Literal(l) => l.is_f64(),
            QueryTerm::Field(_, l) => l.is_f64()
        }
    }

    /// If the `QueryLiteral` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            QueryTerm::Literal(l) => l.as_i64(),
            QueryTerm::Field(_, l) => l.as_i64()
        }
    }

    /// If the `QueryLiteral` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            QueryTerm::Literal(l) => l.as_u64(),
            QueryTerm::Field(_, l) => l.as_u64()
        }
    }

    /// If the `QueryLiteral` is a number, represent it as f64 if possible. Returns
    /// None otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            QueryTerm::Literal(l) => l.as_f64(),
            QueryTerm::Field(_, l) => l.as_f64()
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
        match self {
            QueryTerm::Literal(l) => l.as_bool(),
            QueryTerm::Field(_, l) => l.as_bool()
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
        match self {
            QueryTerm::Literal(l) => l.as_null(),
            QueryTerm::Field(_, l) => l.as_null()
        }
    }
}

impl Debug for QueryTerm {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "QueryTerm({})", self)
    }
}

impl Display for QueryTerm {
    /// Display a QueryLiteral as a string.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryTerm::Literal(l) => write!(formatter, "{}", l),
            QueryTerm::Field(n, l) => write!(formatter, "Field({},{})", n, l),
        }
    }
}