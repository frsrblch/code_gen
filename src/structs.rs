use crate::*;
use crate::formatting::{CamelCase, SnakeCase};

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: CamelCase,
    pub visibility: Visibility,
    pub derives: Derives,
    pub fields: Fields,
}

impl Display for Struct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.derives).ok();

        write!(f, "{vis}struct {name}", vis = self.visibility, name = self.name).ok();

        match &self.fields {
            Fields::None => {
                writeln!(f, ";")
            },
            Fields::Tuple(fields) => {
                write!(f, "(").ok();

                for (i, field) in fields.iter().enumerate() {
                    match i {
                        0 => write!(f, "{}", field).ok(),
                        _ => write!(f, ", {}", field).ok(),
                    };
                }

                writeln!(f, ");")
            },
            Fields::Standard(fields) => {
                writeln!(f, " {ob}", ob='{').ok();

                for field in fields {
                    writeln!(f, "    {}", field).ok();
                }

                writeln!(f, "{cb}", cb='}')
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Fields {
    None,
    Tuple(Vec<AnonField>),
    Standard(Vec<Field>),
}

#[derive(Debug, Clone)]
pub struct AnonField {
    pub visibility: Visibility,
    pub field_type: String,
}

impl Display for AnonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f,
               "{}{}",
               self.visibility,
               self.field_type,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub visibility: Visibility,
    pub name: SnakeCase,
    pub field_type: String,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f,
            "{}{}: {},",
            self.visibility,
            self.name,
            self.field_type,
        )
    }
}

impl Into<AnonField> for Field {
    fn into(self) -> AnonField {
        AnonField {
            visibility: self.visibility,
            field_type: self.field_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn struct_none() {
        let s = Struct {
            name: "Test".try_into().unwrap(),
            visibility: Visibility::Private,
            derives: Derives::new(),
            fields: Fields::None,
        };

        assert_eq!("struct Test;\n", s.to_string());
    }

    #[test]
    fn pub_struct_none() {
        let s = Struct {
            name: "Test".try_into().unwrap(),
            visibility: Visibility::Pub,
            derives: Derives::new(),
            fields: Fields::None,
        };

        assert_eq!("pub struct Test;\n", s.to_string());
    }

    #[test]
    fn pub_crate_struct_none() {
        let s = Struct {
            name: "Test".try_into().unwrap(),
            visibility: Visibility::PubCrate,
            derives: Derives::new(),
            fields: Fields::None,
        };

        assert_eq!("pub (crate) struct Test;\n", s.to_string());
    }

    #[test]
    fn tuple_struct() {
        let s = Struct {
            name: "Test".try_into().unwrap(),
            visibility: Visibility::Pub,
            derives: Derives::new(),
            fields: Fields::Tuple(vec![
                AnonField {
                    visibility: Visibility::Pub,
                    field_type: "u32".to_string(),
                },
                AnonField {
                    visibility: Visibility::Private,
                    field_type: "u8".to_string(),
                }
            ])
        };

        assert_eq!("pub struct Test(pub u32, u8);\n", s.to_string());
    }

    #[test]
    fn field_struct() {
        let s = Struct {
            name: "Test".try_into().unwrap(),
            visibility: Visibility::Pub,
            derives: Derives::new(),
            fields: Fields::Standard(vec![
                Field {
                    visibility: Visibility::Pub,
                    name: "field".try_into().unwrap(),
                    field_type: "u32".to_string()
                },
            ])
        };

        assert_eq!("pub struct Test {\n    pub field: u32,\n}\n", s.to_string());
    }

    #[test]
    fn example() {
        let arena = Struct {
            name: "System".try_into().unwrap(),
            visibility: Visibility::Pub,
            derives: Derives::new(),
            fields: Fields::Standard(vec![
                Field {
                    visibility: Visibility::Pub,
                    name: "name".try_into().unwrap(),
                    field_type: "Component<Self, String>".to_string()
                },
                Field {
                    visibility: Visibility::Pub,
                    name: "position".try_into().unwrap(),
                    field_type: "Component<Self, Position>".to_string()
                },
            ]),
        };

        assert_eq!(
            "pub struct System {\n    pub name: Component<Self, String>,\n    pub position: Component<Self, Position>,\n}\n",
            arena.to_string()
        );
    }

    #[test]
    fn struct_with_derives() {
        let s = Struct {
            name: "Test".try_into().unwrap(),
            visibility: Visibility::Pub,
            derives: Derives::with_debug_default(),
            fields: Fields::None
        };

        assert_eq!("#[derive(Debug, Default)]\npub struct Test;\n", s.to_string());
    }
}