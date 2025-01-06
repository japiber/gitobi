use crate::query_term::QueryTerm;

pub enum List {
    Cons(String, Box<List>),
    Nil
}


pub enum RepoQuery<T> where T: PartialEq + PartialOrd {
    Select(List),
    Where(Clause<T>)
}

enum Clause<T> where T: PartialEq + PartialOrd {
    Eq(T, T),
    Ne(T, T),
    Ge(T, T),
    Gt(T, T),
    Le(T, T),
    Lt(T, T),
    And(Box<Clause<T>>, Box<Clause<T>>),
    Or(Box<Clause<T>>, Box<Clause<T>>),
    Not(Box<Clause<T>>),
}


impl<T> RepoQuery<T> where T: PartialEq + PartialOrd {

    pub fn eq(a: T, b: T) -> Clause<T> {
        Self {
            qry: Clause::Eq(a,b)
        }
    }

    pub fn ne(a: T, b: T) -> Clause<T> {
        Self {
            qry: Clause::Ne(a, b)
        }
    }

    pub fn gt(a: T, b: T) -> Clause<T> {
        Self {
            qry: Clause::Gt(a, b)
        }
    }

    pub fn le(a: T, b: T) -> Clause<T> {
        Self {
            qry: Clause::Le(a, b)
        }
    }

    pub fn lt(a: T, b: T) -> Clause<T> {
        Self {
            qry: Clause::Lt(a, b)
        }
    }

    pub fn and(a: Clause<T>, b: Clause<T>) -> Clause<T> {
        Self {
            qry: Clause::And(Box::new(a), Box::new(b))
        }
    }

    pub fn or(a: Clause<T>, b: Clause<T>) -> Clause<T> {
        Self {
            qry: Clause::Or(Box::new(a), Box::new(b))
        }
    }

    pub fn not(x: Clause<T>) -> Clause<T> {
        Self {
            qry: Clause::Not(Box::new(x))
        }
    }

    pub fn evaluate(&self) -> bool {
        match self {
            Clause::Eq(a, b) => a == b,
            Clause::Ne(a, b) => a != b,
            Clause::Ge(a, b) => a >= b,
            Clause::Gt(a, b) => a > b,
            Clause::Le(a, b) => a <= b,
            Clause::Lt(a, b) => a < b,
            Clause::And(a, b) => a.evaluate() && b.evaluate(),
            Clause::Or(a, b) => a.evaluate() || b.evaluate(),
            Clause::Not(a) => !a.evaluate(),
        }
    }
}



