
pub enum List {
    Cons(String, Box<List>),
    Nil
}

pub enum RepoQuery<T> where T: PartialEq {
    Select(List),
    From(String),
    Where(Clause<T>)
}

pub enum Clause<T> where T: PartialEq {
    Eq(String, T),
    Ne(String, T),
    Ge(String, T),
    Gt(String, T),
    Le(String, T),
    Lt(String, T),
    And(Box<RepoQuery<T>>, Box<RepoQuery<T>>),
    Or(Box<RepoQuery<T>>, Box<RepoQuery<T>>),
    Not(Box<RepoQuery<T>>),
}