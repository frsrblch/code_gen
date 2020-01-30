use std::fmt::{Display, Formatter, Error};
use std::ops::{Range};
use std::str::FromStr;
use crate::*;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CamelCase(String);

impl CamelCase {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for CamelCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl FromStr for CamelCase {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.contains(' ') {
            return Err(format!("CamelCase cannot contain spaces: {}", value))
        }

        if value.contains('_') {
            return Err(format!("CamelCase cannot contain underscores: {}", value))
        }

        if value.is_empty() {
            return Err("CamelCase cannot be empty".to_string());
        }

        if value.chars().nth(0).unwrap().is_lowercase() {
            return Err(format!("CamelCase cannot start with lower case: {}", value));
        }

        Ok(CamelCase(value.to_string()))
    }
}



impl Into<TypeName> for CamelCase {
    fn into(self) -> TypeName {
        TypeName(self.0)
    }
}

impl Into<Type> for CamelCase {
    fn into(self) -> Type {
        Type {
            name: self.into(),
            types: Default::default()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SnakeCase(String);

impl SnakeCase {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for SnakeCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl FromStr for SnakeCase {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err("snake_case cannot be empty".to_string());
        }

        if value.chars().any(char::is_uppercase) {
            return Err(format!("snake_case cannot contain upper case: {}", value));
        }

        if value.contains(' ') {
            return Err(format!("snake_case cannot contain spaces: {}", value));
        }

        if value.contains("__") {
            return Err(format!("snake_case cannot contain double underscores: {}", value));
        }

        Ok(SnakeCase(value.to_string()))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ScreamingSnakeCase(String);

impl ScreamingSnakeCase {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for ScreamingSnakeCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl FromStr for ScreamingSnakeCase {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.chars().any(char::is_lowercase) {
            return Err(format!("SCREAMING_SNAKE_CASE cannot contain lowercase: {}", value));
        }

        if value.contains(' ') {
            return Err(format!("SCREAMING_SNAKE_CASE cannot contain spaces: {}", value));
        }

        if value.contains("__") {
            return Err(format!("SCREAMING_SNAKE_CASE cannot contain double underscores: {}", value));
        }

        Ok(ScreamingSnakeCase(value.to_string()))
    }
}

impl Into<SnakeCase> for CamelCase {
    fn into(self) -> SnakeCase {
        let mut output = String::new();

        for (i, c) in self.0.chars().enumerate() {
            if c.is_ascii_uppercase() && i != 0 {
                output.push('_');
            }

            output.push(c.to_ascii_lowercase());
        }

        SnakeCase(output)
    }
}

impl Into<SnakeCase> for ScreamingSnakeCase {
    fn into(self) -> SnakeCase {
        SnakeCase::from_str(self.0.to_lowercase().as_str()).unwrap()
    }
}

impl Into<ScreamingSnakeCase> for SnakeCase {
    fn into(self) -> ScreamingSnakeCase {
        ScreamingSnakeCase::from_str(self.0.to_uppercase().as_str()).unwrap()
    }
}

impl Into<CamelCase> for SnakeCase {
    fn into(self) -> CamelCase {
        let mut output = String::new();

        self.0.split('_')
            .for_each(|word| {
                for (i, c) in word.chars().enumerate() {
                    if i == 0 {
                        output.push(c.to_uppercase().nth(0).unwrap());
                    } else {
                        output.push(c);
                    }
                }
            });

        CamelCase::from_str(output.as_str()).unwrap()
    }
}

impl Into<CamelCase> for ScreamingSnakeCase {
    fn into(self) -> CamelCase {
        let snake: SnakeCase = self.into();
        snake.into()
    }
}

impl Into<ScreamingSnakeCase> for CamelCase {
    fn into(self) -> ScreamingSnakeCase {
        let snake: SnakeCase = self.into();
        snake.into()
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Indent(pub u8);

impl Indent {
    pub fn new(indent: u8) -> Self {
        Self(indent)
    }

    fn get_range(self) -> Range<u8> {
        Range { start: 0, end: self.0 }
    }
}

impl Display for Indent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for _ in self.get_range().into_iter() {
            write!(f, "    ").ok();
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StrConcat<'a, I: IntoIterator<Item=T> + Clone + 'a, T: Display> {
    pub iter: I,
    pub left_bound: &'a str,
    pub right_bound: &'a str,
    pub item_prepend: &'a str,
    pub item_append: &'a str,
    pub join: &'a str,
}

impl<'a, I: IntoIterator<Item=T> + Clone + 'a, T: Display> Display for StrConcat<'a, I, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.left_bound).ok();

        self.iter.clone().into_iter().enumerate().for_each(|(i, s)| {
            if i != 0 {
                write!(f, "{}", self.join).ok();
            }
            write!(f, "{}{}{}", self.item_prepend, s, self.item_append).ok();
        });

        write!(f, "{}", self.right_bound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camel_case_spaces_returns_error() {
        assert!(CamelCase::from_str("Nope Wont").is_err())
    }

    #[test]
    fn camel_case_underscore_returns_error() {
        assert!(CamelCase::from_str("Nope_Wont").is_err())
    }

    #[test]
    fn camel_case_empty_returns_error() {
        assert!(CamelCase::from_str("").is_err());
    }

    #[test]
    fn camel_case_starts_lower_case_returns_error() {
        assert!(CamelCase::from_str("nope").is_err());
    }

    #[test]
    fn camel_case_valid_input_returns_okay() {
        assert!(CamelCase::from_str("ValidInput").is_ok());
    }

    #[test]
    fn snake_case_cannot_be_empty() {
        assert!(SnakeCase::from_str("").is_err())
    }

    #[test]
    fn snake_case_cannot_contain_uppercase() {
        assert!(SnakeCase::from_str("upperCase").is_err());
    }

    #[test]
    fn snake_case_cannot_contain_spaces() {
        assert!(SnakeCase::from_str("invalid case").is_err());
    }

    #[test]
    fn snake_case_cannot_contain_double_underscores() {
        assert!(SnakeCase::from_str("invalid__case").is_err());
    }

    #[test]
    fn snake_case_valid_input() {
        assert!(SnakeCase::from_str("valid_input").is_ok());
    }

    #[test]
    fn screaming_snake_case_cannot_contain_lowercase() {
        assert!(ScreamingSnakeCase::from_str("NOPE_wont").is_err());
    }

    #[test]
    fn screaming_snake_case_cannot_contain_spaces() {
        assert!(ScreamingSnakeCase::from_str("NOPE WONT").is_err());
    }

    #[test]
    fn screaming_snake_case_cannot_contain_double_underscores() {
        assert!(ScreamingSnakeCase::from_str("NOPE__WONT").is_err());
    }

    #[test]
    fn screaming_snake_valid_input() {
        assert!(ScreamingSnakeCase::from_str("VALID_INPUT").is_ok());
    }

    #[test]
    fn to_snake_case() {
        let snake = SnakeCase::from_str("body_orbit").unwrap();
        let camel = CamelCase::from_str("BodyOrbit").unwrap();
        let screaming = ScreamingSnakeCase::from_str("BODY_ORBIT").unwrap();

        assert_eq!(snake, camel.into());
        assert_eq!(snake, screaming.into());
    }

    #[test]
    fn to_screaming_snake_case() {
        let snake = SnakeCase::from_str("body_orbit").unwrap();
        let camel = CamelCase::from_str("BodyOrbit").unwrap();
        let screaming = ScreamingSnakeCase::from_str("BODY_ORBIT").unwrap();

        assert_eq!(screaming, camel.into());
        assert_eq!(screaming, snake.into());
    }

    #[test]
    fn to_camel_case() {
        let snake = SnakeCase::from_str("body_orbit").unwrap();
        let camel = CamelCase::from_str("BodyOrbit").unwrap();
        let screaming = ScreamingSnakeCase::from_str("BODY_ORBIT").unwrap();

        assert_eq!(camel, snake.into());
        assert_eq!(camel, screaming.into());
    }

    #[test]
    fn indent_zero_doesnt_indent() {
        assert_eq!("", Indent(0).to_string());
    }

    #[test]
    fn indent_one_has_four_spaces() {
        assert_eq!("    ", Indent(1).to_string());
    }

    #[test]
    fn str_concat_for_unique_references() {
        let items = vec!["A", "B", "C"];

        let concat = StrConcat {
            iter: items,
            left_bound: "(",
            right_bound: ")",
            item_prepend: "&mut ",
            item_append: "-1",
            join: ", "
        };

        assert_eq!("(&mut A-1, &mut B-1, &mut C-1)", concat.to_string());
    }
}