use std::cmp::Ordering;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Number {
    n: N,
}

#[derive(Debug, Copy, Clone)]
enum N {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
    NotANumber
}

impl PartialEq<Self> for N {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (N::PosInt(a), N::PosInt(b)) => a == b,
            (N::NegInt(a), N::NegInt(b)) => a == b,
            (N::Float(a), N::Float(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd<Self> for N {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (N::Float(a), N::PosInt(b)) => {
                let c = *b as f64;
                a.partial_cmp(&c)
            },
            (N::PosInt(a), N::Float(b)) => {
                let c = *a as f64;
                c.partial_cmp(b)
            },
            (N::Float(a), N::NegInt(b)) => {
                let c = *b as f64;
                a.partial_cmp(&c)
            },
            (N::NegInt(a), N::Float(b)) => {
                let c = *a as f64;
                c.partial_cmp(b)
            },
            (N::PosInt(a), N::PosInt(b)) => a.partial_cmp(b),
            (N::PosInt(a), N::NegInt(b)) => {
                if *b >= 0 {
                    let c = *b as u64;
                    a.partial_cmp(&c)
                } else {
                    Some(Ordering::Greater)
                }
            },
            (N::NegInt(a), N::PosInt(b)) => {
                if *a >= 0 {
                    let c = *a as u64;
                    c.partial_cmp(&b)
                } else {
                    Some(Ordering::Less)
                }
            },
            (N::NegInt(a), N::NegInt(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl From<Value> for Number {
    fn from(v: Value) -> Self {
        match v.as_number() {
            None => Number { n: N::NotANumber },
            Some(n) => {
                if let Some(f) = n.as_f64() {
                    return Number { n: N::Float(f) }
                }
                if let Some(u) = n.as_u64() {
                    return Number { n: N::PosInt(u) }
                }
                if let Some(i) = n.as_i64() {
                    return Number { n: N::NegInt(i) }
                }
                Number { n: N::NotANumber }
            }
        }
    }
}

impl From<Number> for Value {
    fn from(v: Number) -> Self {
        match v.n {
            N::Float(v) => v.into(),
            N::PosInt(v) => v.into(),
            N::NegInt(v) => v.into(),
            N::NotANumber => Value::Null,
        }
    }
}

#[derive(Clone, Debug)]
pub enum DocumentValue {
    String(String),
    Number(Number),
    Boolean(bool),
    Array(Vec<DocumentValue>),
    Null
}

impl DocumentValue {
    pub fn is_null(&self) -> bool {
        self == &DocumentValue::Null
    }
}

impl PartialEq<Self> for DocumentValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DocumentValue::String(a), DocumentValue::String(b)) => a == b,
            (DocumentValue::Number(a), DocumentValue::Number(b)) => a == b,
            (DocumentValue::Boolean(a), DocumentValue::Boolean(b)) => a == b,
            (DocumentValue::Array(a), DocumentValue::Array(b)) => a == b,
            (DocumentValue::Null, DocumentValue::Null) => true,
            _ => false
        }
    }
}

impl PartialOrd<Self> for DocumentValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (DocumentValue::String(a), DocumentValue::String(b)) => a.partial_cmp(b),
            (DocumentValue::Number(a), DocumentValue::Number(b)) => a.partial_cmp(b),
            (DocumentValue::Boolean(a), DocumentValue::Boolean(b)) => a.partial_cmp(b),
            (DocumentValue::Array(a), DocumentValue::Array(b)) => a.partial_cmp(b),
            (DocumentValue::Null, DocumentValue::Null) => Some(Ordering::Equal),
            _ => None
        }
    }
}

impl From<&Value> for DocumentValue {
    fn from(value: &Value) -> Self {
        if value.as_number().is_some() {
            return DocumentValue::Number(value.clone().into());
        }
        if let Some(s) = value.as_str() {
            return DocumentValue::String(s.to_string())
        }
        if let Some(b) = value.as_bool() {
            return DocumentValue::Boolean(b)
        }
        if let Some(a) = value.as_array() {
            return DocumentValue::Array(a.iter().map(DocumentValue::from).collect())
        }
        DocumentValue::Null
    }
}

impl<T: Into<DocumentValue>> From<Option<T>> for DocumentValue {
    fn from(value: Option<T>) -> Self {
        match value {
            None => DocumentValue::Null,
            Some(v) => v.into()
        }
    }
}

impl From<DocumentValue> for Value {
    fn from(value: DocumentValue) -> Self {
        match value {
            DocumentValue::String(s) => Value::String(s.clone()),
            DocumentValue::Number(v) => Value::from(v),
            DocumentValue::Boolean(v) => Value::from(v),
            DocumentValue::Array(v) => Value::from(v),
            DocumentValue::Null => Value::from(Value::Null),
        }
    }
}

impl From<String> for DocumentValue {
    fn from(s: String) -> DocumentValue {
        DocumentValue::String(s)
    }
}

impl From<&str> for DocumentValue {
    fn from(s: &str) -> DocumentValue {
        DocumentValue::String(s.to_string())
    }
}

impl From<i64> for DocumentValue {
    fn from(s: i64) -> DocumentValue {
        if s < 0 {
            DocumentValue::Number(Number { n: N::NegInt(s) })
        } else {
            DocumentValue::Number(Number { n: N::PosInt(s as u64) })
        }
    }
}

impl From<i32> for DocumentValue {
    fn from(s: i32) -> DocumentValue {
        if s < 0 {
            DocumentValue::Number(Number { n: N::NegInt(s as i64) })
        } else {
            DocumentValue::Number(Number { n: N::PosInt(s as u64) })
        }
    }
}

impl From<u64> for DocumentValue {
    fn from(s: u64) -> DocumentValue {
        DocumentValue::Number(Number{ n: N::PosInt(s)})
    }
}

impl From<u32> for DocumentValue {
    fn from(s: u32) -> DocumentValue {
        DocumentValue::Number(Number{ n: N::PosInt(s as u64) })
    }
}

impl From<f64> for DocumentValue {
    fn from(s: f64) -> DocumentValue {
        DocumentValue::Number(Number{ n: N::Float(s)})
    }
}

impl From<bool> for DocumentValue {
    fn from(s: bool) -> DocumentValue {
        DocumentValue::Boolean(s)
    }
}

impl<T> From<Vec<T>> for DocumentValue where DocumentValue: From<T> {
    fn from(s: Vec<T>) -> DocumentValue { DocumentValue::Array(s.into_iter().map(DocumentValue::from).collect()) }
}
