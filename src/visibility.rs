use crate::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Visibility {
    Pub,
    PubCrate,
    Private,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Pub
    }
}

impl Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Visibility::Private => Ok(()),
            Visibility::Pub => write!(f, "pub "),
            Visibility::PubCrate => write!(f, "pub (crate) "),
        }
    }
}
