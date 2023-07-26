use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

#[derive(Debug)]
pub enum Obj {
    String(String),
}

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    Obj(Rc<Obj>),
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        if let Self::Obj(o) = self {
            if let Obj::String(s) = o.as_ref() {
                return Some(s);
            }
        }
        None
    }

    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn from_string(s: String) -> Value {
        Self::Obj(Rc::new(Obj::String(s)))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Nil => write!(f, "nil"),
            Value::Number(n) => write!(f, "{n}"),
            Value::Obj(o) => match o.as_ref() {
                Obj::String(s) => write!(f, "{s}"),
            },
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Nil
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Nil, Self::Nil) => true,
            (Self::Obj(a), Self::Obj(b)) => match (a.as_ref(), b.as_ref()) {
                (Obj::String(a), Obj::String(b)) => a == b,
            },
            _ => false,
        }
    }
}
