use std::fmt::{Display, Formatter, Error, Debug};
use std::iter::FromIterator;
use crate::StrConcat;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Generics(Vec<String>);

impl Generics {
    pub fn none() -> Self { Default::default() }

    pub fn one(t: &str) -> Self {
        Generics(vec![t.to_string()])
    }

    pub fn two(t: &str, u: &str) -> Self {
        Generics(vec![t.to_string(), u.to_string()])
    }

    pub fn push(&mut self, gen: String) {
        self.0.push(gen);
    }
}

impl<'a> FromIterator<String> for Generics {
    fn from_iter<T: IntoIterator<Item=String>>(iter: T) -> Self {
        Generics(iter.into_iter().collect())
    }
}

impl Display for Generics {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.0.len() == 0 {
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "" {
            return Ok(Generics::none())
        }

        if s.chars().nth(0) != Some('<')
            || s.chars().last() != Some('>')
        {
            return Err("Generics must be wrapped by '<>'".to_string());
        }

        let input = s
            .replace('<', "")
            .replace('>', "");

        let split: Vec<String> = input
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        if split.is_empty() {
            return Err(format!("Input cannot be empty"));
        }

        if split.iter().any(String::is_empty) {
            return Err(format!("Input cannot be empty"));
        }

        if let Some(s) = split.iter().find(|s| s.contains(' ')) {
            return Err(format!("Input cannot contain spaces: {}", s));
        }

        Ok(Generics(split))
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
}