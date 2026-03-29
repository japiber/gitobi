use std::fmt::{Display, Formatter};
use std::hash::Hash;

pub trait QCKey {
    fn key(&self) -> String;
}

pub struct QueryKey<K> where K: Display + for<'a> From<&'a str> + Clone + Hash + Eq {
    key_chain: Vec<K>
}

impl<K> QueryKey<K> where K: Display + for<'a> From<&'a str> + Clone + Hash + Eq {
    pub fn new() -> QueryKey<K> {
        QueryKey { key_chain: vec![] }
    }
    
    pub fn clone(&self) -> QueryKey<K> {
        QueryKey { key_chain: self.key_chain.clone() }
    }

    pub fn push(&mut self, key: K) {
        self.key_chain.push(key.into());
    }

    pub fn suffix(&mut self, suffix: &Vec<K>) {
        for mk in suffix.iter() {
            self.key_chain.push(mk.clone());
        }
    }

    pub fn prefix(&mut self, prefix: &Vec<K>) {
        let mut vjk : Vec<K> = Vec::new();
        for mk in prefix.iter() {
            vjk.push(mk.clone());
        }
        for mk in self.key_chain.iter() {
            vjk.push(mk.clone());
        }
        self.key_chain = vjk;
    }
}

impl<K> QCKey for QueryKey<K> where K: Display + for<'a> From<&'a str> + Clone + Hash + Eq {
    fn key(&self) -> String {
        self.to_string()
    }
}

impl<K: Display + for<'a> From<&'a str> + Clone + Hash + Eq> Display for QueryKey<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vjk: Vec<String> = self.key_chain.iter().map(|k| k.to_string()).collect();
        write!(f, "{}", vjk.join("."))
    }
}

impl<K: Display + for<'a> From<&'a str> + Clone + Hash + Eq> From<&str> for QueryKey<K> {
    fn from(key: &str) -> Self {
        let mut vjk: Vec<K> = vec!();
        for ks in key.to_string().split('.') {
            let kks: K = ks.into();
            vjk.push(kks);
        }
        QueryKey { key_chain: vjk }
    }
}

impl<K: Display + for<'a> From<&'a str> + Clone + Hash + Eq> From<String> for QueryKey<K> {
    fn from(key: String) -> Self {
        let mut vjk: Vec<K> = vec!();
        for ks in key.split('.') {
            let kks: K = ks.into();
            vjk.push(kks);
        }
        QueryKey { key_chain: vjk }
    }
}
