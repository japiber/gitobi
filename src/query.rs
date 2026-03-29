use crate::json_document::DocumentError;
use crate::query_key::{QCKey, QueryKey};
use crate::query_value::DocumentValue;
use serde_json::Value;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::hash::Hash;


#[derive(Clone, Debug)]
pub enum QueryClause<K: QCKey> {
    Eq(K, DocumentValue),
    Ne(K, DocumentValue),
    Ge(K, DocumentValue),
    Gt(K, DocumentValue),
    Le(K, DocumentValue),
    Lt(K, DocumentValue),
    IsNull(K),
    And(Box<QueryClause<K>>, Box<QueryClause<K>>),
    Or(Box<QueryClause<K>>, Box<QueryClause<K>>),
    Not(Box<QueryClause<K>>)
}

pub type QryClause = QueryClause<QueryKey<String>>;

#[derive(Clone, Debug, PartialEq)]
pub enum QueryClauseEvalError {
    ValueNotFound(String),
}

impl Display for QueryClauseEvalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryClauseEvalError::ValueNotFound(s) => write!(f, "Value '{}' not found in the data dictionary", s),
        }
    }
}

impl Error for QueryClauseEvalError {}



pub struct QueryData {
    dictionary: HashMap<String, DocumentValue>
}

impl Default for QueryData {
    fn default() -> Self {
        let xs: [(&str, &str); 0] = [];
        Self::new(&xs)
    }
}

impl QueryData {

    pub fn new<K: Display, V: Clone>(values: &[(K, V)]) -> Self where DocumentValue: From<V> {
        let mut dictionary : HashMap<String, DocumentValue> = HashMap::new();
        for (key, value) in values {
            dictionary.insert(key.to_string(), DocumentValue::from(value.clone()));
        }
        QueryData { dictionary }
    }

    pub fn load<K: Display + for<'a> From<&'a str> + Clone + Hash + Eq>(value: &Value) -> Self {
        let mut qk: QueryKey<K> = QueryKey::new();
        let mut qd = Self::default();
        qd.populate(value, &mut qk);
        qd
    }

    pub fn get<Q: QCKey>(&self, qk: &Q) -> Option<&DocumentValue> {
        self.dictionary.get(&qk.key())
    }

    pub fn do_comparison<Q: QCKey>(&self, qk: &Q, value: &DocumentValue, cmp_results: Vec<CompareOrdering>) -> Result<bool, QueryClauseEvalError> {
        if let Some(dv) = self.get(qk) {
            if let Some(result) = dv.partial_cmp(value) {
                for qry_cmp in cmp_results {
                    match qry_cmp {
                        CompareOrdering::Is(o) =>
                            if result == o {
                                return Ok(true)
                            },
                        CompareOrdering::IsNot(o) =>
                            if result != o {
                                return Ok(true)
                            }
                    }
                }
            }
            return Ok(false)
        }
        Err(QueryClauseEvalError::ValueNotFound(qk.key()))
    }

    pub fn is_null<Q: QCKey>(&self, qk: &Q) -> Result<bool, QueryClauseEvalError> {
        if let Some(dv) = self.get(qk) {
            let dvv : DocumentValue = dv.clone();
            return if dvv.is_null() {
                Ok(true)
            } else {
                Ok(false)
            }

        }
        Err(QueryClauseEvalError::ValueNotFound(qk.key()))
    }

    fn populate<K: Display + for<'a> From<&'a str> + Clone + Hash + Eq>(&mut self, value: &Value, qck: &mut QueryKey<K>) {
        if let Some(od) = value.as_object() {
            for (k, v) in od {
                if !v.is_object() && !v.is_array() {
                    let ks : K = k.as_str().into();
                    let mut qqck : QueryKey<K> = qck.clone();
                    qqck.push(ks);
                    self.dictionary.insert(qqck.key(), v.into());
                }
                if v.is_object() {
                    let mut oqck = qck.clone();
                    let ks : K = k.as_str().into();
                    oqck.push(ks);
                    self.populate(v, &mut oqck);
                }
                if v.is_array() {
                    for (i, item) in v.as_array().unwrap().iter().enumerate() {
                        let ks : K = format!("{}[{}]", k, i).as_str().into();
                        qck.push(ks);
                        self.populate(item, qck);
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum CompareOrdering {
    Is(Ordering),
    IsNot(Ordering),
}

impl<K: QCKey> QueryClause<K>  {
    pub fn equal<QK, V: Into<DocumentValue> + PartialOrd>(key: QK, value: V) -> QueryClause<K> where K: From<QK> {
        QueryClause::Eq(key.into(), value.into())
    }

    pub fn not_equal<QK, V: Into<DocumentValue> + PartialOrd>(key: QK, value: V) -> QueryClause<K> where K: From<QK> {
        QueryClause::Ne(key.into(), value.into())
    }

    pub fn greater_or_equal_than<QK, V: Into<DocumentValue> + PartialOrd>(key: QK, value: V) -> QueryClause<K> where K: From<QK> {
        QueryClause::Ge(key.into(), value.into())
    }

    pub fn less_or_equal_than<QK, V: Into<DocumentValue> + PartialOrd>(key: QK, value: V) -> QueryClause<K> where K: From<QK> {
        QueryClause::Le(key.into(), value.into())
    }

    pub fn greater_than<QK, V: Into<DocumentValue> + PartialOrd>(key:QK, value: V) -> QueryClause<K> where K: From<QK> {
        QueryClause::Gt(key.into(), value.into())
    }

    pub fn less_than<QK, V: Into<DocumentValue> + PartialOrd>(key: QK, value: V) -> QueryClause<K> where K: From<QK> {
        QueryClause::Lt(key.into(), value.into())
    }

    pub fn is_null<QK>(key: QK) -> QueryClause<K> where K: From<QK> {
        QueryClause::IsNull(key.into())
    }

    pub fn and(left: QueryClause<K>, right: QueryClause<K>) -> QueryClause<K> {
        QueryClause::And(Box::new(left), Box::new(right))
    }

    pub fn or(left: QueryClause<K>, right: QueryClause<K>) -> QueryClause<K> {
        QueryClause::Or(Box::new(left), Box::new(right))
    }

    pub fn not(clause: QueryClause<K>) -> QueryClause<K> {
        QueryClause::Not(Box::new(clause))
    }

    pub fn eval(&self, data: &QueryData) -> Result<bool, QueryClauseEvalError> {
        match self {
            QueryClause::Eq(qk, qv) =>
                data.do_comparison(qk, qv, vec![CompareOrdering::Is(Equal)]),
            QueryClause::Ne(qk, qv) =>
                data.do_comparison(qk, qv, vec![CompareOrdering::IsNot(Equal)]),
            QueryClause::Ge(qk, qv) =>
                data.do_comparison(qk, qv, vec![CompareOrdering::Is(Equal), CompareOrdering::Is(Greater)]),
            QueryClause::Gt(qk, qv) =>
                data.do_comparison(qk, qv, vec![CompareOrdering::Is(Greater)]),
            QueryClause::Le(qk, qv) =>
                data.do_comparison(qk, qv, vec![CompareOrdering::Is(Less), CompareOrdering::Is(Equal)]),
            QueryClause::Lt(qk, qv) =>
                data.do_comparison(qk, qv, vec![CompareOrdering::Is(Less)]),
            QueryClause::IsNull(qk) => {
                data.is_null(qk)
            },
            QueryClause::And(ca, cb) => {
                match (ca.eval(data), cb.eval(data)) {
                    (Ok(true), Ok(true)) => Ok(true),
                    (Ok(_), Ok(_)) => Ok(false),
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                }
            }
            QueryClause::Or(ca, cb) => {
                match (ca.eval(data), cb.eval(data)) {
                    (Ok(true), Ok(_)) => Ok(true),
                    (Ok(_), Ok(true)) => Ok(true),
                    (Ok(_), Ok(_)) => Ok(false),
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                }
            }
            QueryClause::Not(ca) => {
                match ca.eval(data) {
                    Ok(v) => Ok(!v),
                    Err(e) => Err(e),
                }
            }
        }
    }
}

pub trait QueryableDocument<K: QCKey> {
    fn update(&mut self, key: &str, value: DocumentValue, clause: Option<QueryClause<K>>) -> Result<(), DocumentError>;
    fn delete(&mut self, key: &str, clause: Option<QueryClause<K>>) -> Result<(), DocumentError>;
    fn select(&self, keys: &[&str], clause: Option<QueryClause<K>>) -> Result<Vec<(String, Value)>, DocumentError>;
}
