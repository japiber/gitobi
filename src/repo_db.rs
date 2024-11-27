use std::fmt::{Debug, Display, Formatter};
use serde_json::Value;

pub enum RepoQuery<T> {
    None,
    Eq(String, T),
    Ne(String, T),
    Ge(String, T),
    Gt(String, T),
    Le(String, T),
    Lt(String, T),
    And(RepoQuery<T>, RepoQuery<T>),
    Or(RepoQuery<T>, RepoQuery<T>),
    Not(RepoQuery<T>),
}

pub enum RepoDocumentErr {
    NotFound(String),
    ReadError(String),
    WriteError(String),
    UpdateError(String),
    DeleteError(String),
    CreateError(String),
    CopyError(String),
    RemoveError(String),
}

impl Debug for RepoDocumentErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for RepoDocumentErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for RepoDocumentErr{}

pub trait RepoDocument<T> {
    fn find_one(&self, qry: RepoQuery<T>) -> Option<T>;
    fn find_many(&self, qry: RepoQuery<T>) -> Vec<T>;
    fn read(&self) -> Option<T>;
    fn write(&self, data: T);
    fn update(&self, key: &str, data: T);
    fn delete(&self, key: &str);
    fn exists(&self) -> bool;
    fn create(&self);
    fn copy(&self, source: &str);
    fn remove(&self);
}

pub type FnModify = dyn Fn(&dyn RepoStore) -> Result<(),Err(&str)>;

pub trait RepoStore {
    fn connect(&self);
    fn document<T>(&self, name: &str) -> impl RepoDocument<T>;
    fn pull(&self);
    fn push(&self);
    fn commit(&self);
    fn rollback(&self);
    fn transaction(&self, name: &str, msg: &str, modify: FnModify);
}