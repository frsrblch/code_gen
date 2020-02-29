use crate::*;
use std::fmt::{Display, Result};

#[derive(Debug, Clone)]
pub struct EnumType {
    pub base: Enum,
    pub enum_impl: Option<Impl>,
    pub enum_traits: Vec<TraitImpl>,
}

impl Display for EnumType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{}", self.base).ok();

        if let Some(i) = &self.enum_impl {
            writeln!(f, "{}", i).ok();
        }

        for i in &self.enum_traits {
            writeln!(f, "{}", i).ok();
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Enum {
    pub typ: Type,
    pub visibility: Visibility,
    pub derives: Derives,
    pub options: Vec<EnumOption>
}

impl Enum {
    pub fn new(name: &str) -> Self {
        Enum {
            typ: Type::new(name),
            visibility: Visibility::Pub,
            derives: Default::default(),
            options: vec![],
        }
    }

    pub fn with_derives(mut self, derives: Derives) -> Self {
        self.derives = derives;
        self
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn add_option(mut self, option: EnumOption) -> Self {
        self.options.push(option);
        self
    }
}

impl Display for Enum {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.options.is_empty() {
            writeln!(f, "{}{}enum {} {{}}", self.derives, self.visibility, self.typ)
        } else {
            writeln!(f, "{}{}enum {} {{", self.derives, self.visibility, self.typ).ok();
            for opt in self.options.iter() {
                write!(f, "    {}", opt).ok();
            }
            writeln!(f, "}}")
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EnumOption {
    pub name: CamelCase,
    pub option_types: Vec<String>,
}

impl EnumOption {
    pub fn new(name: &str, option_types: Vec<&str>) -> Self {
        let option_types = option_types
            .into_iter()
            .map(String::from)
            .collect();

        EnumOption {
            name: name.parse().expect(&format!("EnumOption: name must be CamelCase: {}", name)),
            option_types
        }
    }
}

impl Display for EnumOption {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.option_types.is_empty() {
            writeln!(f, "{},", self.name)
        } else {
            let str = StrConcat {
                iter: self.option_types.iter(),
                left_bound: "",
                right_bound: "",
                item_prepend: "",
                item_append: "",
                join: ", "
            };
            writeln!(f, "{}({}),", self.name, str)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_enum() {
        let empty = Enum::new("Empty");

        let expected = "pub enum Empty {}\n";

        assert_eq!(expected, empty.to_string());
    }

    #[test]
    fn empty_enum_with_debug() {
        let empty = Enum::new("Empty")
            .with_derives(Derives::with_debug());

        let expected = "#[derive(Debug)]\npub enum Empty {}\n";

        assert_eq!(expected, empty.to_string());
    }

    #[test]
    fn singular_enum_with_debug() {
        let single = Enum::new("Single")
            .with_derives(Derives::with_debug())
            .add_option(EnumOption::new("A", vec!["u32", "char"]));

        let expected = "#[derive(Debug)]\npub enum Single {\n    A(u32, char),\n}\n";

        assert_eq!(expected, single.to_string());
    }

    #[test]
    fn complicated_example() {
        let my_enum = Enum::new("Complicated")
            .with_derives(Derives::with_debug_default_clone())
            .add_option(EnumOption::new("First", vec!["Vec<u32>"]))
            .add_option(EnumOption::new("Second", vec!["u32", "(f32, f32)"]));

        let expected =
r#"#[derive(Debug, Default, Clone)]
pub enum Complicated {
    First(Vec<u32>),
    Second(u32, (f32, f32)),
}
"#;
        assert_eq!(expected, my_enum.to_string());
    }

    #[test]
    fn option_with_no_values() {
        let option = Enum::new("Option")
            .add_option(EnumOption::new("Some", vec!["u32"]))
            .add_option(EnumOption::new("None", vec![]));

        let expected =
r#"pub enum Option {
    Some(u32),
    None,
}
"#;
        assert_eq!(expected, option.to_string());
    }

    #[test]
    fn enum_type() {
        let default = Trait::new("Default")
            .add_function_definition(TraitFunction::new("default")
                .with_return("Self"));

        let base = Enum::new("Test")
            .with_derives(Derives::with_debug_default_clone())
            .add_option(EnumOption::new("Number", vec!["u32"]));

        let default_impl = default.impl_for(&base.typ)
            .add_function(TraitFunction::new("default")
                .with_return("Self")
                .add_line(CodeLine::new(0, "Test::Number(0)")));

        let enum_type = EnumType {
            base: base.clone(),
            enum_impl: None,
            enum_traits: vec![default_impl],
        };

        let expected =
r#"#[derive(Debug, Default, Clone)]
pub enum Test {
    Number(u32),
}

impl Default for Test {
    fn default() -> Self {
        Test::Number(0)
    }
}

"#;

        assert_eq!(expected, enum_type.to_string());
    }
}