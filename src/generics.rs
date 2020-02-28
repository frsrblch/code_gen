use std::fmt::{Display, Formatter, Error, Debug};
use std::iter::FromIterator;
use crate::{StrConcat, Type};
use std::str::FromStr;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Generics(Vec<Generic>);

impl Generics {
    pub fn none() -> Self { Default::default() }

    pub fn one(t: &str) -> Self {
        Generics(vec![Generic::from_str(t).unwrap()])
    }

    pub fn two(t: &str, u: &str) -> Self {
        Generics(vec![Generic::from_str(t).unwrap(), Generic::from_str(u).unwrap()])
    }

    pub fn push(&mut self, gen: &str) {
        self.0.push(Generic::from_str(gen).unwrap());
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> FromIterator<Generic> for Generics {
    fn from_iter<T: IntoIterator<Item=Generic>>(iter: T) -> Self {
        Generics(iter.into_iter().collect())
    }
}

impl Display for Generics {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.0.is_empty() {
            return Ok(());
        }

        write!(f, "{}",
            StrConcat {
                iter: &self.0,
                left_bound: "<",
                right_bound: ">",
                item_prepend: "",
                item_append: "",
                join: ", "
            }
        )
    }
}

impl FromStr for Generics {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == "" {
            return Ok(Generics::none())
        }

        if !input.starts_with('<') || !input.ends_with('>') {
            return Err("Generics must be wrapped by '<>'".to_string());
        }

        let input = &input[1..input.len()-1];

        input.split(',')
            .map(|s| s.trim().parse::<Generic>())
            .collect::<Result<Vec<_>,_>>()
            .and_then(|types| {
                if types.is_empty() {
                    Err("Input cannot be empty".to_string())
                } else {
                    Ok(Generics::from_iter(types))
                }
            })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Generic {
    Type(Type),
    Lifetime(Lifetime),
}

impl Display for Generic {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Generic::Type(t) => write!(f, "{}", t),
            Generic::Lifetime(l) => write!(f, "{}", l),
        }
    }
}

impl FromStr for Generic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Type::from_str(s).map(Generic::from)
            .or_else(|_| Lifetime::from_str(s).map(Generic::from))
            .map_err(|_| format!("Could not parse as Type or Lifetime: {}", s))
    }
}

impl From<Type> for Generic {
    fn from(t: Type) -> Self {
        Generic::Type(t)
    }
}

impl From<Lifetime> for Generic {
    fn from(l: Lifetime) -> Self {
        Generic::Lifetime(l)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Lifetime(String);

impl Lifetime {
    pub fn new(lifetime: &str) -> Self {
        Lifetime(String::from(lifetime))
    }
}

impl Display for Lifetime {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "'{}", self.0)
    }
}

impl FromStr for Lifetime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some('\'') = s.chars().nth(0) {
            Ok(Lifetime(s.chars().skip(1).collect()))
        } else {
            Err(format!("Lifetime values must begin with an apostrophe: {}", s))
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
        let g = Generics::two("ID", "T");

        assert_eq!("<ID, T>", g.to_string());
    }

    #[test]
    fn from_str_none() {
        assert_eq!("".parse::<Generics>().unwrap(), Generics::none());
    }

    #[test]
    fn from_str_without_brackets_returns_err() {
        assert!("<A,B".parse::<Generics>().is_err());
        assert!("A,B>".parse::<Generics>().is_err());
    }

    #[test]
    fn from_str_empty_entries_returns_err() {
        assert!("<A,>".parse::<Generics>().is_err());
    }

    #[test]
    fn from_str_brackets_with_no_entries_returns_err() {
        assert!("<>".parse::<Generics>().is_err());
    }

    #[test]
    fn from_str_input_cannot_have_spaces_within_entries() {
        assert!("<A, B C>".parse::<Generics>().is_err());
    }

    #[test]
    fn from_str_valid_input_returns_ok() {
        assert_eq!("<ID, T>".parse::<Generics>().unwrap(), Generics::two("ID", "T"));
    }

    #[test]
    fn generic_lifetime_from_str() {
        assert_eq!(Generic::Lifetime(Lifetime::new("a")), Generic::from_str("'a").unwrap());
    }

    #[test]
    fn generic_type_from_str() {
        assert_eq!(Generic::Type(Type::new("u32")), Generic::from_str("u32").unwrap());
    }

    #[test]
    fn with_lifetime() {
        assert_eq!("<'a, u32>".parse::<Generics>().unwrap(), Generics::two("'a", "u32"));
    }
}