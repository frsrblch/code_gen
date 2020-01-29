use crate::{CamelCase, Generics};
use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TypeName(CamelCase);

impl FromStr for TypeName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(|s| TypeName(s))
    }
}

impl Display for TypeName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Type {
    pub name: TypeName,
    pub types: Generics,
}

impl FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find('<') {
            None => s.parse()
                .map(|name| Type { name, types: Generics::none() }),
            Some(i) => {
                let name = s[0..i].parse()?;
                let types = s[i..].parse()?;
                Ok(Type { name, types })
            },
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}{}", self.name, self.types)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_test() {
        let ty: Type = "Test".parse().unwrap();

        assert_eq!(ty, Type { name: "Test".parse().unwrap(), types: Generics::none() })
    }

    #[test]
    pub fn parse_test_with_generic() {
        let ty: Type = "Test<ID, T>".parse().unwrap();

        assert_eq!(ty, Type { name: "Test".parse().unwrap(), types: Generics::two("ID", "T") })
    }
}
