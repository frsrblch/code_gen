use std::fmt::{Display, Formatter, Error, Debug};
use std::iter::FromIterator;
use crate::StrConcat;

#[derive(Debug, Default, Clone)]
pub struct Generics(Vec<GenericType>);

impl Generics {
    pub fn none() -> Self { Default::default() }

    pub fn one(t: GenericType) -> Self {
        Generics(vec![t])
    }

    pub fn two(t: GenericType, u: GenericType) -> Self {
        Generics(vec![t, u])
    }

    fn get_str_concat(&self) -> impl Display + '_ {
        StrConcat {
            iter: &self.0,
            left_bound: "<",
            right_bound: ">",
            item_prepend: "",
            item_append: "",
            join: ", "
        }
    }
}

impl<'a> FromIterator<GenericType> for Generics {
    fn from_iter<T: IntoIterator<Item=GenericType>>(iter: T) -> Self {
        Generics(iter.into_iter().collect())
    }
}

impl Display for Generics {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.0.len() == 0 {
            return Ok(());
        }

        write!(f, "{}", self.get_str_concat())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GenericType {
    Concrete(String),
    Generic(String),
}

impl GenericType {
    pub fn concrete(ty: &str) -> Self {
        GenericType::Concrete(ty.to_string())
    }

    pub fn generic(ty: &str) -> Self {
        GenericType::Generic(ty.to_string())
    }
}

impl Display for GenericType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            GenericType::Concrete(s) => write!(f, "{}", s),
            GenericType::Generic(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let g = Generics::none();

        assert_eq!("", g.to_string());
    }

    #[test]
    fn one() {
        let g = Generics::two(GenericType::generic("ID"), GenericType::generic("T"));

        assert_eq!("<ID, T>", g.to_string());
    }
}