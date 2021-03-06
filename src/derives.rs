use std::collections::HashSet;
use std::fmt::{Display, Formatter, Error};
use std::iter::FromIterator;
use crate::{StrConcat, TraitName};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Derives(HashSet<Derive>);

impl Derives {
    pub fn new() -> Self {
        Derives::default()
    }

    pub fn with_debug() -> Self {
        let mut derives = Self::default();
        derives.insert(Derive::Debug);
        derives
    }

    pub fn with_debug_default() -> Self {
        let mut derives = Self::default();
        derives.insert(Derive::Debug);
        derives.insert(Derive::Default);
        derives
    }

    pub fn with_debug_clone() -> Self {
        let mut derives = Self::default();
        derives.insert(Derive::Debug);
        derives.insert(Derive::Clone);
        derives
    }

    pub fn with_debug_default_clone() -> Self {
        let mut derives = Self::default();
        derives.insert(Derive::Debug);
        derives.insert(Derive::Default);
        derives.insert(Derive::Clone);
        derives
    }

    pub fn insert(&mut self, derive: Derive) {
        match derive {
            Derive::Ord => {
                self.insert(Derive::PartialOrd);
            },
            Derive::Eq => {
                self.insert(Derive::PartialEq);
            },
            Derive::Hash => {
                self.insert(Derive::Eq);
            },
            Derive::Copy => {
                self.insert(Derive::Clone);
            },
            _ => {},
        }

        self.0.insert(derive);
    }

    fn get_sorted_derive_vec(&self) -> Vec<Derive> {
        let mut derives: Vec<Derive> = self.0.iter().cloned().collect();
        derives.sort();
        derives
    }

    fn get_str_concat<'a>(&self) -> impl Display + 'a {
        StrConcat {
            iter: self.get_sorted_derive_vec(),
            left_bound: "#[derive(",
            right_bound: ")]",
            item_prepend: "",
            item_append: "",
            join: ", "
        }
    }
}

impl FromIterator<Derive> for Derives {
    fn from_iter<T: IntoIterator<Item=Derive>>(iter: T) -> Self {
        let mut derives = Self::default();

        for d in iter {
            derives.insert(d);
        }

        derives
    }
}

impl Display for Derives {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.0.is_empty() {
            return Ok(());
        }

        writeln!(f, "{}", self.get_str_concat())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Derive {
    Debug,
    Default,
    Copy,
    Clone,
    Eq, PartialEq,
    Ord, PartialOrd,
    Hash,
    Custom(TraitName),
}

impl Display for Derive {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Derive::Debug => "Debug",
                Derive::Default => "Default",
                Derive::Copy => "Copy",
                Derive::Clone => "Clone",
                Derive::Eq => "Eq",
                Derive::PartialEq => "PartialEq",
                Derive::Ord => "Ord",
                Derive::PartialOrd => "PartialOrd",
                Derive::Hash => "Hash",
                Derive::Custom(derive) => derive.as_str(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_test() {
        assert_eq!(
            "#[derive(Debug, Default, Clone)]\n",
            Derives::with_debug_default_clone().to_string()
        );
    }

    #[test]
    fn empty_displays_nothing() {
        let derives = Derives::new();
        assert_eq!("", derives.to_string());
    }

    #[test]
    fn hash_implies_eq_and_partial_eq() {
        let mut hash = Derives::new();
        hash.insert(Derive::Hash);

        assert!(hash.0.contains(&Derive::Hash));
        assert!(hash.0.contains(&Derive::Eq));
        assert!(hash.0.contains(&Derive::PartialEq));
    }
}