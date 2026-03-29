use crate::query::{QueryClause, QueryData, QueryableDocument};
use crate::query_key::QCKey;
use crate::query_value::DocumentValue;
use serde_json::{Map, Value};
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io;

pub enum DocumentError {
    Load(Box<dyn Error>),
    Write(Box<dyn Error>),
    Update(String),
    Delete(String),
    Select(String),
}

impl Error for DocumentError {}

impl DocumentError {
    fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentError::Load(e) => write!(f, "Repo document read error: {}", e),
            DocumentError::Write(e) => write!(f, "Repo document write error: {}", e),
            DocumentError::Update(e) => write!(f, "Repo document update error: {}", e),
            DocumentError::Delete(e) => write!(f, "Repo document delete error: {}", e),
            DocumentError::Select(e) => write!(f, "Repo document select error: {}", e),
        }
    }
}

impl Debug for DocumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl Display for DocumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

pub struct Document<T> where T: Debug + Clone + serde::Serialize + for<'a> serde::Deserialize<'a> {
    content: T,
}

impl<T: Clone + Debug + serde::Serialize + for<'a> serde::Deserialize<'a>> Document<T> {
    pub fn load(reader: &mut dyn io::Read, map_from: fn(&str) -> Result<T, Box<dyn Error>>) -> Result<Document<T>, DocumentError> {
        let mut contents = String::new();
        match reader.read_to_string(&mut contents) {
            Ok(_) => {
                match map_from(&contents) {
                    Ok(value) => Ok(
                        Self {
                            content: value
                        }
                    ),
                    Err(e) => Err(DocumentError::Load(e)),
                }
            },
            Err(s) => Err(DocumentError::Load(Box::new(s))),
        }
    }

    pub fn write(&mut self, writer: &mut dyn io::Write, map_into: fn(&T) -> String) -> Result<(), DocumentError> where serde_json::Value: std::convert::From<T> {
        match writer.write_all(map_into(&self.content).as_bytes()) {
            Ok(_) => Ok(()),
            Err(s) => Err(DocumentError::Write(Box::new(s))),
        }
    }

    pub fn content(&self) -> &T {
        &self.content
    }
}


impl<K: QCKey> QueryableDocument<K> for Document<Map<String, Value>> {
    fn update(&mut self, key: &str, data: DocumentValue, clause: Option<QueryClause<K>>) -> Result<(), DocumentError> {
        let value = serde_json::Value::Object(self.content.clone());
        let mut do_update = |k: &str, v: DocumentValue| -> Result<(), DocumentError> {
            match update_key(k, &self.content, v) {
                Ok(v) => {
                    self.content = v;
                    Ok(())
                },
                Err(e) => Err(e),
            }
        };

        if let Some(qry) = clause {
            let qd = QueryData::load::<String>(&value);
            match qry.eval(&qd) {
                Ok(qb) => if qb {
                    do_update(key, data)
                } else {
                    Ok(())
                },
                Err(_) => Err(DocumentError::Update(key.to_string())),
            }
        } else {
            do_update(key, data)
        }
    }

    fn delete(&mut self, key: &str, clause: Option<QueryClause<K>>) -> Result<(), DocumentError> {
        let value = serde_json::Value::Object(self.content.clone());
        let mut do_delete = |key: &str| -> Result<(), DocumentError> {
            match delete_key(key, &self.content) {
                Ok(v) => {
                    self.content = v;
                    Ok(())
                },
                Err(e) => Err(e),
            }
        };

        if let Some(qry) = clause {
            let qd = QueryData::load::<String>(&value);
            match qry.eval(&qd) {
                Ok(qb) => if qb {
                    do_delete(key)
                } else {
                    Ok(())
                },
                Err(_) => Err(DocumentError::Delete(key.to_string())),
            }
        } else {
            do_delete(key)
        }
    }

    fn select(&self, keys: &[&str], clause: Option<QueryClause<K>>) -> Result<Vec<(String, Value)>, DocumentError> {
        let do_select = |ks: &[&str]| -> Vec::<(String, Value)> {
            let mut select_result = Vec::<(String, Value)>::with_capacity(ks.len());
            for key in ks {
                if let Ok(kv) = get_key(key, &self.content) {
                    let vv = (key.to_string(), kv);
                    select_result.push(vv);
                }
            }
            select_result
        };

        let value = serde_json::Value::Object(self.content.clone());
        if let Some(qry) = clause {
            let qd = QueryData::load::<String>(&value);
            match qry.eval(&qd) {
                Ok(qb) => {
                    if qb {
                        Ok(do_select(keys))
                    } else {
                        Ok(vec![])
                    }
                },
                Err(e) => Err(DocumentError::Select(e.to_string())),
            }
        } else {
            Ok(do_select(keys))
        }
    }
}

pub fn map_from_str(content: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
    match serde_json::from_str(content) {
        Ok(value) => Ok(
            value
        ),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn map_into_string(content: &Map<String, Value>) -> String {
    let value: Value = content.clone().into();
    value.to_string()
}

pub fn update_key(key: &str, current: &Map<String, Value>, new_value: DocumentValue) -> Result<Map<String, Value>, DocumentError> {
    let mut keys : VecDeque<String> = VecDeque::from(key.split('.').map(|s| s.to_string()).collect::<Vec<_>>());
    match keys.pop_front() {
        Some(key) => {
            match current.get(&key) {
                Some(iv) => {
                    if keys.is_empty() {
                        let mut vc = current.clone();
                        vc.insert(key.to_string(), serde_json::Value::from(new_value));
                        Ok(vc.clone())
                    } else if iv.is_object() {
                        let mut vc = current.clone();
                        match update_key(build_key(&keys).as_str(), iv.as_object().unwrap(), new_value) {
                            Ok(vv) => {
                                vc.insert(key.to_string(), Value::from(vv));
                                Ok(vc.clone())
                            }
                            Err(e) => Err(e),
                        }
                    } else {
                        Err(DocumentError::Update(format!("{} is not and object", &key)))
                    }
                },
                None => {
                    if keys.is_empty() {
                        let mut vc = current.clone();
                        vc.insert(key.to_string(), serde_json::Value::from(new_value));
                        Ok(vc.clone())
                    } else {
                        let iv = serde_json::Map::new();
                        match update_key(build_key(&keys).as_str(), &iv, new_value) {
                            Ok(vv) => {
                                let mut vc = current.clone();
                                vc.insert(key.to_string(), Value::from(vv));
                                Ok(vc.clone())
                            },
                            Err(e) => Err(e),
                        }
                    }
                }
            }
        },
        None => {
            Ok(current.clone())
        }
    }
}

pub fn delete_key(key: &str, current: &Map<String, Value>) -> Result<Map<String, Value>, DocumentError> {
    let mut keys : VecDeque<String> = VecDeque::from(key.split('.').map(|s| s.to_string()).collect::<Vec<_>>());
    match keys.pop_front() {
        Some(key) => {
            match current.get(&key) {
                Some(iv) => {
                    if keys.is_empty() {
                        let mut vc = current.clone();
                        vc.remove(&key.to_string());
                        Ok(vc.clone())
                    } else if iv.is_object() {
                        let mut vc = current.clone();
                        match delete_key(build_key(&keys).as_str(), iv.clone().as_object().unwrap()) {
                            Ok(vv) => {
                                vc.insert(key.to_string(), Value::from(vv));
                                Ok(vc.clone())
                            }
                            Err(e) => return Err(e),
                        }
                    } else {
                        Err(DocumentError::Delete(format!("{} is not and object", &key)))
                    }
                },
                None => Err(DocumentError::Delete(format!("key '{}' does not exists", &key)))
            }
        }
        None => {
            Ok(current.clone())
        }
    }
}

pub fn contains_key(key: &str, current: &Map<String, Value>) -> bool {
    let mut keys : VecDeque<String> = VecDeque::from(key.split('.').map(|s| s.to_string()).collect::<Vec<_>>());
    match keys.pop_front() {
        Some(key) => {
            match current.get(&key) {
                Some(iv) => {
                    if keys.is_empty() {
                        true
                    } else if iv.is_object() {
                        contains_key(build_key(&keys).as_str(), iv.clone().as_object().unwrap())
                    } else {
                        false
                    }
                },
                None => false
            }
        }
        None => {
            true
        }
    }
}

pub fn get_key(key: &str, current: &Map<String, Value>) -> Result<Value, DocumentError> {
    let mut keys : VecDeque<String> = VecDeque::from(key.split('.').map(|s| s.to_string()).collect::<Vec<_>>());
    match keys.pop_front() {
        Some(key) => {
            match current.get(&key) {
                Some(iv) => {
                    if keys.is_empty() {
                        Ok(iv.clone())
                    } else if iv.is_object() {
                        get_key(build_key(&keys).as_str(), iv.clone().as_object().unwrap())
                    } else {
                        Err(DocumentError::Select(format!("{} not found", &key)))
                    }
                },
                None => Err(DocumentError::Select(format!("{} not found", &key)))
            }
        }
        None => {
            Ok(Value::Object(current.clone()))
        }
    }
}

pub fn build_key(keys: &VecDeque<String>) -> String {
    let (front, back) = keys.as_slices();
    let kj = [front, back].concat();
    kj.join(".")
}
