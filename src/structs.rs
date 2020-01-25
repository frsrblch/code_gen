use crate::*;

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub visibility: Visibility,
    pub fields: Fields,
}

impl Display for Struct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}struct {}",
            self.visibility,
            self.name,
        ).ok();
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
    pub name: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_none() {
        let s = Struct {
            name: "Test".to_string(),
            visibility: Visibility::Private,
            fields: Fields::None,
        };

        assert_eq!("struct Test;\n", s.to_string());
    }

    #[test]
    fn pub_struct_none() {
        let s = Struct {
            name: "Test".to_string(),
            visibility: Visibility::Pub,
            fields: Fields::None,
        };

        assert_eq!("pub struct Test;\n", s.to_string());
    }

    #[test]
    fn pub_crate_struct_none() {
        let s = Struct {
            name: "Test".to_string(),
            visibility: Visibility::PubCrate,
            fields: Fields::None,
        };

        assert_eq!("pub (crate) struct Test;\n", s.to_string());
    }

    #[test]
    fn tuple_struct() {
        let s = Struct {
            name: "Test".to_string(),
            visibility: Visibility::Pub,
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
            name: "Test".to_string(),
            visibility: Visibility::Pub,
            fields: Fields::Standard(vec![
                Field {
                    visibility: Visibility::Pub,
                    name: "field".to_string(),
                    field_type: "u32".to_string()
                },
            ])
        };

        assert_eq!("pub struct Test {\n    pub field: u32,\n}\n", s.to_string());
    }

    #[test]
    fn example() {
        let arena = Struct {
            name: "System".to_string(),
            visibility: Visibility::Pub,
            fields: Fields::Standard(vec![
                Field {
                    visibility: Visibility::Pub,
                    name: "name".to_string(),
                    field_type: "Component<Self, String>".to_string()
                },
                Field {
                    visibility: Visibility::Pub,
                    name: "position".to_string(),
                    field_type: "Component<Self, Position>".to_string()
                },
            ]),
        };

        assert_eq!(
            "pub struct System {\n    pub name: Component<Self, String>,\n    pub position: Component<Self, Position>,\n}\n",
            arena.to_string()
        );
    }
}