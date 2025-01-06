use std::error::Error;
use crate::repo_query::RepoQuery;
use serde_json::{Map, Value};
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use crate::query_term::QueryTerm;

pub enum RepoDocumentErr {
    NotFound(Box<dyn Error>),
    ReadError(Box<dyn Error>),
    WriteError(Box<dyn Error>),
    UpdateError(Box<dyn Error>),
    DeleteError(Box<dyn Error>),
    CreateError(Box<dyn Error>),
    CopyError(Box<dyn Error>),
    RemoveError(Box<dyn Error>),
}

impl Error for RepoDocumentErr {}

impl RepoDocumentErr {
    fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoDocumentErr::NotFound(e) => write!(f, "Repo document not found: {}", e),
            RepoDocumentErr::ReadError(e) => write!(f, "Repo document read error: {}", e),
            RepoDocumentErr::WriteError(e) => write!(f, "Repo document write error: {}", e),
            RepoDocumentErr::UpdateError(e) => write!(f, "Repo document update error: {}", e),
            RepoDocumentErr::DeleteError(e) => write!(f, "Repo document delete error: {}", e),
            RepoDocumentErr::CreateError(e) => write!(f, "Repo document create error: {}", e),
            RepoDocumentErr::CopyError(e) => write!(f, "Repo document copy error: {}", e),
            RepoDocumentErr::RemoveError(e) => write!(f, "Repo document remove error: {}", e),
        }
    }
}

impl Debug for RepoDocumentErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl Display for RepoDocumentErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

pub trait RepoDocument<T, Q> {
    fn find_one(&self, qry: RepoQuery<Q>) -> Result<Option<T>, RepoDocumentErr>;
    fn find_many(&self, qry: RepoQuery<Q>) -> Result<Vec<T>, RepoDocumentErr>;
    fn read(&self) -> Result<T, RepoDocumentErr>;
    fn write(&self, data: T) -> Result<(), RepoDocumentErr>;
    fn update(&self, key: &str, data: T) -> Result<(), RepoDocumentErr>;
    fn delete(&self, key: &str) -> Result<(), RepoDocumentErr>;
    fn exists(&self) -> bool;
    fn copy(&self, source: &str) -> Result<u64, RepoDocumentErr>;
    fn remove(&self) -> Result<(), RepoDocumentErr>;
}

pub struct JsonDocument {
    path: String,
    full_path: PathBuf,
}

impl JsonDocument {
    pub fn new(base_path: &str, path: &str) -> JsonDocument {
        Self {
            path: String::from(path),
            full_path: Path::new(base_path).join(path),
        }
    }
}

impl RepoDocument<Value, QueryTerm> for JsonDocument {

    fn find_one(&self, qry: RepoQuery<QueryTerm>) -> Result<Option<Value>, RepoDocumentErr> {
        Ok(None)
    }

    fn find_many(&self, qry: RepoQuery<QueryTerm>) -> Result<Vec<Value>, RepoDocumentErr> {
        let mut results = Vec::new();
        Ok(results)
    }

    fn read(&self) -> Result<Value, RepoDocumentErr> {
        match fs::read_to_string(&self.full_path) {
            Ok(contents) => Ok(serde_json::from_str(&contents).unwrap()),
            Err(s) => Err(RepoDocumentErr::ReadError(Box::new(s))),
        }
    }

    fn write(&self, data: Value) -> Result<(), RepoDocumentErr> {
        match fs::write(&self.full_path, data.to_string()) {
            Ok(_) => Ok(()),
            Err(s) => Err(RepoDocumentErr::WriteError(Box::new(s))),
        }
    }

    fn update(&self, key: &str, value: Value) -> Result<(), RepoDocumentErr> {
        match self.read() {
            Ok(data) => match self.write(update_json_value(&data, key, &value)) {
                Ok(_) => Ok(()),
                Err(s) => Err(RepoDocumentErr::UpdateError(Box::new(s))),
            },
            Err(s) => Err(RepoDocumentErr::UpdateError(Box::new(s))),
        }
    }

    fn delete(&self, key: &str) -> Result<(), RepoDocumentErr> {
        match self.read() {
            Ok(data) => match self.write(delete_json_key(&data, key)) {
                Ok(_) => Ok(()),
                Err(s) => Err(RepoDocumentErr::UpdateError(Box::new(s))),
            },
            Err(s) => Err(RepoDocumentErr::UpdateError(Box::new(s))),
        }
    }

    fn exists(&self) -> bool {
        fs::exists(&self.full_path).unwrap_or_else(|_| false)
    }


    fn copy(&self, source: &str) -> Result<u64, RepoDocumentErr> {
        fs::copy(source, &self.full_path).map_err(|e| RepoDocumentErr::CopyError(Box::new(e)))
    }

    fn remove(&self) -> Result<(), RepoDocumentErr> {
        fs::remove_file(&self.full_path).map_err(|e| RepoDocumentErr::RemoveError(Box::new(e)))
    }
}

fn update_json_value(data: &Value, key: &str, value: &Value) -> Value {
    match data.clone().as_object_mut() {
        Some(b) => {
            let mut bb = b.clone();
            let mut build = &mut bb;
            let keys: Vec<&str> = key.split('.').collect();
            for (i, k) in keys.iter().enumerate() {
                let key = *k;
                match build.get(key) {
                    Some(v) => {
                        if i == keys.len() - 1 {
                            build[key] = value.clone();
                        } else {
                            if !v.is_object() {
                                build[key] = Value::from(Map::new());
                            }
                            build = build.get_mut(key).unwrap().as_object_mut().unwrap()
                        }
                    }
                    None => {
                        if i == keys.len() - 1 {
                            build.insert(key.into(), value.clone());
                        } else {
                            build.insert(key.into(), Value::from(Map::new()));
                            build = build.get_mut(key).unwrap().as_object_mut().unwrap()
                        }
                    }
                }
            }
            Value::from(bb)
        }
        None => data.clone(),
    }
}


fn delete_json_key(data: &Value, key: &str) -> Value {
    match data.clone().as_object_mut() {
        Some(b) => {
            let mut bb = b.clone();
            let mut build = &mut bb;
            let keys: Vec<&str> = key.split('.').collect();
            for (i, k) in keys.iter().enumerate() {
                let key = *k;
                match build.get(key) {
                    Some(v) => {
                        if i == keys.len() - 1 {
                            build.remove(key);
                        } else {
                            if !v.is_object() {
                                return data.clone();
                            }
                            build = build.get_mut(key).unwrap().as_object_mut().unwrap()
                        }
                    }
                    None => return data.clone(),
                }
            }
            Value::from(bb)
        }
        None => data.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_json_value() {
        let data = r#"
        {
            "name": "John",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;


        let v: Value = serde_json::from_str(data).unwrap();
        let name = "John Doe";
        let name_value = Value::from(String::from(name));
        let new_data_name = update_json_value(&v, "name", &name_value);
        assert_eq!(new_data_name["name"], String::from(name));
        let zip = 999999;
        let zip_value = Value::from(zip);
        let new_data_zip = update_json_value(&v, "address.zip", &zip_value);
        assert_eq!(new_data_zip["address"]["zip"], zip_value);
    }

    #[test]
    fn test_delete_json_key() {
        let data = r#"
        {
            "name": "John",
            "age": 43,
            "address": {
                "zip": "999999"
            },
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;


        let v: Value = serde_json::from_str(data).unwrap();
        let new_data_name = delete_json_key(&v, "name");
        assert_eq!(new_data_name.get("name"), None);
        let new_data_zip = delete_json_key(&v, "address.zip");
        assert_eq!(new_data_zip.get("address").unwrap().get("zip"), None);

    }
}
