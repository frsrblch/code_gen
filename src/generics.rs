use std::fmt::{Display, Formatter, Error, Debug};
use std::iter::FromIterator;
use crate::{StrConcat, Type};
use std::str::FromStr;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Generics(Vec<Type>);

impl Generics {
    pub fn none() -> Self { Default::default() }

    pub fn one(t: Type) -> Self {
        Generics(vec![t])
    }

    pub fn two(t: Type, u: Type) -> Self {
        Generics(vec![t, u])
    }

    pub fn push(&mut self, gen: Type) {
        self.0.push(gen);
    }
}

impl<'a> FromIterator<Type> for Generics {
    fn from_iter<T: IntoIterator<Item=Type>>(iter: T) -> Self {
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

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        println!("{}", input);

        if input == "" {
            return Ok(Generics::none())
        }

        if input.chars().nth(0) != Some('<')
            || input.chars().last() != Some('>')
        {
            return Err("Generics must be wrapped by '<>'".to_string());
        }

        let input = input
            .replace('<', "")
            .replace('>', "");

        let results: Vec<Result<Type, String>> = input.split(',')
            .map(|s| s.trim().parse::<Type>())
            .collect::<Vec<_>>();

        for r in results.iter() {
            println!("{:?}", r);
        }

        if let Some(err) = results.iter().find(|r| r.is_err()).cloned() {
            return Err(err.unwrap_err());
        }

        let types = results.into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        if types.is_empty() {
            return Err(format!("Input cannot be empty"));
        }

        Ok(Generics(types))
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
        let g = Generics::two("ID".parse().unwrap(), "T".parse().unwrap());

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
        assert_eq!("<ID, T>".parse::<Generics>().unwrap(), Generics::two("ID".parse().unwrap(), "T".parse().unwrap()));
    }
}